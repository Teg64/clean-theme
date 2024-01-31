mod sample_extra;

fn main() {
    println!("Hello, world! {}", 100);

    let tokens: Vec<String> = vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]
    .iter()
    .map(|&s| s.into())
    .collect();

    let res = Solution::eval_rpn(tokens);

    println!("Result: {:?}", res);
}

struct Solution;

impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        Self::evaluate(&mut tokens)
    }

    // Treat tokens as a stack, and recursively evaluate the expression via tokens.pop()
    /// docstring
    fn evaluate(tokens: &mut Vec<String>) -> i32 {
        let token = Token::from(
            tokens
                .pop()
                .expect("Attempted to pop from empty tokens vector"),
        );

        match token {
            Token::Operator(op_type) => {
                let num_1 = Self::evaluate(tokens);
                let num_2 = Self::evaluate(tokens);

                match op_type {
                    OpType::Add => num_2 + num_1,
                    OpType::Subtract => num_2 - num_1,
                    OpType::Multiply => num_2 * num_1,
                    OpType::Divide => num_2 / num_1,
                }
            }
            Token::Number(num) => num,
        }
    }
}

#[derive(Debug)]
enum Token {
    Operator(OpType),
    Number(i32),
}

#[derive(Debug)]
enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<String> for Token {
    fn from(token: String) -> Token {
        if let Ok(num) = token.parse::<i32>() {
            return Token::Number(num);
        }

        match token.as_str() {
            "+" => Token::Operator(OpType::Add),
            "-" => Token::Operator(OpType::Subtract),
            "*" => Token::Operator(OpType::Multiply),
            "/" => Token::Operator(OpType::Divide),
            _ => panic!("Unable to parse token with value: {:?}", token),
        }
    }
}
