[![Travis Build Status](https://travis-ci.org/camp4climber/wifi.svg?branch=master)](https://travis-ci.org/camp4climber/wifi)
[![crates.io](https://img.shields.io/crates/v/wifi.svg)](https://crates.io/crates/wifi)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

This is a crate for interacting with wifi networks.

The module uses common command line tools.

- airport on Mac OS-X: airport -s
- netsh on Windows: netsh wlan show networks mode=Bssid
- iwlist on Linux: iwlist scan
