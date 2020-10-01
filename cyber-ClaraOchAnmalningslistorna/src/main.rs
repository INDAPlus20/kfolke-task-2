
//ja jag var ju tvungen att lösa denna uppgift pågrund av dess namn hahaha

/*Är medveten om att jag har lite dålig namngivning, dels att jag har lite 
ospecifika namn samt att de är lite blandad svengelska vilket jag jobbar på*/
use std::io;
use std::io::prelude::*;


fn main() {
    // get standard input stream
    let input = io::stdin();

    // get alla input raderna 
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());

    //get första raden dvs antalet

    //get första raden dvs antalet
    let n = lines
        .next().unwrap()
        .parse().unwrap();
    /*fixa nåt idk*/

    //en array att spara hela namnen
    let mut namn_och_efternamn: Vec<String> = Vec::new();
    
    //en array för att kunna spara alla UNIKA, hela namn
    let mut unika_namn_och_efternamn: Vec<String> = Vec::new();

    //spara alla förnamn
    for _i in 0..n{
        let fornamn = lines
            .next().unwrap();
            namn_och_efternamn.push(fornamn)
    }
    //spara alla efternamn tillsammans med ett förnamn
    for j in 0..n{
        let efternamn = lines
            .next().unwrap();
        let efternamn_med_space = " ".to_owned() + &efternamn;
        namn_och_efternamn[j].push_str(&efternamn_med_space);
    }
    //checka om alla namn och efternamn är unika
    for k in 0..n{
        let tmp_namn_och_efternamn = &namn_och_efternamn[k];

        if!unika_namn_och_efternamn.contains(&tmp_namn_och_efternamn){
            //namnet och efternamnet är unikt och läggs därför till i unik-array
            unika_namn_och_efternamn.push(tmp_namn_och_efternamn.clone());
        }
    }
    //antalet unika namn presenteras
    println!("{}", unika_namn_och_efternamn.len());
}