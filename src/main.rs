#![feature(array_methods)]

use std::{ thread, sync::mpsc, time::Instant, io::{ self, Write } };

fn main() {
    let mut hash = String::new();
    print!("Input the hash: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut hash).unwrap();

    let time = Instant::now();
    let thread_count = num_cpus::get() as u32;
    let hash_bytes = hex::decode(hash.trim_end()).unwrap();
    let (tx, rx) = mpsc::channel::<[u8; 4]>();

    println!("Spawning threads");

    for thread_number in 0..thread_count as u32 {
        let hash_bytes = hash_bytes.clone();
        let tx = tx.clone();

        thread::spawn(move || {
            let mut progress_report = 0;
            let mut ip: u32 = thread_number;
            loop {
                let ip_bytes: [u8; 4] = [
                    ((ip >> 24) & 0xFF) as u8,
                    ((ip >> 16) & 0xFF) as u8,
                    ((ip >> 8) & 0xFF) as u8,
                    (ip & 0xFF) as u8,
                ];

                if progress_report >= 1000000 * thread_count {
                    progress_report = 0;
                    println!("THREAD {} => {}/255", thread_number, ip_bytes[0]);
                }
                progress_report += 1;

                if md5::compute(ip_bytes).as_slice() == hash_bytes.as_slice() {
                    tx.send(ip_bytes).unwrap();
                    break;
                };

                if ip + thread_count < ip {
                    break;
                }

                ip += thread_count;
            }
        });
    }

    drop(tx);

    match rx.recv() {
        Ok(ip) => println!("Found {}.{}.{}.{} in {:.2}s", ip[0], ip[1], ip[2], ip[3], time.elapsed().as_secs_f32()),
        Err(_) => println!("Nothing was found in {:.2}s", time.elapsed().as_secs_f32()),
    };
}
