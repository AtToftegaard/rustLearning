pub(crate) fn is_valid_cpr(cpr: CprNumber) -> bool {
    let multiplication_values: Vec<i32> = vec![4,3,2,7,6,5,4,3,2,1];
    let sum:i32 = cpr.digits.iter()
        .zip(multiplication_values.iter())
        .map(|(a,b)| a*b)
        .sum();
    let multiplication : Vec<_> = cpr.digits.iter()
        .zip(multiplication_values.iter())
        .map(|(a,b)| a*b)
        .collect();
    println!("{:?}", multiplication_values);
    println!("{:?}", multiplication);
    println!("Sum is {:?}. Modulo 11 is {}", sum, sum % 11);
    return (sum % 11) == 0;
}

#[derive(Debug)]
pub(crate) struct CprNumber {
    pub(crate) digits: [i32; 10],
}