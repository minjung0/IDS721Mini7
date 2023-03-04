use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Find the number with the value closest to 0!");

    println!("Please input a list of numbers separated by comma:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut numbers: Vec<i32> = Vec::new();

    for number in input.split(",") {
        let number = number.trim().parse::<i32>().unwrap();
        numbers.push(number);
    }

    let result = find_closest_numb(numbers);

    println!("The number with the value closest to 0 is: {}", result);
}

fn find_closest_numb(nums: Vec<i32>) -> i32 {
    let mut diff = i32::MAX;
    let mut result = 0;

    for num in nums {
        match diff.cmp(&num.abs()) {
            Ordering::Greater => {
                diff = num.abs();
                result = num;
            }
            Ordering::Equal if num > result => result = num,
            _ => {}
        };
    }

    result
}