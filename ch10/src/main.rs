fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for item in list.iter(){
        if *item > largest {
            largest = *item
        }
    }
    // for &item in list.iter(){
    //     if item > largest {
    //         largest = item
    //     }
    // }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn largest<T>(list: &[T]) -> T
where
    // T: std::cmp::PartialOrd + std::marker::Copy,
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    };
    largest
}

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

impl<T> Point<T>{
    fn x(&self) -> &T {
        &self.x
    }
}

// Tがf32のときのみ実装される
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointBoth<T, U>{
    x: T,
    y: U,
}

impl <T, U> PointBoth<T, U>{
    fn mixup<V, W>(self, other: PointBoth<V, W>) -> PointBoth<T, W> {
        PointBoth {
            x: self.x,
            y: other.y,
        }
    }
}

pub trait Summary{
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String{
        String::from("read more")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len(){
        println!("{}", x);
        // x
    }else{
        println!("{}", y);
        // y
    }
    "aaa"
}


fn main() {
    let number_list = vec![34, 50, 25, 100];
    let result = largest_i32(&number_list);
    println!("result: {}", result);
    let number_list = vec![102, 34, 6000, 54, 2, 43, 8];
    let result = largest_i32(&number_list);
    println!("result: {}", result);
    let char_list = vec!['a', 'b', 'c', 'y', 'b', 'm'];
    let result = largest_char(&char_list);
    println!("result: {}", result);
    let number_list = vec![102, 34, 6000, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("result: {}", result);
    let char_list = vec!['a', 'b', 'c', 'y', 'b', 'm'];
    let result = largest(&char_list);
    println!("result: {}", result);

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    // let error = Point {x: 1.0, y: 10};
    println!("{:?}", integer);
    println!("{:?}", float.distance_from_origin());
    
    let both_float = PointBoth {x: 1.0, y: 1.0};
    let integer_and_float = PointBoth {x: 1.0, y: 5};
    println!("{:?}", both_float);
    println!("{:?}", integer_and_float);
    let both_integer = PointBoth {x: 2, y: 2};
    let mixup = both_float.mixup(both_integer);
    println!("{:?}", mixup);

    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());

    //トレイト
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("author: {}", tweet.summarize_author());

    //ライフタイム
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("{}", r);
    println!("{}", longest("aaa","bbbbb"));
    longest_with_an_announcement("aaa", "bb", "ccc");
}
