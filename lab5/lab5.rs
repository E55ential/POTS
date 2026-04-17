#[derive(Debug)]
enum Event {
    CarArrived(i32),
    CarDoorOpened,
    CarDoorClosed,
    LobbyCallButtonPressed(i32, Direction),
    CarFloorButtonPressed(i32),
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::LobbyCallButtonPressed(floor, dir)
}

fn car_floor_button_pressed(floor: i32) -> Event {
    Event::CarFloorButtonPressed(floor)
}

fn main() {
    println!(
        "Пассажир на первом этаже нажал кнопку вызова: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("Лифт приехал на первый этаж: {:?}", car_arrived(0));
    println!("Дверь лифта открылась: {:?}", car_door_opened());
    println!(
        "Пассажир нажал кнопку третьего этажа: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Двери лифта закрылись: {:?}", car_door_closed());
    println!("Лифт прибыл на третий этаж: {:?}", car_arrived(3));
}