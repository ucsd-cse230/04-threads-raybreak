use raybreak::{par_crack, seq_crack};

#[test]
fn test_seq_crack_5_100() {
    assert_eq!(seq_crack("hash/easy_5_100.txt", "dict/rockyou-100.txt"), 5);
}

#[test]
fn test_par_crack_5_100() {
    assert_eq!(par_crack("hash/easy_5_100.txt", "dict/rockyou-100.txt"), 5);
}

#[test]
fn test_seq_crack_20_1k() {
    assert_eq!(seq_crack("hash/easy_20_1k.txt", "dict/rockyou-1k.txt"), 20);
}

#[test]
fn test_par_crack_20_1k() {
    assert_eq!(par_crack("hash/easy_20_1k.txt", "dict/rockyou-1k.txt"), 20);
}

// #[test]
// fn test_seq_crack_50_10k() {
//     assert_eq!(seq_crack("hash/easy_50_10k.txt", "dict/rockyou-10k.txt"), 50);
// }

// #[test]
// fn test_par_crack_50_10k() {
//     assert_eq!(par_crack("hash/easy_50_10k.txt", "dict/rockyou-10k.txt"), 50);
// }
