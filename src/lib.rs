
extern crate hamming;

#[macro_use] extern crate log;
extern crate env_logger;

use hamming::distance;
use std::collections::HashMap;

pub fn avg_normalized_hamming_distance(file_contents : &Vec<u8>, max_keysize : usize) -> HashMap<usize, f32> {

    let mut keysize_to_avg_hamming_dist = HashMap::new();

    for keysize in 1..(max_keysize+1) {

        let mut chunks = file_contents.chunks(keysize);
        let mut num_chunks_compared = 0;
        let mut average_hamming_dist = 0.0_f32;

        // Calculate the mean normalized hamming distance over a
        // number of samples to try to improve accuracy.
        for idx in 1..3 {

            let left_chunk = chunks.next();
            let right_chunk = chunks.next();

            if left_chunk.is_none() {
                break;
            }
            if right_chunk.is_none() {
                break;
            }

            let left = left_chunk.unwrap();
            let right = right_chunk.unwrap();

            if left.len() != right.len() {
                break;
            }

            let hamming_dist = distance(left, right);
            let normalized_hamming = hamming_dist as f32 / keysize as f32;
            average_hamming_dist += normalized_hamming;

            debug!("{:4.3} is the normalized hamming distance for keysize {} and block {}", normalized_hamming, keysize, idx);

            num_chunks_compared += 1;
        }

        if num_chunks_compared != 0 {
            average_hamming_dist = average_hamming_dist / num_chunks_compared as f32;
            keysize_to_avg_hamming_dist.insert(keysize, average_hamming_dist);
        } else {
            debug!("Not enough data in input file to check a keysize of '{}'", keysize);
        }

    }

    keysize_to_avg_hamming_dist
}
