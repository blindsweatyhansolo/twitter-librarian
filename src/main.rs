mod config;

fn main() {
    let twitter_config = config::Config::from_env().unwrap();
    // println!("{}", twitter_config.bearer_token);
}
