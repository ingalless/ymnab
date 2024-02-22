use rusty_money::{iso, Money};

pub fn get_total_as_formatted_string(total: i64) -> String {
    Money::from_minor(total, iso::GBP).to_string()
}

pub fn get_money_from_string(value: String) -> Result<i64, &'static str> {
    let mut b = value.to_string();
    if !b.contains(".") {
        b.insert_str(b.len(), ".00");
    }
    match b.trim().replace(",", "").replace(".", "").parse::<i64>() {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{:?}", e);
            Err("Failed to parse input string")
        }
    }
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_formatted_string_less_than_100() {
        assert_eq!(get_total_as_formatted_string(12), String::from("£0.12"));
    }

    #[test]
    fn test_formatted_string_more_than_100() {
        assert_eq!(get_total_as_formatted_string(1200), String::from("£12.00"));
    }

    #[test]
    fn test_formatted_string_more_than_1000() {
        assert_eq!(get_total_as_formatted_string(1_200_00), String::from("£1,200.00"));
        assert_eq!(get_total_as_formatted_string(12_200_00), String::from("£12,200.00"));
        assert_eq!(get_total_as_formatted_string(123_200_00), String::from("£123,200.00"));
        // I don't think I need to worry about millionaires, but it's here
        assert_eq!(get_total_as_formatted_string(1_123_200_00), String::from("£1,123,200.00"));
    }

    #[test]
    fn test_get_money_from_string() {
        // no pence
        assert_eq!(get_money_from_string(String::from("1,000")), Ok(1_000_00));
        assert_eq!(get_money_from_string(String::from("1000")), Ok(1_000_00));
        assert_eq!(get_money_from_string(String::from("1100.21")), Ok(1_100_21));
        assert_eq!(get_money_from_string(String::from("10,000")), Ok(10_000_00));
        assert_eq!(get_money_from_string(String::from("10,000.22")), Ok(10_000_22));
        assert_eq!(get_money_from_string(String::from("10000.22")), Ok(10_000_22));
        assert_eq!(get_money_from_string(String::from("00.00")), Ok(0));
        assert_eq!(get_money_from_string(String::from("00")), Ok(0));
        assert_eq!(get_money_from_string(String::from("0")), Ok(0));
        assert_eq!(get_money_from_string(String::from("100,00.22")), Ok(10_000_22));
        assert_eq!(get_money_from_string(String::from("100,00.22")), Ok(10_000_22));
        assert_eq!(get_money_from_string(String::from("abcdefg")), Err("Failed to parse input string"));
    }
}
