use std::io::Read;
use crate::handlers::device::device::USBDevice;

pub fn retrieve(plain: &std::fs::DirEntry, dev: &mut USBDevice) {
    let mut buf = [1; 1000];
    let mut final_slice: Box<&[u8]> = Box::new(&[1]);

    std::fs::File::open(plain.path())
        .expect(
            format!(
                "Failed to read configuration content of {:?} device",
                plain.file_name()
            ).as_str()
        )
    .read(&mut buf)
    .and_then(|size: usize| {
        final_slice = Box::from(&buf[0..size]);
        Ok(size)
    })
    .unwrap();

    if final_slice.len() == 0 {
        dev.name = None;
    } else {
        dev.name = Some(
            std::str::from_utf8(&final_slice)
                .unwrap()
                .to_string()
        );
    };
}