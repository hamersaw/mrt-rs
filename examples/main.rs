extern crate rustmrt;

use std::fs::File;

use rustmrt::MRTScanner;
use rustmrt::mrt_message::MRTSubType;

fn main() {
    //open reader
    let file = match File::open("/home/hamersaw/Downloads/route-views.chicago-updates.20160928.0000") {
    //let file = match File::open("/home/hamersaw/Downloads/updates.20160101.1230") {
    //let file = match File::open("/home/hamersaw/Downloads/updates-201606010014") {
        Ok(file) => file,
        Err(e) => panic!("{}", e),
    };

    //open scanner and parse messages
    let mut scanner = MRTScanner::new(Box::new(file));
    loop {
        let mrt_message = match scanner.scan() {
            Ok(mrt_message) => mrt_message,
            Err(e) => panic!("{}", e),
        };

        match mrt_message.mrt_subtype {
            MRTSubType::Bgp4mpStateChange => println!("state change"),
            MRTSubType::Bgp4mpMessage => {
                let msg = match mrt_message.parse_bgp4mp_message() {
                    Ok(msg) => msg,
                    Err(e) => panic!("{}", e),
                };

                println!("bgp4mp message\n\tpeer_as_number:{}\n\tlocal_as_number:{}\n\tpeer_ip_address:{:?}\n\tlocal_ip_address:{:?}", msg.peer_as_number, msg.local_as_number, msg.peer_ip_address, msg.local_ip_address); 
            },
            MRTSubType::Bgp4mpMessageAs4 => {
                let msg = match mrt_message.parse_bgp4mp_message_as4() {
                    Ok(msg) => msg,
                    Err(e) => panic!("{}", e),
                };

                println!("bgp4mp message as4\n\tpeer_as_number:{}\n\tlocal_as_number:{}\n\tpeer_ip_address:{:?}\n\tlocal_ip_address:{:?}", msg.peer_as_number, msg.local_as_number, msg.peer_ip_address, msg.local_ip_address); 
            },
            MRTSubType::Bgp4mpStateChangeAs4 => println!("state change as4"),
            MRTSubType::Bgp4mpMessageLocal => println!("message local"),
            MRTSubType::Bgp4mpMessageAs4Local => println!("messgae as4 local"),
            _ => println!("skipping message"),
        }
    }
}
