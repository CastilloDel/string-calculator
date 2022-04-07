use std::num::ParseIntError;

fn parseAndSum(mut input: &str) -> Result<usize, ParseIntError> {
    if input.is_empty() {
        return Ok(0);
    }
    let mut delimiters = vec![',', '\n'];
    if let Some(delimiter) = read_delimiter(input) {
        input = &input[input.find("\n").unwrap()..];
        delimiters = vec![delimiter];
    }
    Ok(input
        .split(&delimiters[..])
        .map(|val| val.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?
        .into_iter()
        .sum::<usize>())
}

fn read_delimiter(input: &str) -> Option<char> {
    if input.starts_with("//") {
        return input.chars().nth(2);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_return_0_if_input_is_empty() {
        assert_eq!(parseAndSum(""), Ok(0));
    }

    #[test]
    fn should_parse_if_there_is_only_one_number() {
        assert_eq!(parseAndSum("4"), Ok(4));
    }

    #[test]
    fn should_sum_comma_separated_numbers() {
        assert_eq!(parseAndSum("4,6,2"), Ok(12));
    }

    #[test]
    fn should_sum_newline_separated_numbers() {
        assert_eq!(parseAndSum("4,6\n3"), Ok(13));
    }

    #[test]
    fn should_be_possible_to_read_a_one_char_delimiter() {
        assert_eq!(read_delimiter("//;\n1;4;4"), Some(';'))
    }
}
