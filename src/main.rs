pub mod lib;
pub mod serialization;
use lib::ClapCommand;
pub use lib::Records;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args = lib::clap_initialize();
    match args.command {
        ClapCommand::Run => run(args.interval),
        ClapCommand::Add { domain, key } => serialization::add_record(domain, Some(key), false),
        ClapCommand::Remove { domain } => serialization::add_record(domain, None, true),
    }
}

fn run(interval: u8) {
    let records = serialization::load_records();
    let interval_h: Duration = Duration::from_secs(interval as u64 * 3600);
    let (myip4, myip6) = lib::get_my_ips();

    loop {
        for record in &records {
            lib::send_message(&record.domain, &record.key, myip4, myip6);
        }
        println!("updated records with {myip4} {myip6}");
        sleep(interval_h);
    }
}

//TODO: test the functionality
// TODO : write unit tests
// refactor functions
