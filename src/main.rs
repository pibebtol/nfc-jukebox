use nfc::{Nfc, NfcError};

fn main() -> Result<(), NfcError> {
    // Initialize NFC
    let mut nfc = Nfc::new()?;

    // Open the NFC device
    let device = nfc.open()?;

    // Wait for a tag to be present
    println!("Place an NFC tag on the reader...");
    let tag = device.wait_for_tag()?;

    // Read the UID of the tag
    println!("Tag UID: {:?}", tag.uid());

    Ok(())
}

// use rusb::{Context, Device, DeviceHandle, UsbContext, Error};
// use std::{os::unix::process, process::Command, usize};

// fn main() -> Result<(), Error> {
//     use_rusb()?;
//     Ok(())
// }

// fn use_rusb() -> Result<(), Error> {
//     Command::new("ls")
//         .arg("-l")
//         .arg("-a")
//         .status()
//         .expect("ls command failed to start");
//     // Command::new("spotify_player")
//     //     .arg("playback")
//     //     .arg("start")
//     //     .arg("context")
//     //     .arg("album")
//     //     .arg("--id")
//     //     .arg("3jPF3K9Uar57XfdHZiToKa")
//     //     .status()
//     //     .expect("spotify failed");

//     // return Ok(());
//     let context = Context::new().unwrap();

//     let devices = context.devices().unwrap();

//     for device in devices.iter() {
//         let descriptor = device.device_descriptor().unwrap();
//         println!(
//             "Bus: {:03x} Device: {:?} VID: {:04x}, PID: {:04x}",
//             device.bus_number(),
//             device.port_number(),
//             descriptor.vendor_id(),
//             descriptor.product_id()
//         );
//     }

//     let target_vid = 0x072f; // Replace with your device's vendor ID
//     let target_pid = 0x2200; // Replace with your device's product ID

//     if let Some(device) = devices.iter().find(|d| {
//         let desc = d.device_descriptor().unwrap();
//         desc.vendor_id() == target_vid && desc.product_id() == target_pid
//     }) {
//         let mut handle: DeviceHandle<Context> = device.open().unwrap();
//         // Now you can communicate with the device using `handle`
//         // For example, you can claim an interface, read/write data, etc.

//         println!("uiae resetting handle");
//         // handle.detach_kernel_driver(0)?;
//         handle.set_auto_detach_kernel_driver(true)?;
//         println!("uiae claiming interface");
//         handle.claim_interface(0)?;
//         println!("uiae claimed interface");

//         let mut buffer = [0u8; 40]; // Adjust the size based on your device's requirements
//         // Read data from the device
//         for i in 0..10 {
//             match handle.read_bulk(0x81, &mut buffer, std::time::Duration::from_secs(5)) {
//                 Ok(size) => {
//                     println!("Read {} bytes: {:?}", size, &buffer[..size]);
//                     process_buffer(&buffer[..size]);

//                     // let data = [0xFF, 0x00, 0x40, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00];
//                     // let data = [0x90, 0x0A, 0x00, 0x00, 0x01, 0x00, 0x00];
//                     let data = [0x0A, 0x00];
//                     match handle.write_bulk(0x02, &data, std::time::Duration::from_secs(2)) {
//                         Ok(size) => {
//                             println!("wrote {} bytes: {:?}", size, &data[..size]);
//                         }
//                         Err(e) => {
//                             eprintln!("Error writing to device: {:?}", e);
//                             break;
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     eprintln!("Error reading from device: {:?}", e);
//                     break;
//                 }
//             }
//         }

//         // match handle.read_bulk(0x81, &mut buffer, std::time::Duration::from_secs(5)) {
//         //     Ok(size) => {
//         //         println!("Read {} bytes: {:?}", size, &buffer[..size]);
//         //         process_buffer(&buffer[..size]);

//         //         let data = [0xFF, 0x00, 0x40, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00];
//         //         match handle.write_bulk(0x02, &data, std::time::Duration::from_secs(2)) {
//         //             Ok(size) => {
//         //                 println!("wrote {} bytes: {:?}", size, &data[..size]);
//         //             }
//         //             Err(e) => {
//         //                 eprintln!("Error writing to device: {:?}", e);
//         //             }
//         //         }
//         //     }
//         //     Err(e) => {
//         //         eprintln!("Error reading from device: {:?}", e);
//         //     }
//         // }

//         // Release the interface and close the handle
//         handle.release_interface(0)?;
//         // handle.attach_kernel_driver(0)?;


//         // Don't forget to release the interface and close the handle when done
//         // handle.release_interface(0).unwrap(); // Replace with the correct interface number
//     } else {
//         println!("Device not found");
//     }
//     Ok(())
// }

// fn process_buffer(buffer: &[u8]) {
//     // Example: Print each byte in hexadecimal format
//     for byte in buffer {
//         print!("{:02} ", byte);
//     }
//     println!();
    
//     if buffer[1] == 0x03 { 
//         println!("nfc tag added");
//         Command::new("spotify_player")
//             .arg("playback")
//             .arg("play")
//             .status()
//             .expect("spotify failed");
//         // Command::new("spotify_player")
//         //     .arg("playback")
//         //     .arg("start")
//         //     .arg("context")
//         //     .arg("album")
//         //     .arg("--id")
//         //     .arg("3jPF3K9Uar57XfdHZiToKa")
//         //     .status()
//         //     .expect("spotify failed");
//     }

//     if buffer[1] == 0x02 {
//         println!("nfc tag removed");
//         Command::new("spotify_player")
//             .arg("playback")
//             .arg("pause")
//             .status()
//             .expect("spotify failed");
//     }


//     // Example: Interpret the data based on your device's protocol
//     // This is just a placeholder; you need to adapt it to your specific use case.
//     // For example, if the buffer contains a string:
//     if let Ok(string) = std::str::from_utf8(buffer) {
//         println!("Received string: {}", string);
//     } else {
//         println!("Received non-UTF-8 data.");
//     }

//     // Add more processing logic based on the expected data format
// }
