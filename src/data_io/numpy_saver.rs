use std::fs::File;
use std::io::Write;

use crate::data_io::Savable;

pub struct Numpy;

impl Savable<Numpy, std::io::Error> for Vec<f64> {
    fn save(&self, file_path : &str, _ : &Numpy) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path.to_owned() + ".npy")?;

        file.write_all(&[0x93, 'N' as u8, 'U' as u8, 'M' as u8, 'P' as u8, 'Y' as u8, 1u8, 0u8])?;
    
        let header_size : u16 = 118;
        file.write_all(&(header_size.to_le_bytes()))?;
    
        let header_string = String::from("{'descr': '<f8', 'fortran_order': False, 'shape': (") + &(self.len().to_string()) + ",), }";
        file.write_all(header_string.as_bytes())?;
    
        let remaining_header_length = header_size - header_string.as_bytes().len() as u16 - 1;
    
        if remaining_header_length > 0 {
            file.write_all(" ".repeat(remaining_header_length.into()).as_bytes())?;
            file.write_all(&[0x0A])?;
        }
    
        file.write_all(as_u8_slice(&self))?;
    
        Ok(())
    }
}

impl Savable<Numpy, std::io::Error> for Vec<Vec<f64>> {
    fn save(&self, file_path : &str, _ : &Numpy) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path.to_owned() + ".npy")?;

        file.write_all(&[0x93, 'N' as u8, 'U' as u8, 'M' as u8, 'P' as u8, 'Y' as u8, 1u8, 0u8])?;
    
        let header_size : u16 = 118;
        file.write_all(&(header_size.to_le_bytes()))?;
    
        let header_string = String::from("{'descr': '<f8', 'fortran_order': False, 'shape': (") + &(self.len().to_string()) + "," + &(self[0].len().to_string()) + "), }";
        file.write_all(header_string.as_bytes())?;
    
        let remaining_header_length = header_size - header_string.as_bytes().len() as u16 - 1;
    
        if remaining_header_length > 0 {
            file.write_all(" ".repeat(remaining_header_length.into()).as_bytes())?;
            file.write_all(&[0x0A])?;
        }
    
        for i in 0..=self.len()-1 {
            file.write_all(as_u8_slice(&self[i]))?;
        }
    
        Ok(())
    }
}

fn as_u8_slice<T>(v: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<T>(),
        )
    }
}