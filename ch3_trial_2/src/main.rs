use std::env;

fn main() {
    println!("フィボナッチ数列");
    let times = match env::args().nth(1){
        Some(times) => times,
        None => {
            println!("no args error");
            return;
        }
    };
    let times: u32 = match times.trim().parse(){
        Ok(times) => times,
        Err(_) => {
            println!("Please input number (0 ~)");
            return;
        },
    };
    println!("10番目: {}", fib(10));
    let mut two_before_val: u128 = 0;
    let mut previous_val: u128 = 0;
    let mut current_val: u128;

    for i in 0..times {
        if i < 2 {
            current_val = i as u128;
        }else{
            current_val = previous_val + two_before_val
        }
        println!("{}番: {}", i, current_val);
        two_before_val = previous_val;
        previous_val = current_val;
    }
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    }else{
        fib(n-1) + fib(n-2)
    }
}
