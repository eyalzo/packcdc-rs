use std::thread;
use std::time::{Duration, Instant};
use rand::distributions::{Distribution, Uniform};
use rand::{rngs::SmallRng, SeedableRng};
use packcdc::PackCDC;

fn generate_buffer(size: usize) -> Vec<u8> {
    let mut rng = SmallRng::seed_from_u64(1);
    let buffer: Vec<u8> = Uniform::new_inclusive(u8::MIN, u8::MAX)
        .sample_iter(&mut rng).take(size).collect();
    buffer
}

fn pack_count_anchors(buffer: &mut Vec<u8>, anchors_offsets: &mut Vec<usize>) {
    let buffer_len = buffer.len();
    let window_bytes_len = 48;
    // Mask for 15 bits (32KB) where 7 bits from right and left are not in
    let mask_value = 0x00008A3114583280u64;

    //
    // For loop
    //

    // Warm-up phase
    let mut hash = 0;
    let slice = &buffer[0..window_bytes_len];
    for item in slice.iter() {
        hash = (hash << 1) ^ *item as u64;
    }

    // Anchors
    let slice = &buffer[window_bytes_len..buffer_len];
    for (pos, item) in slice.iter().enumerate() {
        // Check for anchor
        if (hash & mask_value) == 0 {
            anchors_offsets.push(pos);
        }

        // Next hash
        hash = (hash << 1) ^ *item as u64;
    }
}

fn print_slice(slice: &&[u8], sleep_secs: u64) {
    thread::sleep(Duration::from_secs(sleep_secs));
    println!("slice = {:?}", slice);
}

fn main() {
    static mut BUFFER: Vec<u8> = vec![];

    unsafe {
        println!("Generating buffer...");
        BUFFER.append(&mut generate_buffer(100000));

        let packcdc = PackCDC::new(BUFFER.as_slice());
        let anchors_count = packcdc.count();
        println!("Pack found {} anchors", anchors_count);

        //
        // Anchoring
        //
        let mut anchors_offsets = Vec::with_capacity(BUFFER.len() / 25000);
        let start_time = Instant::now();
        pack_count_anchors(&mut BUFFER, &mut anchors_offsets);
        let elapsed_micros = start_time.elapsed().as_micros() as f64;
        let speed_gbps = BUFFER.len() as f64 / elapsed_micros / 1000.0 * 8.0;
        println!("Processed (slices) {} MB in {} us = {:.3} Gbps", BUFFER.len() / 1000000,
                 elapsed_micros, speed_gbps);
        let avg_chunk = if anchors_offsets.len() < 1 { 0 } else { BUFFER.len() / anchors_offsets.len() };
        println!("   Anchors count {} with average size {} KB", anchors_offsets.len(), avg_chunk / 1000);

        let slice1 = &BUFFER[0..3];
        let slice2 = &BUFFER[4..8];

        thread::spawn(move || {
            print_slice(&slice1, 3);
        });

        thread::spawn(move || {
            print_slice(&slice2, 2);
        });

        BUFFER.push(10);

        let slice3 = &BUFFER[10..11];
        thread::spawn(move || {
            print_slice(&slice3, 0);
        }).join().unwrap();

        thread::sleep(Duration::from_secs(4));
    }
}
