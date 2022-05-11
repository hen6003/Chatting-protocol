mod client;
mod commands;

use std::io::stdout;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:6078".parse().unwrap();
    let mut stdout = stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen)?;

    client::client(addr, "nobody".to_string()).await;

    execute!(stdout, LeaveAlternateScreen)?;

    disable_raw_mode()
}
