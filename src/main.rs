use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use rand::distributions::{Distribution, Uniform};
use rand::{rngs::SmallRng, SeedableRng};

use packcdc::PackCDC;
use colored::*;

/// Generate a random buffer of given size using a seeded RNG.
fn generate_buffer(size: usize) -> Vec<u8> {
    let mut rng = SmallRng::seed_from_u64(1);
    Uniform::new_inclusive(u8::MIN, u8::MAX)
        .sample_iter(&mut rng)
        .take(size)
        .collect()
}

/// Naive anchor-counting function for comparison purposes.
fn pack_count_anchors(buffer: &[u8], anchors_offsets: &mut Vec<usize>) {
    let buffer_len = buffer.len();
    let window_bytes_len = 48;
    let mask_value = 0x00008A3114583280u64;

    let mut hash = 0;
    let slice = &buffer[0..window_bytes_len];
    for item in slice.iter() {
        hash = (hash << 1) ^ *item as u64;
    }

    let slice = &buffer[window_bytes_len..buffer_len];
    for (pos, item) in slice.iter().enumerate() {
        if (hash & mask_value) == 0 {
            anchors_offsets.push(pos + window_bytes_len);
        }
        hash = (hash << 1) ^ *item as u64;
    }
}

/// Print a slice after sleeping for the given seconds (to simulate delay).
fn print_slice(slice: Vec<u8>, sleep_secs: u64) {
    thread::sleep(Duration::from_secs(sleep_secs));
    println!("{}", format!("Slice = {:?}", slice).blue());
}

fn main() {
    println!("{}", "=== Generating Buffer ===".green().bold());
    let buffer = Arc::new(Mutex::new(generate_buffer(100_000)));

    println!("{}", "=== PackCDC Initialization ===".green().bold());
    let locked = buffer.lock().unwrap();
    let packcdc = PackCDC::new(&locked);
    let anchors_count = packcdc.count();
    drop(locked); // unlock buffer before reusing

    println!("{}", format!("Pack found {} anchors", anchors_count).yellow());

    println!("{}", "=== Anchoring ===".green().bold());
    let mut anchors_offsets = Vec::with_capacity(buffer.lock().unwrap().len() / 25_000);

    let locked = buffer.lock().unwrap();
    let start_time = Instant::now();
    pack_count_anchors(&locked, &mut anchors_offsets);
    let elapsed_micros = start_time.elapsed().as_micros() as f64;
    let speed_gbps = locked.len() as f64 / elapsed_micros / 1000.0 * 8.0;

    println!(
        "{}",
        format!(
            "Processed (slices) {} MB in {} us = {:.3} Gbps",
            locked.len() / 1_000_000,
            elapsed_micros,
            speed_gbps
        )
        .cyan()
    );

    let avg_chunk = if anchors_offsets.is_empty() {
        0
    } else {
        locked.len() / anchors_offsets.len()
    };

    println!(
        "{}",
        format!(
            "Anchors count {} with average size {} KB",
            anchors_offsets.len(),
            avg_chunk / 1000
        )
        .cyan()
    );
    drop(locked); // unlock again

    println!("{}", "=== Printing Slices ===".green().bold());

    let slice1 = buffer.lock().unwrap()[0..3].to_vec();
    let slice2 = buffer.lock().unwrap()[4..8].to_vec();

    let buffer_clone = Arc::clone(&buffer);
    thread::spawn(move || {
        print_slice(slice1, 3);
    });

    thread::spawn(move || {
        print_slice(slice2, 2);
    });

    buffer_clone.lock().unwrap().push(10);

    let slice3 = buffer_clone.lock().unwrap()[10..11].to_vec();
    thread::spawn(move || {
        print_slice(slice3, 0);
    })
    .join()
    .unwrap();

    thread::sleep(Duration::from_secs(4));
}
