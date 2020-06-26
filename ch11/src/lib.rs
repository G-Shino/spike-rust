#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a+2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}


pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1  {
            panic!("less {}", value);
        } else if  value > 100 {
            panic!("Greater {}", value);
        }
        Guess {
            value
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another(){
    //     panic!("Make this test fail");
    // }

    use super::*;
    #[test]
    fn  larger_can_hold_smaller(){
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2));
        assert_eq!(add_two(2), 4);
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "result: {}", result);
    }

    #[test]
    #[should_panic(expected = "Greater")]
    fn greater_than_100(){
        Guess::new(200);
    }
}
