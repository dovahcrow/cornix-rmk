#![allow(unused)]

use rmk::config::KeyboardUsbConfig;

pub const MANUFACTURE: &'static str = "Jezail Funder Studio";
pub const PRODUCT_NAME: &'static str = "Cornix";
pub const VID: u16 = 0xe11b;
pub const PID: u16 = 0x0001;
pub const KEYBOARD_USB_CONFIG: KeyboardUsbConfig = KeyboardUsbConfig {
    vid: VID,
    pid: PID,
    manufacturer: MANUFACTURE,
    product_name: PRODUCT_NAME,
    serial_number: "vial:f64c2b3c:000001",
};

pub const INPUT_PIN_NUM: usize = 4;
pub const OUTPUT_PIN_NUM: usize = 7;

/// How many outgoing L2CAP buffers per link
pub const L2CAP_TXQ: u8 = 4;

/// How many incoming L2CAP buffers per link
pub const L2CAP_RXQ: u8 = 4;

/// Size of L2CAP packets
pub const L2CAP_MTU: usize = 251;

pub const CLEAR_STORAGE: bool = false;
