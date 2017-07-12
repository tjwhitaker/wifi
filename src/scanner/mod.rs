mod osx;
mod linux;
mod windows;

#[derive(Debug)]
pub struct Network {
    pub agr_ctl_rssi: i32,
    pub agr_ext_rssi: i32,
    pub agr_ctl_noise: i32,
    pub agr_ext_noise: i32,
    pub state: String,
    pub op_mode: String, 
    pub last_tx_rate: i32,
    pub max_rate: i32,
    pub last_assoc_status: i32,
    pub wireless_auth: String,
    pub link_auth: String,
    pub bssid: String,
    pub ssid: String,
    pub mcs: i32,
    pub channel: String,
}

pub fn get_info() -> Network {
    osx::get_info()
}
