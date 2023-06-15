use regex::Regex;

fn remove_comments(content: &str) -> String {
    let re = Regex::new(r"/\*.*?\*/").unwrap();
    re.replace_all(content, "").to_string()
}

pub fn preprocess(content: &str) -> String {
    let content = remove_comments(content);

    return content;
}
