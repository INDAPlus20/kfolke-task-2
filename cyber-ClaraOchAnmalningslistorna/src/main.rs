
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
        .parse().unwrap();
    /*fixa nåt idk*/

    //en array att spara hela namnen
    let mut namnOchEfternamn: Vec<String> = Vec::new();
    
    //en array för att kunna spara alla UNIKA, hela namn
    let mut unikaNamnOchEfternamn: Vec<String> = Vec::new();

    //loopa igenom första input halvan och spara i arrayen
    for i in 0..n{
        let fornamn = lines
            .next().unwrap();
        namnOchEfternamn.push(fornamn)
    }

    for j in 0..n{
        let efternamn = lines
            .next().unwrap();
        let efterNamnMedSpace = " ".to_owned() + &efternamn;
        namnOchEfternamn[j].push_str(efterNamnMedSpace);
    }

    for k in 0..n{
        let namn = &namnOchEfternamn[k];

        if!unikaNamnOchEfternamn.contains(&namn){
            unikaNamnOchEfternamn.push(namn.clone());
        }
    }

    println!("{}", unikaNamnOchEfternamn.len());
}