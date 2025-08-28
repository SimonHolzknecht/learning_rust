use std::io;

fn main() {
  println!("Do you want to convert Fahrenheit (f) or Celsius (c)?");

  let convert_f_to_c = get_temp_convert_from_fahrenheit();

  let temp_value = get_temp_value();

  let result =
    match convert_f_to_c {
      true => convert_fahrenheit_to_celsius(temp_value),
      false => convert_celsius_to_fahrenheit(temp_value)
    };

  println!("The converted temperature is [{0}]", result);
}



fn get_temp_convert_from_fahrenheit() -> bool {
  loop {
    let mut unit = String::new();

    io::stdin().read_line(&mut unit).expect("Parsing of input failed");

    match unit.trim() {
      "f" => return true,
      "c" => return false,
      _ => println!("Input either 'f' for Fahrenheit or 'c' for celsius"),
    };
  }
}



fn get_temp_value() -> f64 {
  println!("Input the raw value to convert:");

  let mut value : String;

  loop {
    value = String::new();
    io::stdin().read_line(&mut value).expect("Parsing of input failed");

    match value.trim().parse::<f64>() {
      Ok(num) => return num,
      Err(_) => {
        println!("Input a valid integer");
        continue
      }
    }
  }
}



fn convert_fahrenheit_to_celsius(temp_fahrenheit: f64) -> f64
{
  return (temp_fahrenheit - 32.0) * (5.0/9.0);
}



fn convert_celsius_to_fahrenheit(temp_celsius: f64) -> f64
{
  return temp_celsius * (9.0/5.0) + 32.0;
}
