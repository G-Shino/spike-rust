#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrSecond{
    V4(String),
    V6(String),
}


#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

fn plus_one(x: Option<i32>) -> i32 {
    match x{
        None => 0,
        Some(i) => i,
    }
}

fn main() {
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let home = IpAddrSecond::V4(String::from("127.0.0.1"));
    let loopback = IpAddrSecond::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);
    
    println!("Hello, world!");

    let m = Message::Write(String::from("hello"));
    m.call();
    let m = Message::Quit;
    m.call();
    let m = Message::Move{x: 10, y: 10};
    m.call();
    let m = Message::ChangeColor(5, 5, 5);
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{}, {}", six, none);

    let some_u8_value = Some(0u8);
    match some_u8_value{
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = some_u8_value{
        println!("three");
    }
}
