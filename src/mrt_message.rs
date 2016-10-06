use std::io::{Cursor, Error, ErrorKind, Read};

use byteorder::{BigEndian, ReadBytesExt};

use bgp4mp_message::{BGP4MPMessage, BGP4MPMessageAs4};

pub struct MRTMessage {
    pub timestamp: u32,
    pub mrt_type: MRTType,
    pub mrt_subtype: MRTSubType,
    buffer: Vec<u8>,
}

pub enum MRTType {
    OspfV2,
    TableDump,
    TableDumpV2,
    Bgp4mp,
    Bgp4mpEt,
    Isis,
    IsisEt,
    OspfV3,
    OspfV3Et
}

pub enum MRTSubType {
    Bgp4mpStateChange,
    Bgp4mpMessage,
    Bgp4mpMessageAs4,
    Bgp4mpStateChangeAs4,
    Bgp4mpMessageLocal,
    Bgp4mpMessageAs4Local,
    Unknown,
}

impl MRTMessage {
    pub fn parse(reader: &mut Box<Read>) -> Result<MRTMessage, Error> {
        //read header information
        let timestamp = try!(reader.read_u32::<BigEndian>());
        let _mrt_type = try!(reader.read_u16::<BigEndian>());
        let _mrt_subtype = try!(reader.read_u16::<BigEndian>());

        let (mrt_type, mrt_subtype) = match _mrt_type {
            11 => (MRTType::OspfV2, MRTSubType::Unknown),
            12 => (MRTType::TableDump, MRTSubType::Unknown),
            13 => (MRTType::TableDumpV2, MRTSubType::Unknown),
            16 => {
                ( 
                    MRTType::Bgp4mp,
                    match _mrt_subtype {
                        0 => MRTSubType::Bgp4mpStateChange,
                        1 => MRTSubType::Bgp4mpMessage,
                        4 => MRTSubType::Bgp4mpMessageAs4,
                        5 => MRTSubType::Bgp4mpStateChangeAs4,
                        6 => MRTSubType::Bgp4mpMessageLocal,
                        7 => MRTSubType::Bgp4mpMessageAs4Local,
                        _ => return Err(Error::new(ErrorKind::Other, format!("unknown mrt subtype '{}'", _mrt_subtype))),
                    }
                )
            },
            17 => (MRTType::Bgp4mpEt, MRTSubType::Unknown),
            32 => (MRTType::Isis, MRTSubType::Unknown),
            33 => (MRTType::IsisEt, MRTSubType::Unknown),
            48 => (MRTType::OspfV3, MRTSubType::Unknown),
            49 => (MRTType::OspfV3Et, MRTSubType::Unknown),
            _ => return Err(Error::new(ErrorKind::Other, format!("unknown mrt type '{}'", _mrt_type))),
        };

        //read message body
        let length = try!(reader.read_u32::<BigEndian>());
        let mut buffer = vec![0; length as usize];
        try!(reader.read_exact(&mut buffer));

        //create mrt message
        let msg = MRTMessage {
            timestamp: timestamp,
            mrt_type: mrt_type,
            mrt_subtype: mrt_subtype,
            buffer: buffer,
        };

        Ok(msg)
    }

    /*pub fn parse_bgp4mp_state_change(&self) -> Result<BGP4MPStateChange, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpStateChange => BGP4MPStateChange::parse(&self.buffer),
            _ => return Err(Error::new(ErrorKind::Other, "incorrect subtype on mrt message")),
        }
    }*/

    pub fn parse_bgp4mp_message<'a>(&'a self) -> Result<BGP4MPMessage, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpMessage => {
                let mut reader: Box<Read> = Box::new(Cursor::new(self.buffer.clone()));
                BGP4MPMessage::parse(&mut reader)
            },
            _ => return Err(Error::new(ErrorKind::Other, "incorrect subtype on mrt message")),
        }
    }

    pub fn parse_bgp4mp_message_as4(&self) -> Result<BGP4MPMessageAs4, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpMessageAs4 => {
                let mut reader: Box<Read> = Box::new(Cursor::new(self.buffer.clone()));
                BGP4MPMessageAs4::parse(&mut reader)
            },
            _ => return Err(Error::new(ErrorKind::Other, "incorrect subtype on mrt message")),
        }
    }

    /*pub fn parse_bgp4mp_state_change(&self) -> Result<BGP4MPStateChangeMessage, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpStateChange => BGP4MPStateChangeMessage::parse(&self.buffer),
            _ => return Err(Error::new(ErrorKind::Other, "unable to parse bgp4mp state change message because message is incorrect sub type")),
        }
    }

    pub fn parse_bgp4mp_state_change(&self) -> Result<BGP4MPStateChangeMessage, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpStateChange => BGP4MPStateChangeMessage::parse(&self.buffer),
            _ => return Err(Error::new(ErrorKind::Other, "unable to parse bgp4mp state change message because message is incorrect sub type")),
        }
    }

    pub fn parse_bgp4mp_state_change(&self) -> Result<BGP4MPStateChangeMessage, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpStateChange => BGP4MPStateChangeMessage::parse(&self.buffer),
            _ => return Err(Error::new(ErrorKind::Other, "unable to parse bgp4mp state change message because message is incorrect sub type")),
        }
    }

    pub fn parse_bgp4mp_state_change(&self) -> Result<BGP4MPStateChangeMessage, Error> {
        match self.mrt_subtype {
            MRTSubType::Bgp4mpStateChange => BGP4MPStateChangeMessage::parse(&self.buffer),
            _ => return Err(Error::new(ErrorKind::Other, "unable to parse bgp4mp state change message because message is incorrect sub type")),
        }
    }*/
}
