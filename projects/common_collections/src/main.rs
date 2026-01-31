mod list_info;
mod pig_latin;
mod employee_database;
use employee_database::api::Api;
use std::io;

fn print_list_info() {
    let numbers = vec![5,3,1,3,2,6,6,6];
    let median = list_info::get_median(&numbers);
    let mode = list_info::get_mode(&numbers);
    println!("processing numbers: {:?}", numbers);
    println!("median:{} mode:{}", &median, &mode);
}

fn convert_string_to_pig_latin() {
    let mut input = String::new();
    println!("Please input the phrase that should be converted into pig-latin");
    io::stdin().read_line(&mut input).expect("Could not parso input string");
    let pig_latin = pig_latin::convert(&input);
    println!("Converted phrase: [{}]", pig_latin);
}

fn main() {
    println!("Choose which mode to run: 1=return list info, 2=convert input string to pig-latin, 3=run employee database");

    let mut selection = String::new();
    io::stdin().read_line(&mut selection).expect("Could not parse input");

    let selection: u8 = match selection.trim().parse() {
        Ok(num)=>num,
        Err(_) => {
            println!("Could not parse input into number.");
            return;
        }
    };

    match selection {
        1 => print_list_info(),
        2 => convert_string_to_pig_latin(),
        3 => {
            let mut api = Api::new();
            api.run()
        },
        _ => print!("Mode number is not supported")
    }
}
