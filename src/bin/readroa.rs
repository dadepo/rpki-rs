extern crate rpki;

use std::{env, fs};
use std::io::Read;
use rpki::roa::Roa;


fn main() {
    let path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: readroa <path>");
            return
        }
    };
    let mut file = match fs::File::open(path) {
        Ok(file) => file,
        Err(err) => {
            println!("Can’t open file: {}", err);
            return;
        }
    };
    let mut data = Vec::new();
    if let Err(err) = file.read_to_end(&mut data) {
        println!("Can’t read file: {}", err);
        return;
    }

    let _cert = match Roa::decode(data.as_ref(), true) {
        Ok(cert) => cert,
        Err(err) => {
            println!("Can’t decode roa: {}", err);
            return
        }
    };
}


