
use std::fs;
use hidapi::HidApi;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::{ thread};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WinchState {
    Idle,
    Winding,
    Unwinding,
}
impl WinchState {
    fn apply(&self) {
        match self {
            &WinchState::Idle => {}
            &WinchState::Winding => {}
            &WinchState::Unwinding => {}
        }
    }
}
pub struct RelayHid {
    vendor_id : u16,
    product_id : u16,
    device : hidapi::HidDevice,

}

impl RelayHid {
    pub fn set_state(&mut self, new_state : WinchState) {
        match new_state {
            WinchState::Idle => {
                self.turn_off_relay_1();
                self.turn_off_relay_2();
                println!(" Winch turned off");
            }
            WinchState::Winding => {
                self.turn_off_relay_1();
                self.turn_off_relay_2();
                thread::sleep(Duration::from_millis(250));
                self.turn_on_relay_1();
                thread::sleep(Duration::from_millis(250));
                println!("Winch is Winding");
            }
            WinchState::Unwinding => {
                self.turn_off_relay_1();
                self.turn_off_relay_2();
                thread::sleep(Duration::from_millis(250));
                self.turn_on_relay_2();
                thread::sleep(Duration::from_millis(250));
                println!(" Winch is Unwinding");
            }
        }
    }
    pub fn connect(vendor_id : u16, product_id : u16) -> Self{
        let api = HidApi::new().unwrap();
        let device = match api.open(vendor_id, product_id) {
            Ok(device) => device,
            Err(error) => panic!(" Erroe while connection to device {:#?}", error)
        };
        RelayHid{vendor_id, product_id, device}
    }
    fn turn_on_relay_1 (&mut self) {
        let turn_on_command : [u8; 9] = [0x00, 0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.device.send_feature_report(&turn_on_command).unwrap();
        println!("Tunrn on realy 1");

    }
    fn turn_off_relay_1 (&mut self) {
        let relay1_off: [u8; 9] = [0x00, 0xFD, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.device.send_feature_report(&relay1_off).unwrap();
        println!("Tunrn off really 1");
    }
    fn turn_on_relay_2 (&mut self) {
        let relay2_on:  [u8; 9] = [0x00, 0xFF, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.device.send_feature_report(&relay2_on).unwrap();
        println!("Tunrn on really 2");
    }
    fn turn_off_relay_2 (&mut self) {
        let relay2_off: [u8; 9] = [0x00, 0xFD, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        self.device.send_feature_report(&relay2_off).unwrap();
        println!("Tunrn off really 2");
    }


}

