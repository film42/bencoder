use std::collections::HashMap;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum BType {
    ByteString(String),
    Integer(i64),
    List(Vec<BType>),
    Dict(HashMap<String, BType>)
}

pub struct BEncoder;

impl BEncoder {
    pub fn decode(input: String) -> Result<BType, &'static str> {
        if input.len() < 2 {
            return Err("Input string is too short.");
        }

        let mut chars = input.as_str().chars();
        let mut cursor = chars.by_ref();
        let next = cursor.next();

        match next {
            // Parse an integer
            Some('i') => BEncoder::decode_integer(cursor),
            // Parse a list
            Some('l') => BEncoder::decode_list(cursor),
            // Parse a string
            Some(chr) if chr.is_digit(10) => BEncoder::decode_string(chr, cursor),
            _ => Err("Something is missing.")
        }
    }

    fn decode_list(cursor: &mut Chars) -> Result<BType, &'static str> {
        let mut acc = vec![];

        let mut next = cursor.next();

        while next.is_some() {
            let result = match next {
                // Check for base case (closed list)
                Some('e') => return Ok(BType::List(acc)),

                // Parse an integer
                Some('i') => BEncoder::decode_integer(cursor),

                // Parse a list
                Some('l') => BEncoder::decode_list(cursor),

                // Parse a string
                Some(chr) if chr.is_digit(10) => BEncoder::decode_string(chr, cursor),
                _ => Err("Something is missing.")
            };

            if result.is_ok() {
                acc.push(result.unwrap());
            } else {
                return result;
            }

            // Adv the cursor
            next = cursor.next();
        }

        Err("A list was not terminated with an 'e'.")
    }

    fn decode_integer(cursor: &mut Chars) -> Result<BType, &'static str> {
        let mut current = '0';
        let num_as_string = cursor
            // HACK: This let's us keep track of the current
            // position of the cursor.
            .inspect(|x| current = x.clone())
            .take_while(|chr| *chr != 'e').collect::<String>();

        let num_result = num_as_string.parse::<i64>();

        if current == 'e' {
            if num_result.is_err() {
                return Err("Error while parsing integer.");
            }

            let integer = num_result.unwrap();

            Ok(BType::Integer(integer))
        } else {
            Err("No ending 'e' for integer.")
        }
    }

    fn decode_string(first: char, cursor: &mut Chars) -> Result<BType, &'static str> {
        let appended_chrs = cursor.take_while(|chr| *chr != ':').collect::<String>();

        let mut num_as_string = String::new();
        num_as_string.push(first);
        num_as_string.push_str(appended_chrs.as_str());

        let number_of_bytes_to_read_result = num_as_string.parse::<usize>();


        match number_of_bytes_to_read_result {
            Ok(number_of_bytes_to_read) => {
                let string = cursor.take(number_of_bytes_to_read).collect::<String>();

                Ok(BType::ByteString(string))
            },
            Err(_) => Err("Could not parse number for reading a string.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BEncoder;
    use super::BType;

    #[test]
    fn it_errors_when_string_is_too_short() {
        let result = BEncoder::decode("l".to_string());

        assert_eq!(result, Err("Input string is too short."));
    }

    #[test]
    fn it_can_parse_a_positive_integer() {
        let result = BEncoder::decode("i123456789e".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BType::Integer(123456789));
    }

    #[test]
    fn it_can_parse_a_negative_integer() {
        let result = BEncoder::decode("i-123e".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BType::Integer(-123));
    }

    #[test]
    fn it_can_parse_a_string() {
        let result = BEncoder::decode("5:hello".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BType::ByteString("hello".to_string()));
    }

    #[test]
    fn it_only_parses_the_number_of_bytes_specified() {
        let result = BEncoder::decode("4:hello".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BType::ByteString("hell".to_string()));
    }

    #[test]
    fn it_can_parse_an_empty_list() {
        let result = BEncoder::decode("le".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BType::List(vec![]));
    }

    #[test]
    fn it_can_parse_a_basic_list() {
        let result = BEncoder::decode("l5:helloe".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),
                   BType::List(vec![BType::ByteString("hello".to_string())]));
    }

    #[test]
    fn it_can_parse_a_nested_list() {
        let result = BEncoder::decode("ll5:helloee".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),
                   BType::List(vec![BType::List(vec![BType::ByteString("hello".to_string())])]));
    }

    #[test]
    fn it_can_parse_a_complex_list() {
        let result = BEncoder::decode("ll5:helloei-10ee".to_string());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),
                   BType::List(vec![
                       BType::List(vec![BType::ByteString("hello".to_string())]),
                       BType::Integer(-10)
                           ]));
    }
}
