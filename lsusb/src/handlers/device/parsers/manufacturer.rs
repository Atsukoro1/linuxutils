use std::io::Read;

use crate::handlers::device::device::{
    USBDevice
};

pub fn write(file: &std::fs::DirEntry, dev: &mut USBDevice) {
    let mut raw_b: [u8; 1] = [1;1];

    std::fs::File::open(file.path())
        .unwrap()
        .read(&mut raw_b)
        .expect("Failed to read device manufacturer file");
    
    dev.manufacturer = Some(format!(
        // Display value in string containing hexadecimal value
        "{:#04x}",
        raw_b.first()
            .unwrap()
    ));
}