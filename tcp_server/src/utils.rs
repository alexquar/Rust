
pub fn print_error(message: &str) {
    eprintln!("Error: {}", message);
}

pub fn print<T>(message: T)
where T: std::fmt::Display + std::fmt::Debug + std::fmt::LowerHex + std::fmt::UpperHex + std::fmt::Binary + std::fmt::Octal + std::fmt::Pointer + std::fmt::Debug{
    println!("{}", message);
}