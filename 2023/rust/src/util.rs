use std::str::FromStr;

pub fn parse_many<'a, F, I, S>(lines: I) -> Result<Vec<F>, <F as FromStr>::Err>
where
    F: FromStr,
    I: IntoIterator<Item = &'a S>,
    S: AsRef<str> + 'a,
{
    lines
        .into_iter()
        .map(|s| s.as_ref().parse::<F>())
        .collect::<Result<_, _>>()
}
