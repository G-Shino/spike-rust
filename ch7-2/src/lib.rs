mod outermost {
    pub fn middle_function(){
        middle_secret_function()
    }
    fn middle_secret_function(){}
    mod inside{
        pub fn inner_function(){}
        fn secret_function(){}
        fn secret_function_second(){
            self::secret_function();
        }
    }
    mod inside_second{
        pub fn inner_function(){}
    }
    pub fn middle_function_second(){
        self::inside_second::inner_function();
    }
    mod inside_third{
        pub fn inner_function(){
            super::inside_second::inner_function();
            super::inside::inner_function();
            crate::outermost::inside::inner_function();
        }
    }
    mod inside_use{
        use crate::outermost::inside_third;
        pub fn inner_function(){
            inside_third::inner_function();
        }
    }
}

pub mod a{
    pub mod series{
        pub mod of {
            pub fn nested_modules(){}
            pub enum Fuga{
                Hoge,
                Fuga
            }
        }
    }
}
enum TrafficLight{
    red,
    yellow,
    green
}

use a::series::of;
use a::series::of::Fuga::{Hoge, Fuga};
use TrafficLight::{red, yellow};

fn main(){
    of::nested_modules();
    let Red = red;
    let Yellow = yellow;
}

// outermost::middle_function!();

fn try_me(){
    outermost::middle_function();
    // outermost::middle_secret_function();
    // outermost::inside::inner_function();
    // outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
