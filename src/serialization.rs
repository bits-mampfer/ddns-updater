use crate::Records;
use dirs;
use std::fs::{read_to_string, write};

pub fn add_record(domain: String, key: Option<String>, remove: bool) {
    let mut records = load_records();

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
    write(get_user_home(), serialized).expect("couldnt write to file");
}

pub fn load_records() -> Vec<Records> {
    //serde deserializarion
    let records_string = match read_to_string(get_user_home()) {
        Ok(value) => value,
        Err(_) => "[]".to_string(),
    };
    let records: Vec<Records> =
        serde_json::from_str(&records_string).expect("deserialization error");
    return records;
}

fn get_user_home() -> std::path::PathBuf {
    let mut user_home = dirs::home_dir().expect("error getting users home file");
    user_home.push(".config");
    user_home.push("ddns-updater.dat");
    return user_home;
}

#[cfg(test)]
mod tests {
    #[test]
    fn user_home_sanity() {
        let user_home: String = super::get_user_home()
            .into_os_string()
            .into_string()
            .expect("cant convert path to string");
        // println!("{:?}", &user_home[0..6]);
        assert_eq!(&user_home[0..6], "/home/")
    }
}
