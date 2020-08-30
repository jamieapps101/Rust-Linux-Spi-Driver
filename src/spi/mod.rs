use nix;
use libc;
use std::path::PathBuf;
use std::ffi::CString;

// testing
const CS_TEST_VALUE: u32 = 100;

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
    let data_p = data.as_mut_ptr();
    if let Ok(val) = spi_read_mode(fp, data_p) {
        // return Some(val);
        if (val as u8) == mode {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

unsafe fn set_spi_bits_per_word(path: PathBuf, bits: u8) -> bool {
        // sort out file pointers
        let path_string : &str = path.to_str().unwrap();
        let dev_path: *const libc::c_char =  CString::new(path_string).unwrap().into_raw();
        let fp_mode: libc::c_int =  114; // ascii 'r', to open file in read mode
        let fp = libc::open(dev_path, fp_mode);
        
        // write mode
        let _result = spi_write_bits_per_word(fp, bits.into());
    
        // read mode back
        let mut data : Vec<u8> = vec![0];
        let data_p = data.as_mut_ptr();
        if let Ok(val) = spi_read_bits_per_word(fp, data_p) {
            // return Some(val);
            if (val as u8) == bits {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
}

unsafe fn set_spi_max_speed(path: PathBuf, speed: u32)  -> bool {
     // sort out file pointers
     let path_string : &str = path.to_str().unwrap();
     let dev_path: *const libc::c_char =  CString::new(path_string).unwrap().into_raw();
     let fp_mode: libc::c_int =  114; // ascii 'r', to open file in read mode
     let fp = libc::open(dev_path, fp_mode);
     
     // write mode
     let _result = spi_write_max_speed(fp, speed.into());
 
     // read mode back
     let mut data : Vec<u8> = vec![0];
     let data_p = data.as_mut_ptr();
     if let Ok(val) = spi_read_max_speed(fp, data_p) {
         // return Some(val);
         if (val as u32) == speed {
             return true;
         } else {
             return false;
         }
     } else {
         return false;
     }
}


// Public functions
struct SPIDev {
    bus: SPIBus,
}

pub struct SPIBus {
    config: SPIBusConfig
}

pub struct SPIBusConfig {
    spi_dev_path: PathBuf,
    spi_mode: SPIMode,
    bits_per_word: BitsPerWord,
    bus_clock_speed: ClockSpeed,
}

pub enum SPIMode {
    SpiMode0,
    SpiMode1,
    SpiMode2,
    SpiMode3,
}

pub enum BitsPerWord {
    EightBits,
    NineBits,
}

#[allow(non_camel_case_types)]
pub enum ClockSpeed {
    cs_125_MHz,
    cs_62_5_MHz,
    cs_31_2_MHz,
    cs_15_6_MHz,
    cs_7_8_MHz,
    cs_3_9_MHz,
    cs_1953_kHz,
    cs_976_kHz,
    cs_488_kHz,
    cs_244_kHz,
    cs_122_kHz,
    cs_61_kHz,
    cs_30_5_kHz,
    cs_15_2_kHz,
    cs_7629_Hz,
    TEST,
}

// strict taken from spidev.h struct of same name, line 94
struct spi_ioc_transfer {
    tx_buf: u64,
    rx_buf: u64,
    len: u32,
    speed_hz: u32,
    delay_usecs: u16,
    bits_per_word: u8,
    cs_change: u8,
    tx_nbits: u8,
    rx_nbits: u8,
    word_delay_usecs: u8,
    pad: u8,
}

impl spi_ioc_transfer {
    fn new(
        tx_buf: Option<u64>,
        rx_buf: Option<u64>,
        len: Option<u32>,
        speed_hz: Option<u32>,
        delay_usecs: Option<u16>,
        bits_per_word: Option<u8>,
        cs_change: Option<u8>,
        tx_nbits: Option<u8>,
        rx_nbits: Option<u8>,
        word_delay_usecs: Option<u8>,
        pad: Option<u8>
    ) -> spi_ioc_transfer {
        spi_ioc_transfer {
            tx_buf:             if let Some(val) = tx_buf           {val} else {0},
            rx_buf:             if let Some(val) = rx_buf           {val} else {0},
            len:                if let Some(val) = len              {val} else {0},
            speed_hz:           if let Some(val) = speed_hz         {val} else {0},
            delay_usecs:        if let Some(val) = delay_usecs      {val} else {0},
            bits_per_word:      if let Some(val) = bits_per_word    {val} else {0},
            cs_change:          if let Some(val) = cs_change        {val} else {0},
            tx_nbits:           if let Some(val) = tx_nbits         {val} else {0},
            rx_nbits:           if let Some(val) = rx_nbits         {val} else {0},
            word_delay_usecs:   if let Some(val) = word_delay_usecs {val} else {0},
            pad:                if let Some(val) = pad              {val} else {0},
        }
    }
} 

impl SPIBus {
    pub fn new(setup: SPIBusConfig) -> SPIBus {
        // this simply creates the struct and calls the reconfig method
        // to do the ioctl operations
        let mut return_val = SPIBus {
            config: setup,
        };
        return_val.reconfigure(setup);
        return return_val;
    }

    pub fn reconfigure(&mut self, setup: SPIBusConfig) -> Result<(),SPIError> {
        // this function performs the ioctl operations to
        // setup physical spi bus
        self.config = setup;
        let spi_mode_int : u8 = match self.config.spi_mode {
            SPIMode::SpiMode0 => 0,
            SPIMode::SpiMode1 => 1,
            SPIMode::SpiMode2 => 2,
            SPIMode::SpiMode3 => 3,
        };
        if !(unsafe {set_spi_mode(self.config.spi_dev_path, spi_mode_int)}) {
            // if this returns false, then it didn't work
            return Err(SPIError::CouldNotSetMode);
        }
        let spi_bpw_int : u8 = match self.config.bits_per_word {
            BitsPerWord::EightBits => 8,
            BitsPerWord::NineBits => 9,
        };
        if !(unsafe {set_spi_bits_per_word(self.config.spi_dev_path, spi_bpw_int)}) {
            // if this returns false, then it didn't work
            return Err(SPIError::CouldNotSetBitsPerWord);
        }
        let spi_speed_int : u32 = match self.config.bus_clock_speed {
            ClockSpeed::cs_125_MHz => 0,
            ClockSpeed::cs_62_5_MHz => 0,
            ClockSpeed::cs_31_2_MHz => 0,
            ClockSpeed::cs_15_6_MHz => 0,
            ClockSpeed::cs_7_8_MHz => 0,
            ClockSpeed::cs_3_9_MHz => 0,
            ClockSpeed::cs_1953_kHz => 0,
            ClockSpeed::cs_976_kHz => 0,
            ClockSpeed::cs_488_kHz => 0,
            ClockSpeed::cs_244_kHz => 0,
            ClockSpeed::cs_122_kHz => 0,
            ClockSpeed::cs_61_kHz => 0,
            ClockSpeed::cs_30_5_kHz => 0,
            ClockSpeed::cs_15_2_kHz => 0,
            ClockSpeed::cs_7629_Hz => 0,
            ClockSpeed::TEST => CS_TEST_VALUE,
        };
        if !(unsafe {set_spi_max_speed(self.config.spi_dev_path, spi_speed_int)}) {
            // if this returns false, then it didn't work
            return Err(SPIError::CouldNotSetMaxSpeed);
        }

        Ok(())
    }

    pub fn write(data: &Vec<u8>) -> bool {
        false
    }

    pub fn read(data: &mut Vec<u8>) -> bool {
        false
    }
}


#[derive(Clone,Copy,Debug)]
enum SPIError {
    CouldNotSetMode,
    CouldNotSetBitsPerWord,
    CouldNotSetMaxSpeed,
    NotImplemented,
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
        let data_p = data.as_mut_ptr();
        
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

