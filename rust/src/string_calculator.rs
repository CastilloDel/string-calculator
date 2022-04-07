use std::num::ParseIntError;

fn parseAndSum(mut input: &str) -> Result<usize, ParseIntError> {
    if input.is_empty() {
        return Ok(0);
    }
    let mut delimiters = vec![",", "\n"];
    if let Some(delimiter) = read_delimiter(input) {
        input = &input[input.find("\n").unwrap() + 1..];
        delimiters = vec![delimiter];
    }
    Ok(split_by_string(input, &delimiters[..])
        .into_iter()
        .map(|val| val.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?
        .into_iter()
        .sum::<usize>())
}

fn read_delimiter(input: &str) -> Option<&str> {
    if input.starts_with("//") {
        return Some(&input[2..input.find('\n')?]);
    }
    None
}

fn split_by_string<'a>(input: &'a str, delimiters: &[&str]) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last_division = 0;
    for i in 1..input.len() {
        for delimiter in delimiters {
            if input[i..].starts_with(delimiter) {
                result.push(&input[last_division..i]);
                last_division = i + delimiter.len();
                break;
            }
        }
    }
    result.push(&input[last_division..]);
    result
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
    fn should_be_possible_to_read_a_delimiter() {
        assert_eq!(read_delimiter("//;\n1;4;4"), Some(";"));
        assert_eq!(read_delimiter("//;-\n1;-4;-4"), Some(";-"));
    }

    #[test]
    fn should_be_possible_to_use_a_custom_delimiter() {
        assert_eq!(parseAndSum("//;-\n1;-4;-4"), Ok(9));
    }
}
