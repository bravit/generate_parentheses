#[derive(Clone, Debug, PartialEq)]
struct ParenthesesCombination {
    pub line: String,
    pub open: u8,
    pub close: u8,
}

impl ParenthesesCombination {
    #[inline(always)]
    fn new(n: usize) -> Self {
        Self {
            line: String::with_capacity(n as usize * 2),
            open: n as u8,
            close: 0,
        }
    }

    #[inline(always)]
    fn add_opening(&mut self) {
        self.line.push('(');
        self.open -= 1;
        self.close += 1
    }

    #[inline(always)]
    fn add_closing(&mut self) {
        self.line.push(')');
        self.close -= 1;
    }

    fn has_alternatives(&self) -> bool {
        self.open > 0 && self.close > 0
    }

    fn add_parenthesis(
        &mut self,
        provisioned: &mut Vec<Self>
    ) {
        if self.has_alternatives() {
            let mut comb = self.clone();
            comb.line.reserve_exact(self.line.capacity() - comb.line.len());
            comb.add_opening();
            provisioned.push(comb);

            self.add_closing();
        } else if self.open > 0 {
            self.add_opening();
        } else {
            self.add_closing();
        }
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut combinations = vec![];

    let mut initial_combination = ParenthesesCombination::new(n as usize);
    initial_combination.add_opening();
    combinations.push(initial_combination);

    for _ in 1..=2 * n - 1 {
        let expected = combinations.iter().filter(
            |pc| pc.has_alternatives()
        ).count();
        let mut provisioned = Vec::with_capacity(expected);
        for comb in combinations.iter_mut() {
            comb.add_parenthesis(&mut provisioned)
        }
        combinations.append(&mut provisioned);
    }
    combinations.into_iter().map(|l| l.line).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_parenthesis_test() {
        let mut comb = ParenthesesCombination {
            line: "((".into(),
            open: 1,
            close: 2,
        };
        let comb_o = ParenthesesCombination {
            line: "(((".into(),
            open: 0,
            close: 3,
        };
        let comb_c = ParenthesesCombination {
            line: "(()".into(),
            open: 1,
            close: 1,
        };

        let mut vec: Vec<ParenthesesCombination> = vec![];
        comb.add_parenthesis(&mut vec);

        assert_eq!(comb, comb_c);
        assert_eq!(vec[0], comb_o);
    }

}