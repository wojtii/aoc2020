use std::fs::read_to_string;

pub fn run() {
    let input = read_to_string("src/day18-input.txt").unwrap();
    let expressions: Vec<Vec<_>> = input
        .lines()
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();

    let result = eval(&expressions, false);
    assert_eq!(4696493914530, result);
    println!("{}", result);

    let result = eval(&expressions, true);
    assert_eq!(362880372308125, result);
    println!("{}", result);
}

fn eval(expressions: &Vec<Vec<char>>, use_precedence: bool) -> u64 {
    expressions
        .iter()
        .map(|x| {
            let mut num_stack = vec![];
            let mut op_stack = vec![];
            for token in x.iter() {
                match *token {
                    '(' => op_stack.push('('),
                    ')' => drain(&mut op_stack, &mut num_stack, false, None),

                    '+' | '*' => {
                        if use_precedence {
                            drain(&mut op_stack, &mut num_stack, true, Some(*token));
                        } else {
                            drain(&mut op_stack, &mut num_stack, true, None);
                        }
                        op_stack.push(*token);
                    }

                    n => num_stack.push(n as u64 - '0' as u64),
                }
            }

            drain(&mut op_stack, &mut num_stack, false, None);
            num_stack[0]
        })
        .sum()
}

fn drain(
    op_stack: &mut Vec<char>,
    num_stack: &mut Vec<u64>,
    give_back: bool,
    next_op: Option<char>,
) {
    while let Some(op) = op_stack.pop() {
        if op == '(' {
            if give_back {
                op_stack.push('(');
            }
            return;
        }

        if let Some(next_op) = next_op {
            if next_op == '+' && op == '*' {
                op_stack.push('*');
                break;
            }
        }

        let (n1, n2) = (num_stack.pop().unwrap(), num_stack.pop().unwrap());
        num_stack.push(match op {
            '+' => n1 + n2,
            '*' => n1 * n2,
            x => unreachable!(format!("unexpected '{}'", x)),
        });
    }
}
