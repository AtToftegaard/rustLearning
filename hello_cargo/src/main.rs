mod math;
mod cpr;

fn main() {
    let cpr = "0307960931";
    let toDigits: Vec<i32> = math::string_to_digits(cpr);
    let x: [i32; 10] = toDigits.try_into()
        .unwrap_or_else(|v: Vec<i32>| panic!("Expected a Vec of length {} but it was {}", 10, v.len()));
    let valid = cpr::is_valid_cpr(cpr::CprNumber { digits: x });
    println!("{} is {result}", cpr, result = if valid { "valid! "} else { "not valid!" } );
}

