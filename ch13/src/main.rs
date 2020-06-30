use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// fn simulated_expensive_calculation(intensity: u32) -> u32{
//     println!("calculating slowly ...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T, U>
    where T: Fn(U) -> U
{
    calculation: T,
    value_map: HashMap<U, U>
}

impl<T, U> Cacher<T, U>
    where T:Fn(U) -> U, U:Eq + std::hash::Hash + Copy
{
    fn new(calculation: T) -> Cacher<T, U>{
        Cacher{
            calculation,
            value_map: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> &U{
        if self.value_map.contains_key(&arg) {
            return &self.value_map[&arg]
        }
        self.value_map.insert(arg.clone(), (self.calculation)(arg.clone()));
        self.value_map.get(&arg).unwrap()
    }

    fn value_second(&mut self, arg: U) -> U{
        let result =  self.value_map.get(&arg);
        match result {
            Some(v) => {
                v.clone()
            }
            None => {
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, v);
                v
            }
        }
    }

    // fn value_error(&mut self, arg: u32) -> &u32 {
    //     let result = self.value_map.get(&arg);
    //     match result {
    //         Some(v) => {
    //             // &10
    //             v
    //         }
    //         None => {
    //             let v = (self.calculation)(arg);
    //             self.value_map.insert(arg, v);
    //             &v
    //             // &10
    //         }
    //     }
    // }
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25{
        let a = expensive_result.value_second(intensity);
        let b = expensive_result.value_second(intensity);
        println!("{}, {}", a, b);
        // let a = expensive_result.value_error(intensity);
        // let b = expensive_result.value_error(intensity);
        // println!("{}, {}", a, b);
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
        println!("Today, do {} pushups!", expensive_result.value_second(intensity+5));
        println!("Next, do {} situps!", expensive_result.value_second(intensity+5));
    }else{
        if random_number == 3 {
            println!("Take a break today! Remember to stay hedrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
    let mut expensive_result = Cacher::new(|quote: &str| {
        println!("calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        quote
    });
    println!("1 {}", expensive_result.value("hoge"));
    println!("2 {}", expensive_result.value("hoge"));
    println!("3 {}", expensive_result.value("hoge"));
    println!("4 {}", expensive_result.value("fuga"));
    println!("5 {}", expensive_result.value("fuga"));
    println!("6 {}", expensive_result.value("hoge"));
    println!("7 {}", expensive_result.value("fuga"));
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    
}


// impl<T> Cacher<T>
//     where T: Fn(u32) -> u32
// {
//     fn new(calculation: T) -> Cacher<T> {
//         Cacher {
//             calculation,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.calculation)(arg);
//                 self.value = Some(v);
//                 v
//             },
//         }
//     }
// }

#[cfg(test)]
mod test {
    #[test]
    fn iterator_demonstration(){
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        let v1: Vec<i32> = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x+1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }
}
