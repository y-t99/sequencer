use std::time::{SystemTime, UNIX_EPOCH};

const ALPHABET: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn next() -> String {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("system time before Unix epoch");
    let seconds_since_epoch = now.as_secs();
    let years_since_epoch = seconds_since_epoch / (365 * 24 * 60 * 60) + 1970;
    years_since_epoch.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_sequence() {
        let sequence = next();
        assert_eq!(sequence, "2023");
    }
}
