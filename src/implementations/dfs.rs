pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut sequences: Vec<String> = vec![];
    let mut str = String::new();
    dfs(&mut sequences, &mut str, n as usize, 0);
    sequences
}

fn dfs(sequences: &mut Vec<String>, str: &mut String, open: usize, close: usize) {
    if (open == 0) && (close == 0) {
        sequences.push(str.clone());
        return
    }
    if open > 0 {
        str.push('(');
        dfs(sequences, str, open-1, close+1);
        str.pop();
    }
    if close > 0 {
        str.push(')');
        dfs(sequences, str, open, close-1);
        str.pop();
    }
}
