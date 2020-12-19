use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let part_1: usize = reader.lines().map(|line| run(&parse(&line.unwrap()))).sum();
    println!("Part 1: {}", part_1);
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Token {
    Number(usize),
    Plus,
    Star,
    LeftParen,
    RightParen,
}

fn parse(expr: &str) -> Vec<Token> {
    let mut result = vec![];
    let mut chars = expr.chars().filter(|c| !c.is_whitespace()).peekable();
    let chars_iter = chars.by_ref();

    while let Some(c) = chars_iter.next() {
        let token = match c {
            '+' => Token::Plus,
            '*' => Token::Star,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            n if n.is_digit(10) => {
                let mut number = vec![n];

                while chars_iter.peek().map_or(false, |c| c.is_digit(10)) {
                    number.push(chars_iter.next().unwrap());
                }

                let number: String = number.iter().copied().collect();
                Token::Number(number.parse().unwrap())
            }
            other => panic!("Unexpected character `{}`", other),
        };

        result.push(token);
    }

    result
}

fn run(tokens: &Vec<Token>) -> usize {
    run_recursive(&mut tokens.iter())
}

fn run_recursive<'a>(tokens: &mut impl Iterator<Item = &'a Token>) -> usize {
    let mut result = 0;
    let mut last_operator = None;

    while let Some(token) = tokens.next() {
        match token {
            Token::Plus | Token::Star => last_operator = Some(token),
            Token::Number(n) => match last_operator {
                Some(Token::Plus) => result += n,
                Some(Token::Star) => result *= n,
                None => result = *n,
                _ => unreachable!(),
            },
            Token::LeftParen => {
                let inner_result = run_recursive(tokens);

                match last_operator {
                    Some(Token::Plus) => result += inner_result,
                    Some(Token::Star) => result *= inner_result,
                    None => result = inner_result,
                    _ => unreachable!(),
                }
            }
            Token::RightParen => return result,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = parse("2 + (3 * 4)");
        assert_eq!(
            tokens,
            vec![
                Token::Number(2),
                Token::Plus,
                Token::LeftParen,
                Token::Number(3),
                Token::Star,
                Token::Number(4),
                Token::RightParen
            ]
        )
    }

    #[test]
    fn test_run() {
        assert_eq!(3, run(&parse("1 + 2")));
        assert_eq!(9, run(&parse("1 + 2 * 3")));
        assert_eq!(7, run(&parse("1 + (2 * 3)")));
        assert_eq!(9, run(&parse("(1 + 2) * 3")));
        assert_eq!(51, run(&parse("1 + (2 * 3) + (4 * (5 + 6))")));
    }
}
