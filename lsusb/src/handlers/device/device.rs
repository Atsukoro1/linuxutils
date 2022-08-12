use std::{collections::HashMap, ops::DerefMut};
use crate::handlers::device::parsers;

/*
    Class code is used to indentify device's functionality and to 
    nominally load a device driver based on that functionality.

    https://www.usb.org/defined-class-codes
*/
#[derive(Debug)]
pub struct USBDeviceClass {
    pub usage: USBClassUsage,
    pub description: String
}

#[derive(Debug)]
pub enum USBClassUsage {
    Device,
    Interface,
    Both,
    Endpoint,
    String,
    DeviceQualifier,
    Unrecognizable
}

#[derive(Debug)]
pub struct USBDevice {
    // Bus and channel number used to locate this directory
    pub bus_chan: Option<HashMap<u8, u8>>,

    /*
        Class code (assigned by the USB-IF).

        More at: https://www.keil.com/pack/doc/mw/USB/html/group__usbh__data__types.html#struct_u_s_b___d_e_v_i_c_e___d_e_s_c_r_i_p_t_o_r
    */
    pub b_class: Option<USBDeviceClass>,

    /*
        Name of the USB device
    */
    pub name: Option<String>,

    /*
        This value is the identificator of Manufacturer/Vendor and will be displayed 
        as a string containing hexadecimal value, if you want to know the vendor name
        and other information please visit https://the-sz.com/products/usbid/index.php
    */
    pub manufacturer: Option<String>
}

/*
    Parse function will parse /sys/bus/usb/*-*/ device directory
    and return USBDevice struct including all it's properties 
    from parsed files
*/
pub fn parse(dir: &mut std::fs::DirEntry) {
    let mut dev = USBDevice {
        bus_chan: None,
        b_class: None,
        name: None,
        manufacturer: None
    };

    // Bus and channel will be always defined
    parsers::bus_chan::write(
        dir.file_name(), 
        &mut dev
    );

    dir.path()
        .read_dir()
        .unwrap()
        .for_each(|file: Result<std::fs::DirEntry, std::io::Error>| {
            match file.as_ref().unwrap()
                .file_name()
                .to_str()
                .unwrap() {
                    "configuration" => {
                        parsers::configuration::retrieve(
                            file.as_ref()  
                                .unwrap(), 
                            &mut dev
                        );
                    },

                    "bDeviceClass" => {
                        parsers::bClass::write(
                            file.as_ref()
                                .unwrap(), 
                            &mut dev
                        );
                    },

                    "manufacturer" => {
                        parsers::manufacturer::write(
                            file.as_ref()
                                .unwrap(), 
                            &mut dev
                        );
                    },

                    _ => {}
                };
        });
}

/*
    Print function will print the device in human readable format depending
    on user's configuration
*/
pub fn print() {

}