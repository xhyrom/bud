use std::collections::HashMap;

pub struct Template {
    owner: Option<String>,
    name: String,
    url: String,
    files: HashMap<String, String>,
}

impl Template {
    pub fn new(owner: Option<String>, name: String) -> Self {
        Template {
            owner: owner.clone(),
            name: name.clone(),
            url: match owner {
                Some(owner) => format!("https://api.github.com/repos/{}/{}/contents", owner, name),
                None => format!(
                    "https://api.github.com/repos/xhyrom/bud/contents/templates/{}",
                    name
                ),
            },
            files: HashMap::new(),
        }
    }

    pub fn download(&self) {
        self.fetch_files(None);
    }

    fn fetch_files(&self, path: Option<String>) {
        let url = match path {
            Some(path) => format!("{}/{}", self.url, path),
            None => self.url.clone(),
        };

        let response = reqwest::blocking::Client::new()
            .get(url)
            .header("User-Agent", "bud (https://github.com/xHyroM/bud")
            .send()
            .expect("Failed to fetch files");

        println!("{:#?}", response.text().unwrap())
    }
}
