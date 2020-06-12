const MAX_POINTS: u32 = 1000;

fn main() {
    //constはグローバルな定義に使える
    println!{"const value: {}", MAX_POINTS}

    //可変変数
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    x = x+1;
    println!("The value of x is: {}", x);
    
    //constはグローバルな定義に使える
    println!{"const value: {}", MAX_POINTS}

    //シャドーイング・再束縛
    let x = 5;
    let x = x+1;
    println!("The value of x is : {}", x);

    let spaces = "    ";
    println!("spaces: {}!", spaces);
    let spaces = spaces.len();
    println!("spaces length: {}", spaces);

    //型
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!"); <- これはエラー
    println!("guess: {}", guess);

    let x = 2.0; //f64
    let y: f32 = 3.0; //f32
    println!("x: {}, y: {}", x, y);
    let x = 2; //i32
    let y: u64 = 3; //u64
    println!("x: {}, y: {}", x, y);

    let sum = 5+10;
    let difference = sum - 1;
    let product = 4 * difference;
    let quotient = product / 4;
    let remainder = quotient % 10;
    println!("result: {}", remainder);

    let t = true;
    let f: bool = false;
    println!("bool: {:?} or {:?}", t, f);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup: {:?}", tup);
    println!("(x, y, z): {}, {}, {}", x, y, z);
    println!("(x, y, z): {}, {}, {}", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    //関数
    another_function();
    let x = 5;
    let y = 0;
    print_xy(x, y);
    let x = plus_one(x);
    println!("plus one: {}", x);

    //if
    let number = 7;
    if number != 0 {
        println!("number was something other than zero");
    }
    if number < 5 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }
    //分岐が多い時はmatchがベター
    if number % 3 == 0 {
        println!("number is divisible by 3");
    }else if number % 5 == 0 {
        println!("number is divisible by 5");
    } else {
        println!("number is not divisible by 3, 5");
    }
    //結果を変数に束縛することもできる
    let condition = true;
    let number = if condition{
        5
    } else {
        6
    };
    println!("number: {}", number);

    //ループ制御
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number -1;
    }
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5{
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
    for element in a.iter() {
        println!("elem: {}", element);
    }
    for number in (1..4).rev(){
        println!("{}!", number);
    }
}

fn another_function() {
    println!("Another function.")
}

fn print_xy(x: i32, y: i32){
    println!("y; {}", y);
    println!("x: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
