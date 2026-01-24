use raybreak::{par_crack, seq_crack};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: raybreak MODE path/to/hash.txt path/to/dict.txt");
        eprintln!("  MODE: seq or par");
        std::process::exit(1);
    }

    let mode = &args[1];
    let hash_path = &args[2];
    let dict_path = &args[3];

    let matches = match mode.as_str() {
        "seq" => seq_crack(hash_path, dict_path),
        "par" => par_crack(hash_path, dict_path),
        _ => {
            eprintln!("Invalid mode: {}. Use 'seq' or 'par'.", mode);
            std::process::exit(1);
        }
    };

    println!("Cracked {} passwords", matches);
}
