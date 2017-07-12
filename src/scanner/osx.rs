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
    let lines = string.lines();
    let mut hash = HashMap::new();

    for l in lines {
        let split = l.trim().split(":").collect::<Vec<&str>>();
        hash.insert(split[0], split[1].trim());
    }

    let current_network = Network {
        agr_ctl_rssi: hash.get("agrCtlRSSI").unwrap().parse::<i32>().unwrap(),
        agr_ext_rssi: hash.get("agrExtRSSI").unwrap().parse::<i32>().unwrap(),
        agr_ctl_noise: hash.get("agrCtlNoise").unwrap().parse::<i32>().unwrap(),
        agr_ext_noise: hash.get("agrExtNoise").unwrap().parse::<i32>().unwrap(),
        state: hash.get("state").unwrap().to_string(),
        op_mode: hash.get("op mode").unwrap().to_string(), 
        last_tx_rate: hash.get("lastTxRate").unwrap().parse::<i32>().unwrap(),
        max_rate: hash.get("maxRate").unwrap().parse::<i32>().unwrap(),
        last_assoc_status: hash.get("lastAssocStatus").unwrap().parse::<i32>().unwrap(),
        wireless_auth: hash.get("802.11 auth").unwrap().to_string(),
        link_auth: hash.get("link auth").unwrap().to_string(),
        bssid: hash.get("BSSID").unwrap().to_string(),
        ssid: hash.get("SSID").unwrap().to_string(),
        mcs: hash.get("MCS").unwrap().parse::<i32>().unwrap(),
        channel: hash.get("channel").unwrap().to_string(),
    };

    current_network
}
