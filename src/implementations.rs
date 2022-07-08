pub mod dfs;
pub mod bfs;
pub mod closure_number;

pub type GenerateParenthesesFn = fn(i32) -> Vec<String>;

pub static IMPLEMENTATIONS: &[(&str, GenerateParenthesesFn)] = &[
    ("dfs", dfs::generate_parenthesis),
    ("bfs", bfs::generate_parenthesis),
    ("closure_number", closure_number::generate_parenthesis)
];

pub fn find_implementation(impl_name: &str) -> Option<GenerateParenthesesFn> {
    IMPLEMENTATIONS.iter().find(
        |(name, _)|
            impl_name == *name
    ).map(|(_, fun)| *fun)
}