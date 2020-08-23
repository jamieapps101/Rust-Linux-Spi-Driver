use nix;
use libc;
use std::path::PathBuf;
use std::ffi::{CString, CStr};

// Constants and macro definitions for IOCTL functions
const SPI_IOC_MAGIC: u8 = b'k'; // Defined in linux/spi/spidev.h
const SPI_IOC_TYPE_MODE: u8 = 1;
const SPI_IOC_TYPE_BITS_PER_WORD: u8 = 3;
const SPI_IOC_TYPE_MAX_SPEED: u8 = 4;

nix::ioctl_write_int!(spi_write_mode, SPI_IOC_MAGIC, SPI_IOC_TYPE_MODE);
// generates func:  spi_write_mode(fp, data_p) -> result
nix::ioctl_read!(spi_read_mode, SPI_IOC_MAGIC, SPI_IOC_TYPE_MODE, u8);
// generates func:  spi_read_mode(fp, data_p)

nix::ioctl_write_int!(spi_write_bits_per_word, SPI_IOC_MAGIC, SPI_IOC_TYPE_BITS_PER_WORD);
nix::ioctl_read!(spi_read_bits_per_word, SPI_IOC_MAGIC, SPI_IOC_TYPE_BITS_PER_WORD, u8);

nix::ioctl_write_int!(spi_write_max_speed, SPI_IOC_MAGIC, SPI_IOC_TYPE_MAX_SPEED);
nix::ioctl_read!(spi_read_max_speed, SPI_IOC_MAGIC, SPI_IOC_TYPE_MAX_SPEED, u8);


// Series of unsafe IOCTL functions, to be wrapped up in public functions below
// each function should return a bool, based on checks done on value written
unsafe fn set_spi_mode(path: PathBuf, mode: u8) -> bool {
    // sort out file pointers
    let path_string : &str = path.to_str().unwrap();
    let dev_path: *const libc::c_char =  CString::new(path_string).unwrap().into_raw();
    let fp_mode: libc::c_int =  114; // ascii 'r', to open file in read mode
    let fp = libc::open(dev_path, fp_mode);
    
    // write mode
    let _result = spi_write_mode(fp, mode.into());

    // read mode back
    let mut data : Vec<u8> = vec![0];
    let mut data_p = data.as_mut_ptr();
    // if let Ok(val) = spi_read_mode(fp, data_p) {
        // return Some(val);
    // } else {
        // return None;
    // }
    
    false
}

unsafe fn set_spi_bits_per_word(path: PathBuf, bits: u8) -> bool {
    false
}

unsafe fn set_spi_max_speed(path: PathBuf, speed: u32)  -> bool {
    false
}


// Public functions
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
        println!("A");
        let dev_path: *const libc::c_char =  CString::new("/dev/spidev0.0").unwrap().into_raw();
        println!("B");
        let fp_mode: libc::c_int =  114;
        println!("C");
        let fp = libc::open(dev_path, fp_mode);
        println!("D");
        let mut data : Vec<u8> = vec![0;20];
        let mut data_p = data.as_mut_ptr();
        
        println!("E");
        if let Ok(val) = spi_read_mode(fp, data_p) {
            println!("val: {}", val);
            println!("data: {:?}", data);
            return Some(val);
        } else {
            println!("F");
            return None;
        }
    }

    #[test]
    fn test_nix() -> Result<(),String> {
        let dev_path = PathBuf::from("/dev/spidev0.0");
        if dev_path.exists() {
            if let Some(val) = unsafe { get_data() } {
                println!("val: {}", val);
                return Ok(());
            } else {
                return Err(String::from("Didn't get a val"));
            }
        } else {
            return Err(format!("dev_path ({:?}) didn't exist, cannot test", dev_path));
        }
    }

    
}

