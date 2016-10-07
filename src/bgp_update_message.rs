use std::io::{Error, ErrorKind, Read};
use std::net::IpAddr;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub enum AttributeTypeCode {
    Origin,
    AsPath,
    NextHop,
    MultiExitDisc,
    LocalPref,
    AtomicAggregate,
    Aggregator,
    Unknown,
}

pub enum Origin {
    Igp,
    Egp,
    Incomplete,
}

pub struct BGPUpdateMessage {
    pub origin: Option<Origin>,
    pub next_hop: Option<IpAddr>,
    pub multi_exit_disc: Option<u32>,
    pub local_pref: Option<u32>,
    pub atomic_aggregate: Option<bool>,
    pub aggregator: Option<(u32, IpAddr)>,
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

        let mut origin: Option<Origin> = None;
        let mut next_hop: Option<IpAddr> = None;
        let mut multi_exit_disc: Option<u32> = None;
        let mut local_pref: Option<u32> = None;
        let mut atomic_aggregate: Option<bool> = None;
        let mut aggregator: Option<(u32, IpAddr)> = None;

        if total_path_attributes_length == 0 {
            return Ok (
                BGPUpdateMessage {
                    origin: origin,
                    next_hop: next_hop,
                    multi_exit_disc: multi_exit_disc,
                    local_pref: local_pref,
                    atomic_aggregate: atomic_aggregate,
                    aggregator: aggregator,
                }
            )
        }

        while total_path_attributes_length > 0 {
            let attribute_flags = try!(reader.read_u8());
            let optional_bit = attribute_flags & 128 == 128;
            let transitive_bit = attribute_flags & 64 == 64;
            let partial_bit = attribute_flags & 32 == 32;
            let extended_length_bit = attribute_flags & 16 == 16;
            if attribute_flags & 15 != 0 {
                return Err(Error::new(ErrorKind::Other, "attribute flags lower 4 bits must be unused"));
            }

            let _attribute_type_code = try!(reader.read_u8());
            let attribute_type_code = match _attribute_type_code {
                1 => AttributeTypeCode::Origin,
                2 => AttributeTypeCode::AsPath,
                3 => AttributeTypeCode::NextHop,
                4 => AttributeTypeCode::MultiExitDisc,
                5 => AttributeTypeCode::LocalPref,
                6 => AttributeTypeCode::AtomicAggregate,
                7 => AttributeTypeCode::Aggregator,
                _ => AttributeTypeCode::Unknown,
            };

            println!("{:?} {} {} {} {}", attribute_type_code, optional_bit, transitive_bit, partial_bit, extended_length_bit);
            //parse out attribute_length
            let (attribute_length, length_bytes) = match extended_length_bit {
                true => (try!(reader.read_u16::<BigEndian>()), 2),
                false => (try!(reader.read_u8()) as u16, 1),
            };

            //reduce total_path_attributes_length
            total_path_attributes_length -= 1 + 1 + length_bytes + attribute_length; //attribute_flags + attribute_type_code + length_bytes + attribute_length

            match attribute_type_code {
                AttributeTypeCode::Origin => {
                    let _origin = try!(reader.read_u8());
                    origin = match _origin {
                        0 => Some(Origin::Igp),
                        1 => Some(Origin::Egp),
                        2 => Some(Origin::Incomplete),
                        _ => return Err(Error::new(ErrorKind::Other, format!("unknown origin '{}'", _origin))),
                    };
                },
                /*AttributeTypeCode::AsPath => {
                    TODO
                },*/
                AttributeTypeCode::NextHop => {
                    next_hop = match attribute_length {
                        4 => Some(try!(super::parse_ipv4_address(reader))),
                        16 => Some(try!(super::parse_ipv6_address(reader))),
                        _ => return Err(Error::new(ErrorKind::Other, format!("unknown length for next hop '{}'", attribute_length))),
                    };
                },
                AttributeTypeCode::MultiExitDisc => multi_exit_disc = Some(try!(reader.read_u32::<BigEndian>())),
                AttributeTypeCode::LocalPref => local_pref = Some(try!(reader.read_u32::<BigEndian>())),
                AttributeTypeCode::AtomicAggregate => atomic_aggregate = Some(true),
                AttributeTypeCode::Aggregator => {
                    aggregator = match attribute_length {
                        6 => Some((try!(reader.read_u16::<BigEndian>()) as u32, try!(super::parse_ipv4_address(reader)))),
                        8 => Some((try!(reader.read_u32::<BigEndian>()), try!(super::parse_ipv4_address(reader)))),
                        18 => Some((try!(reader.read_u16::<BigEndian>()) as u32, try!(super::parse_ipv6_address(reader)))),
                        20 => Some((try!(reader.read_u32::<BigEndian>()), try!(super::parse_ipv6_address(reader)))),
                        _ => return Err(Error::new(ErrorKind::Other, format!("unknown length for aggregator '{}'", attribute_length))),
                    };
                },
                _ => {
                    //read bytes from reader
                    for _ in 0..attribute_length {
                        let _ = try!(reader.read_u8());
                    }
                },
            }
        }

        if atomic_aggregate.is_none() {
            atomic_aggregate = Some(false);
        }

        //read network layer reachability information
        /*let mut network_layer_reachability_length = try!(reader.read_u16::<BigEndian>());
        while network_layer_reachability_length > 0 {
            let length = try!(reader.read_u8());
            let mut bytes = length / 8;
            if length % 8 != 0 {
                bytes += 1;
            }

            //TODO parse ip address            
            for _ in 0..bytes {
                let _ = try!(reader.read_u8());
            }

            network_layer_reachability_length -= (bytes + 1) as u16;
        }*/

        Ok (
            BGPUpdateMessage {
                origin: origin,
                next_hop: next_hop,
                multi_exit_disc: multi_exit_disc,
                local_pref: local_pref,
                atomic_aggregate: atomic_aggregate,
                aggregator: aggregator,
            }
        )
    }
}
