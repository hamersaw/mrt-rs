use std::io::{Cursor, Error, ErrorKind, Read};

use byteorder::{BigEndian, ReadBytesExt};

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
    pub address_family : u16,
    pub peer_ip_address: u32,
    pub local_ip_address: u32
}

impl BGP4MPMessage {
    pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPMessage, Error> {
        let mut cursor = Cursor::new(buffer);
        let peer_as_number = try!(cursor.read_u16::<BigEndian>());
        let local_as_number = try!(cursor.read_u16::<BigEndian>());
        let interface_index = try!(cursor.read_u16::<BigEndian>());
        let address_family = try!(cursor.read_u16::<BigEndian>());
        let peer_ip_address = try!(cursor.read_u32::<BigEndian>());
        let local_ip_address = try!(cursor.read_u32::<BigEndian>());

        Ok (
            BGP4MPMessage {
                peer_as_number: peer_as_number,
                local_as_number: local_as_number,
                interface_index: interface_index,
                address_family: address_family,
                peer_ip_address: peer_ip_address,
                local_ip_address: local_ip_address,
            }
        )
    }
}

//BGP4MPMessageAs4
pub struct BGP4MPMessageAs4 {
    pub peer_as_number: u32,
    pub local_as_number: u32,
    pub interface_index: u16,
    pub address_family : u16,
    pub peer_ip_address: u32,
    pub local_ip_address: u32
}

impl BGP4MPMessageAs4{
    pub fn parse(buffer: &Vec<u8>) -> Result<BGP4MPMessageAs4, Error> {
        let mut cursor = Cursor::new(buffer);
        let peer_as_number = try!(cursor.read_u32::<BigEndian>());
        let local_as_number = try!(cursor.read_u32::<BigEndian>());
        let interface_index = try!(cursor.read_u16::<BigEndian>());
        let address_family = try!(cursor.read_u16::<BigEndian>());
        let peer_ip_address = try!(cursor.read_u32::<BigEndian>());
        let local_ip_address = try!(cursor.read_u32::<BigEndian>());

        Ok (
            BGP4MPMessageAs4 {
                peer_as_number: peer_as_number,
                local_as_number: local_as_number,
                interface_index: interface_index,
                address_family: address_family,
                peer_ip_address: peer_ip_address,
                local_ip_address: local_ip_address,
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
