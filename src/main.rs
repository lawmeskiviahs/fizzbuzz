fn main() {
    for num in 1..100 {
        if num % 15 == 0 {println!("FizzBuzz {}", num);}
        else if num % 5 == 0 {println!("Buzz {}", num);}
        else if num % 3 == 0 {println!("Fizz {}", num);}
    }
}
