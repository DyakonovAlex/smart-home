//! # Smart Home Library

pub mod controllers;
pub mod devices;
pub mod emulators;
pub mod house;
pub mod protocol;
pub mod room;
pub mod traits;
pub mod units;

pub mod prelude {
    pub use super::{
        controllers::{
            DeviceController, SocketController, SocketError, SubscriptionHandle, ThermController,
            ThermError,
        },
        devices::{Device, SmartSocket, SmartTherm},
        emulators::{EmulationScenario, SocketEmulator, ThermEmulator},
        house, // макрос
        house::{SmartHouse, SmartHouseError},
        protocol::{SocketCommand, SocketData, SocketResponse, ThermData, send_command},
        room, // макрос
        room::Room,
        traits::Reporter,
        units::{Celsius, Watts},
    };
}
