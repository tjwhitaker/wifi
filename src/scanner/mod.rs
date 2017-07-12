mod osx;
mod linux;
mod windows;

#[derive(Debug)]
pub struct Network {
    agr_ctl_rssi: i32,
    agr_ext_rssi: i32,
    agr_ctl_noise: i32,
    agr_ext_noise: i32,
    state: String,
    op_mode: String, 
    last_tx_rate: i32,
    max_rate: i32,
    last_assoc_status: i32,
    wireless_auth: String,
    link_auth: String,
    bssid: String,
    ssid: String,
    mcs: i32,
    channel: String,
}

pub fn get_info() -> Network {
    osx::get_info()
}
