use std::io::Read;

use crate::{
    handlers::device::device::{
        USBClassUsage,
        USBDevice,
        USBDeviceClass
    }
};

pub fn write(file: &std::fs::DirEntry, dev: &mut USBDevice) {
    let mut final_str: String = String::new(); 

    match std::fs::File::open(file.path()) {
        Ok(mut raw) => {
            let mut temp: [u8; 2] = [1; 2];

            raw.read(&mut temp)
                .and_then(|size| {
                    final_str = std::str::from_utf8(&temp)
                        .unwrap()
                        .trim()
                        .to_string();

                    Ok(size)
                })
                .expect("Failed to read this file");
        },

        Err(..) => {
            panic!("Failed to read bClassDevice file for device")
        }
    };

    /*
        If you want more information about USB device classes,
        please reffer to https://microchipdeveloper.com/usb:device-classes#toc0
        or to https://www.usb.org/defined-class-codes for more detailed description
    */
    dev.b_class = Some(match final_str.to_ascii_uppercase().as_str() {
        "00" => {
            USBDeviceClass {
                description: "Use class information in the Interface Descriptors"
                    .to_string(),
                usage: USBClassUsage::Device
            }
        },

        "01" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Audio"
                    .to_string()
            }
        },

        "02" => {
            USBDeviceClass {
                usage: USBClassUsage::Both,
                description: "Communications and Communications Device Class (CDC) Control"
                    .to_string()
            }
        }

        "03" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Human Interface Device (HID)"
                    .to_string()
            }
        },

        "05" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Physical"
                    .to_string()
            }
        },

        "06" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Image"
                    .to_string()
            }
        },

        "07" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Printer"
                    .to_string()
            }
        },

        "08" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Mass Storage (MSD)"
                    .to_string()
            }
        },

        "FF" => {
            USBDeviceClass {
                usage: USBClassUsage::Both,
                description: "Vendor Specific"
                    .to_string()
            }
        }

        "FE" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Application specific"
                    .to_string()
            }
        },

        "EF" => {
            USBDeviceClass {
                usage: USBClassUsage::Both,
                description: "Miscellaneous"
                    .to_string()
            }
        },

        "0E" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Wireless Controller"
                    .to_string()
            }
        },

        "DC" => {
            USBDeviceClass {
                usage: USBClassUsage::Both,
                description: "Diagnostic Device"
                    .to_string()
            }
        },

        "11" => {
            USBDeviceClass {
                usage: USBClassUsage::Device,
                description: "Billboard"
                    .to_string()
            }
        },

        "10" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface, 
                description: "Audio/Video Devices"
                    .to_string()
            }
        },

        "0F" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Personal Healthcare"
                    .to_string()
            }
        },

        "0D" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Content Security"
                    .to_string()
            }
        },

        "0B" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "Smart Card"
                    .to_string()
            }
        },

        "0A" => {
            USBDeviceClass {
                usage: USBClassUsage::Interface,
                description: "CDC-Data"
                    .to_string()
            }
        },

        "09" => {
            USBDeviceClass {
                usage: USBClassUsage::Device,
                description: "8Hub"
                    .to_string()
            }
        },

        _ => {
            USBDeviceClass {
                usage: USBClassUsage::Unrecognizable,
                description: "\0"
                    .to_string()
            }
        }
    })
}