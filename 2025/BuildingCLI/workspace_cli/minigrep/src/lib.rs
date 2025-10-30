// Using lifetimes as search has reference args
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

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //unimplemented!();
    //vec![]
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            // Do something with line
            results.push(line);
        }
    }
    results
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";
//
//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
//
//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";
//
//         assert_eq!(
//             vec!["Rust", "Systems Programming Lang"],
//             search_insensitive(query, contents)
//         );
//     }
// }

//-----
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
