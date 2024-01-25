mod hamming_distance;

use std::cmp::min;
use std::env::args;
use std::char::MAX as CHAR_MAX;
use hamming_distance::hamming_distance;
use std::time::Instant;

#[derive(Debug)]
enum Algorithms{
    Levenshtein,
    WagnerFischer,
    Bitap,
    Hamming,
    OptimalStringAlignment
}

fn help(args : Vec<String>) {
    println!("Usage: {} <word_1> <word_2>", args.get(0).unwrap());
    println!("Options:");
    println!("\t-a, --algorithm\t\tAlgorithm to use. Default: WagnerFischer");
    println!("\t-h, --help\t\tPrints help information");
    println!("\t-v, --version\t\tPrints version information");
    println!("\nAlgorithms:");
    println!("\tlev\t->\tLevenshtein distance");
    println!("\twf\t->\tWagner-Fischer distance");
    println!("\tbt\t->\tBitap distance");
    println!("\thm\t->\tHamming distance");
    println!("\tosa\t->\tOptimal String Alignment distance");
}

fn main() {
    let args : Vec<String> = args().collect();
    let mut raw = false;
    if args.len() < 3 {
        help(args);
    }else {
        let mut algorithm = "";
        let mut words: Vec<&str> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            if arg == "-h" || arg == "--help" {
                help(args);
                return;
            }
            if arg == "-v" || arg == "--version" {
                println!("{} {}", args.get(0).unwrap(), env!("CARGO_PKG_VERSION"));
                return;
            }
            if arg == "-a" || arg == "--algorithm" {
                algorithm = args.get(i+1).unwrap().as_str();
            }

            if arg == "-r" {
                raw = true;
            }

            if !arg.starts_with("-") && arg != algorithm && arg != args.get(0).unwrap(){
                words.push(arg.as_str());
            }

        }
        let a : Algorithms;
        match algorithm.to_lowercase().as_str() {
            "levenshtein" | "lev" => { a = Algorithms::Levenshtein },
            "wagnerfischer" | "wf" => { a = Algorithms::WagnerFischer },
            "bitap" | "bt" => { a = Algorithms::Bitap },
            "hamming" | "hm" => { a = Algorithms::Hamming },
            "optimalstringalinment" | "osa" => { a = Algorithms::OptimalStringAlignment },
            _ => { a = Algorithms::WagnerFischer }
        }
        let start_time = Instant::now();
        println!("{}: {:?}", algorithm, a);
        match a {
            Algorithms::Levenshtein => { println!("{}", lev(words.get(0).unwrap(), words.get(1).unwrap())) }
            Algorithms::WagnerFischer => { println!("{}", wagner_fischer(words.get(0).unwrap(), words.get(1).unwrap())) }
            Algorithms::Bitap => { println!("{}", bitap_bitwise_search(words.get(0).unwrap(), words.get(1).unwrap()).unwrap()) }
            Algorithms::Hamming => { println!("{}", hamming_distance(words.get(0).unwrap(), words.get(1).unwrap())) }
            Algorithms::OptimalStringAlignment => { println!("{}", osa_distance(words.get(0).unwrap(), words.get(1).unwrap())) }
        }
        if !raw {
            let end_time = Instant::now();
            println!("Took {} microseconds", end_time.duration_since(start_time).as_micros());
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
fn osa_distance(a: &str, b: &str) -> usize {
    let mut d = vec![vec![0; b.len() + 1]; a.len() + 1];

    for i in 0..a.len() +1 {
        d[i][0] = i;
    }

    for j in 0..b.len() +1 {
        d[0][j] = j;
    }

    for i in 1..a.len() {
        for j in 1..b.len() {
            let cost = if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                0
            } else {
                1
            };
            d[i][j] = min(
                min(d[i - 1][j] + 1, d[i][j - 1] + 1),
                d[i - 1][j - 1] + cost,
            );

            if i > 1 && j > 1 && a.as_bytes()[i - 1] == b.as_bytes()[j - 1] && a.as_bytes()[i - 2] == b.as_bytes()[j] {
                d[i][j] = min(d[i][j], d[i - 2][j - 2] + 1);
            }
        }
    }

    d[a.len()][b.len()]
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
