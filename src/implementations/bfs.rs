#[derive(Clone, Debug, PartialEq)]
struct ParenthesesSequence {
    pub line: String,
    pub open: u8,
    pub close: u8,
}

impl ParenthesesSequence {
    #[inline(always)]
    fn new(n: usize) -> ParenthesesSequence {
        ParenthesesSequence {
            line: String::new(),
            open: n as u8,
            close: 0,
        }
    }

    #[inline(always)]
    fn add_opening(self: &mut ParenthesesSequence) {
        self.line.push('(');
        self.open -= 1;
        self.close += 1
    }

    #[inline(always)]
    fn add_closing(self: &mut ParenthesesSequence) {
        self.line.push(')');
        self.close -= 1;
    }

    fn has_alternatives(self:&ParenthesesSequence) -> bool {
        self.open > 0 && self.close > 0
    }

    fn add_parenthesis(
        self: &mut ParenthesesSequence,
        provisioned: &mut Vec<ParenthesesSequence>
    ) {
        if self.has_alternatives() {
            let mut pseq = self.clone();
            pseq.add_opening();
            provisioned.push(pseq);

            self.add_closing();
        } else if self.open > 0 {
            self.add_opening();
        } else {
            self.add_closing();
        }
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut sequences = vec![];

    let mut initial_sequence = ParenthesesSequence::new(n as usize);
    initial_sequence.add_opening();
    sequences.push(initial_sequence);

    for _ in 1..=2 * n - 1 {
        let mut provisioned = vec![];
        for seq in sequences.iter_mut() {
            seq.add_parenthesis(&mut provisioned)
        }
        sequences.append(&mut provisioned);
    }
    sequences.into_iter().map(|l| l.line).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_parenthesis_test() {
        let mut line = ParenthesesSequence {
            line: "((".into(),
            open: 1,
            close: 2,
        };
        let line_o = ParenthesesSequence {
            line: "(((".into(),
            open: 0,
            close: 3,
        };
        let line_c = ParenthesesSequence {
            line: "(()".into(),
            open: 1,
            close: 1,
        };

        let mut vec: Vec<ParenthesesSequence> = vec![];
        line.add_parenthesis(&mut vec);

        assert_eq!(line, line_c);
        assert_eq!(vec[0], line_o);
    }

}