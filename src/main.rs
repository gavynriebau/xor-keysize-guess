
extern crate hamming;
extern crate clap;

use hamming::distance;
use clap::{App, Arg};
use std::fs::File;
use std::io::{Read};

fn main() {

    let mut matches = App::new("xor-keysize-guess")
        .version("1.0.0")
        .author("Gavyn Riebau")
        .about("Guesses the most likely keysize used to XOR encrypt a given file.")
        .arg(Arg::with_name("INPUT")
             .short("i")
             .long("input")
             .takes_value(true)
             .help("The input file for which the XOR encryption keysize will be guessed")
             .required(true))
        .get_matches();

    let file_path = matches.value_of("INPUT").unwrap();
    let mut file = File::open(file_path).unwrap();
    let mut file_contents : Vec<u8> = Vec::new();

    let _ = file.read_to_end(&mut file_contents);

    //println!("Input file path: {}", file_path);

    for keysize in (1..40) {
        //println!("Keysize '{}'", keysize);

        let mut chunks = file_contents.chunks(keysize);
        let mut chunk_count = 0;
        let mut average_hamming_dist = 0.0_f32;

        for idx in (1..3) {
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

            //println!("{:4.3} is the normalized hamming distance for keysize {} and block {}", normalized_hamming, keysize, idx);
            chunk_count += 1;
        }
        average_hamming_dist = average_hamming_dist / chunk_count as f32;
        println!("{:4.3} average hamming distance for keysize {}", average_hamming_dist, keysize);


        /*
        loop {
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

            chunk_count += 1;
        }
        average_hamming_dist = average_hamming_dist / (chunk_count as f32);
        println!("{:4.3} average hamming distance for keysize {}", average_hamming_dist, keysize);
        */
    }



}
