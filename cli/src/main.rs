#[allow(unused_imports)]
use cli_of_doom::learn;
use dotenv::dotenv;

// mod grep;

fn main() {
    dotenv().ok();

    learn::run();
    // grep::run();
}