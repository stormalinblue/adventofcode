use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum Operand {
    Variable(String),
    Constant(u16),
}

#[derive(Debug, Clone)]
struct BitAnd {
    left: Operand,
    right: Operand,
}

impl BitAnd {
    fn substitute(&self, left_val: u16, right_val: u16) -> u16 {
        left_val & right_val
    }
}

#[derive(Debug, Clone)]
struct BitOr {
    left: Operand,
    right: Operand,
}

impl BitOr {
    fn substitute(&self, left_val: u16, right_val: u16) -> u16 {
        left_val | right_val
    }
}

#[derive(Debug, Clone)]
struct Not {
    op: Operand,
}

impl Not {
    fn substitute(&self, op_val: u16) -> u16 {
        !op_val
    }
}

#[derive(Debug, Clone)]
struct LShift {
    op: Operand,
    shift: u64,
}

impl LShift {
    fn substitute(&self, op_val: u16) -> u16 {
        op_val << self.shift
    }
}

#[derive(Debug, Clone)]
struct RShift {
    op: Operand,
    shift: u64,
}

impl RShift {
    fn substitute(&self, op_val: u16) -> u16 {
        op_val >> self.shift
    }
}

#[derive(Debug, Clone)]
struct Direct {
    op: Operand,
}

#[derive(Debug, Clone)]
enum Expression {
    BitAnd(BitAnd),
    BitOr(BitOr),
    Not(Not),
    LShift(LShift),
    RShift(RShift),
    Direct(Direct),
}

fn parse_operand(string: &str) -> Operand {
    match string.parse::<u16>() {
        Ok(val) => Operand::Constant(val),
        _ => Operand::Variable(string.to_string()),
    }
}

fn parse_input() -> HashMap<String, Expression> {
    let mut expr_map = HashMap::<String, Expression>::new();
    for line_buf in io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("Could not read line"))
    {
        let line = line_buf.trim();
        let (expr, expr_name) = line.split_once(" -> ").expect("Should have arrow");

        if expr.contains("AND") {
            let (left, right) = expr.split_once(" AND ").expect("Should have ' AND '");
            expr_map.insert(
                expr_name.to_string(),
                Expression::BitAnd(BitAnd {
                    left: parse_operand(left),
                    right: parse_operand(right),
                }),
            );
        } else if expr.contains("OR") {
            let (left, right) = expr.split_once(" OR ").expect("Should have ' OR '");
            expr_map.insert(
                expr_name.to_string(),
                Expression::BitOr(BitOr {
                    left: parse_operand(left),
                    right: parse_operand(right),
                }),
            );
        } else if expr.contains("LSHIFT") {
            let (op, shift) = expr.split_once(" LSHIFT ").expect("Should have ' LSHIFT '");
            expr_map.insert(
                expr_name.to_string(),
                Expression::LShift(LShift {
                    op: parse_operand(op),
                    shift: shift.parse().expect("Should be a positive shift"),
                }),
            );
        } else if expr.contains("RSHIFT") {
            let (op, shift) = expr.split_once(" RSHIFT ").expect("Should have ' RSHIFT '");
            expr_map.insert(
                expr_name.to_string(),
                Expression::RShift(RShift {
                    op: parse_operand(op),
                    shift: shift.parse().expect("Should be a positive shift"),
                }),
            );
        } else if expr.starts_with("NOT ") {
            let op = &expr[4..];
            expr_map.insert(
                expr_name.to_string(),
                Expression::Not(Not {
                    op: parse_operand(op),
                }),
            );
        } else {
            expr_map.insert(
                expr_name.to_string(),
                Expression::Direct(Direct {
                    op: parse_operand(expr),
                }),
            );
        }
    }
    expr_map
}

fn evaluate(
    name: &str,
    defns: &HashMap<String, Expression>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if cache.contains_key(name) {
        *cache.get(name).unwrap()
    } else {
        let mut eval_op = |x: &Operand| match x {
            Operand::Constant(val) => *val,
            Operand::Variable(name) => evaluate(&name, defns, cache),
        };
        let result = {
            match defns
                .get(name)
                .expect(format!("Should have definition for '{}'", name).as_str())
            {
                Expression::BitAnd(expr) => {
                    expr.substitute(eval_op(&expr.left), eval_op(&expr.right))
                }
                Expression::BitOr(expr) => {
                    expr.substitute(eval_op(&expr.left), eval_op(&expr.right))
                }
                Expression::LShift(expr) => expr.substitute(eval_op(&expr.op)),
                Expression::RShift(expr) => expr.substitute(eval_op(&expr.op)),
                Expression::Not(expr) => expr.substitute(eval_op(&expr.op)),
                Expression::Direct(expr) => eval_op(&expr.op),
            }
        };
        cache.insert(name.to_string(), result);
        result
    }
}

fn part1(defns: &HashMap<String, Expression>) -> u16 {
    let mut cache = HashMap::<String, u16>::new();
    evaluate("a", defns, &mut cache)
}

fn part2(orig_defns: &HashMap<String, Expression>) -> u16 {
    let orig_a_val = part1(orig_defns);

    let defns = {
        let mut defns = orig_defns.clone();
        defns.insert(
            "b".to_string(),
            Expression::Direct(Direct {
                op: Operand::Constant(orig_a_val),
            }),
        );
        defns
    };

    let mut cache = HashMap::<String, u16>::new();
    evaluate("a", &defns, &mut cache)
}

fn main() {
    // println!("{:#?}", parse_input());
    let input = parse_input();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
