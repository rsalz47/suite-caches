use crate::memory;

#[derive(Debug)]
pub enum CacheError {
    LoadFail,
    StoreFail,
}

/// Cache interface. It exposes load and store functions in the same manner as memory.rs
/// as well as detailed println output of current cache status for debugging purposes

pub struct Cache {
    mem: memory::Memory,
    size: usize,
}

impl Cache {
    pub fn new(size: usize) -> Self {
        Self {
          mem: memory::Memory::new(size),
          size
        }
    }

    pub fn load(&self, address: usize) -> Result<u32, CacheError> {
        let load_result = self.mem.load(address);

        if load_result.is_err() {
            return Err(CacheError::LoadFail);
        }

        return Ok(load_result.unwrap());
    }
    
    pub fn store(&mut self, address: usize, data: u32) -> Result<(), CacheError> {
        let store_result = self.mem.store(address, data);

        if store_result.is_err() {
            return Err(CacheError::StoreFail);
        }

        return Ok(());
    }
}
