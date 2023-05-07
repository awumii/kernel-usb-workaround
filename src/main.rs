use std::{fs::File, io::Write, thread, time::Duration};
use rmesg;

fn main() {
    // read kernel logs
    let entries = rmesg::log_entries(rmesg::Backend::KLogCtl, true).unwrap();

    // reverse the loop because only the recent errors are relevant.
    for entry in entries.into_iter().rev() {
        // The error message can appear as "usb 2-4: device descriptor read/8, error -71", check for the part that always appears.
        if entry.message.contains("device descriptor read") {
            // Get the device, eg. "2-4:"
            let device = entry.message.split_whitespace().nth(1).unwrap();

            // Get the exact port, eg. "2"
            let port = device.split("-").nth(0).unwrap();

            print!("Detected malfunctioning usb port: {}, disabling...\n", port);
            disable_port(port.parse::<u8>().unwrap());
            break;
        }
    }
}

fn disable_port(port: u8) {
    let mut file = File::create(format!("/sys/bus/usb/devices/usb{}/authorized", port)).unwrap();
    file.write_all(b"0").unwrap();
    
    // This can take up to one minute, i should probably look into the kernel to see what causes it...
    match file.flush() {
        Ok(_) => println!("Flushed buffer successfully."),
        Err(e) => eprintln!("Error flushing buffer: {}", e),
    }
    
}
