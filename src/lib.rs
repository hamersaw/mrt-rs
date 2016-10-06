pub mod bgp4mp_message;
pub mod bgp_message;
pub mod bgp_update_message;
pub mod mrt_message;

extern crate byteorder;

use std::io::{Error, Read};

use bgp_message::BGPMessage;
use mrt_message::MRTMessage;

pub struct MRTScanner {
    reader: Box<Read>,
}

impl MRTScanner {
    pub fn new(reader: Box<Read>) -> MRTScanner {
        MRTScanner {
            reader: reader,
        }
    }

    pub fn scan(&mut self) -> Result<MRTMessage, Error> {
        MRTMessage::parse(&mut self.reader)
    }
}

pub struct BGPScanner {
    reader: Box<Read>,
}

impl BGPScanner {
    pub fn new(reader: Box<Read>) -> BGPScanner {
        BGPScanner {
            reader: reader,
        }
    }

    pub fn scan(&mut self) -> Result<BGPMessage, Error> {
        BGPMessage::parse(&mut self.reader)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
