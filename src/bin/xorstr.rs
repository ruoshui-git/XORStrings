use std::{fs::File, io::prelude::*, process};

use cyber_rust::xorstr::{xor_to_fmt_hex, xor_to_ascii};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 + 1 {
        eprintln!("usage: xorstr human/numOut keyfile messagefile");
        process::exit(1);
    }

    let (mode, keyfile, messagefile) = (&args[1], &args[2], &args[3]);
    let mut key = vec![];
    let mut text = vec![];
    File::open(keyfile).expect("error opening keyfile").read_to_end(&mut key).expect("error reading key");
    File::open(messagefile).expect("error opening messagefile").read_to_end(&mut text).expect("error reading message");
    
    println!("{}", match mode.as_str() {
        "human" => xor_to_ascii(&text, &key),
        "numOut" => xor_to_fmt_hex(&text, &key),
        m => {
            eprintln!("unrecognized mode: {}", m);
            process::exit(1);
        },
    });
}