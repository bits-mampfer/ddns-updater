mod lib;
use lib::ClapCommand;

fn main() {
    let args = lib::clap_initialize();
    match args.command {
        ClapCommand::Run => {}
        ClapCommand::Add { domain, key } => {}
        ClapCommand::Remove { domain } => {}
    }
    //if args.command == "add" {};
    // let (myip4, myip6) = lib::get_my_ips();
    // println!("{:?} , {:?}", myip4, myip6);
    // let updates = vec![
    //     ("www.bits-mampfer.eu", "5puht192xu1b6-l9"),
    //     ("bits-mampfer.eu", "pejhipgnqknt2q-1"),
    // ];
    // for update in updates {
    //     lib::send_message(update.0, update.1, myip4, myip6);
    // }
}
