use super::expression::Expr;

#[derive(Debug)]
struct ParseState {
    parse_stack: Vec<Expr>,
    parse_buffer: Vec<char>,
    tokens: Vec<String>,
    in_symbol: bool,
}

impl ParseState {
    fn new() -> Self {
        ParseState {
            parse_stack: vec![Expr::Program(Vec::new())],
            parse_buffer: Vec::new(),
            tokens: Vec::new(),
            in_symbol: false,
        }
    }

    fn complete_token(&mut self) {
        if !self.in_symbol {
            return;
        }
        let tok: String = self.parse_buffer.iter().collect();
        if let Some(Expr::List(list)) = self.parse_stack.last_mut() {
            let first = tok.chars().next().unwrap();
            if first.is_digit(10) {
                list.push(Expr::Integer(tok.parse::<i32>().unwrap()))
            } else {
                list.push(Expr::Symbol(tok));
            }
        } else {
            panic!("AAAaargh!");
        }
        self.in_symbol = false;
        self.parse_buffer.clear();
    }

    fn complete_list(&mut self) {
        if let Some(exp) = self.parse_stack.pop() {
            if let Some(Expr::List(v)) = self.parse_stack.last_mut() {
                v.push(exp);
            } else if let Some(Expr::Program(v)) = self.parse_stack.last_mut() {
                v.push(exp);
            } else {
                panic!(
                    "Unable to push expression to list: {:?}",
                    self.parse_stack.last()
                );
            }
        } else {
            panic!("Unexpected ')'");
        }
    }

    fn complete(mut self) -> Expr {
        assert!(self.parse_stack.len() == 1);
        self.parse_stack.pop().unwrap()
    }
}

/// Parse a program into syntax tree
pub fn parse(input: &str) -> Expr {
    // TODO: Create helpers
    let state = input.chars().fold(ParseState::new(), |mut state, ch| {
        if ch == '(' {
            state.complete_token();
            state.parse_stack.push(Expr::List(Vec::new()));
        } else if ch == ')' {
            state.complete_token();
            state.complete_list();
        } else if ch.is_whitespace() {
            state.complete_token();
        } else {
            state.parse_buffer.push(ch);
            state.in_symbol = true;
        }

        state
    });
    state.complete()
}
