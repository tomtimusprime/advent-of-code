use std::num::ParseIntError;
fn main() {
    let input = include_str!("./text.txt");
    process(input);
}

fn process(input: &str) -> u32 {
    let output = input
    .lines()
    .map(|line| {
        let mut it = line.chars().filter_map(|character| {
            character.to_digit(10)
        });
        let first = it.next().expect("Should be a number");
        let last = it.last();

        match last {
            Some(num) => format!("{first}{num}").parse::<u32>(),
            None => format!("{first}{first}").parse::<u32>(),
        }
    })
    .fold(0, |accumulator, result| match result {
        Ok(num) => accumulator + num,
        Err(_) => accumulator, // or handle the error as needed
    });

    dbg!(output);
    output
}
//The following is really handy to use on iterators to see what's coming back on them.
// .inspect(|line| {
//     dbg!(line);
// }).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = process(input);
        assert_eq!(output, 142);
    }
}
