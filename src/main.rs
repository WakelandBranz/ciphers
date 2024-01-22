mod cipher;

use cipher::Cipher;
use std::env;

fn main() {

    // arg[0] is options, arg[1] is the first CLI arg
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
       panic!("Please input text to convert.");
    }

    let input: &String = &args[1];
    let ciphers: Cipher = Cipher::new(input);

    println!("Original: {}\nRot13: {}", ciphers.original, ciphers.rot13);
}
