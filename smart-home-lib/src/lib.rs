//! # Smart Home Library

pub mod devices;
pub mod house;
pub mod room;

pub mod prelude {
    pub use super::{
        devices::{SmartDevice, SmartSocket, SmartTherm},
        house::SmartHouse,
        room::Room,
    };
}
