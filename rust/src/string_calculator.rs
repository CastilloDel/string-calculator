use std::num::ParseIntError;

fn parseAndSum(input: &str) -> Result<usize, ParseIntError> {
    if input.is_empty() {
        return Ok(0);
    }
    Ok(input
        .split(&[',', '\n'])
        .map(|val| val.parse())
        .collect::<Result<Vec<usize>, ParseIntError>>()?
        .into_iter()
        .sum::<usize>())
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
}
