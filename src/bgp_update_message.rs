use std::io::{Error, ErrorKind, Read};
use std::net::{IpAddr, Ipv4Addr};

use byteorder::{BigEndian, ReadBytesExt};

use super::Prefix;

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
    pub withdrawn_routes: Option<Vec<Prefix>>,
    pub network_layer_reachability_information: Option<Vec<Prefix>>,
    pub origin: Option<Origin>,
    pub next_hop: Option<IpAddr>,
    pub multi_exit_disc: Option<u32>,
    pub local_pref: Option<u32>,
    pub atomic_aggregate: Option<bool>,
    pub aggregator: Option<(u32, IpAddr)>,
}


impl BGPUpdateMessage {
    pub fn parse(reader: &mut Box<Read>) -> Result<BGPUpdateMessage, Error> {
        let mut withdrawn_routes: Option<Vec<Prefix>> = None;
        let mut network_layer_reachability_information: Option<Vec<Prefix>> = None;
        let mut origin: Option<Origin> = None;
        let mut next_hop: Option<IpAddr> = None;
        let mut multi_exit_disc: Option<u32> = None;
        let mut local_pref: Option<u32> = None;
        let mut atomic_aggregate: Option<bool> = None;
        let mut aggregator: Option<(u32, IpAddr)> = None;

        //read withdrawn routes
        let mut withdrawn_routes_length = try!(reader.read_u16::<BigEndian>());
        let mut withdrawn_routes_vec = vec!();
        while withdrawn_routes_length > 0 {
            let length = try!(reader.read_u8());
            let mut byte_count = length / 8;
            if length % 8 != 0 {
                byte_count += 1;
            }

            //parse ip address            
            let mut bytes = vec!();
            for _ in 0..byte_count - 1 {
                bytes.push(try!(reader.read_u8()));
            }

            if length % 8 == 0 {
                bytes.push(try!(reader.read_u8()));
            } else {
                let mut index_value = 128;
                let mut and_value = 0;
                for _ in 0..(length % 8) {
                    and_value += index_value;
                    index_value /= 2;
                }

                bytes.push(try!(reader.read_u8()) & and_value);
            }

            while bytes.len() < 4 {
                bytes.push(0);
            }

            let ip_addr = IpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]));

            withdrawn_routes_vec.push(Prefix::new(ip_addr, length));
            withdrawn_routes_length -= (byte_count + 1) as u16;
        }

        if withdrawn_routes_vec.len() != 0 {
            withdrawn_routes = Some(withdrawn_routes_vec);
        }

        //read total path attributes
        let mut total_path_attributes_length = try!(reader.read_u16::<BigEndian>());
        if total_path_attributes_length == 0 {
            return Ok (
                BGPUpdateMessage {
                    withdrawn_routes: withdrawn_routes,
                    network_layer_reachability_information: network_layer_reachability_information,
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
        let mut vec = vec!();
        loop {
            let length = match reader.read_u8() {
                Ok(length) => length,
                Err(e) => {
                    match e.kind() {
                        ErrorKind::UnexpectedEof => break,
                        _ => return Err(e),
                    }
                }
            };

            let mut byte_count = length / 8;
            if length % 8 != 0 {
                byte_count += 1;
            }

            //parse ip address            
            let mut bytes = vec!();
            for _ in 0..byte_count - 1 {
                bytes.push(try!(reader.read_u8()));
            }

            if length % 8 == 0 {
                bytes.push(try!(reader.read_u8()));
            } else {
                let mut index_value = 128;
                let mut and_value = 0;
                for _ in 0..(length % 8) {
                    and_value += index_value;
                    index_value /= 2;
                }

                bytes.push(try!(reader.read_u8()) & and_value);
            }

            while bytes.len() < 4 {
                bytes.push(0);
            }

            let ip_addr = IpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3]));
            vec.push(Prefix::new(ip_addr, length));
        }

        if vec.len() != 0 {
            network_layer_reachability_information = Some(vec);
        }

        Ok (
            BGPUpdateMessage {
                withdrawn_routes: withdrawn_routes,
                network_layer_reachability_information: network_layer_reachability_information,
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
