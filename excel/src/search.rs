pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    haystack
        .lines()
        .filter(|line| line.contains(needle))
        .collect()
}

pub fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let needle = needle.to_lowercase();
    let mut result = Vec::new();

    for line in haystack.lines() {
        if line.to_lowercase().contains(&needle) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let needle = "duct";
        let haystack = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(needle, haystack));
    }

    #[test]
    fn case_insensitive() {
        let needle = "rUsT";
        let haystack = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(needle, haystack)
        );
    }
}
