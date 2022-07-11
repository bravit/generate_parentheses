pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut combinations: Vec<String> = vec![];
    let mut str = String::new();
    dfs(&mut combinations, &mut str, n as usize, 0);
    combinations
}

fn dfs(combinations: &mut Vec<String>, str: &mut String, open: usize, close: usize) {
    if (open == 0) && (close == 0) {
        combinations.push(str.clone());
        return
    }
    if open > 0 {
        str.push('(');
        dfs(combinations, str, open-1, close+1);
        str.pop();
    }
    if close > 0 {
        str.push(')');
        dfs(combinations, str, open, close-1);
        str.pop();
    }
}
