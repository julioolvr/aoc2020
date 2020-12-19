use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let operations: Vec<Vec<Token>> = reader.lines().map(|line| parse(&line.unwrap())).collect();

    let part_1: usize = operations.iter().map(|operation| run(operation)).sum();
    println!("Part 1: {}", part_1);

    let part_2: usize = operations
        .iter()
        .map(|operation| run_with_precedence(operation))
        .sum();
    println!("Part 2: {}", part_2);
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

#[derive(PartialEq, PartialOrd, Debug)]
enum Precedence {
    None,
    Multiplication,
    Addition,
}

fn run_with_precedence(tokens: &Vec<Token>) -> usize {
    let mut instructions: Vec<Token> = vec![];
    let mut tokens = tokens.iter().peekable();

    parse_tokens(&mut tokens, &mut instructions, Precedence::None);

    let mut stack: Vec<usize> = vec![];

    for instruction in instructions {
        match instruction {
            Token::Number(n) => stack.push(n),
            Token::Plus => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a + b);
            }
            Token::Star => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a * b);
            }
            _ => unreachable!(), // Parenthesis don't exist at this point
        }
    }

    stack.pop().unwrap()
}

// For such simple expressions we can reuse the tokens as instructions
type Instruction = Token;

fn parse_tokens<'a>(
    tokens: &mut Peekable<impl Iterator<Item = &'a Token>>,
    instructions: &mut Vec<Instruction>,
    precedence: Precedence,
) {
    while let Some(token) = tokens.peek() {
        match token {
            Token::LeftParen => {
                tokens.next();
                parse_tokens(tokens, instructions, Precedence::None);
                tokens.next(); // Consume right paren
            }
            Token::Number(_) => {
                let number = tokens.next().unwrap();
                instructions.push(*number);
            }
            other => panic!("Unexpected token {:?}", other),
        }

        while let Some(token) = tokens.peek() {
            match token {
                Token::Plus if precedence <= Precedence::Addition => {
                    tokens.next();
                    parse_tokens(tokens, instructions, Precedence::Addition);
                    instructions.push(Token::Plus);
                }
                Token::Star if precedence <= Precedence::Multiplication => {
                    tokens.next();
                    parse_tokens(tokens, instructions, Precedence::Multiplication);
                    instructions.push(Token::Star);
                }
                _ => return,
            }
        }
    }
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

    #[test]
    fn test_run_with_precedence() {
        assert_eq!(132, run_with_precedence(&parse("(9 * 8 + 6) + 6")));
        assert_eq!(
            51,
            run_with_precedence(&parse("1 + (2 * 3) + (4 * (5 + 6))"))
        );
        assert_eq!(46, run_with_precedence(&parse("2 * 3 + (4 * 5)")));
        assert_eq!(
            1445,
            run_with_precedence(&parse("5 + (8 * 3 + 9 + 3 * 4 * 3)"))
        );
        assert_eq!(
            669060,
            run_with_precedence(&parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"))
        );
        assert_eq!(
            23340,
            run_with_precedence(&parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"))
        );
    }
}
