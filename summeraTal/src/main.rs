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

    // fixa 
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    /* add code here ... */
    let n = lines
        .next().unwrap()
        .parse::<usize>().unwrap();
    
    let mut nums = lines
        .next().unwrap()
        .split_whitespace()
        .map(|_num| _num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    if
    //bestämmer antalet tal som läses in
    //let mut n: usize = lines.next().unwrap().parse().unwrap();

    //lagra n stycket tal 

    //bestäm om n är jämn eller udda

    //n är jämn och hitta de n/2 största talen och skriv ut

    //n är udda och hitta de (n+1)/2 största talen och skriv ut

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
