
//ja jag var ju tvungen att lösa denna uppgift pågrund av dess namn hahaha
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

    // get alla input raderna 
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());

    /* add code here ... */
    
    //get första raden dvs antalet
    let n = lines
        .next().unwrap()
        .parse::<usize>().unwrap();
    /*fixa nåt idk*/
    
    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}