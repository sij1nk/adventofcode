use std::{collections::HashMap, str::Bytes};

type Label = u8;
type FocalLength = u8;

use anyhow::anyhow;

fn words<'a, I, S>(lines: I) -> anyhow::Result<impl Iterator<Item = &'a str>>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    Ok(lines
        .into_iter()
        .map(|l| l.as_ref())
        .next()
        .ok_or(anyhow!("There is no input"))?
        .split(','))
}

fn label(letters: Bytes) -> u8 {
    letters.map(|v| v as u32).fold(0, |mut acc, value| {
        acc += value;
        acc *= 17;
        acc % 256
    }) as u8
}

pub fn part1<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    Ok(words(lines)?.fold(0, |acc, word| acc + label(word.bytes()) as u32))
}

pub fn part2<'a, I, S>(lines: I) -> anyhow::Result<u32>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut boxes: HashMap<Label, Vec<(&str, FocalLength)>> = HashMap::new();
    let mut sum = 0;

    for word in words(lines)? {
        if word.contains('-') {
            let label_str = &word[0..word.len() - 1];
            let label = label(label_str.bytes());
            let bx = boxes.entry(label).or_default();
            bx.retain(|&(str, _)| str != label_str);
        } else {
            let (label_str, focal_length_str) = word
                .split_once('=')
                .ok_or(anyhow!("Could not split word by '='"))?;
            let label = label(label_str.bytes());
            let focal_length = focal_length_str.parse::<u8>()?;
            let bx = boxes.entry(label).or_default();
            if let Some(lens_pos) = bx.iter().position(|&(str, _)| str == label_str) {
                bx[lens_pos] = (label_str, focal_length);
            } else {
                bx.push((label_str, focal_length));
            }
        }
    }

    for (label, bx) in boxes {
        for (i, &(_, focal_length)) in bx.iter().enumerate() {
            sum += (label as u32 + 1) * (i + 1) as u32 * (focal_length as u32);
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &["rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 1320);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 145);
    }
}
