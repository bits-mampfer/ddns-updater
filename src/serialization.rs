use crate::Records;
use dirs;
use std::fs::{read_to_string, write};

pub fn add_record(domain: String, key: Option<String>, remove: bool) {
    let mut records = load_records();

    let user_home = get_user_home();
    if remove {
        let index = records
            .iter()
            .position(|a| *a.domain == domain)
            .expect("domain not found in list");
        records.remove(index);
    } else {
        let key = key.expect("error: no key provided");
        let new_record = Records { domain, key };
        records.push(new_record);
    };
    //serde serialization
    let serialized = serde_json::to_string(&records).expect("couldnt serialize");
    write(user_home, serialized).expect("couldnt write to file");
}

pub fn load_records() -> Vec<Records> {
    //serde deserializarion
    let user_home = get_user_home();
    let records_string_wraped = read_to_string(user_home);
    let records_string = match records_string_wraped {
        Ok(value) => value,
        Err(error) => handle_deserialize_error(error),
    };
    let records: Vec<Records> =
        serde_json::from_str(&records_string).expect("deserialization error");
    return records;
}

fn handle_deserialize_error(error: std::io::Error) -> String {
    if error.kind() == std::io::ErrorKind::NotFound {
        //let output = std::fs::File::create("~/.config/ddns-updater.dat")
        //   .expect("cant create file ~/.config/ddns-updater.dat");
        //write!(output, " ");
        return "[]".to_string();
    } else {
        panic!("error in handle_deserialize_error");
    }
}

fn get_user_home() -> std::path::PathBuf {
    let mut user_home = dirs::home_dir().expect("error getting users home file");
    user_home.push(".config");
    user_home.push("ddns-updater.dat");
    return user_home;
}
