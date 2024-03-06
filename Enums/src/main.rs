use crate::UsState::Utah;

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Utah
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr1 {
    V4(String),
    V6(String)
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call (&self) {
    }
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);



fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
    println!("{:?}", m);

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;

    let quarter = Coin::Quarter(Utah);
    let penny = Coin::Penny;
    if let Coin::Quarter(state) = penny {
        println!("State quarter from {:?}!", state);
    }
    else {
        count += 1;
    }

}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
