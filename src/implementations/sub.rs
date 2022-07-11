pub fn generate_parenthesis(n: i32) -> Vec<String> {

    let mut combinations:Vec<Vec<String>> = vec![vec!["".to_string()], vec!["()".to_string()]];

    for k in 2..=n as usize {
        let mut k_combination = vec![];
        for c in 0..k {
            for left in &combinations[c] {
                for right in &combinations[k-1-c] {
                    // valid combinations of size `c + (k-1-c) + 1 = k`
                    k_combination.push(format!("({}){}", left, right))
                }
            }
        }
        combinations.push(k_combination)
    }
    combinations.pop().unwrap()
}