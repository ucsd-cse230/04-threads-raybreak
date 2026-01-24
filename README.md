# raybreak

Assignment using `rayon` to build a dictionary-based password cracker.

## Overview

In this assignment, you will implement a password cracker that takes a list of password hashes and a dictionary of common passwords, then attempts to find the original passwords by generating variants and checking their hashes.

You need to implement the `todo!()` sections in the `src/crack.rs`.

## Problems

### Problem 1: `first_letter_uppercase`

Implement a function that capitalizes the first letter of a word while keeping the rest unchanged.

- Input: `"hello"` → Output: `"Hello"`
- Input: `"WORLD"` → Output: `"WORLD"` (only first letter changes)

For words with length ≤ 2, return the word unchanged.

After you have implemented this, `cargo test test_first_letter_uppercase` should pass.

### Problem 2: `alternating_case`

Implement a function that converts a word to alternating case, starting with lowercase at index 0.

- Input: `"abcd"` → Output: `"aBcD"`
- Input: `"hello"` → Output: `"hElLo"`

Iterate through the characters, making even indices lowercase and odd indices uppercase.

After you have implemented this, `cargo test test_alternating_case` should pass.

### Problem 3: `leetspeak_variants`

Generate _all possible_ leetspeak variants of a word by substituting characters according to the `leet_replacements` function (already provided).
For example, `"leet"` should generate variants including `"leet"`, `"l33t"`, `"1337"`, `"le3t"`, etc.

After you have implemented this, `cargo test test_leetspeak_variants` should pass.

### Problem 4: `number_variants`

Generate variants by appending numeric suffixes from 0 to `max_suffix` (inclusive).

- Input: `("pass", 2)` → Output: `["pass0", "pass1", "pass2"]`

After you have implemented this, `cargo test test_number_variants` should pass.

### Problem 5: `variants`

Combine all variant generators to produce the full set of password candidates:

1. Generate easy case variants of the word
2. Generate leetspeak variants of the word
3. For each variant from steps 1-2, also generate number variants with suffixes 0-99
4. Sort and deduplicate the results

After you have implemented this, `cargo test test_seq_crack` should pass.

Alternatively, you can manually test your implementation using:

```sh
$ cargo run -- seq hash/easy_5_100.txt dict/rockyou-100.txt
```

### Problem 6: `par_crack`

Parallelize the password cracking using Rayon. The sequential version `seq_crack` is provided as a reference.

Your implementation should:

- Use `par_iter()` to parallelize iteration over dictionary words
- Use `into_par_iter()` to parallelize iteration over variants (nested parallelism)
- Use `flat_map` and `filter_map` to collect matches
- Use `Match::new(&candidate, hash)` to check if a candidate matches a hash

Refer to the [Rayon documentation](https://docs.rs/rayon/latest/rayon/) for `ParallelIterator` methods.

After you have implemented this, `cargo test test_par_crack` should pass.

```sh
$ cargo run -- par hash/easy_5_100.txt dict/rockyou-100.txt
```

## Running Tests

```bash
# Run all tests
cargo test --release --no-fail-test

# Run a specific test
cargo test test_first_letter_uppercase --release

# Run tests with output
cargo test --release
```

Test cases of increasing difficulty:

- `test_*_5_100` - 5 passwords, 100-word dictionary (fastest)
- `test_*_20_1k` - 20 passwords, 1k-word dictionary
- `test_*_50_10k` - 50 passwords, 10k-word dictionary (slowest)

Compare the runtime of `seq_crack` vs `par_crack` to see the speedup from parallelism.

Specifically, you should see a 4x (or more) speedup on the largest test case when using `par_crack` (assuming you run on a machine with 4 or more cores).
