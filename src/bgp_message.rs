use std::io::{Cursor, Error, ErrorKind, Read};

use byteorder::{BigEndian, ReadBytesExt};

use bgp_update::BGPUpdate;

pub struct BGPMessage {
    pub bgp_type: BGPType,
    buffer: Vec<u8>
}

pub enum BGPType {
    Open,
    Update,
    Modification,
    KeepAlive,
}

impl BGPMessage {
    pub fn parse(reader: &mut Box<Read>) -> Result<BGPMessage, Error> {
        for _ in 0..15 {
            //read marker
            let marker = try!(reader.read_u8());
            if marker != 255 {
                return Err(Error::new(ErrorKind::Other, "bgp marker incorrect"));
            }
        }

        //read header information
        let length = try!(reader.read_u16::<BigEndian>());
        let _bgp_type = try!(reader.read_u8());
        let bgp_type = match _bgp_type {
            1 => BGPType::Open,
            2 => BGPType::Update,
            3 => BGPType::Modification,
            4 => BGPType::KeepAlive,
            _ => return Err(Error::new(ErrorKind::Other, format!("unknown bgp type '{}'", _bgp_type))),
        };

        let mut buffer = vec![0; length as usize];
        try!(reader.read_exact(&mut buffer));

        //create message
        Ok (
            BGPMessage {
                bgp_type: bgp_type,
                buffer: buffer,
            }
        )
    }

    pub fn parse_update(&self) -> Result<BGPUpdate, Error> {
        match self.bgp_type {
            BGPType::Update => {
                let mut reader: Box<Read> = Box::new(Cursor::new(self.buffer.clone()));
                BGPUpdate::parse(&mut reader)
            },
            _ => return Err(Error::new(ErrorKind::Other, "incorrect subtype on mrt message")),
        }
    }
}
