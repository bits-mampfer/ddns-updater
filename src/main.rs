#![feature(ip)]
pub mod clap;
pub mod internet;
pub mod serialization;

use crate::clap::ClapCommand;
use crate::clap::Records;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args = clap::clap_initialize();
    match args.command {
        ClapCommand::Run => run(args.interval),
        ClapCommand::Add { domain, key } => serialization::add_record(domain, Some(key), false),
        ClapCommand::Remove { domain } => serialization::add_record(domain, None, true),
    }
}

fn run(interval: u8) {
    let records = serialization::load_records();
    let (myip4, myip6) = internet::get_my_ips();

    loop {
        for record in &records {
            internet::send_message(&record.domain, &record.key, myip4, myip6);
        }
        println!("updated records with {myip4} {myip6}");
        if interval == 0 {
            break;
        } else {
            sleep(Duration::from_secs(interval as u64 * 3600));
        }
    }
}

// TODO : write unit tests
