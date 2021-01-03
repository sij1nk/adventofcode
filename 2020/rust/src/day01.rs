use std::fs;

fn partition_around(numbers: &Vec<i32>, middle: i32) -> (Vec<i32>, Vec<i32>) {
    numbers.iter().partition(|&n| n < &middle)
}

fn try_for_sum((low_nums, high_nums): (Vec<i32>, Vec<i32>), sum: i32) -> Option<(i32, i32)> {
    for low_num in low_nums {
        let &high_num = high_nums
            .iter()
            .rev()
            .find(|&&high_num| low_num + high_num <= sum)
            .unwrap();
        if low_num + high_num == sum {
            return Some((low_num, high_num));
        } else {
            continue;
        }
    }

    None
}

fn find_product(numbers: &Vec<i32>, sum: i32) -> Option<i32> {
    let (low_nums, high_nums) = partition_around(numbers, sum / 2);
    try_for_sum((low_nums, high_nums), sum).map(|(low_nums, high_nums)| low_nums * high_nums)
}

fn main() {
    let plain_text = fs::read_to_string("input.txt").unwrap();
    let mut numbers: Vec<i32> = plain_text
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
    numbers.sort();

    // Part 1
    match find_product(&numbers, 2020) {
        Some(product) => println!("{}", product),
        None => println!("Sum not found"),
    }

    // Part 2
    for number in numbers.iter() {
        let new_numbers = &mut numbers.clone();
        new_numbers.retain(|&n| n != 2020);
        match find_product(&new_numbers, 2020 - number) {
            Some(product) => {
                println!("{}", product * number);
                break;
            }
            None => (),
        }
    }
}
