use crate::core::*;

pub struct State<N: Num> {
    expr: Vec<char>,
    cursor: usize,
    last_result: Option<N>,
}

impl<N: Num> State<N> {
    pub fn new() -> State<N> {
        State {
            expr: Vec::new(),
            cursor: 0,
            last_result: None,
        }
    }

    pub fn input(&mut self, c: char) {
        self.expr.insert(self.cursor.into(), c);
        self.cursor += 1;
        self.update();
    }

    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor < self.expr.len() {
            self.cursor += 1;
        }
    }

    pub fn cursor_first(&mut self) {
        self.cursor = 0;
    }

    pub fn cursor_last(&mut self) {
        self.cursor = self.expr.len();
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.expr.remove(self.cursor - 1);
            self.cursor -= 1;
            self.update();
        }
    }

    pub fn delete(&mut self) {
        if self.cursor < self.expr.len() {
            self.expr.remove(self.cursor);
            self.update();
        }
    }

    pub fn clear(&mut self) {
        self.expr.clear();
        self.cursor = 0;
    }

    pub fn expr(&self) -> String {
        // TODO: メモ化
        let mut ret = String::new();
        self.expr.iter().for_each(|c| ret.push(*c));
        ret
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn last_result(&self) -> Option<N> {
        self.last_result
    }

    fn update(&mut self) {
        match parse(&self.expr()) {
            Ok((_, expr)) => {
                let res = expr.calc();
                self.last_result = Some(res);
            }
            _ => {}
        }
    }
}
