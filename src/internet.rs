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
    println!("{:?}", String::from_utf8(send.stdout).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ip_sanity() {
        let (myip4, myip6) = super::get_my_ips();
        assert!(myip4.is_global());
        assert!(myip6.is_global());
        if myip4.is_ipv6() {
            panic!("wrong ip address type returned: 4")
        }
        if myip6.is_ipv4() {
            panic!("wrong ip address type returned: 6")
        }
    }
}
