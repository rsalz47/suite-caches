mod memory;

fn main() {
    println!("Hello world!");

    let mut memory = memory::Memory::new(4096);
    memory.print();

    println!("Storing a value of 10 at address 0...");
    assert!(memory.store(0, 10).is_ok());

    println!("Loading the value stored at address 0...");
    assert!(memory.load(0).is_ok());

    println!("Store and load tests pass succesfully. Congratulations!");
}
