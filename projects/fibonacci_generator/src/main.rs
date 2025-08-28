use std::io;

fn main() {
  let n = get_n();

  let fib_num = get_fibonacci_number(n);

  println!("The fibonacci number for n={0} is {1}", n, fib_num);
}

fn get_n() -> u128 {
  println!("This program will generate the nth fibonacci number");
  println!("Please enter n:");

  let mut n = String::new();

  io::stdin().read_line(&mut n).expect("Failed parsing the input");

  let n : u128 = n.trim().parse().expect("Not a valid number given");

  return n;
}



fn get_fibonacci_number(n : u128) -> u128 {
  if n == 0 {
    return 0;
  }
  else if n == 1 {
    return 1;
  }

  let mut current_num : u128 = 1;
  let mut previous_num : u128 = 0;

  for _ in 0..n-1 {
    let next_num = current_num + previous_num;
    previous_num = current_num;
    current_num = next_num;
  }

  return current_num;
}
