use std::collections::HashMap;
use std::process::Command;
use scanner::Network;

pub fn get_info() -> Network {
    let output = Command::new("airport")
                         .arg("info")
                         .output()
                         .expect("Failed to execute scan");

    parse_info(String::from_utf8(output.stdout).unwrap())
}

fn parse_info(string: String) -> Network {
    let mut lines = string.lines();
    let mut hash = HashMap::new();

    for l in lines {
        let mut split = l.trim().split(":").collect::<Vec<&str>>();
        hash.insert(split[0], split[1].trim());
    }

    let current_network = Network {
        agrCtlRSSI: hash.get("agrCtlRSSI").unwrap().parse::<i32>().unwrap(),
        agrExtRSSI: hash.get("agrExtRSSI").unwrap().parse::<i32>().unwrap(),
        agrCtlNoise: hash.get("agrCtlNoise").unwrap().parse::<i32>().unwrap(),
        agrExtNoise: hash.get("agrExtNoise").unwrap().parse::<i32>().unwrap(),
        state: hash.get("state").unwrap().to_string(),
        op_mode: hash.get("op mode").unwrap().to_string(), 
        lastTxRate: hash.get("lastTxRate").unwrap().parse::<i32>().unwrap(),
        maxRate: hash.get("maxRate").unwrap().parse::<i32>().unwrap(),
        lastAssocStatus: hash.get("lastAssocStatus").unwrap().parse::<i32>().unwrap(),
        wireless_auth: hash.get("802.11 auth").unwrap().to_string(),
        link_auth: hash.get("link auth").unwrap().to_string(),
        BSSID: hash.get("BSSID").unwrap().to_string(),
        SSID: hash.get("SSID").unwrap().to_string(),
        MCS: hash.get("MCS").unwrap().parse::<i32>().unwrap(),
        channel: hash.get("channel").unwrap().to_string(),
    };

    current_network
}
