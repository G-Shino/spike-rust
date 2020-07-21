fn main() {
    // ここは本来は入力に当たる
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // if letとかと混ぜたもの
    if let Some(color) = favorite_color{
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday{
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // while let条件分岐ループ
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let some_option_value = Some(2);
    if let Some(x) = some_option_value{
        println!("{}", x);
    };

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    // マッチ内部は外部とは異なる環境となる マッチガードが必要になる
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // match式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    let x = 5;
    match x {
        1 ..= 5 => println!("one thorugh five"),
        _ => println!("something else")
    }

    let x = 'c';
    match x {
        'a' ..= 'j' => println!("early ASCII letter"),
        'k' ..= 'z' => println!("late ASCII letter"),
        _ => println!("something else")
    }

    // 構造体の分配
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point{x: 0, y: 7};
    let Point{x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
    let p = Point{x: 1, y: 2};
    let Point{x: a, y: b} = p;
    assert_eq!(1, a);
    assert_eq!(2, b);

    let p = Point {x: 0, y: 7};
    match p {
        Point {x, y: 0} => println!("On the x axis at {}", x),
        Point {x: 0, y} => println!("On the y axis at {}", y),
        Point {x, y} => println!("On neither axis: ({}, {})", x, y),
    }

    // enumの分配
    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32)
    }
    let msg0 = Message::ChangeColor(0, 160, 255);
    let msg1 = Message::Move{x: 1, y: 2};
    let msg2 = Message::Quit;
    let msg3 = Message::Write(String::from("hoge"));
    fn match_msg(msg: &Message) {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure");
            },
            Message::Move {x, y} => {
                println!("Move in the x direction {} and the y direction {}", x, y);
            },
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to (r, g, b) = ({}, {}, {})", r, g, b);
            }
        }
    }
    match_msg(&msg0);
    match_msg(&msg1);
    match_msg(&msg2);
    match_msg(&msg3);

    // 参照を分配する
    let points = vec![
        Point {x: 0, y: 0},
        Point {x: 1, y: 5},
        Point {x: 10, y: -3},
    ];
    let sum_of_squares: i32 = points.iter().map(|&Point {x, y}| x*x + y*y).sum();
    println!("{}", sum_of_squares);

    // 値の無視
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3 {x: 0, y: 0, z: 0};
    match origin {
        Point3 {x, y: 1, z: 2} => println!("x is {}", x),
        Point3 {x: 0, y: 0, z: 2} => println!("y is {}", y),
        Point3 {z, ..} => println!("z is {}", z),
    }

    // ref, ref mut
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
    match robot_name {
        Some(ref mut name ) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // 追加の条件式 マッチガード
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // @束縛 値を指定しつつ変数として取得できる
    enum MessageHello {
        Hello { id: i32}
    }
    let msg = MessageHello::Hello{id: 5};
    match msg {
        MessageHello::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        MessageHello::Hello {id: 10..=12} => {
            println!("Found an id in another range")
        },
        MessageHello::Hello {id} => {
            println!("Found some other id: {}", id)
        }
    }

}