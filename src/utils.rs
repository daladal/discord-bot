pub fn parse_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable(); 

    while let Some(c) = chars.next() {
       match c {
           '"' => {
               in_quotes = !in_quotes;
           }
           ' ' if !in_quotes => {
               if !current_arg.is_empty() {
                   args.push(current_arg.clone());
                   current_arg.clear();
               }
           }
           _ => {
               current_arg.push(c);
           }
       }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    args
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_args() {
        assert_eq!(parse_args("Hello world"), vec!["Hello", "world"]);
    }

    #[test]
    fn test_quoted_args() {
        assert_eq!(parse_args("hello \"world test\" foo"), vec!["hello", "world test", "foo"]);
    }

    #[test]
    fn test_multiple_quotes() {
        assert_eq!(parse_args("\"hello there\" \"Chovy Faker\" CN"), vec!["hello there", "Chovy Faker", "CN"]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(parse_args(""), Vec::<String>::new());
    }
}
