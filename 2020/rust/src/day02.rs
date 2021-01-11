use std::fs;

struct ValidationParams {
    n1: usize,
    n2: usize,
    c: char,
}

struct Password<'a> {
    password: &'a str,
    params: ValidationParams,
}

impl<'a> Password<'a> {
    fn new(n1: usize, n2: usize, c: char, password: &str) -> Password {
        Password {
            password,
            params: ValidationParams { n1, n2, c },
        }
    }

    fn validate_bounds(&self) -> bool {
        let occurences = self
            .password
            .chars()
            .filter(|&c| c == self.params.c)
            .count() as usize;

        self.params.n1 <= occurences && occurences <= self.params.n2
    }

    fn validate_positions(&self) -> bool {
        // Probably disgusting
        let c1 = self.password[self.params.n1 - 1..self.params.n1]
            .chars()
            .next()
            .unwrap_or(' ');
        let c2 = self.password[self.params.n2 - 1..self.params.n2]
            .chars()
            .next()
            .unwrap_or(' ');
        (c1 == self.params.c) ^ (c2 == self.params.c)
    }
}

pub fn main() {
    let plain_text = fs::read_to_string("../../day02.txt").unwrap();
    let passwords: Vec<Password> = plain_text
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let tokens: Vec<&str> = s.split(' ').collect();

            if let [numbers, character, password] = &tokens[..] {
                let numbers: Vec<usize> = numbers
                    .split('-')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                let character = character.chars().next().unwrap();

                Some(Password::new(numbers[0], numbers[1], character, password))
            } else {
                None
            }
        })
        .collect();

    // Part 1
    println!(
        "{}",
        passwords.iter().filter(|pw| pw.validate_bounds()).count()
    );

    // Part 2
    println!(
        "{}",
        passwords
            .iter()
            .filter(|pw| pw.validate_positions())
            .count()
    );
}
