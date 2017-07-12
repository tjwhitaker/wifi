wifi
----

[![Travis Build Status](https://travis-ci.org/camp4climber/wifi.svg?branch=master)](https://travis-ci.org/camp4climber/wifi)
[![crates.io](https://img.shields.io/crates/v/wifi.svg)](https://crates.io/crates/wifi)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A rust library that will eventually allow for interacting with and getting information about local wifi networks. 

Not for production use yet! The code quality is terrible right now as I'm still learning rust, so I appreciate pull requests and or guidance if you have them!

The module uses the following command line tools and assumes they are installed on your system.

- airport on Mac OS-X

current features
----------------

get info about current network in osx

```
extern crate wifi;

use wifi::scanner;

fn main() {
  let network_info = scanner::get_info();
  println!("{:?}", network_info);
}
```

todo
---

- [ ] error handling
- [ ] write tests
- [ ] detect operating system
- [ ] scan networks
- [ ] linux support
- [ ] windows support

