use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("{:?}", v);
    //糖衣構文
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("{}", third);
    let third: &i32 = match v.get(2){
        Some(val) => val,
        None => {
            println!("error");
            return;
        }
    };
    println!("{}", third);

    let v = vec![100, 32, 57];
    for i in &v{
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{}", i);
        *i += 50;
    }
    for i in &v{
        println!("{}", i);
    }

    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{}", s1);
    println!("{}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{}", s3);
    println!("{}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{}", s);
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);
    println!("{:?}", scores);
    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
    println!("{:?}", teams);
    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("{:?}", map);
    let field_value = String::from("Blue");
    map.get(&field_value);
    // println!("{:?}", field_name);
    // println!("{:?}", field_value);
    

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
