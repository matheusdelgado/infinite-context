use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io::Result;
use std::path::Path;
use zerocopy::AsBytes; // Mantido apenas o que está sendo usado

pub struct DiskPager {
    mmap: MmapMut,
    page_size_bytes: usize,
}

impl DiskPager {
    pub fn new<P: AsRef<Path>>(file_path: P, total_pages: usize, page_size_bytes: usize) -> Result<Self> {
        let total_size = total_pages * page_size_bytes;
        let file = OpenOptions::new().read(true).write(true).create(true).open(file_path)?;
        let file_len = file.metadata()?.len();
        if file_len < total_size as u64 {
            file.set_len(total_size as u64)?;
        }
        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self { mmap, page_size_bytes })
    }

    pub fn write_page(&mut self, page_id: u64, data: &[u32]) -> Result<()> {
        let offset = (page_id as usize) * self.page_size_bytes;
        let byte_slice = data.as_bytes();
        
        if offset + byte_slice.len() > self.mmap.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Out of bounds"));
        }
        
        self.mmap[offset..offset + byte_slice.len()].copy_from_slice(byte_slice);
        Ok(())
    }

    pub fn read_page(&self, page_id: u64, output: &mut [u32]) -> Result<()> {
        let offset = (page_id as usize) * self.page_size_bytes;
        let output_bytes = output.device_mut_bytes();
        
        if offset + output_bytes.len() > self.mmap.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Out of bounds"));
        }
        
        output_bytes.copy_from_slice(&self.mmap[offset..offset + output_bytes.len()]);
        Ok(())
    }
}

trait DeviceBytes {
    fn device_mut_bytes(&mut self) -> &mut [u8];
}

// CORRIGIDO: Removido o 'impl' extra aqui
impl DeviceBytes for [u32] {
    fn device_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut_ptr() as *mut u8,
                self.len() * std::mem::size_of::<u32>(),
            )
        }
    }
}