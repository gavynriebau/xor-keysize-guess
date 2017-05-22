
extern crate clap;
extern crate xor_keysize_guess;

#[macro_use] extern crate log;
extern crate env_logger;

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
    let max_keysize = max_keysize_str.parse::<usize>().unwrap();
    let key_to_distance_dict = xor_keysize_guess::avg_normalized_hamming_distance(&file_contents, max_keysize);

    for (keysize, distance) in key_to_distance_dict {
        println!("{:4.3} average normalized hamming distance for keysize {}", distance, keysize);
    }
}



