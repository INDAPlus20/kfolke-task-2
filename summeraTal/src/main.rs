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

    //get första raden dvs antalet
    let n = lines
        .next().unwrap()
        .parse::<u32>().unwrap();

    //spara alla inmatade tal
    let mut nums = lines
        .next().unwrap()
        .split_whitespace()
        .map(|component|component.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
        nums.sort(); //sortera nnummer från minst till störst
    
    //summan av de största talen 
    let mut sum: u32 =0;

    //Kolla om antalet tal är jämnt eller udda
    if(n%2)== 0 {
        //antalet tal är jämnt

        //addera alla tal störrelika med medianen till summan
        for i in (n/2)..n{
            sum = sum + sum[i as usize];
        }
    }
    else{
        //antalet tal är jämnt
        for i in 
    }

    //bestämmer antalet tal som läses in
    //let mut n: usize = lines.next().unwrap().parse().unwrap();

    //lagra n stycket tal 

    //bestäm om n är jämn eller udda

    //n är jämn och hitta de n/2 största talen och skriv ut

    //n är udda och hitta de (n+1)/2 största talen och skriv ut

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}
