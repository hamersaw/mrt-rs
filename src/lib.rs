extern crate byteorder;

use std::io::{BufReader, Error, ErrorKind, Read};

use byteorder::{BigEndian, ReadBytesExt};

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

pub struct MRTMessage {
    pub timestamp: u32,
    pub mrt_type: MRTType,
    pub mrt_subtype: u16,
    pub bgp4mp_message: Option<BGP4MPMessage>,
}

pub enum MRTType {
    OSPFV2,
    TABLEDUMP,
    TABLEDUMPV2,
    BGP4MP,
    BGP4MPET,
    ISIS,
    ISISET,
    OSPFV3,
    OSPFV3ET,
}

impl MRTMessage {
    pub fn parse(reader: &mut BufReader<Box<Read>>) -> Result<MRTMessage, Error> {
        //read header information
        let timestamp = try!(reader.read_u32::<BigEndian>());
        let mrt_type_u16 = try!(reader.read_u16::<BigEndian>());
        let mrt_type = match mrt_type_u16 {
            11 => MRTType::OSPFV2,
            12 => MRTType::TABLEDUMP,
            13 => MRTType::TABLEDUMPV2,
            16 => MRTType::BGP4MP,
            17 => MRTType::BGP4MPET,
            32 => MRTType::ISIS,
            33 => MRTType::ISISET,
            48 => MRTType::OSPFV3,
            49 => MRTType::OSPFV3ET,
            _ => return Err(Error::new(ErrorKind::Other, format!("unknown mrt type '{}'", mrt_type_u16))),
        };
        let mrt_subtype = try!(reader.read_u16::<BigEndian>());

        //parse message
        let bgp4mp_message = match mrt_type {
            MRTType::BGP4MP => Some(try!(BGP4MPMessage::parse(reader))),
            _ => None,
        };

        let length = try!(reader.read_u32::<BigEndian>());
        println!("message length:{}", length);
        //TMP read bytes
        for _ in 0..length {
            try!(reader.read_u8());
        }

        //create mrt message
        let msg = MRTMessage {
            timestamp: timestamp,
            mrt_type: mrt_type,
            mrt_subtype: mrt_subtype,
            bgp4mp_message: bgp4mp_message,
        };

        Ok(msg)
    }
}

pub struct BGP4MPMessage {

}

impl BGP4MPMessage {
    pub fn parse(reader: &mut BufReader<Box<Read>>) -> Result<BGP4MPMessage, Error> {
        Ok(BGP4MPMessage{})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
