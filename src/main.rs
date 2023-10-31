use std::{
    env::{args, Args},
    io::stdin,
    process::exit,
};

#[derive(Copy, Clone)]
enum NameConvention {
    PascalCase,
    CamelCase,
    SnakeCase,
    ScreammingSnakeCase,
    KebabCase,
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
            NameConvention::PascalCase => Self::separate_by_upper(&buff),
            NameConvention::CamelCase => Self::separate_by_upper(&buff),
            NameConvention::SnakeCase => buff.split('_').collect::<Vec<&str>>(),
            NameConvention::KebabCase => todo!(),
            NameConvention::ScreammingSnakeCase => buff.split('_').collect::<Vec<&str>>(),
        };

        match to {
            NameConvention::PascalCase => Self::convert_to_pascal_case(global_format),
            NameConvention::CamelCase => Self::convert_to_camel_case(global_format),
            NameConvention::SnakeCase => Self::convert_to_snake_case(global_format),
            NameConvention::KebabCase => todo!(),
            NameConvention::ScreammingSnakeCase => {
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

        while let Some(word) = buff.next() {
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
            "p" | "P" | "pascal" | "Pascal" => NameConvention::PascalCase,
            "c" | "C" | "camel" | "Camel" => NameConvention::CamelCase,
            "s" | "S" | "snake" | "Snake" => NameConvention::SnakeCase,
            "k" | "K" | "kebab" | "Kebab" => NameConvention::KebabCase,
            "ss" | "SS" | "screamming" | "Screamming" => NameConvention::ScreammingSnakeCase,
            _ => {
                println!("Invalid name convention");
                exit(1);
            }
        }
    }
}

fn next_arg(program_name: &str, args: &mut Args) -> String {
    args.next().unwrap_or_else(|| {
        println!("Usage: {} <from> <to>", program_name);
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
        .map(|l| NameConvention::convert(from, &l, to))
        .for_each(|k| println!("{}", k));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camel_case_to_snake_case() {
        let buff = String::from("helloWorld");

        let result =
            NameConvention::convert(NameConvention::CamelCase, &buff, NameConvention::SnakeCase);

        assert_eq!(result, "hello_world");
    }

    #[test]
    fn snake_case_to_camel_case() {
        let buff = String::from("hello_world");

        let result =
            NameConvention::convert(NameConvention::SnakeCase, &buff, NameConvention::CamelCase);

        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn camel_case_to_pascal_case() {
        let buff = String::from("helloWorld");

        let result =
            NameConvention::convert(NameConvention::CamelCase, &buff, NameConvention::PascalCase);

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn pascal_case_to_camel_case() {
        let buff = String::from("HelloWorld");

        let result =
            NameConvention::convert(NameConvention::PascalCase, &buff, NameConvention::CamelCase);

        assert_eq!(result, "helloWorld");
    }

    #[test]
    fn snake_case_to_pascal_case() {
        let buff = String::from("hello_world");

        let result =
            NameConvention::convert(NameConvention::SnakeCase, &buff, NameConvention::PascalCase);

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn pascal_case_to_snake_case() {
        let buff = String::from("HelloWorld");

        let result =
            NameConvention::convert(NameConvention::PascalCase, &buff, NameConvention::SnakeCase);

        assert_eq!(result, "hello_world");
    }

    #[test]
    fn screamming_snake_case_to_pascal_case() {
        let buff = String::from("HELLO_WORLD");

        let result = NameConvention::convert(
            NameConvention::ScreammingSnakeCase,
            &buff,
            NameConvention::PascalCase,
        );

        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn pascal_case_to_screamming_snake_case() {
        let buff = String::from("HelloWorld");

        let result = NameConvention::convert(
            NameConvention::PascalCase,
            &buff,
            NameConvention::ScreammingSnakeCase,
        );

        assert_eq!(result, "HELLO_WORLD");
    }
}
