
extern crate hamming;
extern crate clap;

#[macro_use] extern crate log;
extern crate env_logger;

use hamming::distance;
use clap::{App, Arg};
use std::fs::File;
use std::io::{Read};

fn main() {
    env_logger::init().unwrap();

    let matches = App::new("xor-keysize-guess")
        .version("1.0.0")
        .author("Gavyn Riebau")
        .about("Guesses the most likely keysize used to XOR encrypt a given file.")
        .arg(Arg::with_name("INPUT")
             .short("i")
             .long("input")
             .takes_value(true)
             .help("The input file for which the XOR encryption keysize will be guessed")
             .required(true))
        .arg(Arg::with_name("MAX_KEYSIZE")
             .short("k")
             .long("keysize")
             .takes_value(true)
             .help("The maximum key size to guess")
             .default_value("40"))
        .get_matches();

    let file_path = matches.value_of("INPUT").unwrap();
    let mut file = File::open(file_path).unwrap();
    let mut file_contents : Vec<u8> = Vec::new();

    let _ = file.read_to_end(&mut file_contents);

    debug!("Input file path: {}", file_path);

    let max_keysize_str = matches.value_of("MAX_KEYSIZE").unwrap();
    let mut max_keysize = max_keysize_str.parse::<usize>().unwrap();
    max_keysize += 1;

    for keysize in 1..max_keysize {

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
            println!("{:4.3} average hamming distance for keysize {}", average_hamming_dist, keysize);
        } else {
            debug!("Not enough data in input file to check a keysize of '{}'", keysize);
        }

    }
}
