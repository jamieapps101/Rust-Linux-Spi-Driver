use std::{
    path::PathBuf,
    boxed::Box,
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
                            // uint32_t *rx_words, 
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
    CouldNotSendMessage,
}

trait Write<T> {
    fn write(&self, data: T) -> Result<(),BusError>;
}

#[allow(dead_code)]
impl SpiBus {
    pub fn new(bus_id: &str, delay_us: u16 , 
        speed_hz: u32, bits: WordLength) -> Result<SpiBus, BusError> {
        let dev_path = PathBuf::from(bus_id);
        // check this is ok
        if !dev_path.exists() { return Err(BusError::DevicePathNotFound);}

        let path_string_with_null: String = dev_path.clone().into_os_string().into_string().unwrap()+"\0";
        let temp = CStr::from_bytes_with_nul(path_string_with_null.as_str().as_bytes()).unwrap().as_ptr();
        
        
        // perform setup hardcoded fornow
        let op_result : u8 = unsafe {
            set_mode(temp.clone(), 16) // this is hard coded to work
        };

        println!("op_result: {:?}", op_result);
        
        let op_result : u8 = unsafe {
            set_speed(temp.clone(), 500000) // this works!
        };
        println!("op_result: {:?}", op_result);
        
        let op_result : u8 = unsafe {
            set_bits_per_word(temp.clone(), 8)
        };
        println!("op_result: {:?}", op_result);
        
        return Ok(SpiBus {
            dev_path,
            delay_us,
            speed_hz,
            bits,
            path_cstr_ptr: Some(temp),
        });
    }


    fn test_set_speed(&self) -> Result<(), BusError> {
        let path_string_with_null: String = self.dev_path.clone().into_os_string().into_string().unwrap()+"\0";
        let dev_path_cstr = CStr::from_bytes_with_nul(path_string_with_null.as_str().as_bytes()).unwrap().as_ptr();
        let op_result = unsafe {
            set_speed(dev_path_cstr, 1000)
        };
        match op_result {
            0 => Ok(()),
            1 => Err(BusError::CouldNotSetMaxSpeed),
            2 => Err(BusError::CouldNotGetMaxSpeed),
            _ => unreachable!(),
        } 
    }

    pub fn transaction(&self, tx_data: Vec<u8>, max_rx_words: Option<u32>) -> Result<Vec<u8>, BusError> {
        let mut return_vec: Vec<u8> = vec![0; tx_data.len()];
        let path_string_with_null: String = self.dev_path.clone().into_os_string().into_string().unwrap()+"\0";
        let dev_path_cstr = CStr::from_bytes_with_nul(path_string_with_null.as_str().as_bytes()).unwrap().as_ptr();
        
        let op_result : u8 = unsafe {
            transfer_8_bit( dev_path_cstr,
                tx_data.as_ptr(), tx_data.len() as u32,
                return_vec.as_mut_ptr(), 
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
        if let Err(val) = SpiBus::new(
                "/dev/spi_bus",
                0, 0, WordLength::EightBit,
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
    fn test_set_speed_function() -> Result<(), String> {
        let spi_dev : SpiBus;
        if let Ok(dev) = SpiBus::new("/dev/spidev0.0", 0, 0, WordLength::EightBit) {
            spi_dev = dev;
        } else {
            return Err("could not get dev".to_string());
        }
        match spi_dev.test_set_speed() {
            Ok(_) => Ok(()),
            Err(reason) => {
                Err(format!("I errored bc: {:?}", reason))
            }
        }
    }

    #[test]
    fn test_transfer_send() -> Result<(), String> {
        let spi_dev : SpiBus;
        if let Ok(dev) = SpiBus::new("/dev/spidev0.0", 0, 500000, WordLength::EightBit) {
            spi_dev = dev;
        } else {
            return Err("could not get dev".to_string());
        }

        let data: Vec<u8> = vec![0,1,2,3,4,5];

        match spi_dev.transaction(data, None) {
            Ok(_) => Ok(()),
            Err(reason) => {
                Err(format!("I errored bc: {:?}", reason))
            }
        }
    }
}