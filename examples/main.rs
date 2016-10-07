extern crate mrt_rs;

use std::fs::File;

use mrt_rs::MRTScanner;
use mrt_rs::bgp_message::BGPType;
use mrt_rs::mrt_message::MRTSubType;

fn main() {
    //open reader
    let file = match File::open("/home/hamersaw/Downloads/route-views.chicago-updates.20160928.0000") {
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
                let msg = mrt_message.parse_bgp4mp_message().unwrap();
                println!("bgp4mp message\n\tpeer_as_number:{}\n\tlocal_as_number:{}\n\tpeer_ip_address:{:?}\n\tlocal_ip_address:{:?}", msg.peer_as_number, msg.local_as_number, msg.peer_ip_address, msg.local_ip_address); 

                match msg.bgp_message.bgp_type {
                    BGPType::Open => println!("\tOPEN MESSAGE"),
                    BGPType::Update => {
                        let bgp_msg = msg.bgp_message.parse_update_message().unwrap();
                    },
                    BGPType::Modification => println!("\tMODIFICATION MESSAGE"),
                    BGPType::KeepAlive => println!("\tKEEP ALIVE MESSAGE"),
                }
            },
            MRTSubType::Bgp4mpMessageAs4 => {
                let msg = mrt_message.parse_bgp4mp_message_as4().unwrap();
                println!("bgp4mp message as4\n\tpeer_as_number:{}\n\tlocal_as_number:{}\n\tpeer_ip_address:{:?}\n\tlocal_ip_address:{:?}", msg.peer_as_number, msg.local_as_number, msg.peer_ip_address, msg.local_ip_address); 

                match msg.bgp_message.bgp_type {
                    BGPType::Open => println!("\tOPEN MESSAGE"),
                    BGPType::Update => {
                        let bgp_msg = msg.bgp_message.parse_update_message().unwrap();
                    },
                    BGPType::Modification => println!("\tMODIFICATION MESSAGE"),
                    BGPType::KeepAlive => println!("\tKEEP ALIVE MESSAGE"),
                }
            },
            MRTSubType::Bgp4mpStateChangeAs4 => println!("state change as4"),
            MRTSubType::Bgp4mpMessageLocal => println!("message local"),
            MRTSubType::Bgp4mpMessageAs4Local => println!("messgae as4 local"),
            _ => println!("skipping message"),
        }
    }
}
