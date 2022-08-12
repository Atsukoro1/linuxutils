/*
    Author - Jakub Dornicak

    A Linux lspci-like command &line tool to list
    through usb devices

    Licensed under GNU General Public License version 2
*/
use std::{
    fs,
    path,
    io::Read
};

mod handlers;

static DEFAULT_BUS_PATH: &str = "/sys/bus/usb/devices";

fn parse_dev(entry: &mut fs::DirEntry) {
    let mut descriptor: [u8; 1] = [1;1];

    let mut entryDir: fs::ReadDir = entry.path()    
        .read_dir()
        .unwrap();

    entryDir.find(|res| {
        res.as_ref().unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .eq("bConfigurationValue")
            
    })
    .and_then(|res| {
        std::fs::File::open(res.unwrap().path())
            .expect("Failed to open configuration value file")
            .read(&mut descriptor)
            .expect("Failed to read configuration value file");

        Some(1)
    });

    /*
        Matching a USB device descriptor, every device, interface etc. 
        will be handled in different file

        Descriptors and their properties can be
        found at https://www.keil.com/pack/doc/mw/USB/html/
        under the USB Concepts -> USB descriptors table
    */
    match std::str::from_utf8(&descriptor)
        .unwrap()
        .parse::<u8>()
        .unwrap() {
            /*
                The Device Descriptor (USB_DEVICE_DESCRIPTOR) is the root of the descriptor tree 
                and contains basic device information. 

                The unique numbers, idVendor and idProduct, identify the connected device. 

                Operating system is then using these numbers to determine which device driver must be loaded.

                As defined at https://www.keil.com/pack/doc/mw/USB/html/_u_s_b__device__descriptor.html
            */
            1 => {
                handlers::device::device::parse(entry);
            }

            /*
                The Configuration Descriptor (USB_CONFIGURATION_DESCRIPTOR) contains information 
                about the device power requirements and the number of interfaces it can support. 

                A device can have multiple configurations. 

                The host can select the configuration that best matches 
                the requirements of the application software.

                As defined at https://www.keil.com/pack/doc/mw/USB/html/_u_s_b__configuration__descriptor.html
            */
            2 => {
                panic!("Configurators are not implemented for now!");
            }

            /*
                The Interface Descriptor (USB_INTERFACE_DESCRIPTOR) defines the collection of endpoints. 
                This interface supports a group of pipes that are suitable for a particular task. 
                Each configuration can have multiple interfaces. 
                The interface can be selected dynamically by the USB Host. 
                The Interface Descriptor can associate its collection of pipes with a device class, 
                which in turn has an associated class device driver within the host operating system. 
                Typically, the device class is a functional type such as a printer class or mass storage class.

                An interface descriptor never includes Endpoint 0 in the numbering of endpoints. If an interface uses only Endpoint 0, then the field bNumEndpoints must be set to zero.

                If no class type has been selected for the device, then none of the standard USB drivers is loaded, and the developer has to provide its own device driver.
                
                As defined at https://www.keil.com/pack/doc/mw/USB/html/_u_s_b__interface__descriptor.html
            */
            3 => {
                panic!("Interfaces are not implemented for now!");
            }

            /*
                The Endpoint Descriptor (USB_ENDPOINT_DESCRIPTOR) specifies the transfer type, 
                direction, polling interval, and maximum packet size for each endpoint. 

                Endpoint 0 (zero), the default endpoint, is always assumed to be a control
                endpoint and never has a descriptor.

                As defined at https://www.keil.com/pack/doc/mw/USB/html/_u_s_b__endpoint__descriptor.html
            */
            4 => {
                panic!("Endpoints are not implemented for now!");
            }

            /*
                String descriptors (USB_STRING_DESCRIPTOR) are optional and add human readable information
                to the other descriptors. If a device does not support string descriptors, all references
                to string descriptors within device, configuration, and interface descriptors must be set 
                to zero.

                String descriptors are encoded in Unicode so that multiple languages can be supported 
                with a single product. When requesting a string descriptor, the requester specifies the
                desired language using a 16-bit language ID (LANGID) 
                defined by the USB-IF (refer to Language Identifiers (LANGIDs)). 
                
                String index zero is used for all languages and returns a string descriptor that contains
                an array of two-byte LANGID codes supported by the device.

                The array of LANGID codes is not NULL-terminated. The size of the array (in byte) is computed
                by subtracting two from the value of the first byte to the descriptor.
                
                As defined at https://www.keil.com/pack/doc/mw/USB/html/_u_s_b__string__descriptor.html
            */
            5 => {
                panic!("String descriptors are not implemented for now!");
            }

            /*
                A high-speed capable device that has different device information for full-speed and high-speed 
                must have a Device Qualifier Descriptor (USB_DEVICE_QUALIFIER_DESCRIPTOR). 
            
                For example, if the device is currently operating at full-speed, the Device Qualifier returns 
                information about how it would operate at high-speed and vice-versa.

                The fields for the vendor, product, device, manufacturer, and serial number are not included. 
            
                This information is constant for a device regardless of the supported speeds.

                If a full-speed only device receives a GetDescriptor() request for a device_qualifier, 
                it must respond with a request error. Then, the host must not make a request for an 
                other_speed_configuration descriptor.

                As defined at https://www.keil.com/pack/doc/mw/USB/html/_u_s_b__device__qualifier__descriptor.html
            */
            6 => {
                panic!("Device qualifiers are not implemented for now!");
            }

            _ => {
                panic!(
                    "Descriptor type: {} is not implemeted", 
                    descriptor.first().unwrap()
                );
            }
    }
}

fn list_devices(entry: &fs::DirEntry) -> Vec<fs::DirEntry> {
    let mut dev_vec: Vec<fs::DirEntry> = vec![];
    let p_buf: path::PathBuf = entry.path();

    p_buf.read_dir()
        .unwrap()
        .into_iter()
        .for_each(|res: Result<fs::DirEntry, std::io::Error>| {
            let name: Vec<char> = res.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .unwrap()
                .chars()
                .collect();

            if name[0].is_numeric() && name[1] == '-' && name[2].is_numeric() && name.len() == 3 {
                dev_vec.push(res.unwrap());
            }
        });

    dev_vec
}

fn list_bus() -> Vec<fs::DirEntry> {
    let mut valid_bus_files: Vec<fs::DirEntry> = vec![];

    let bus_files: fs::ReadDir = match fs::read_dir(
        path::Path::new(&DEFAULT_BUS_PATH)
            .as_os_str()
    ) {
        Ok(bus) => bus.into_iter(),
        Err(err) => panic!(
            "Failed to read from {} directory.",
            err.to_string()
        ) 
    };

    bus_files.for_each(|file: Result<fs::DirEntry, std::io::Error>| {
        let f_name: String = file.as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .to_string();

        if f_name.starts_with("usb") {
            valid_bus_files.push(file.unwrap());
        }
    });

    valid_bus_files
}

fn main() {
    list_bus()
        .into_iter()
        .for_each(|ntr: fs::DirEntry| {
            for mut device in list_devices(&ntr) {
                parse_dev(&mut device)
            }
        });
}