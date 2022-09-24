use crate::Records;
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
        let key = key.expect("error: no key prvided");
        let new_record = Records { domain, key };
        records.push(new_record);
    };
    //serde serialization
    let serialized = serde_json::to_string(&records).expect("couldnt serialize");
    write("~/.config/ddns-updater", serialized).expect("couldnt write to file");
}

pub fn load_records() -> Vec<Records> {
    //serde deserializarion
    let records_string = read_to_string("~/.config/ddns-updater").expect("couldnt deserialize");
    let records: Vec<Records> =
        serde_json::from_str(&records_string).expect("deserialization error");
    return records;
}
