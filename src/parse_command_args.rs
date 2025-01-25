pub fn parse_command_args(input: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    let mut i = 0;
    let mut j;
    while i < input.len() {
        if input.chars().nth(i).unwrap() == ' ' {
            while i < input.len() && input.chars().nth(i).unwrap() == ' ' {
                i += 1;
            }
            j = i;
        } else if input.chars().nth(i).unwrap() == '\'' {
            j = i + 1;
            while input.chars().nth(j).unwrap() != '\'' {
                j += 1;
            }
            result.push(&input[i + 1..j]);
        } else {
            j = i;
            while j < input.len() && input.chars().nth(j).unwrap() != ' ' {
                j += 1;
            }
            result.push(&input[i..j]);
        }
        i = j + 1;
    }
    return result;
}
