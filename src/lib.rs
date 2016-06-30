pub mod bgp4mp_message;
pub mod mrt_message;

extern crate byteorder;

use std::io::{BufReader, Error, Read};

use mrt_message::MRTMessage;

pub struct MRTScanner {
    reader: BufReader<Box<Read>>,
}

impl MRTScanner {
    pub fn new(reader: Box<Read>) -> MRTScanner {
        MRTScanner {
            reader: BufReader::new(reader),
        }
    }

    pub fn scan(&mut self) -> Result<MRTMessage, Error> {
        MRTMessage::parse(&mut self.reader)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
