use std::collections::HashMap;

use crate::{
    handlers::device::device::USBDevice
};

pub fn write(plain: std::ffi::OsString, dev: &mut USBDevice) {
    let plain_string: String = plain.to_str()
        .unwrap()
        .to_string();
        
    let mut map: HashMap<u8, u8> = HashMap::new();

    if plain.to_str().unwrap() == "port" {
        dev.bus_chan = None;
        return ()
    }

    map.insert(
        plain_string.split("-")
            .nth(0)
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap(),

        plain_string.split("-")
            .nth(1)
            .unwrap()
            .trim()
            .parse::<u8>()
            .unwrap(),
    );

    dev.bus_chan = Some(map);
}