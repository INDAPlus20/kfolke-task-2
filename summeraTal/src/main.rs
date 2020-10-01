//Klara Folkes solution till kattis summera tal problem
use std::io;
use std::io::prelude::*;


fn main() {
    // get standard input stream
    let input = io::stdin();

    //alla input raderna
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
        
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
        nums.sort(); //sortera alla tal från minst till störst
    
    //för att kunna spara summan av de största talen 
    let mut summa: u32 =0;

    //Kolla om antalet tal är jämnt eller udda
    if(n%2)== 0 {
        //antalet tal är jämnt

        //addera alla tal störrelikamed medianen till summan
        for i in (n/2)..n{
            summa = summa + nums[i as usize];
        }
    }
    else{
        //antalet tal är jämnt
        //addera alla tal som är större lika med tal nummer (n+1)/2
        for i in ((n-1)/2)..n{
            summa = summa + nums[i as usize]
        }
    }

    //output av den sökta summan
    println!("{}", summa);
}
