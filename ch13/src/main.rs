use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// fn simulated_expensive_calculation(intensity: u32) -> u32{
//     println!("calculating slowly ...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value_map: HashMap<u32, u32>
}

impl<T> Cacher<T>
    where T:Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T>{
        Cacher{
            calculation,
            value_map: HashMap::new(),
        }
    }
    fn value(&mut self, arg: u32) -> &u32{
        if self.value_map.contains_key(&arg) {
            return &self.value_map[&arg]
        }
        self.value_map.insert(arg.clone(), (self.calculation)(arg.clone()));
        self.value_map.get(&arg).unwrap()
    }

    fn value_second(&mut self, arg: u32) -> u32{
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
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    )
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
