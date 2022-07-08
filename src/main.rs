use gp_algo::implementations::{*};
use memuse::DynamicUsage;
use human_bytes::human_bytes;
use std::time::Instant;
use clap::{CommandFactory, ErrorKind, Parser};
use itertools::Itertools;
use termion::{color, style};

#[derive(Parser)]
#[clap(author, about, long_about = None, trailing_var_arg=true)]
struct Arguments {

    #[clap(short, long, value_parser)]
    /// Set problem size
    size: i32,

    #[clap(short, long, action)]
    /// Provide information on memory usage
    verbose: bool,

    #[clap(multiple_values=true)]
    /// Implementations requested (at least one implementation is expected)
    implementations: Vec<String>,
}

fn report(n: i32, implementation_name: String, generator: GenerateParenthesesFn, verbose: bool) {
    println!("{}== Running: {} =={}", style::Bold, implementation_name, style::Reset);
    let now = Instant::now();
    let vec = generator(n);
    let elapsed = now.elapsed();
    println!("Got results in: {:.2?}", elapsed);

    if verbose {
        println!("Resulting vector info:");
        println!("  Size: {}", vec.len());
        println!("  Capacity: {}", vec.capacity());
        let total_str_capacity:usize = vec.iter().map(|s| s.capacity()).sum();
        println!("  Total Strings' Capacity: {}", human_bytes(total_str_capacity as f64));
        println!("  Average Capacity per String: {:.2}", total_str_capacity / vec.len());
        println!("  Total memory used: {}", human_bytes(vec.dynamic_usage() as f64));
    }
}

fn main() {
    let args = Arguments::parse();

    let available_implementations_info =
        format!("{}Available implementations are: {}.{}",
                 color::Fg(color::Yellow),
                 IMPLEMENTATIONS.iter().map(|(name, _)| name).join(", "),
                 style::Reset);

    if args.implementations.is_empty() {
        Arguments::command().error(
            ErrorKind::ArgumentNotFound,
            format!("{}No implementations requested.{}\n{}.", style::Bold, style::Reset, available_implementations_info)
        ).exit();
    }

    for impl_name in args.implementations {
        match find_implementation(impl_name.as_str()) {
            None => {
                eprintln!("{}error:{} {}Unknown implementation: `{}`.{}\n{}",
                         color::Fg(color::Red), style::Reset, style::Bold,
                         impl_name, style::Reset,
                         available_implementations_info);
            }
            Some(gen) => {
                report(args.size, impl_name, gen, args.verbose)
            }
        }
    }
}
