use rsf::{get_balance_from_private_key, get_private_key_from_phrase};
use std::fs::OpenOptions;

fn main() {
    let phrases: Vec<String> = Vec::new();

    let mut gt_than_10000_file = fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_name)
    .expect("Could not open file `{}`", gt_than_10000_file);

    let mut gt_than_1000_file = fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_name)
    .expect("Could not open file `{}`", gt_than_1000_file);

    let mut gt_than_100_file = fs::OpenOptions::new()
    .append(true)
    .create(true)
    .open(file_name)
    .expect("Could not open file `{}`", gt_than_100_file);

    for phrase in phrases {
        let amount = get_balance_from_private_key(get_private_key_from_phrase(phrase));
        match x {
            x if x > 10000 => writeln!(gt_10000, "{} : {}", phrase, amount),
            x if x > 1000 => writeln!(gt_1000, "{} : {}", phrase, amount),
            x if x > 100 => writeln!(gt_1000, "{} : {}", phrase, amount),
            _ => {},
        }
    }
}
