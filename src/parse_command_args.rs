pub fn parse_command_args(input: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut i = 0;
    let mut j;
    while i < input.len() {
        if input.chars().nth(i).unwrap() == ' ' {
            while i < input.len() && input.chars().nth(i).unwrap() == ' ' {
                i += 1;
            }
            j = i;
        } else if input.chars().nth(i).unwrap() == '\'' {
            i += 1;
            j = i;
            while input.chars().nth(j).unwrap() != '\'' {
                j += 1;
                if input.chars().nth(j).unwrap() == '\''
                    && j + 1 < input.len()
                    && input.chars().nth(j + 1).unwrap() == '\''
                {
                    j += 2;
                }
            }
            let s = &input[i..j];
            let mut temp = String::new();
            for c in s.chars() {
                if c != '\'' {
                    temp.push(c);
                }
            }
            result.push(temp);
            j += 1;
        } else if input.chars().nth(i).unwrap() == '"' {
            i += 1;
            j = i;
            while input.chars().nth(j).unwrap() != '"' {
                j += 1;
                if input.chars().nth(j).unwrap() == '"'
                    && j + 1 < input.len()
                    && input.chars().nth(j + 1).unwrap() == '"'
                {
                    j += 2;
                }
            }
            let s = &input[i..j];
            let mut temp = String::new();
            for c in s.chars() {
                if c != '"' {
                    temp.push(c);
                }
            }
            result.push(temp);
            j += 1;
        } else {
            j = i;
            while j < input.len() && input.chars().nth(j).unwrap() != ' ' {
                j += 1;
            }
            result.push(input[i..j].to_string());
        }
        i = j;
    }
    return result;
}
