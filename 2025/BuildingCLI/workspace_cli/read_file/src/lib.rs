#[allow(unused_variables)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //unimplemented!();
    //vec![]
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            // Do something with line
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
