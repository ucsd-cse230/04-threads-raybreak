pub mod common;
pub mod crack;

use common::Matches;

pub fn seq_crack(hash_path: &str, dictionary_path: &str) -> usize {
    Matches::new(hash_path, dictionary_path, crack::seq_crack).len()
}

pub fn par_crack(hash_path: &str, dictionary_path: &str) -> usize {
    Matches::new(hash_path, dictionary_path, crack::par_crack).len()
}
