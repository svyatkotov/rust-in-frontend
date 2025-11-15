fn main() {
    for n in 0..=15 {
        fiz_buzz(n);
    }
}

fn fiz_buzz(num: i32) {
    let divisible_by_3 = num % 3 == 0;
    let divisible_by_5 = num % 5 == 0;

    match num {
        _ if divisible_by_3 && divisible_by_5 => println!("{num}: FizzBuzz"),
        _ if divisible_by_3 => println!("{num}: Fizz"),
        _ if divisible_by_5 => println!("{num}: Buzz"),
        _ => println!("{num}"),
    }
}
