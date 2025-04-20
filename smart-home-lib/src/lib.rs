//! # Smart Home Library

pub mod device;
pub mod house;
pub mod room;
pub mod traits;

pub mod prelude {
    pub use super::{
        device::{SmartDevice, SmartSocket, SmartTherm},
        house,
        house::SmartHouse,
        room,
        room::Room,
        traits::Reporter,
    };
}
