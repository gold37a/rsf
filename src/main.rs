use rsf::{create_wallet_get_balance, sort_balance};
use std::fs::OpenOptions;
// use std::io::Write;

fn main() {
    // let mut phrases: Vec<String> = Vec::new();

    let mut above_10000_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("results/above_10000.txt")
        .expect(&format!("Could not open file `{}`", "above_10000"));

    let mut above_1000_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("results/above_1000.txt")
        .expect(&format!("Could not open file `{}`", "above_1000"));

    let mut above_100_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("results/above_100.txt")
        .expect(&format!("Could not open file `{}`", "above_100"));

    let mut below_100_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("results/below_100.txt")
        .expect(&format!("Could not open file `{}`", "below_100"));

    match std::env::args().nth(1) {
        Some(times) => match &times.parse::<u64>() {
            Ok(t) => {
                for _ in 1..=*t {
                    sort_balance(
                        create_wallet_get_balance().unwrap(),
                        &mut above_10000_file,
                        &mut above_1000_file,
                        &mut above_100_file,
                        &mut below_100_file,
                    )
                }
            }
            Err(e) => panic!("Error while parsing command arg: {}", e.to_string()),
        },
        None => panic!("Please provide the number of phrases to generate"),
    }

    // for phrase in phrases {
    //     match get_balance_from_phrase(&phrase) {
    //         Ok(balance) => match balance {
    //             balance if balance > 10000 => {
    //                 writeln!(&mut above_10000_file, "{} : {}", phrase, balance).expect(
    //                     &format!(
    //                         "Encountered error writing `{} : {}` to above_10000_file",
    //                         phrase, balance
    //                     ),
    //                 );
    //             }
    //             balance if balance > 1000 => {
    //                 writeln!(&mut above_1000_file, "{} : {}", phrase, balance).expect(
    //                     &format!(
    //                         "Encountered error writing `{} : {}` to above_1000_file",
    //                         phrase, balance
    //                     ),
    //                 );
    //             }
    //             balance if balance > 100 => {
    //                 writeln!(&mut above_100_file, "{} : {}", phrase, balance).expect(
    //                     &format!(
    //                         "Encountered error writing `{} : {}` to above_100_file",
    //                         phrase, balance
    //                     ),
    //                 );
    //             }
    //             _ => {
    //                 // println!("phra{}")
    //             }
    //         },
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             continue;
    //             // panic!("{}", e.to_string())
    //         }
    //     }
    // }
}
