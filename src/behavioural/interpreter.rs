
use std::str::Chars;

// Mathematical expression to postfix expression (reverse polish notation).

pub struct Interpreter<'a> {
    chars:  Chars<'a>,
}
impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self {
            chars: infix.chars(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn consume(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{}'", ch),
            None => panic!("Unexpected end of expression",)
        }
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.consume(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.consume(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{}'", op);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    
    use super::Interpreter;

    #[test]
    fn test_valid_simple_expression() {
        let mut interpreter = Interpreter::new("2+3");
        let mut postfix = String::new();
        interpreter.interpret(&mut postfix);
        assert_eq!("23+", postfix)
    }

    #[test]
    fn test_valid_long_expression() {
        let mut interpreter = Interpreter::new("1-2+3-4");
        let mut postfix = String::new();
        interpreter.interpret(&mut postfix);
        assert_eq!("12-3+4-", postfix);
    }

    #[test]
    #[should_panic(expected = "Unexpected end of expression")]
    fn test_unpexected_end_of_expression() {
        let mut interpreter = Interpreter::new("1-2+");
        let mut postfix = String::new();
        interpreter.interpret(&mut postfix);
    }

    #[test]
    #[should_panic(expected = "Unexpected symbol '*'")]
    fn test_unpexected_symbol() {
        let mut interpreter = Interpreter::new("1*2");
        let mut postfix = String::new();
        interpreter.interpret(&mut postfix);
    }

   #[test]
    #[should_panic(expected = "Unexpected symbol '2'")]
    fn test_unpexected_symbol_double_digit() {
        let mut interpreter = Interpreter::new("1+22");
        let mut postfix = String::new();
        interpreter.interpret(&mut postfix);
    }        
}