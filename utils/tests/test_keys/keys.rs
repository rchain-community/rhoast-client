use csv;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct TestKey {
    pk: String,
    pubk: String,
    eth: String,
    rev: String,
}

fn read_test_keys() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("testing-keys.csv");
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: TestKey = result?;
        println!("{:?}", record);
        Ok(record)
    }
    Error()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_key_read() {
        let keys: TestKey = read_test_keys();
        assert!(keys.pk != "");
    }
}
