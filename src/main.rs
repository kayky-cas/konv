use std::{
    env::{args, Args},
    io::stdin,
    process::exit,
};

#[derive(Copy, Clone)]
enum NameConvention {
    Pascal,
    Camel,
    Snake,
    ScreammingSnake,
    Kebab,
}

impl NameConvention {
    fn separate_by_upper(buff: &str) -> Vec<&str> {
        let mut words = Vec::new();

        let mut last = 0;

        for (i, c) in buff.chars().enumerate() {
            if c.is_uppercase() {
                let buff = &buff[last..i];

                if !buff.is_empty() {
                    words.push(buff);
                }

                last = i;
            }
        }

        words.push(&buff[last..]);

        words
    }

    fn convert(from: Self, buff: &str, to: Self) -> String {
        let global_format = match from {
            NameConvention::Pascal => Self::separate_by_upper(buff),
            NameConvention::Camel => Self::separate_by_upper(buff),
            NameConvention::Snake => buff.split('_').collect::<Vec<&str>>(),
            NameConvention::Kebab => todo!(),
            NameConvention::ScreammingSnake => buff.split('_').collect::<Vec<&str>>(),
        };

        match to {
            NameConvention::Pascal => Self::convert_to_pascal_case(global_format),
            NameConvention::Camel => Self::convert_to_camel_case(global_format),
            NameConvention::Snake => Self::convert_to_snake_case(global_format),
            NameConvention::Kebab => todo!(),
            NameConvention::ScreammingSnake => {
                Self::convert_to_screamming_snake_case(global_format)
            }
        }
    }

    fn convert_to_camel_case(buff: Vec<&str>) -> String {
        let mut result = String::new();
        let mut buff = buff.iter();

        if let Some(word) = buff.next() {
            result.push_str(&word.to_lowercase());
        }

        for word in buff {
            let mut chars = word.chars();
            if let Some(c) = chars.next() {
                result.push(c.to_ascii_uppercase());
            }
            result.push_str(&chars.collect::<String>());
        }

        result
    }

    fn convert_to_snake_case(buff: Vec<&str>) -> String {
        let mut result = String::new();

        for (i, word) in buff.iter().enumerate() {
            let word = word.to_lowercase();

            if i == 0 {
                result.push_str(&word);
            } else {
                result.push('_');
                result.push_str(&word);
            }
        }

        result
    }

    fn convert_to_screamming_snake_case(buff: Vec<&str>) -> String {
        let mut result = String::new();

        for (i, word) in buff.iter().enumerate() {
            let word = word.to_uppercase();

            if i == 0 {
                result.push_str(&word);
            } else {
                result.push('_');
                result.push_str(&word);
            }
        }

        result
    }

    fn convert_to_pascal_case(buff: Vec<&str>) -> String {
        let mut result = String::new();

        for word in buff.iter() {
            let mut chars = word.chars();
            if let Some(c) = chars.next() {
                result.push(c.to_ascii_uppercase());
            }
            result.push_str(&chars.map(|c| c.to_ascii_lowercase()).collect::<String>());
        }

        result
    }
}

impl From<String> for NameConvention {
    fn from(value: String) -> Self {
        match value.as_str() {
            "p" | "P" | "pascal" | "Pascal" => NameConvention::Pascal,
            "c" | "C" | "camel" | "Camel" => NameConvention::Camel,
            "s" | "S" | "snake" | "Snake" => NameConvention::Snake,
            "k" | "K" | "kebab" | "Kebab" => NameConvention::Kebab,
            "sc" | "SC" | "screamming" | "Screamming" => NameConvention::ScreammingSnake,
            _ => {
                eprintln!("Invalid name convention");
                exit(1);
            }
        }
    }
}

fn next_arg(program_name: &str, args: &mut Args) -> String {
    args.next().unwrap_or_else(|| {
        eprintln!("Usage: {} <from> <to>", program_name);
        exit(1);
    })
}

fn main() {
    let mut args = args();
    let program_name = args.next().unwrap();

    let from = next_arg(&program_name, &mut args).into();
    let to = next_arg(&program_name, &mut args).into();

    stdin()
        .lines()
        .flatten()
        .map(|l| NameConvention::convert(from, &l.trim(), to))
        .for_each(|k| println!("{}", k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case_to_snake_case() {
        let buff = String::from("helloWorld");

        let result = NameConvention::convert(NameConvention::Camel, &buff, NameConvention::Snake);

        assert_eq!(result, "hello_world");
    }

    #[test]
    fn snake_case_to_camel_case() {
        let buff = String::from("hello_world");

        let result = NameConvention::convert(NameConvention::Snake, &buff, NameConvention::Camel);

        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn camel_case_to_pascal_case() {
        let buff = String::from("helloWorld");

        let result = NameConvention::convert(NameConvention::Camel, &buff, NameConvention::Pascal);

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn pascal_case_to_camel_case() {
        let buff = String::from("HelloWorld");

        let result = NameConvention::convert(NameConvention::Pascal, &buff, NameConvention::Camel);

        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn snake_case_to_pascal_case() {
        let buff = String::from("hello_world");

        let result = NameConvention::convert(NameConvention::Snake, &buff, NameConvention::Pascal);

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn pascal_case_to_snake_case() {
        let buff = String::from("HelloWorld");

        let result = NameConvention::convert(NameConvention::Pascal, &buff, NameConvention::Snake);

        assert_eq!(result, "hello_world");
    }

    #[test]
    fn screamming_snake_case_to_pascal_case() {
        let buff = String::from("HELLO_WORLD");

        let result = NameConvention::convert(
            NameConvention::ScreammingSnake,
            &buff,
            NameConvention::Pascal,
        );

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn pascal_case_to_screamming_snake_case() {
        let buff = String::from("HelloWorld");

        let result = NameConvention::convert(
            NameConvention::Pascal,
            &buff,
            NameConvention::ScreammingSnake,
        );

        assert_eq!(result, "HELLO_WORLD");
    }
}
