use rsf::get_balance_from_phrase;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let phrases: Vec<String> = Vec::new();

    let mut greater_than_10000_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("greater_than_10000")
        .expect(&format!("Could not open file `{}`", "greater_than_10000"));

    let mut greater_than_1000_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("greater_than_1000")
        .expect(&format!("Could not open file `{}`", "greater_than_1000"));

    let mut greater_than_100_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("greater_than_100")
        .expect(&format!("Could not open file `{}`", "greater_than_100"));

    for phrase in phrases {
        match get_balance_from_phrase(&phrase) {
            Ok(balance) => match balance {
                balance if balance > 10000 => {
                    writeln!(&mut greater_than_10000_file, "{} : {}", phrase, balance).expect(
                        &format!(
                            "Encountered error writing `{} : {}` to greater_than_10000_file",
                            phrase, balance
                        ),
                    );
                }
                balance if balance > 1000 => {
                    writeln!(&mut greater_than_1000_file, "{} : {}", phrase, balance).expect(
                        &format!(
                            "Encountered error writing `{} : {}` to greater_than_1000_file",
                            phrase, balance
                        ),
                    );
                }
                balance if balance > 100 => {
                    writeln!(&mut greater_than_100_file, "{} : {}", phrase, balance).expect(
                        &format!(
                            "Encountered error writing `{} : {}` to greater_than_100_file",
                            phrase, balance
                        ),
                    );
                }
                _ => {}
            },
            Err(e) => {
                panic!("{}", e.to_string())
            }
        }
    }
}
