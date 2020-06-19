use std::collections::HashMap;

fn main() {
    let mut val_list = vec![1, 2, 3, 3, 2, 3 , 3, 3, 3, 3, 3, 5, 3, 2, 7, 8, 0];
    let mut sum: i32 = 0;
    for val in &val_list{
        sum += val;
    }
    println!("sum: {}", sum);
    let sum = sum as f64;
    let length = val_list.len() as f64;
    let ave = sum / length;
    println!("ave: {}", ave);
    val_list.sort();
    let  median_key;
    if val_list.len() % 2 == 0 {
        median_key = val_list.len() / 2;
    }else{
        median_key = val_list.len() / 2 + 1;
    }
    println!("median: {}", val_list[median_key]);
    let mut mode_map = HashMap::new();
    for (i, val) in val_list.iter().enumerate(){
        println!("{}, {}", i, val);
        let count = mode_map.entry(*val).or_insert(0);
        *count += 1;
    }
    println!("{:?}", mode_map);
    let mut mode = 0;
    let mut mode_val = 0;
    for (key, value) in &mode_map{
        if mode_val < *value {
            mode = *key;
            mode_val = *value;
        }
    }
    println!("mode: {}", mode);
}
