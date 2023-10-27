#[derive(Debug)]
pub enum MemoryError {
    LoadFail,
    StoreFail,
}

/// Memory interface. It exposes and only has two functions, `load` and `store`, which
/// behave as expected. In the case that an address exceeds the allocated space, a MemoryError
/// is thrown according to what kind of operation performed the invalid access.
#[derive(Debug)]
pub struct Memory {
    mem: Vec<u32>,
}

impl Memory {

    // TODO: Decide what I want to use as the memory data structure. My preference is for a 2D
    // vector, since that makes word-sized rows easier to visualize and work with. 
    // I'll have to see if that introduces any notable overhead vs. a single array access. 

    /// Creates a Memory struct with `size` bytes of addressable memory.
    pub fn new(size: usize) -> Self {
        Self {
            mem: vec![0u32; size]
        }
    }

    pub fn len(&self) -> usize {
        return self.mem.len();
    }

    /// Loads the data stored at `addr` or dies trying
    pub fn load(&self, address: usize) -> Result<u32, MemoryError>{
        if address > self.mem.len() {
            return Err(MemoryError::LoadFail)
        }

        Ok(self.mem[address])
    }

    /// Stores the provided data at the specified address or dies trying
    pub fn store(&mut self, address: usize, data: u32) -> Result<(), MemoryError> {
        if address > self.mem.len() {
            return Err(MemoryError::StoreFail)
        }

        self.mem[address] = data;
        Ok(())
    }

    /// Debugging function used to print out the struct's members. This will be deprecated and
    /// replaced with `to_string()` and an API to access the contents of memory for GUI purposes 
    pub fn print(&self) {
        println!("This memory object has size {}", self.mem.len());
    }
}
