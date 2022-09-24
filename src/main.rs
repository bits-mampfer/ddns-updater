mod lib;
use lib::ClapCommand;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args = lib::clap_initialize();
    match args.command {
        ClapCommand::Run => run(args.interval),
        ClapCommand::Add { domain, key } => lib::add_record(domain, key),
        ClapCommand::Remove { domain } => {}
    }

    //if args.command == "add" {};
    // println!("{:?} , {:?}", myip4, myip6);
    // let updates = vec![
    //     ("www.bits-mampfer.eu", "5puht192xu1b6-l9"),
    //     ("bits-mampfer.eu", "pejhipgnqknt2q-1"),
    // ];
    // for update in updates {
    //     lib::send_message(update.0, update.1, myip4, myip6);
    // }
}

fn run(interval: u8) {
    let records = lib::load_records();
    let interval_h: Duration = Duration::from_secs(interval as u64 * 3600);
    let (myip4, myip6) = lib::get_my_ips();

    loop {
        for record in records {
            send_message(record.domain, record.key, myip4, myip6);
        }
        sleep(interval_h);
    }
}
