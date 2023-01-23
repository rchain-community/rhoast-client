use config as config_lib;

pub fn read_config(cfg: &str) -> config_lib::Config {
    config_lib::Config::builder()
        .add_source(config_lib::File::with_name(cfg))
        .add_source(config_lib::Environment::with_prefix("RUST_CLIENT"))
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_works() {
        let settings = read_config("examples/example-config.toml");
        println!("{:?}", settings);
    }
}
