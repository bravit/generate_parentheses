pub mod dfs;
pub mod bfs;
pub mod sub;

pub type GenerateParenthesesFn = fn(i32) -> Vec<String>;

pub static IMPLEMENTATIONS: &[(&str, GenerateParenthesesFn)] = &[
    ("dfs", dfs::generate_parenthesis),
    ("bfs", bfs::generate_parenthesis),
    ("sub", sub::generate_parenthesis)
];

pub fn find_implementation(impl_name: &str) -> Option<GenerateParenthesesFn> {
    IMPLEMENTATIONS.iter().find(
        |(name, _)|
            impl_name == *name
    ).map(|(_, fun)| *fun)
}

pub fn catalan_number(n: usize) -> usize {
    match n {
        0 => 1,
        _ => catalan_number(n - 1) * 2 * (2 * n - 1) / (n + 1)
    }
}
