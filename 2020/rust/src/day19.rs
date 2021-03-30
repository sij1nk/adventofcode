use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseRuleError {
    InvalidFormat(&'static str),
}

impl fmt::Display for ParseRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl Error for ParseRuleError {}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Symbol {
    Nonterminal(u8),
    Terminal(char),
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Nonterminal(n) => write!(f, "{}", n),
            Self::Terminal(c) => write!(f, "{}", c),
        }
    }
}

#[derive(Debug, Clone)]
enum Product {
    List(Vec<Product>),
    Branch(Vec<Product>, Vec<Product>),
    Symbol(Symbol),
}

#[derive(Default, Debug)]
struct Grammar {
    rules: HashMap<Symbol, Product>,
}

impl Grammar {
    fn add_rule(&mut self, line: &str) -> Result<(), ParseRuleError> {
        let mut words = line.split(' ');
        let (mut list1, mut list2) = (vec![], vec![]);
        let mut is_branch = false;

        let start_symbol = words
            .next()
            .ok_or(ParseRuleError::InvalidFormat("missing start symbol"))?
            .strip_suffix(':')
            .ok_or(ParseRuleError::InvalidFormat("missing colon"))?
            .parse::<u8>()
            .map_err(|_| ParseRuleError::InvalidFormat("not an integer"))?;
        let start_symbol = Symbol::Nonterminal(start_symbol);

        for word in words {
            if let Ok(num) = word.parse::<u8>() {
                if is_branch {
                    list2.push(Product::Symbol(Symbol::Nonterminal(num)));
                } else {
                    list1.push(Product::Symbol(Symbol::Nonterminal(num)));
                }
            } else if word.starts_with("\"") {
                let character = Symbol::Terminal(
                    word.chars()
                        .skip(1)
                        .next()
                        .ok_or(ParseRuleError::InvalidFormat("missing char"))?,
                );
                if is_branch {
                    list2.push(Product::Symbol(character));
                } else {
                    list1.push(Product::Symbol(character));
                }
            } else if word == "|" {
                is_branch = true;
            }
        }
        if is_branch {
            self.rules
                .insert(start_symbol, Product::Branch(list1.clone(), list2.clone()));
        } else {
            self.rules
                .insert(start_symbol, Product::List(list1.clone()));
        }
        Ok(())
    }

    fn get_product(&self, n: u8) -> &Product {
        self.rules.get(&Symbol::Nonterminal(n)).unwrap()
    }

    fn to_regex(&self, start: &Product) -> String {
        match start {
            Product::List(l) => l
                .iter()
                .map(|p| self.to_regex(&p).chars().collect::<Vec<_>>())
                .flatten()
                .collect::<String>(),
            Product::Branch(l, r) => {
                let left = l
                    .iter()
                    .map(|p| self.to_regex(&p).chars().collect::<Vec<_>>())
                    .flatten()
                    .collect::<String>();
                let right = r
                    .iter()
                    .map(|p| self.to_regex(&p).chars().collect::<Vec<_>>())
                    .flatten()
                    .collect::<String>();
                format!("({}|{})", left, right)
            }
            Product::Symbol(s) => match s {
                Symbol::Terminal(c) => format!("{}", c),
                Symbol::Nonterminal(_) => match self.rules.get(s) {
                    Some(p) => self.to_regex(p),
                    None => "".to_string(),
                },
            },
        }
    }
}

pub fn part1<'a, I, S>(lines: I) -> Result<usize, ParseRuleError>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut grammar: Grammar = Default::default();
    let mut iter = lines.into_iter();

    loop {
        if let Some(line) = iter.next() {
            let line = line.as_ref();

            if line == "" {
                break;
            }

            grammar.add_rule(line)?;
        }
    }

    let re = Regex::new(&format!("^{}$", grammar.to_regex(grammar.get_product(0)))).unwrap();

    Ok(iter.filter(|&l| re.is_match(l.as_ref())).count())
}

pub fn part2<'a, I, S>(lines: I) -> Result<usize, ParseRuleError>
where
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    let mut grammar: Grammar = Default::default();
    let mut iter = lines.into_iter();

    loop {
        if let Some(line) = iter.next() {
            let line = line.as_ref();

            if line == "" {
                break;
            }

            grammar.add_rule(line)?;
        }
    }

    let re42 = grammar.to_regex(grammar.get_product(42));
    let re31 = grammar.to_regex(grammar.get_product(31));

    let re42 = Regex::new(&re42).unwrap();
    let re31 = Regex::new(&re31).unwrap();

    Ok(iter
        .filter(|&l| {
            let l = l.as_ref();
            let mut offset = 0;
            let (mut m42, mut m31) = (0, 0);
            while let Some(m) = re42.find_at(l, offset) {
                if m.start() != offset {
                    break;
                }
                offset += m.as_str().len();
                m42 += 1;
            }
            while let Some(m) = re31.find_at(l, offset) {
                if m.start() != offset {
                    break;
                }
                offset += m.as_str().len();
                m31 += 1;
            }

            m42 > m31 && m31 > 0 && offset == l.len()
        })
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &[&str] = &[
        "42: 9 14 | 10 1",
        "9: 14 27 | 1 26",
        "10: 23 14 | 28 1",
        "1: \"a\"",
        "11: 42 31",
        "5: 1 14 | 15 1",
        "19: 14 1 | 14 14",
        "12: 24 14 | 19 1",
        "16: 15 1 | 14 14",
        "31: 14 17 | 1 13",
        "6: 14 14 | 1 14",
        "2: 1 24 | 14 4",
        "0: 8 11",
        "13: 14 3 | 1 12",
        "15: 1 | 14",
        "17: 14 2 | 1 7",
        "23: 25 1 | 22 14",
        "28: 16 1",
        "4: 1 1",
        "20: 14 14 | 1 15",
        "3: 5 14 | 16 1",
        "27: 1 6 | 14 18",
        "14: \"b\"",
        "21: 14 1 | 1 14",
        "25: 1 1 | 1 14",
        "22: 14 14",
        "8: 42",
        "26: 14 22 | 1 20",
        "18: 15 15",
        "7: 14 5 | 1 21",
        "24: 14 1",
        "",
        "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
        "bbabbbbaabaabba",
        "babbbbaabbbbbabbbbbbaabaaabaaa",
        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
        "bbbbbbbaaaabbbbaaabbabaaa",
        "bbbababbbbaaaaaaaabbababaaababaabab",
        "ababaaaaaabaaab",
        "ababaaaaabbbaba",
        "baabbaaaabbaaaababbaababb",
        "abbbbabbbbaaaababbbbbbaaaababb",
        "aaaaabbaabaaaaababaa",
        "aaaabbaaaabbaaa",
        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
        "babaaabbbaaabaababbaabababaaab",
        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    ];

    #[test]
    fn part1_test() {
        let result = part1(EXAMPLE).unwrap();

        assert_eq!(result, 3);
    }

    #[test]
    fn part2_test() {
        let result = part2(EXAMPLE).unwrap();

        assert_eq!(result, 12);
    }
}
