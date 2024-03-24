use read_process_memory::{copy_address, ProcessHandle};
use std::io;

/// Read n bytes from memory, potentially following higher level pointers
pub fn read_memory(address: usize, pid: u32, bytes: usize, offsets: Vec<usize>) -> io::Result<Vec<u8>> {
    let handle: ProcessHandle = pid.try_into()?;

    let mut pointer = address;
    let mut read_data;

    // If offsets are present, the data position is specified by a higher level pointer
    // Read data and cycle through the offsets until final pointer is reached
    for offset in offsets {
        read_data = copy_address(pointer, 4, &handle)?;
        pointer = u32::from_le_bytes(read_data.try_into().unwrap()) as usize;
        pointer += offset;
    }

    Ok(copy_address(pointer, bytes, &handle)?)
}

/// Decode u8 vector to 32 bit uint little endian value
pub fn decode_to_u32(bytes: io::Result<Vec<u8>>) -> Option<u32> {
    match bytes {
        Ok(bytes) => {
            if let Ok(value) = bytes.try_into() {
                Some(u32::from_le_bytes(value))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Decode u8 vector to 32 bit float little endian value
pub fn decode_to_f32(bytes: io::Result<Vec<u8>>) -> Option<f32> {
    match bytes {
        Ok(bytes) => {
            if let Ok(value) = bytes.try_into() {
                Some(f32::from_le_bytes(value))
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

/// Decode u8 vector to String
pub fn decode_to_string(bytes: Vec<u8>) -> Option<String> {
    match String::from_utf8(bytes) {
        Ok(string) => Some(string),
        Err(_) => None,
    }
}