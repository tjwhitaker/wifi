mod osx;
mod linux;
mod windows;

#[derive(Debug)]
pub struct Network {
    agrCtlRSSI: i32,
    agrExtRSSI: i32,
    agrCtlNoise: i32,
    agrExtNoise: i32,
    state: String,
    op_mode: String, 
    lastTxRate: i32,
    maxRate: i32,
    lastAssocStatus: i32,
    wireless_auth: String,
    link_auth: String,
    BSSID: String,
    SSID: String,
    MCS: i32,
    channel: String,
}

pub fn get_info() -> Network {
    osx::get_info()
}
