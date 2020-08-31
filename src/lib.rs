use std::{
    path::PathBuf,
    convert::{From,Into},
    ffi::CStr,
};

extern crate libc;
use libc::{c_char};

#[allow(dead_code)]
extern "C" {
    fn set_mode(          device: *const c_char, encoded_mode: u8) -> u8;
    fn set_speed(         device: *const c_char, speed: u32) -> u8;
    fn transfer_8_bit(    device: *const c_char,
                            tx: *const u8, tx_words: u32, 
                            rx: *mut u8, 
                            rx_words: u32, 
                            delay_us: u16 , 
                            speed_hz: u32, 
                            bits: u8 
                            ) -> u8;
    fn set_bits_per_word( device: *const c_char, bits: u8) -> u8;
    
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
    spi_mode: SpiMode,
    cs_mode: CsMode,
    bit_order: BitOrder,
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
    dev_path: PathBuf,
    delay_us: u16 , 
    speed_hz: u32, 
    bits: WordLength, 
    path_cstr_ptr: Option<*const u8>,
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum BusError {
    DevicePathNotFound,
    NotImplemented,
    CouldNotConvertPathToCStr,
    CouldNotOpenFile,
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

        let temp = path_string_with_null_ptr.clone();
        let op_result : u8 = unsafe {
            set_mode(temp, encoded_mode)
        };
        // todo assert this is correct result
        match op_result {
            0 => {/* do nothing, this is correct result */},
            1 => return Err(BusError::CouldNotSetMode),
            2 => return Err(BusError::CouldNotGetMode),
            _ => unreachable!(),
        }
        
        return Ok(SpiBus {
            dev_path,
            delay_us,
            speed_hz,
            bits,
            path_cstr_ptr: Some(path_string_with_null_ptr),
        });
    }

    pub fn transaction(&self, tx_data: Vec<u8>, max_rx_words: Option<u32>) -> Result<Vec<u8>, BusError> {
        let mut return_vec: Vec<u8> = vec![0; tx_data.len()];
        let path_string_with_null: String = self.dev_path.clone().into_os_string().into_string().unwrap()+"\0";
        let dev_path_cstr = CStr::from_bytes_with_nul(path_string_with_null.as_str().as_bytes()).unwrap().as_ptr();
        
        let max_rx_words_val: u32 = match max_rx_words {
            Some(val) => val,
            None => 0,
        };

        let op_result : u8 = unsafe {
            transfer_8_bit( 
                // self.path_cstr_ptr.unwrap(),
                dev_path_cstr,
                tx_data.as_ptr(), tx_data.len() as u32,
                return_vec.as_mut_ptr(), max_rx_words_val,
                self.delay_us, 
                self.speed_hz, 
                self.bits.into())
        };

        println!("transfer_8_bit op_result: {:?}", op_result);

        match op_result {
            0 => Ok(return_vec),
            1 => Err(BusError::CouldNotSendMessage),
            2 => Err(BusError::CouldNotOpenFile),
            _ => unreachable!(),
        }
    }
}

impl Write<&str> for SpiBus {
    fn write(&self, data: &str)  -> Result<(),BusError> {
        return Err(BusError::NotImplemented)
    }
}

impl Write<Vec<u8>> for SpiBus {
    fn write(&self, tx_data: Vec<u8>)  -> Result<(),BusError> {
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

        match spi_dev.transaction(data, None) {
            Ok(_) => Ok(()),
            Err(reason) => {
                Err(format!("I errored bc: {:?}", reason))
            }
        }
    }
}