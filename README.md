# Generate Parentheses with Rust and CLion

I use this repo with different solutions to [Leetcode's Problem #22](https://leetcode.com/problems/generate-parentheses/)
in Rust to demonstrate how various CLion features can help with the development process. 

## The problem

Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

**Example 1:**

```
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
```

**Example 2:**

```
Input: n = 1
Output: ["()"]
```

**Constraints:**

- `1 <= n <= 8`

### Solutions

#### Approach 1: Breadth-first Search

* Code: [gp_algo::implementations::bfs](./src/implementations/bfs.rs)
* We construct a vector that will contain all the valid combinations eventually, starting with `["(")]`. For every combination, we keep the number of `open` and `close` parenthesis available and depending on them add either an opening parenthesis, or a closing parenthesis, or both. In the latter case, we extend the vector with the new combination. We are done when there are no available parenthesis of any kind. We call this approach *breadth-first search* because we build the tree of all the combinations level by level. For example, with `n=3`:
```
                           (
               ((                  ()
      (()           (((            ()(
(())      (()(      ((()      ()((     ()()  
(())(     (()()     ((())     ()(()    ()()(
(())()    (()())    ((()))    ()(())   ()()()
```

#### Approach 2: Depth-first Search

* Code: [gp_algo::implementations::dfs](./src/implementations/dfs.rs)
* We use recursion to build combinations based on the numbers of opening and closing parentheses available. Every recursion step adds one parenthesis. Once we are out of parentheses we emit the combination we've built and step back to explore other available options to build combinations. We call this approach *depth-first search* because we try to reach leaves of the tree above as early as possible.

#### Approach 3: Subcombinations

* Code: [gp_algo::implementations::sub](./src/implementations/sub.rs)
* Every valid combination has the following form: `(...)...` where `...` corresponds to a valid combination of a strictly smaller size (including empty string). To enumerate all the combinations, we enumerate all the combinations of smaller sizes and put them into such a form. We should be careful to go other all the sizes without exemption and make sure that the final combination has precisely the size we need.

## Main features

- Using command-line arguments with `clap`
- Benchmarking code with `criterion`
- Testing
- Profiling
- Exploring memory usage
- Using Instruments (macOS) and Valgrind (Linux)
- Integrating with Docker

## Links

- [Cargo criterion plugin](https://crates.io/crates/cargo-criterion)
- [Cargo instruments plugin](https://crates.io/crates/cargo-instruments)
- [Valgrind](https://valgrind.org/)