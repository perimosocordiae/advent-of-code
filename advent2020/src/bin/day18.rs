use advent2020::*;

fn main() {
    let data = read_string("inputs/18.full");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
}

#[test]
fn part1_small() {
    assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
    assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let tokens = tokenize_expr(line);
            let rpn = convert_to_rpn(&tokens, false);
            run_stack_machine(&rpn)
        })
        .sum()
}

#[test]
fn part2_small() {
    assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6"), 231);
    assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
    assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}

fn part2(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let tokens = tokenize_expr(line);
            let rpn = convert_to_rpn(&tokens, true);
            run_stack_machine(&rpn)
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Num(usize),
    Plus,
    Star,
    LParen,
    RParen,
}

fn tokenize_expr(input: &str) -> Vec<Token> {
    input
        .chars()
        .filter(|ch| *ch != ' ')
        .map(|ch| match ch {
            '+' => Token::Plus,
            '*' => Token::Star,
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => Token::Num(ch.to_digit(10).unwrap() as usize),
        })
        .collect()
}

fn run_stack_machine(tokens: &[Token]) -> usize {
    let mut stack = Vec::<usize>::new();
    for tok in tokens {
        match tok {
            Token::Num(x) => {
                stack.push(*x);
            }
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
            _ => panic!("unexpected token {:?}", tok),
        }
    }
    stack[0]
}

fn convert_to_rpn(tokens: &[Token], plus_precedence: bool) -> Vec<Token> {
    // Shunting-yard algorithm
    let mut out_queue = vec![];
    let mut oper_queue: Vec<Token> = vec![];
    for tok in tokens {
        match tok {
            Token::Num(_) => {
                out_queue.push(*tok);
            }
            Token::Plus | Token::Star => {
                while let Some(op) = oper_queue.pop() {
                    if op == Token::LParen
                        || (plus_precedence
                            && *tok == Token::Plus
                            && op == Token::Star)
                    {
                        oper_queue.push(op);
                        break;
                    }
                    out_queue.push(op);
                }
                oper_queue.push(*tok);
            }
            Token::LParen => {
                oper_queue.push(*tok);
            }
            Token::RParen => {
                while let Some(op) = oper_queue.pop() {
                    if op != Token::LParen {
                        out_queue.push(op);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    while let Some(op) = oper_queue.pop() {
        out_queue.push(op);
    }
    out_queue
}
