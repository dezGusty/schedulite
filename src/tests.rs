#[cfg(test)]
mod tests {

    use crate::filecopy;
    #[test]
    fn test_replace_special_variables_1() {
        let input = "Today is :{DD}/:{MM}/:{YYYY}";
        let expected_output = format!(
            "Today is {}/{}/{}",
            chrono::offset::Local::now().format("%d"),
            chrono::offset::Local::now().format("%m"),
            chrono::offset::Local::now().format("%Y")
        );
        assert_eq!(filecopy::replace_special_variables(input), expected_output);
    }

    #[test]
    fn test_replace_special_variables_2() {
        let input = "The time is: :{HH}::{mm}";
        let expected_output = format!(
            "The time is: {}:{}",
            chrono::offset::Local::now().format("%H"),
            chrono::offset::Local::now().format("%M")
        );
        assert_eq!(filecopy::replace_special_variables(input), expected_output);
    }

    #[test]
    fn test_replace_special_variables_3_nothing_to_do() {
        let input = "No special chars here.";
        let expected_output = "No special chars here.".to_string();
        assert_eq!(filecopy::replace_special_variables(input), expected_output);
    }

    #[test]
    fn test_replace_special_variables_4_invalid_vars() {
        let input = "The value of this variable is [:{DOESNOTEXIST}]";
        let expected_output = "The value of this variable is [:{DOESNOTEXIST}]".to_string();
        assert_eq!(filecopy::replace_special_variables(input), expected_output);
    }

    #[test]
    fn test_replace_special_variables_5_same_var_twice() {
        let input = "The hour is: :{HH}::{HH}";
        let expected_output = format!(
            "The hour is: {}:{}",
            chrono::offset::Local::now().format("%H"),
            chrono::offset::Local::now().format("%H")
        );
        assert_eq!(filecopy::replace_special_variables(input), expected_output);
    }
}
