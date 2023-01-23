use std::error::Error;
use serde::Deserialize;
use csv;

#[derive(Debug, Deserialize)]
struct TestKey {
    pk: String,
    pubk: String,
    eth: String,
    rev: String,
}

fn read_test_keys() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: TestKey = result?;
        println!("{:?}", record);
    }
    Ok(())
}
