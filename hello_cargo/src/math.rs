pub(crate) fn number_to_digits(n: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        println!("{:?}", digits);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}

pub(crate) fn string_to_digits(s: &str) -> Vec<i32> {
    let digits: Vec<_> = s.chars().map(|d| d.to_digit(10).unwrap() as i32).collect();
    println!("{:?}", digits);
    digits
}