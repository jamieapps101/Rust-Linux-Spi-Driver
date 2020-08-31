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
    fn set_mode(       device: *const c_char, encoded_mode: u8) -> u8;
    fn set_speed(      device: *const c_char, speed: u32) -> u8;
    fn transfer_8_bit( device: *const c_char,
                            tx: *const u8, tx_words: u32, 
                            rx: *mut u8, 
                            // uint32_t *rx_words, 
                            delay_us: u16 , 
                            speed_hz: u32, 
                            bits: u8 
                            ) -> u8;
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
    path_cstr_ptr: Option<*const i8>,
}

#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum BusError {
    DevicePathNotFound,
    NotImplemented,
    CouldNotConvertPathToCStr,
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

        let op_result = unsafe {
            transfer_8_bit( self.path_cstr_ptr.unwrap(),
                tx_data.as_ptr(), tx_data.len() as u32,
                return_vec.as_mut_ptr(), 
                self.delay_us, 
                self.speed_hz, 
                self.bits.into())
        };

        match op_result {
            0 => Ok(return_vec),
            1 => Err(BusError::CouldNotSendMessage),
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

}