pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    // data on stack
    values: Vec<Value>,
    // operations to do
    ops: Vec<String>,
    // user defined operations
    user_ops: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            values: Vec::new(),
            ops: vec![
                "+".to_string(),
                "-".to_string(),
                "*".to_string(),
                "/".to_string(),
                "DUP".to_string(),
                "DROP".to_string(),
                "SWAP".to_string(),
                "OVER".to_string(),
            ],
            user_ops: Vec::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.values
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let orig = input.to_string();
        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        for (i, word) in words.iter().enumerate() {
            if word.len() < 1 {
                continue;
            }

            println!("{}\tword: {}", i, word);

            if is_number(word) {
                self.values.push(word.parse::<Value>().unwrap());
                continue;
            }
        }

        Ok(())
    }
}

fn is_number(text: &str) -> bool {
    text.chars().all(|c| c.is_digit(10))
}

/*  Implement an evaluator for a very simple subset of Forth.
Forth is a stack-based programming language. Implement a very basic evaluator
for a small subset of Forth.

Your evaluator has to support the following words:

    - +, -, *, / (integer arithmetic)
    - DUP, DROP, SWAP, OVER (stack manipulation)
    - Your evaluator also has to support defining new words using the customary syntax:
        : word-name definition ;.

To keep things simple the only data type you need to support is signed integers
of at least 16 bits size.

You should use the following rules for the syntax: a number is a sequence of
one or more (ASCII) digits, a word is a sequence of one or more letters, digits,
symbols or punctuation that is not a number. (Forth probably uses slightly different rules,
but this is close enough.)

Words are case-insensitive.
 */
