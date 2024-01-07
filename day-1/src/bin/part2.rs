fn main() {
    let input = include_str!("./text.txt");
    /*let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";*/
    process(input);
}

fn process(input: &str) -> u32 {
    let output = input.lines().map(process_line).sum::<u32>();
    dbg!(&output);
    output
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            //index += "one".len();
            Some('1')
        } else if reduced_line.starts_with("two") {
            //index += "two".len();
            Some('2')
        } else if reduced_line.starts_with("three") {
           //index += "three".len();
            Some('3')
        } else if reduced_line.starts_with("four") {
            //index += "four".len();
            Some('4')
        } else if reduced_line.starts_with("five") {
            //index += "five".len();
            Some('5')
        } else if reduced_line.starts_with("six") {
            //index += "six".len();
            Some('6')
        } else if reduced_line.starts_with("seven") {
            //index += "seven".len();
            Some('7')
        } else if reduced_line.starts_with("eight") {
            //index += "eight".len();
            Some('8')
        } else if reduced_line.starts_with("nine") {
            //index += "nine".len();
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };
        index += 1;
        result
    });
    //dbg!(&line_iter);
    let mut it = line_iter.filter_map(|character| character.to_digit(10));
    let first = it.next().expect("Should be a number");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Should be a valid number")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
        let output = process(input);
        dbg!(&output);
        assert_eq!(output, 281);
    }
}
