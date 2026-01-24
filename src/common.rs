use blake3;

pub fn hash_password(password: &str) -> String {
    blake3::hash(password.as_bytes()).to_hex().to_string()
}

#[derive(Debug)]
pub struct Match {
    pub hash: String,
    pub password: String,
}

impl Match {
    pub fn new(password: &str, hash: &str) -> Option<Self> {
        if hash_password(password) == hash {
            Some(Match {
                hash: hash.to_string(),
                password: password.to_string(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Matches(Vec<Match>);

impl Matches {
    pub fn new<F>(hash_path: &str, dictionary_path: &str, cracker: F) -> Self
    where
        F: Fn(&[String], &[String]) -> Vec<Match>,
    {
        let hashes = read_lines(hash_path);
        let words = read_lines(dictionary_path);
        let mut matches = cracker(&hashes, &words);
        matches.sort_by(|a, b| a.password.cmp(&b.password));
        matches.dedup_by(|a, b| a.password == b.password);
        println!("Total matches found: {}", matches.len());
        Self(matches)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn read_lines(path: &str) -> Vec<String> {
    let bytes = std::fs::read(path).expect(&format!("Failed to read {}", path));
    let contents = String::from_utf8_lossy(&bytes);
    contents.lines().map(|s| s.to_string()).collect()
}
