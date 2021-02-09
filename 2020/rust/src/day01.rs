use super::util;
use std::error::Error;

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
    let (low_nums, high_nums) = numbers.into_iter().partition(|&&n| n < sum / 2);
    try_for_sum((low_nums, high_nums), sum).map(|(low_nums, high_nums)| low_nums * high_nums)
}

pub fn part1<'a, I, S>(lines: I) -> Result<Option<i32>, Box<dyn Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut nums = util::parse_many(lines)?;
    nums.sort();
    Ok(find_product(&nums, 2020))
}

pub fn part2<'a, I, S>(lines: I) -> Result<Option<i32>, Box<dyn Error + Send + Sync>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut nums: Vec<i32> = util::parse_many(lines)?;
    nums.sort();
    for num in nums.iter() {
        if let Some(value) = find_product(&nums, 2020 - num) {
            return Ok(Some(num * value));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["1721", "979", "366", "299", "675", "1456"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap().unwrap();
        assert_eq!(result, 514579);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap().unwrap();
        assert_eq!(result, 241861950);
    }
}
