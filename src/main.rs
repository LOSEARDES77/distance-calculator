use std::cmp::min;
use std::env::args;
use std::char::MAX as CHAR_MAX;

enum Algorithms{
    Levenshtein,
    WagnerFischer,
    Bitap
    // damerau_levenshtein,       // todo
    // optimal_string_alignment, // todo
    // hamming                  // todo
}

fn main() {
    let args : Vec<String> = args().collect();
    if args.len() < 3 {
        println!("Usage: {} <word_1> <word_2>", args.get(0).unwrap())
    }else {
        let mut algorithm = "";
        let mut words: Vec<&str> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            if arg == "-h" || arg == "--help" {
                println!("Usage: {} <word_1> <word_2>", args.get(0).unwrap());
                println!("Options:");
                println!("\t-a, --algorithm\t\tAlgorithm to use. Default: WagnerFischer");
                println!("\t-h, --help\t\tPrints help information");
                println!("\t-v, --version\t\tPrints version information");
                println!("\nAlgorithms:");
                println!("\tlevenshtein, lev\t\tLevenshtein distance");
                println!("\twagnerfischer, wf\t\tWagner-Fischer distance");
                println!("\tbitap, bt\t\tBitap distance");
                return;
            }
            if arg == "-v" || arg == "--version" {
                println!("{} {}", args.get(0).unwrap(), env!("CARGO_PKG_VERSION"));
                return;
            }
            if arg == "-a" || arg == "--algorithm" {
                algorithm = args.get(i+1).unwrap().as_str();
            }

            if !arg.starts_with("-") && arg != algorithm && arg != args.get(0).unwrap(){
                words.push(arg.as_str());
            }

        }
        let algorithm: &str = args.get(1).unwrap();
        let a : Algorithms;
        match algorithm.to_lowercase().as_str() {
            "levenshtein" | "lev" => { a = Algorithms::Levenshtein },
            "wagnerfischer" | "wf" => { a = Algorithms::WagnerFischer },
            "bitap" | "bt" => { a = Algorithms::Bitap },
            _ => { a = Algorithms::WagnerFischer }
        }

        match a {
            Algorithms::Levenshtein => { println!("{}", lev(words.get(0).unwrap(), words.get(1).unwrap())) }
            Algorithms::WagnerFischer => { println!("{}", wagner_fischer(words.get(0).unwrap(), words.get(1).unwrap())) }
            Algorithms::Bitap => { println!("{}", bitap_bitwise_search(words.get(0).unwrap(), words.get(1).unwrap()).unwrap()) }
        }

    }
}

fn lev(a: &str , b : &str) -> usize{
    if a.len() == 0 { return b.len()}
    if b.len() == 0 { return a.len()}

    if a.starts_with(b.chars().next().unwrap()){
        return lev(&a[1..], &b[1..])
    }

    // Multithreading
    1 + min(min(
        lev(&a[1..], b),
        lev(a, &b[1..])),
        lev(&a[1..], &b[1..])
    )

}

fn wagner_fischer(s: &str , t : &str) -> usize{
    let m = s.len();
    let n = t.len();
    let mut d: Vec<Vec<usize>> = vec![vec![0; n+1]; m+1];

    for i in 1..=m {
        d[i][0] = i;
    }

    for j in 1..=n {
        d[0][j] = j;
    }

    for j in 1..=n {
        for i in 1..=m {
            let substitution_cost = if s.chars().nth(i-1) == t.chars().nth(j-1) { 0 } else { 1 };

            d[i][j] = min(min(
                d[i-1][j] + 1,  // deletion
                d[i][j-1] + 1), // insertion
                                    d[i-1][j-1] + substitution_cost // substitution
            );
        }
    }

    d[m][n]
}

fn bitap_bitwise_search(text: &str, pattern: &str) -> Option<usize> {
    let m = pattern.len();
    let mut r: u64 = !0;
    let mut pattern_mask: Vec<u64> = vec![!0; CHAR_MAX as usize + 1];

    if pattern.is_empty() {
        return Some(0);
    } else if m > 31 {
        return None;
    }

    // Initialize the bit array R
    for i in 0..CHAR_MAX as usize + 1 {
        pattern_mask[i] = !0;
    }

    for i in 0..m {
        pattern_mask[pattern.as_bytes()[i] as usize] &= !(1 << i);
    }

    for i in 0..text.len() {
        // Update the bit array
        r |= pattern_mask[text.as_bytes()[i] as usize];
        r <<= 1;

        if 0 == (r & (1 << m)) {
            return Some(i - m + 1);
        }
    }

    None
}
