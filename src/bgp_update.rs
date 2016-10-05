use std::io::{Error, Read};

pub struct BGPUpdate {

}

impl BGPUpdate {
    pub fn parse(reader: &mut Box<Read>) -> Result<BGPUpdate, Error> {
        unimplemented!();
    }
}
