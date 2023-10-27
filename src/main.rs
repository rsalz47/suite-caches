use std::env;
mod memory;
mod cache;

fn main() {
    println!("Hello world!");
    println!("Collecting command line arguments...");

    let args: Vec<String> = env::args().collect();

    println!("Command lines arguments are: {:?}", args);

    // Default value is 2**20, i.e. 1 MB
    // Since each entry in memory vector stores a 32 bit value,
    // 2**15 * 32 = 2**20

    let mut cache = cache::Cache::new(32768);

    println!("Storing a value of 10 at address 0 in the cache...");
    assert!(cache.store(0, 10).is_ok());


    println!("Loading the value stored at address 0 in the cache, should be 10...");
    assert!(cache.load(0).unwrap() == 10);

    println!("Storing a value of 20 in last address 32767...");
    assert!(cache.store(32767, 20).is_ok());

    println!("Loading the value stored at last address 32767, should be 20...");
    assert!(cache.load(32767).unwrap() == 20);


    println!("Store and load tests pass succesfully. Congratulations!");
}
