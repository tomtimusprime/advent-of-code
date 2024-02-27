fn main() {
   let array = [-4, 3, -9, 0, 4, 1];
   //plusMinus(&array);
   staircase(5);
}
//worked on rust at work
//worked on rust at work.
//worked on rust all day at work.

fn plusMinus(arr:&[i32]) {
    let n = arr.len();
    let mut num_of_positive:i32 = 0;
    let mut num_of_negative:i32 = 0;
    let mut num_of_zero:i32 = 0;
    for &i in arr {
        if i < 0 {
            num_of_negative += 1;
        } else if i == 0 {
            num_of_zero += 1;
        } else {
            num_of_positive += 1;
        }
    }
    let array_size = n as i32;
    let proportion_of_negative:f64 = (num_of_negative as f64 / array_size as f64).into();
    let proportion_of_positive:f64 = (num_of_positive as f64/ array_size as f64).into();
    let proportion_of_zero:f64 = (num_of_zero as f64 / array_size as f64).into();
    println!("Proportion of Positive: {}, proportion of negative: {}, proportion of zero {}", proportion_of_positive, proportion_of_negative, proportion_of_zero);
}

fn staircase(input: i32) {
    let mut count = input;
    for i in 0..input {
        let mut line:Vec<char> = vec![];
        for i in 0..count {
            if i == count-1 {
                line.push('#');
            } else {
                line.insert(0,' ');
            }
        }
        println!("{:?}", line);
        count -= 1;
    }
}