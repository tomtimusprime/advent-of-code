fn main() {
    let input = include_str!("./text.txt");
    let list_of_numbers = create_list_of_numbers(input);
    let sum = sum_of_number_list(list_of_numbers);
    println!("The sum of numbers is {:?}", sum);
}

fn create_list_of_numbers(input: &str) -> Vec<u32> {
    let list_of_numbers = input
    .lines()
    .map(|line| {
        let mut first_digit = 0;
        let mut second_digit = 0;
        for c in line.chars() {
            if c.is_digit(10) && first_digit == 0 {
                first_digit = c.to_digit(10).unwrap();
                second_digit = c.to_digit(10).unwrap();
            } else if c.is_digit(10) {
                second_digit = c.to_digit(10).unwrap();
            }
        }
    (first_digit * 10) + second_digit
    })
    .collect();
    return list_of_numbers;
}

fn sum_of_number_list(list: Vec<u32>) -> u32 {
    let mut sum = 0;
    for num in list {
        sum += num;
    }
    sum
}