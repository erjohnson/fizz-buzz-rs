fn main() {

  fn div_by_three(num: int) -> bool {
    num % 3 == 0
  }

  fn div_by_five(num: int) -> bool {
    num % 5 == 0
  }

  fn div_by_fifteen(num: int) -> bool {
    num % 15 == 0
  }

  for i in range(1i, 101) {
    if div_by_fifteen(i) { 
      println!("FizzBuzz"); 
    } else if div_by_three(i) { 
      println!("Fizz"); 
    } else if div_by_five(i) { 
      println!("Buzz"); 
    } else { 
      println!("{}", i);
    }
  }
}
