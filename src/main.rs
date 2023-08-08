use rsf::{create_wallet_get_balance, sort_balance};
use std::fs::OpenOptions;
// use std::io::Write;

fn main() {
    // let mut phrases: Vec<String> = Vec::new();

    let mut all_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/all.txt")
        .expect(&format!("Could not open file `{}`", "all"));

    let mut above_10000_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/above_10000.txt")
        .expect(&format!("Could not open file `{}`", "above_10000.txt"));

    let mut above_1000_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/above_1000.txt")
        .expect(&format!("Could not open file `{}`", "above_1000.txt"));

    let mut above_100_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/above_100.txt")
        .expect(&format!("Could not open file `{}`", "above_100.txt"));

    let mut above_0_to_100_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/above_0_to_100.txt")
        .expect(&format!("Could not open file `{}`", "above_0_to_100.txt"));

    let mut equals_0_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/equals_0.txt")
        .expect(&format!("Could not open file `{}`", "equals_0.txt"));

    let mut below_0_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("balance_results/below_0.txt")
        .expect(&format!("Could not open file `{}`", "below_0.txt"));

    match std::env::args().nth(1) {
        Some(times) => match &times.parse::<u64>() {
            Ok(t) => {
                for _ in 1..=*t {
                    sort_balance(
                        create_wallet_get_balance().unwrap(),
                        &mut all_file,
                        &mut above_10000_file,
                        &mut above_1000_file,
                        &mut above_100_file,
                        &mut above_0_to_100_file,
                        &mut equals_0_file,
                        &mut below_0_file,
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
