use std::{
    path::PathBuf,
    convert::{From,Into},
    ffi::CStr,
};

extern crate libc;
use libc::{c_char};

#[allow(dead_code)]
extern "C" {
    fn get_dev_fd(device: *const c_char, fd: *mut i32) -> u8;
    fn set_mode_on_fd(fd: i32, encoded_mode: u8) -> u8;
    fn transfer_8_bit_on_fd(fd: i32, 
        tx: *const u8, tx_words: u32, 
        rx: *mut u8, 
        rx_words: u32, 
        delay_us: u16 , 
        speed_hz: u32, 
        bits: u8 
    ) -> u8;    
    fn close_dev_fd(fd:*mut i32);
    fn transfer_8_bit_DC_on_fd(
        fd: i32, 
        gpio_dev: *const c_char,
        dc_line_no: u8,
        command_tx: *const u64,
        command_tx_words: u32,
        data_tx: *const u64,
        data_tx_words: u32,
        command_mode_active_high: bool,
        rx: *mut u64,
        rx_words: u32,
        delay_us: u16,
        speed_hz: u32,
        bits: u8
    ) -> u8;
}

#[derive(Copy, Clone)]
pub enum WordLength {
    EightBit,
    NineBit,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum SpiMode {
    SpiMode0,
    SpiMode1,
    SpiMode2,
    SpiMode3,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum CsMode {
    CsHigh,
    CsLow,
    NoCs,
}

#[derive(PartialEq)]
pub enum BitOrder {
    LSB,
    MSB,
}

pub struct SpiSetup {
    pub spi_mode: SpiMode,
    pub cs_mode: CsMode,
    pub bit_order: BitOrder,
}

impl From<u8> for WordLength {
    fn from(w: u8) -> WordLength {
        match w {
            8 => WordLength::EightBit,
            9 => WordLength::NineBit,
            _ => panic!("Invalid conversion to wordlength, value {}", w),
        }
    }
}

impl From<WordLength> for u8 {
    fn from(w: WordLength) -> u8 {
        match w {
            WordLength::EightBit => 8,
            WordLength::NineBit => 9,
        }
    }
}

pub struct SpiBus {
    delay_us: u16 , 
    speed_hz: u32, 
    bits: WordLength,
    c_fd: i32, 
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum BusError {
    DevicePathNotFound,
    NotImplemented,
    CouldNotConvertPathToCStr,
    CouldNotOpenFile,
    CouldNotGetFileDescriptor,
    CouldNotSetMaxSpeed,
    CouldNotGetMaxSpeed,
    CouldNotSetMode,
    CouldNotGetMode,
    CouldNotSendMessage,
}


trait Write<T> {
    fn write(&self, data: T) -> Result<(),BusError>;
}

#[allow(dead_code)]
impl SpiBus {
    pub fn new(bus_id: &str, delay_us: u16 , 
        speed_hz: u32, bits: WordLength, setup: SpiSetup) -> Result<SpiBus, BusError> {
        let dev_path = PathBuf::from(bus_id);
        // check this is ok
        if !dev_path.exists() { return Err(BusError::DevicePathNotFound);}
        
        let path_string_with_null: String = dev_path.clone().into_os_string().into_string().unwrap()+"\0";
        let path_string_with_null_ptr = CStr::from_bytes_with_nul(path_string_with_null.as_str().as_bytes()).unwrap().as_ptr();
        
        let mut c_fd: i32 = 0;
        let op_result = unsafe {
            get_dev_fd(path_string_with_null_ptr, &mut c_fd)
        };
        match op_result {
            0 => {}, // all ok, do nothing
            1 => return Err(BusError::CouldNotGetFileDescriptor),
            _ => unreachable!(),
        }

        // decode setup struct
        let mut encoded_mode : u8 = 0;
        if setup.bit_order == BitOrder::LSB {
            encoded_mode |= 1<<3;
        }

        if setup.cs_mode == CsMode::NoCs {
            encoded_mode |= 1<<6;
        } else {
            if setup.cs_mode == CsMode::CsHigh {
                encoded_mode |= 1<<4;
            }
        }
        
        match setup.spi_mode {
            SpiMode::SpiMode0 => {
                // do nothing for this one
            },
            SpiMode::SpiMode1 => {
                encoded_mode |= 1<<1;
            },
            SpiMode::SpiMode2 => {
                encoded_mode |= 1<<2;
            },
            SpiMode::SpiMode3 => {
                encoded_mode |= 1<<1;
                encoded_mode |= 1<<2;
            },
        }

        let op_result : u8 = unsafe {
            set_mode_on_fd(c_fd.clone(), encoded_mode)
        };

        match op_result {
            0 => {/* do nothing, this is correct result */},
            1 => return Err(BusError::CouldNotSetMode),
            2 => return Err(BusError::CouldNotGetMode),
            _ => unreachable!(),
        }
        
        return Ok(SpiBus {
            c_fd,
            delay_us,
            speed_hz,
            bits,
        });
    }

    pub fn transaction(&self, tx_data: Vec<u8>, max_rx_words: Option<u32>) -> Result<Vec<u8>, BusError> {
        let mut return_vec: Vec<u8> = vec![0; tx_data.len()];
        let max_rx_words_val: u32 = match max_rx_words {
            Some(val) => val,
            None => 0,
        };

        let op_result : u8 = unsafe {
            transfer_8_bit_on_fd(
                self.c_fd.clone(),
                tx_data.as_ptr(), tx_data.len() as u32,
                return_vec.as_mut_ptr(), max_rx_words_val,
                self.delay_us, 
                self.speed_hz, 
                self.bits.into())
        };

        match op_result {
            0 => Ok(return_vec),
            1 => Err(BusError::CouldNotSendMessage),
            2 => Err(BusError::CouldNotOpenFile),
            _ => unreachable!(),
        }
    }

    pub fn dc_transaction(&self, 
        tx_command: Vec<u64>, 
        tx_data: Vec<u64>, 
        max_rx_words: Option<u32>, 
        csdc_gpio_dev: &str,
        dc_gpio_line_no: u8,
        command_mode_active_high: bool,
    ) -> Result<Vec<u64>, BusError> {
        println!("c");
        let max_rx_words_val: u32 = match max_rx_words {
            Some(val) => val,
            None => 0,
        };
        println!("1");
        let mut return_vec: Vec<u64> = Vec::with_capacity((max_rx_words_val+1) as usize);
        for _ in 0..(max_rx_words_val+1) {
            println!("adding 1");
            return_vec.push(0);
        }
        // let mut return_vec: Vec<u64> = vec![0; max_rx_words_val as usize];
        println!("2");
        
        // let mut gpio_dev_path: String = csdc_gpio_dev.to_owned();
        // gpio_dev_path.push_str("\0");
        // let a: &CStr = CStr::from_bytes_with_nul(gpio_dev_path.as_bytes()).unwrap();
        let dev_path = PathBuf::from(csdc_gpio_dev);
        println!("3");
        let path_string_with_null: String = dev_path.clone().into_os_string().into_string().unwrap()+"\0";
        println!("4");
        let path_string_with_null_ptr = CStr::from_bytes_with_nul(path_string_with_null.as_str().as_bytes()).unwrap().as_ptr();

        println!("d");
        println!("tx_command: {:?}", tx_command);
        println!("tx_data: {:?}", tx_data);
        println!("return_vec: {:?}", return_vec);

        
        let op_result : u8 = unsafe {
            transfer_8_bit_DC_on_fd(
                self.c_fd.clone(),
                path_string_with_null_ptr,
                dc_gpio_line_no,
                tx_command.as_ptr(), 
                tx_command.len() as u32,
                tx_data.as_ptr(), 
                tx_data.len() as u32,
                command_mode_active_high,
                return_vec.as_mut_ptr(), 
                max_rx_words_val,
                self.delay_us, 
                self.speed_hz, 
                self.bits.into()
            )
        };
        println!("there was a result");

        println!("");
        
        if op_result==0 {
            Ok(return_vec)
        } else {
            panic!("I got {:?}", op_result);
        }
    }
}

// destructor trait, used to make sure fd is properly close on fd side
impl Drop for SpiBus {
    fn drop(&mut self) {
        println!("Closing fd");
       unsafe {
            close_dev_fd(&mut self.c_fd);
       };
    }
}

impl Write<&str> for SpiBus {
    fn write(&self, _data: &str)  -> Result<(),BusError> {
        return Err(BusError::NotImplemented)
    }
}

impl Write<Vec<u8>> for SpiBus {
    fn write(&self, _tx_data: Vec<u8>)  -> Result<(),BusError> {
        return Err(BusError::NotImplemented)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init_bus_false_path() -> Result<(),String> {
        let setup = SpiSetup {
            spi_mode: SpiMode::SpiMode0,
            cs_mode: CsMode::CsLow,
            bit_order: BitOrder::MSB,
        };

        if let Err(val) = SpiBus::new(
                "/dev/spi_bus",
                0, 0, WordLength::EightBit,setup
            ) {
            if val == BusError::DevicePathNotFound {
                Ok(())
            } else {
                return Err("wrong error given".to_string())
            }
        } else {
            return Err("\"found\" a non existant path".to_string());
        }
    }

    #[test]
    fn test_transfer_send() -> Result<(), String> {
        let setup = SpiSetup {
            spi_mode: SpiMode::SpiMode0,
            cs_mode: CsMode::CsHigh,
            bit_order: BitOrder::MSB,
        };
        
        let spi_dev : SpiBus;
        match SpiBus::new("/dev/spidev0.0", 0, 500000, WordLength::EightBit, setup) {
            Ok(dev) =>  {
                spi_dev = dev;
            }
            Err(reason) => {
                return Err(format!("could not get dev: {:?}", reason));
            }
        }

        let data: Vec<u8> = vec![0,0x55,2,0xff,128,0x69];

        match spi_dev.transaction(data.clone(), None) {
            Ok(_) => {},
            Err(reason) => {
                return Err(format!("I errored bc: {:?}", reason))
            }
        }

        match spi_dev.transaction(data.clone(), None) {
            Ok(_) => {},
            Err(reason) => {
                return Err(format!("I errored bc: {:?}", reason))
            }
        }
        return Ok(())
    }

    #[test]
    fn test_dc() -> Result<(), String> {
        let setup = SpiSetup {
            spi_mode: SpiMode::SpiMode0,
            cs_mode: CsMode::CsHigh,
            bit_order: BitOrder::MSB,
        };

        let spi_dev : SpiBus;
        match SpiBus::new("/dev/spidev0.0", 0, 500000, WordLength::EightBit, setup) {
            Ok(dev) =>  {
                spi_dev = dev;
            }
            Err(reason) => {
                return Err(format!("could not get dev: {:?}", reason));
            }
        }

        let command:  Vec<u64> = vec![170];
        let command2: Vec<u64> = vec![170];
        let data:  Vec<u64> = vec![0,0x55,2,0xff,128,0x69];
        let data2: Vec<u64> = vec![0,0x55,2,0xff,128,0x69];
        let command3:  Vec<u64> = vec![170];
        let command4: Vec<u64> = vec![];
        let data3:  Vec<u64> = vec![];
        let data4: Vec<u64> = vec![0,0x55,2,0xff,128,0x69];


        println!("test1");
        let result = spi_dev.dc_transaction( 
            command, 
            data, 
            None, 
            "gpiochip0",
            25,
            true,
        );
        match result {
            Ok(_) => {},
            Err(reason) => {return Err(format!("test1 I errored bc: {:?}", reason))},
        }
        println!("test2");
        let result = spi_dev.dc_transaction( 
            command2, 
            data2, 
            None, 
            "gpiochip0",
            25,
            true,
        );
        match result {
            Ok(_) => {},
            Err(reason) => {return Err(format!("test2 I errored bc: {:?}", reason))},
        }

        // send a zero data packet
        println!("test3");
        match spi_dev.dc_transaction( 
            command3, 
            data3, 
            None, 
            "gpiochip0",
            25,
            true,
        ) {
            Ok(_) => {},
            Err(reason) => {return Err(format!("test3 I errored bc: {:?}", reason))},
        }

        // send a zero command packet
        println!("test4");
        match spi_dev.dc_transaction( 
            command4, 
            data4, 
            None, 
            "gpiochip0",
            25,
            true,
        ) {
            Ok(_) => {},
            Err(reason) => {return Err(format!("test4 I errored bc: {:?}", reason))},
        }


        return Ok(())
    }
}