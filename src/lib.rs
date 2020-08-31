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

pub enum WordLength {
    Eight_Bit,
    Nine_Bit,
}

impl From<u8> for WordLength {
    fn from(w: u8) -> WordLength {
        match w {
            8 => WordLength::Eight_Bit,
            9 => WordLength::Nine_Bit,
            _ => panic!("Invalid conversion to wordlength, value {}", w),
        }
    }
}

pub struct SpiBus {
    dev_path: PathBuf,
    delay_us: u16 , 
    speed_hz: u32, 
    bits: WordLength, 
}


#[derive(PartialEq, Debug)]
#[allow(dead_code)]
pub enum BusError {
    DevicePathNotFound,
    NotImplemented,
    CouldNotConvertPathToCStr,
    CouldNotSetMaxSpeed,
    CouldNotGetMaxSpeed,
}

trait Write<T> {
    fn write(&self, data: T) -> Result<(),BusError>;
}

trait Read<T> {
    fn read(&self) -> Result<T,BusError>;
}

#[allow(dead_code)]
impl SpiBus {
    pub fn new(bus_id: &str, delay_us: u16 , 
        speed_hz: u32, bits: WordLength) -> Result<SpiBus, BusError> {
        let dev_path = PathBuf::from(bus_id);
        // check this is ok
        if !dev_path.exists() { return Err(BusError::DevicePathNotFound);}
        return Ok(SpiBus {
            dev_path,
            delay_us,
            speed_hz,
            bits,
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
            _ => panic!("This should be unreachable"),
        } 
    }
}

impl Write<&str> for SpiBus {
    fn write(&self, data: &str)  -> Result<(),BusError> {
        return Err(BusError::NotImplemented)
    }
}

impl Write<Vec<u8>> for SpiBus {
    fn write(&self, data: Vec<u8>)  -> Result<(),BusError> {
        return Err(BusError::NotImplemented)
    }
}

impl Read<Box<str>> for SpiBus {
    fn read(&self) -> Result<Box<str>,BusError> {
        return Err(BusError::NotImplemented);
    }
}

impl Read<Vec<u8>> for SpiBus {
    fn read(&self) -> Result<Vec<u8>,BusError> {
        return Err(BusError::NotImplemented);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn init_bus_false_path() -> Result<(),String> {
        if let Err(val) = SpiBus::new(
                "/dev/spi_bus",
                0, 0, WordLength::Eight_Bit,
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
        if let Ok(dev) = SpiBus::new("/dev/spidev0.0", 0, 0, WordLength::Eight_Bit) {
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