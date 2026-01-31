
use std::collections::HashMap;

pub fn get_median(input_numbers: &[i32]) ->i32
{
    let mut numbers = input_numbers.to_vec();
    numbers.sort();
    let length = numbers.len();
    if length % 2 != 1
    {
        return numbers[length / 2];
    }
    numbers[(length-1) / 2]
}

pub fn get_mode(numbers: &[i32]) -> i32
{
    let number_counts = count_numbers(&numbers);

    let mut most_common_number = number_counts.keys().next().unwrap();

    for (number, count) in &number_counts
    {
        if number_counts.get(most_common_number).expect("value not part of hash map") < count
        {
            most_common_number = number;
        }
    }
    *most_common_number
}

fn count_numbers(numbers: &[i32]) -> HashMap<i32, u8>
{
    let mut number_counts: HashMap<i32, u8> = HashMap::new();
    for number in numbers
    {
        let count = number_counts.entry(*number).or_insert(0);
        *count += 1;
    }
    number_counts
}
