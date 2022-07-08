pub fn generate_parenthesis(n: i32) -> Vec<String> {

    let mut parenthesis_sequences:Vec<Vec<String>> = vec![vec!["".to_string()],vec!["()".to_string()]];

    for k in 2..=n as usize {
        let mut k_seqs = vec![];
        for c in 0..k {
            for left in &parenthesis_sequences[c] {
                for right in &parenthesis_sequences[k-1-c] {
                    k_seqs.push(format!("({}){}", left, right))
                }
            }
        }
        parenthesis_sequences.push(k_seqs)
    }
    parenthesis_sequences.pop().unwrap()
}