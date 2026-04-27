#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expression {
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },
    Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

fn eval(e: Expression) -> Result<i64, DivideByZeroError> {
    match e {
        Expression::Op { op, left, right } => {
            let left = eval(*left)?;
            let right = eval(*right)?;
            match op {
                Operation::Add => Ok(left + right),
                Operation::Sub => Ok(left - right),
                Operation::Mul => Ok(left * right),
                Operation::Div => {
                    if right != 0 {
                        Ok(left / right)
                    } else {
                        Err(DivideByZeroError)
                    }
                }
            }
        }
        Expression::Value(v) => Ok(v),
    }
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(DivideByZeroError)
    );
}

fn main() {
    let expr = Expression::Op {
        op: Operation::Div,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(0)),
    };
    
    println!("выражение: {expr:?}");
    match eval(expr) {
        Ok(res) => println!("результат: {res}"),
        Err(_) => println!("ошибка: деление на ноль"),
    }
}
