extern crate rustmrt;

use std::fs::File;

use rustmrt::{MRTScanner, MRTType};

fn main() {
    //open reader
    let file = match File::open("/home/hamersaw/Downloads/updates.20160101.1230") {
        Ok(file) => file,
        Err(e) => panic!("{}", e),
    };

    //open scanner and parse messages
    let mut scanner = MRTScanner::new(Box::new(file));
    loop {
        let mrt_message = match scanner.scan() {
            Ok(mrt_message) => mrt_message,
            Err(e) => panic!("{}", e),
        };

        match mrt_message.mrt_type {
            MRTType::BGP4MP => {
                
            },
            _ => println!("skipping message"),
        }
    }
}
