//fn main() {
 //   println!("Hello, world!");
//}

/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    //fixa input raderna till string-typ 
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap().to_string);
    /* add code here ... */

    //fixa till första raden dvs antalet
    let n = lines
        .next().unwrap()
        .parse::<u32>().unwrap();
    //fixa till andra raden som en vektor 
    let mut nums = lines
        .next().unwrap()
        .split_whitespace()
        .map(|_num| _num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    
    if n=
    //bestämmer antalet tal som läses in
    //let mut n: usize = lines.next().unwrap().parse().unwrap();

    //lagra n stycket tal 

    //bestäm om n är jämn eller udda

    //n är jämn och hitta de n/2 största talen och skriv ut

    //n är udda och hitta de (n+1)/2 största talen och skriv ut

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
