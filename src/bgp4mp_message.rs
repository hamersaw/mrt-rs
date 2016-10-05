use std::io::{Error, ErrorKind, Read};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use byteorder::{BigEndian, ReadBytesExt};

use bgp_message::BGPMessage;

pub enum AddressFamily {
    IpV4,
    IpV6,
}

//BGP4MPStateChange
pub struct BGP4MPStateChange{

}

impl BGP4MPStateChange{
    pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPStateChange, Error> {
        unimplemented!();
    }
}

//BGP4MPMessasge
pub struct BGP4MPMessage {
    pub peer_as_number: u16,
    pub local_as_number: u16,
    pub interface_index: u16,
    pub address_family : AddressFamily,
    pub peer_ip_address: IpAddr,
    pub local_ip_address: IpAddr,
    pub bgp_message: BGPMessage,
}

impl BGP4MPMessage {
    //pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPMessage, Error> {
    pub fn parse(reader: &mut Box<Read>) -> Result<BGP4MPMessage, Error> {
        //create cursor and parse header information
        let peer_as_number = try!(reader.read_u16::<BigEndian>());
        let local_as_number = try!(reader.read_u16::<BigEndian>());
        let interface_index = try!(reader.read_u16::<BigEndian>());

        //parse ip addresses
        let _address_family = try!(reader.read_u16::<BigEndian>());
        let (address_family, parse_ip_address): (AddressFamily, fn(&mut Box<Read>) -> Result<IpAddr, Error>) = match _address_family {
            1 => (AddressFamily::IpV4, parse_ipv4_address),
            2 => (AddressFamily::IpV6, parse_ipv6_address),
            _ => return Err(Error::new(ErrorKind::Other, format!("unknown address family type '{}'", _address_family))),
        };

        let peer_ip_address = try!(parse_ip_address(reader));
        let local_ip_address = try!(parse_ip_address(reader));

        //parse bgp message
        let bgp_message = try!(BGPMessage::parse(reader));

        //create message
        Ok (
            BGP4MPMessage {
                peer_as_number: peer_as_number,
                local_as_number: local_as_number,
                interface_index: interface_index,
                address_family: address_family,
                peer_ip_address: peer_ip_address,
                local_ip_address: local_ip_address,
                bgp_message: bgp_message,
            }
        )
    }
}

//BGP4MPMessageAs4
pub struct BGP4MPMessageAs4 {
    pub peer_as_number: u32,
    pub local_as_number: u32,
    pub interface_index: u16,
    pub address_family : AddressFamily,
    pub peer_ip_address: IpAddr,
    pub local_ip_address: IpAddr,
    pub bgp_message: BGPMessage,
}

impl BGP4MPMessageAs4{
    //pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPMessageAs4, Error> {
    pub fn parse(cursor: &mut Box<Read>) -> Result<BGP4MPMessageAs4, Error> {
        //create cursor and parse header information
        let peer_as_number = try!(cursor.read_u32::<BigEndian>());
        let local_as_number = try!(cursor.read_u32::<BigEndian>());
        let interface_index = try!(cursor.read_u16::<BigEndian>());

        //parse ip addresses
        let _address_family = try!(cursor.read_u16::<BigEndian>());
        let (address_family, parse_ip_address): (AddressFamily, fn(&mut Box<Read>) -> Result<IpAddr, Error>) = match _address_family {
            1 => (AddressFamily::IpV4, parse_ipv4_address),
            2 => (AddressFamily::IpV6, parse_ipv6_address),
            _ => return Err(Error::new(ErrorKind::Other, format!("unknown address family type '{}'", _address_family))),
        };

        let peer_ip_address = try!(parse_ip_address(cursor));
        let local_ip_address = try!(parse_ip_address(cursor));

        //parse bgp message
        let bgp_message = try!(BGPMessage::parse(cursor));

        //create message
        Ok (
            BGP4MPMessageAs4 {
                peer_as_number: peer_as_number,
                local_as_number: local_as_number,
                interface_index: interface_index,
                address_family: address_family,
                peer_ip_address: peer_ip_address,
                local_ip_address: local_ip_address,
                bgp_message: bgp_message,
            }
        )
    }
}

//BGP4MPStateChangeAs4
pub struct BGP4MPStateChangeAs4 {

}

impl BGP4MPStateChangeAs4 {
    pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPStateChangeAs4, Error> {
        unimplemented!();
    }
}


//BGP4MPMessageLocal
pub struct BGP4MPMessageLocal {

}

impl BGP4MPMessageLocal{
    pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPMessageLocal, Error> {
        unimplemented!();
    }
}

//BGP4MPMessageLocalAs4
pub struct BGP4MPMessageLocalAs4 {

}

impl BGP4MPMessageLocalAs4{
    pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPMessageLocalAs4, Error> {
        unimplemented!();
    }
}

//miscellaneous functions
fn parse_ipv4_address(cursor: &mut Box<Read>) -> Result<IpAddr, Error> {
    let mut buffer = [0u8; 4];
    try!(cursor.read_exact(&mut buffer));
    Ok(IpAddr::V4(Ipv4Addr::new(buffer[0], buffer[1], buffer[2], buffer[3])))
}

//fn parse_ipv6_address(cursor: &mut Cursor<&Vec<u8>>) -> Result<IpAddr, Error> {
fn parse_ipv6_address(cursor: &mut Box<Read>) -> Result<IpAddr, Error> {
    let mut buffer = [0u16; 8];
    for i in 0..8 {
        buffer[i] = try!(cursor.read_u16::<BigEndian>());
    }
    Ok(IpAddr::V6(Ipv6Addr::new(buffer[0], buffer[1], buffer[2], buffer[3], buffer[4], buffer[5], buffer[6], buffer[7])))
}
