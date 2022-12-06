use std::error::Error;
use std::fs::read_to_string;

pub fn sum_of_max_3() -> Result<i32, Box<dyn Error>>{
    let input = read_to_string("src/day_1_input")?;
    let mut calories_carrying = vec![0];

    for line in input.lines() {
        if line.is_empty() {
            calories_carrying.push(0);
        } else {
            *calories_carrying.last_mut().unwrap() += line.parse::<i32>()?;
        }
    }

    let mut max = vec![0, 0, 0];
    for calorie in calories_carrying.drain(..) {
        keep_greatest(&mut max, calorie);
    }

    return Ok(max.iter().fold(0, |acc, i| acc + *i));
}

fn keep_greatest<T: Ord>(vec: &mut Vec<T>, n: T) {
    for i in 0..vec.len() {
        if vec[i] < n {
            vec.insert(i, n);
            vec.pop();
            return
        }
    }
}
