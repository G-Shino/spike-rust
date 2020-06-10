use std::io;

fn main() {
    println!("Please select mode");
    println!("input 1 -> celsius to fahrenheit");
    println!("input 2 -> fahrenheit to celsius");
    
    let mut mode: u32;
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        mode = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input number");
                continue;
            },
        };
        match mode {
            1 => {
                convert_celsius_to_fahrenheit();
                break;
            },
            2 => {
                convert_fahrenheit_to_celsius();
                break;
            },
            _ => {
                println!("Please input 1 or 2");
                continue;
            },
        };
    }
}

fn convert_celsius_to_fahrenheit(){
    println!("++++++++++++++++++++++++++++++++++++++++++++");
    println!("Convert celsius to fahrenheit");
    println!("Please input celsius");
    let celsius: f64;
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        celsius = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input number");
                continue;
            },
        };
        break;
    }
    let fahrenheit = celsius * 1.8 + 32.0;
    println!("{} °F", fahrenheit);
}

fn convert_fahrenheit_to_celsius(){
    println!("++++++++++++++++++++++++++++++++++++++++++++");
    println!("Convert fahrenheit to celsius");
    println!("Please input fahrenheit");
    let fahrenheit: f64;
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        fahrenheit = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please input number");
                continue;
            },
        };
        break;
    }
    let celsius =  (fahrenheit - 32.0) / 1.8;
    println!("{} °C", celsius);
}
