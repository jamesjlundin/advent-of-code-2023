Advent of Code 2023!

Utilizing the Rust Language for this year's Advent of Code!

Setup

1. If you don't have Rust installed yet you can run this command on Linux or Mac which I pulled from https://doc.rust-lang.org/cargo/getting-started/installation.html
   `curl https://sh.rustup.rs -sSf | sh`

2. Each day in the base directory of the repo run the command
   `cargo new day-{00}`

3. CD into the new day folder created via the command in step 1
   `cd day-{00}`

4. Run this command to create a directory structure that will make it easy to run parts 1 and 2

   ```
   rm src/main.rs && mkdir -p src/bin && touch src/bin/input_1.txt src/bin/input_2.txt && { echo 'fn main() {'; echo '    let input = include_str!("input_1.txt");'; echo '    let output = part1(input);'; echo '    dbg!(output);'; echo '}'; echo ''; echo 'fn part1(input: &str) -> String {'; echo '    "todo!()".to_string()'; echo '}'; echo ''; echo '#[cfg(test)]'; echo 'mod tests {'; echo '    use super::*;'; echo ''; echo '    #[test]'; echo '    fn it_works() {'; echo '        let result = part1("");'; echo '        assert_eq!(result, "4".to_string());'; echo '    }'; echo '}'; } > src/bin/part_1.rs && { echo 'fn main() {'; echo '    let input = include_str!("input_2.txt");'; echo '    let output = part1(input);'; echo '    dbg!(output);'; echo '}'; echo ''; echo 'fn part1(input: &str) -> String {'; echo '    "todo!()".to_string()'; echo '}'; echo ''; echo '#[cfg(test)]'; echo 'mod tests {'; echo '    use super::*;'; echo ''; echo '    #[test]'; echo '    fn it_works() {'; echo '        let result = part1("");'; echo '        assert_eq!(result, "4".to_string());'; echo '    }'; echo '}'; } > src/bin/part_2.rs
   ```

5. To run your code you will then use the command based on which part you want to run
   `cargo run --bin part{1 or 2}`

6. To run tests
   `cargo test`
