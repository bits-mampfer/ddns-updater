use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::process::Command;

pub fn get_my_ips() -> (IpAddr, IpAddr) {
    let myipv4_command = Command::new("curl")
        .arg("icanhazip.com")
        .arg("-4")
        .output()
        .expect("error getting own ip address (ipv4)");
    let mut string = String::from_utf8(myipv4_command.stdout).unwrap();
    string.pop();
    let myipv4: IpAddr = string.parse().unwrap();

    let myipv6_command = Command::new("curl")
        .arg("icanhazip.com")
        .arg("-6")
        .output()
        .expect("error getting own ip address (ipv6)");
    let mut string = String::from_utf8(myipv6_command.stdout).unwrap();
    string.pop();
    let myipv6: IpAddr = string.parse().unwrap();

    return (myipv4, myipv6);
}

pub fn send_message(domain: &str, key: &str, addressev4: IpAddr, addressev6: IpAddr) {
    let send = Command::new("curl")
        .arg(format!(
            "https://njal.la/update/?h={domain}&k={key}&a={addressev4}&aaaa={addressev6}"
        ))
        .output()
        .expect("error sending message to server");
    let string: String = String::from_utf8(send.stdout).unwrap();
    println!("{:?}", string);
}

#[derive(Serialize, Deserialize)]
pub struct Records {
    pub domain: String,
    pub key: String,
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: ClapCommand,

    /// the interval to update in hours, set to 0 to update only once for use in e.g. crontab
    #[clap(short, long, value_parser, default_value_t = 24)]
    pub interval: u8,
}

#[derive(Subcommand)]
pub enum ClapCommand {
    /// run the updater
    Run,
    /// add a domain and associated key to keep updated
    Add {
        #[clap(value_parser)]
        domain: String,
        #[clap(value_parser)]
        key: String,
    },
    /// remove a doman from being updated
    Remove { domain: String }, // (domain)
}

pub fn clap_initialize() -> Args {
    let args = Args::parse();
    //added_records
    return args;
}
