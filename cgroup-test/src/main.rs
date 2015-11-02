use std::thread;

fn waste_memory() -> Vec<u8> {
    const SIZE: usize = 512 * 1024 * 1024; // 512MB
    let mut buffer: Vec<u8> = Vec::with_capacity(SIZE);
    println!("Allocated {} bytes", SIZE);

    unsafe {
        buffer.set_len(SIZE);
    }

    for i in &mut buffer {
        *i = 1;
    }
    println!("Committed writes to {} bytes", SIZE);

    buffer
}

fn calculate_prime() {
    const MAX_PRIME: u64 = 200000; 
    let mut primes: u64 = 0;

    for num in 2..MAX_PRIME {

        let mut i: u64 = 2;
        while num % i != 0 {
            i += 1;
        }

        if i == num {
            primes += 1;
        }


    }
    println!("Calculated {} prime numbers", primes);
}

fn main() {    
    const NUM_OF_THREADS: i64 = 8;
    let mut threads = Vec::new();
    
    for _ in 0..NUM_OF_THREADS {
        threads.push(thread::spawn(move || {
            let buffer = waste_memory();

            calculate_prime();

            for i in buffer {
                assert!(i == 1);
            }
        }));
    }

    for t in threads {
        let _ = t.join();
    }
}
