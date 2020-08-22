mod spi;
use spi::SPI_Config;

pub enum EinkError {
    NotImplemented,
}

// struct representing a "sheet" of e paper
pub struct Sheet {

}

pub fn new(
    spi_dev: &str, 
    spi_channel: usize, 
    spi_setup: Option<SPI_Config>,
    height: usize,
    width:usize) -> Result<Sheet,EinkError> {
    // create internal buffer to represent the sheet
    Err(EinkError::NotImplemented)
}

#[derive(Clone,Copy,Debug)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Sheet {
    pub fn set_pxl(&mut self) {

    }

    pub fn get_dimensions(&self) -> Coord {
        Coord {
            x: 0,
            y: 0,
        }
    }

    // render text supplied onto internal buffer
    pub fn write_text(&mut self, text: &str, pos: Coord) {

    }

    pub fn push_buffer(&mut self) -> Result<(), EinkError> {
        Err(EinkError::NotImplemented)
    }
}
