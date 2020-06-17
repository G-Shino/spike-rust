mod outermost {
    pub fn middle_function(){
        middle_secret_function()
    }
    fn middle_secret_function(){}
    pub mod inside{
        pub fn inner_function(){}
        fn secret_function(){}
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
    outermost::inside::inner_function();
    // outermost::inside::secret_function();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
