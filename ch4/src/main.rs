fn main() {
    println!("Hello, world!");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let mut s = String::from("hello");
    let r1 = &s;
    println!("{}", r1);
    change(&mut s);
    println!("{}", s);
    let s = no_dangle();
    println!("{}", s);
    let slice = &s[3..];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);
    let word = first_word(&s[..]);
    println!("{}", word);
    // s.clear(); clearしたあとに参照を使用することはできない
    // println!("{}", word);
}

fn no_dangle() -> String{
    let s = String::from("hello cat");
    s
    //&s  dangleでエラーになる　ライフタイムの問題
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
