pub mod bgp4mp_message;
pub mod bgp_message;
pub mod bgp_update_message;
pub mod mrt_message;

extern crate byteorder;

use std::io::{Error, Read};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use byteorder::{BigEndian, ReadBytesExt};

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

//miscellaneous functions
fn parse_ipv4_address(reader: &mut Box<Read>) -> Result<IpAddr, Error> {
    let mut buffer = [0u8; 4];
    try!(reader.read_exact(&mut buffer));
    Ok(IpAddr::V4(Ipv4Addr::new(buffer[0], buffer[1], buffer[2], buffer[3])))
}

fn parse_ipv6_address(reader: &mut Box<Read>) -> Result<IpAddr, Error> {
    let mut buffer = [0u16; 8];
    for i in 0..8 {
        buffer[i] = try!(reader.read_u16::<BigEndian>());
    }
    Ok(IpAddr::V6(Ipv6Addr::new(buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7])))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
