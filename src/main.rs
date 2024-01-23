use std::env::args;

enum Algorithms{
    Levenshtein,
    WagnerFischer
    // damerau_levenshtein,       // todo
    // optimal_string_alignment, // todo
    // hamming                  // todo
}

fn main() {
    let args : Vec<String> = args().collect();
    if args.len() != 4 {
        println!("Usage: {} <algorithm> <word_1> <word_2>", args.get(0).unwrap())
    }else {
        let algorithm: &str = args.get(1).unwrap();
        let a : Algorithms;
        match algorithm {
            "Levenshtein" | "lev" => { a = Algorithms::Levenshtein },
            "WagnerFischer" | "wf" => { a = Algorithms::WagnerFischer },
            _ => { a = Algorithms::WagnerFischer }
        }

        match a {
            Algorithms::Levenshtein => { println!("{}", lev(args.get(2).unwrap(), args.get(3).unwrap())) }
            Algorithms::WagnerFischer => { println!("{}", wagner_fischer(args.get(2).unwrap(), args.get(3).unwrap())) }
        }

        println!("{}", lev(args.get(1).unwrap(), args.get(2).unwrap()))
    }
}

fn lev(a: &str , b : &str) -> usize{
    if a.len() == 0 { return b.len()}
    if b.len() == 0 { return a.len()}

    if a.starts_with(b.chars().next().unwrap()){
        return lev(&a[1..], &b[1..])
    }

    // Multithreading
    1 + min(
        lev(&a[1..], b),
        lev(a, &b[1..]),
        lev(&a[1..], &b[1..])
    )

}

fn min(a: usize, b: usize, c : usize) -> usize {
    return if a < b {
        if a < c {
            a
        } else {
            c
        }
    } else {
        if b < c {
            b
        } else {
            c
        }
    }
}

fn wagner_fischer(a: &str, b: &str) -> usize{
    let m = a.len();
    let n = b.len();
    let mut d: Vec<Vec<usize>> = vec![vec![0; n+1]; m+1];

    for i in 1..=m {
        d[i][0] = i;
    }

    for j in 1..=n {
        d[0][j] = j;
    }

    for j in 1..=n {
        for i in 1..=m {
            let substitution_cost = if a.chars().nth(i-1) == b.chars().nth(j-1) { 0 } else { 1 };

            d[i][j] = min(
                d[i-1][j] + 1,  // deletion
                d[i][j-1] + 1, // insertion
                d[i-1][j-1] + substitution_cost // substitution
            );
        }
    }

    d[m][n]
}