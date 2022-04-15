use crossterm::terminal::enable_raw_mode;
use http_client::interface::{Interface, ListType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("Cannot run in raw mode");

    let list = ListType::API(vec![]);
    Interface::new(list).run();

    Ok(())
}
