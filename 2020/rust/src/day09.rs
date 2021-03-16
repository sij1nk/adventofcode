use std::collections::VecDeque;

fn find_addends(nums: &[u64], sum: u64, preamble_len: usize) -> bool {
    let mut sorted_nums = vec![0; preamble_len];
    sorted_nums.copy_from_slice(&nums);
    sorted_nums.sort();
    let mut first_idx = 0;
    loop {
        let first = match sorted_nums.get(first_idx) {
            Some(n) => n,
            None => return false,
        };

        if *first == sum / 2 && sum % 2 == 0 {
            // If `first` is sum/2 and sum is an even number, binary search might find a match, but
            // if it's on the same index as `first`, we can't be sure if there's another number of
            // the same value right after it. Instead, we can just check if `first` and the
            // following value are equal.  (note that the slice is always sorted)
            let second_idx = first_idx + 1;
            if let Some(_) = sorted_nums.get(second_idx) {
                return true;
            } else {
                return false;
            }
        } else {
            let second = match sum.checked_sub(*first) {
                Some(n) => n,
                None => return false,
            };

            if let Ok(_) = sorted_nums.binary_search(&second) {
                return true;
            } else {
                first_idx += 1;
            };
        }
    }
}

fn find_first_invalid<'a, I, S>(lines: I, preamble_len: usize) -> Option<(u64, Vec<u64>)>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut iter = lines.into_iter();
    let mut nums: VecDeque<u64> = VecDeque::new();
    let mut before: Vec<u64> = Vec::new();
    let mut i = 0;

    while i < preamble_len {
        nums.push_back(iter.next()?.as_ref().parse::<u64>().ok()?);
        i += 1;
    }

    loop {
        let next = iter.next()?.as_ref().parse::<u64>().ok()?;

        nums.make_contiguous();
        if !find_addends(nums.as_slices().0, next, preamble_len) {
            before.extend_from_slice(nums.as_slices().0);
            return Some((next, before));
        } else {
            if let Some(num) = nums.pop_front() {
                before.push(num);
            }
            nums.push_back(next);
        }
    }
}

pub fn part1<'a, I, S>(lines: I, preamble_len: usize) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    find_first_invalid(lines, preamble_len).map(|v| v.0)
}

pub fn part2<'a, I, S>(lines: I, preamble_len: usize) -> Option<u64>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let (invalid, nums) = find_first_invalid(lines, preamble_len)?;
    let mut skip = 0;

    'outer: loop {
        let mut addends = vec![];
        let mut sum = 0;
        let mut iter = nums.iter().rev().skip(skip);
        while let Some(num) = iter.next() {
            sum += num;
            if sum > invalid {
                skip += 1;
                continue 'outer;
            }

            addends.push(num);
            if sum == invalid {
                return Some(addends.iter().min()?.checked_add(**addends.iter().max()?)?);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE, 5).unwrap();

        assert_eq!(result, 127);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE, 5).unwrap();

        assert_eq!(result, 62);
    }
}
