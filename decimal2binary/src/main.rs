fn main() {
    let decimal = 3;
    let binary = convert_decimal_to_binary(decimal);
    println!("Decimal: {} -> Binary: {}", decimal, binary);
}

fn convert_decimal_to_binary(decimal: i32) -> String {
    if decimal == 0 {
        return "0".to_string();
    }

    let mut binary = String::new();
    let mut n = decimal;
    while n > 0 {
        binary.push_str(&format!("{}", n % 2));
        n /= 2;
    }
    binary.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_decimal_to_binary() {
        assert_eq!(convert_decimal_to_binary(0), "0");
        assert_eq!(convert_decimal_to_binary(1), "1");
        assert_eq!(convert_decimal_to_binary(2), "10");
        assert_eq!(convert_decimal_to_binary(3), "11");
        assert_eq!(convert_decimal_to_binary(5), "101");
        assert_eq!(convert_decimal_to_binary(10), "1010");
    }
}
