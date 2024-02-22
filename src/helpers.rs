use rusty_money::{iso, Money};

pub fn get_total_as_formatted_string(total: i64) -> String {
    Money::from_minor(total, iso::GBP).to_string()
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_less_than_100() {
        assert_eq!(get_total_as_formatted_string(12), String::from("£0.12"));
    }

    #[test]
    fn test_more_than_100() {
        assert_eq!(get_total_as_formatted_string(1200), String::from("£12.00"));
    }

    #[test]
    fn test_more_than_1000() {
        assert_eq!(get_total_as_formatted_string(1_200_00), String::from("£1,200.00"));
        assert_eq!(get_total_as_formatted_string(12_200_00), String::from("£12,200.00"));
        assert_eq!(get_total_as_formatted_string(123_200_00), String::from("£123,200.00"));
        // I don't think I need to worry about millionaires, but it's here
        assert_eq!(get_total_as_formatted_string(1_123_200_00), String::from("£1,123,200.00"));
    }
}
