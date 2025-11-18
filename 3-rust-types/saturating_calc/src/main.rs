fn main() {
    assert_eq!(saturating_calc(10, 20, '+'), 30);
    assert_eq!(saturating_calc(100, 200, '+'), 300);
    assert_eq!(saturating_calc(i32::MAX, 1, '+'), i32::MAX);

    assert_eq!(saturating_calc(50, 30, '-'), 20);
    assert_eq!(saturating_calc(i32::MIN, 1, '-'), i32::MIN);

    assert_eq!(saturating_calc(5, 6, '*'), 30);
    assert_eq!(saturating_calc(i32::MAX, 2, '*'), i32::MAX);

    assert_eq!(saturating_calc(10, 20, '/'), 0);

    println!("Все тесты прошли успешно!");
}

fn saturating_calc(a: i32, b: i32, operation: char) -> i32 {
    match operation {
        '+' => a.saturating_add(b),
        '-' => a.saturating_sub(b),
        '*' => a.saturating_mul(b),
        _ => 0
    }    
}
