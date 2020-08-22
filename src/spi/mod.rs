use nix;
use libc;
use std::ffi::{CString, CStr};

struct SPIDev {
    bus: SPIBus,
}

struct SPIBus {

}

#[derive(Clone,Copy,Debug)]
pub struct SPI_Config {
    // use for setting spu parameters, eg speed phase
    
}

impl SPIBus {
    pub fn write(data: &u8) -> bool {
        false
    }
}

#[derive(Clone,Copy,Debug)]
enum SPIError {
    NotImplemented,
}


fn new_bus(spi_dev: &str) -> Result<SPIBus, SPIError> {
    Err(SPIError::NotImplemented)
}


#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn init_bus_correct() -> Result<(),SPIError> {
        if let Err(err) = new_bus("/dev/spidev0.0") {
            return Err(err);
        } 
        return Ok(());
    }
    #[test]
    fn init_bus_incorrect() -> Result<(),String> {
        if let Ok(inst) = new_bus("/dev/spidev3.0") {
            return Err(String::from("Should not be able to instantiate this"));
        } 
        return Ok(());
    }

    const SPI_IOC_MAGIC: u8 = b'k'; // Defined in linux/spi/spidev.h
    const SPI_IOC_TYPE_MODE: u8 = 1;
    nix::ioctl_read!(spi_read_mode, SPI_IOC_MAGIC, SPI_IOC_TYPE_MODE, u8);

    unsafe fn get_data() -> Option<i32> {
        let dev_path: *const libc::c_char =  CString::new("/dev/spidev3.0").unwrap().into_raw();
        let fp_mode: libc::c_int =  114;
        let fp = libc::open(dev_path, fp_mode);
        let mut data : Vec<u8> = vec![0;20];
        let mut data_p = data.as_mut_ptr();

        if let Ok(val) = spi_read_mode(fp, data_p) {
            return Some(val);
        } else {
            return None;
        }
    }

    #[test]
    fn test_nix() -> Result<(),String> {
        let dev_path = PathBuf::from("/dev/spidev3.0");
        if dev_path.exists() {
            if let Some(val) = unsafe { get_data() } {
                println!("vaL: {}", val);
                return Ok(());
            } else {
                return Err(String::from("Didn't get a val"));
            }
        } else {
            return Err(String::from("dev_path didn't exist, cannot test"));
        }
    }

    
}

