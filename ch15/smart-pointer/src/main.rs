use List::{Cons, Nil};
use ListRc::{ConsRc, NilRc};
use std::ops::Deref;
use std::rc::Rc;


#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    NilRc,
}

struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartpointer with data '{}", self.data);
    }
}

fn main() {
    // ただし、単独の値をヒープにおく意味はない
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer{data: String::from("my stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};
    println!("CustomSmartPointers created.");
    println!("{:?}, {:?}", c, d);

    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(10, Rc::new(NilRc)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsRc(3, Rc::clone(&a));
    println!("{:?}", b);
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsRc(4, Rc::clone(&a));
        println!("{:?}", c);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count = {}", Rc::strong_count(&a));
}
