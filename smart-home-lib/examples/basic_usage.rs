use smart_home_lib::prelude::*;

fn main() {
    let therm = SmartTherm::new(22.5);
    let mut socket = SmartSocket::new(1500.0);
    socket.turn_on();

    let kitchen = Room::new(vec![SmartDevice::Therm(therm)]);
    let living_room = Room::new(vec![SmartDevice::Socket(socket)]);

    let mut house = SmartHouse::new(vec![kitchen, living_room]);

    println!("=== Initial Report ===");
    println!("{}", house.report().join("\n"));

    if let SmartDevice::Socket(s) = house.get_room_mut(1).get_device_mut(0) {
        s.turn_off();
    }

    println!("\n=== After Turning Off Socket ===");
    println!("{}", house.report().join("\n"));
}
