use std::io::{Error, Read};

pub struct BGPUpdateMessage {

}

impl BGPUpdateMessage {
    pub fn parse(reader: &mut Box<Read>) -> Result<BGPUpdateMessage, Error> {
        unimplemented!();
    }
}
