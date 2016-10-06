use std::io::{Error, ErrorKind, Read};

use byteorder::{BigEndian, ReadBytesExt};

pub enum AttributeTypeCode {
    Origin,
    AsPath,
    NextHop,
    MultiExitDisc,
    LocalPref,
    AtomicAggregate,
    Aggregator,
}

pub struct BGPUpdateMessage {
    pub as_list: Option<()>,
    pub as_set: Option<()>,
}

impl BGPUpdateMessage {
    pub fn parse(reader: &mut Box<Read>) -> Result<BGPUpdateMessage, Error> {
        //read withdrawn routes
        let mut withdrawn_routes_length = try!(reader.read_u16::<BigEndian>());
        while withdrawn_routes_length > 0 {
            let length = try!(reader.read_u8());
            let mut bytes = length / 8;
            if length % 8 != 0 {
                bytes += 1;
            }

            //TODO parse ip address            
            for _ in 0..bytes {
                let _ = try!(reader.read_u8());
            }

            withdrawn_routes_length -= (bytes + 1) as u16;
        }

        //read total path attributes
        let mut total_path_attributes_length = try!(reader.read_u16::<BigEndian>());
        while total_path_attributes_length > 0 {
            let attribute_flags = try!(reader.read_u8());
            let _attribute_type_code = try!(reader.read_u8());
            /*let attribute_type_code = match _attribute_type_code {
                1 => AttributeTypeCode::Origin,
                2 => AttributeTypeCode::AsPath,
                3 => AttributeTypeCode::NextHop,
                4 => AttributeTypeCode::MultiExitDisc,
                5 => AttributeTypeCode::LocalPref,
                6 => AttributeTypeCode::AtomicAggregate,
                7 => AttributeTypeCode::Aggregator,
                _ => return Err(Error::new(ErrorKind::Other, format!("unknown attribute type code '{}'", _attribute_type_code))),
            };*/

            //TODO parse extended length

            total_path_attributes_length -= total_path_attributes_length;
        }

        Ok (
            BGPUpdateMessage {
                as_list: None,
                as_set: None,
            }
        )
    }
}
