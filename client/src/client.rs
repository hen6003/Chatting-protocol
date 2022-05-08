use crate::commands::Commands;

use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use std::{
    collections::VecDeque,
    io::{stdout, Write},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crossterm::{
    cursor,
    event::{Event, EventStream, KeyCode},
    execute, queue,
    terminal::{self, ClearType},
};

pub async fn client() {
    // Connect to server
    let stream = TcpStream::connect("127.0.0.1:6379").await.unwrap();
    let (reader, mut writer) = stream.into_split();
    let reader = BufReader::new(reader);

    // Start thread handling user input
    tokio::spawn(async move {
        let stdout = stdout();
        let mut command = String::new();
        let mut eventreader = EventStream::new();

        loop {
            let mut delay = Delay::new(Duration::from_millis(1_000)).fuse();
            let mut event = eventreader.next().fuse();

            {
                let mut stdout = stdout.lock();
                execute!(stdout, cursor::MoveToRow(terminal::size().unwrap().1)).unwrap();
            }

            #[rustfmt::skip]
            select! {
		_ = delay => (),
		maybe_event = event => {
                    match maybe_event {
			Some(Ok(event)) => match event {
                            Event::Key(key) => match key.code {
				KeyCode::Char(c) => {
				    command.push(c);

				    let mut stdout = stdout.lock();
				    write!(stdout, "{}", c).unwrap();
				    stdout.flush().unwrap();
				},

				KeyCode::Backspace => {
				    command.pop();

				    let mut stdout = stdout.lock();
				    execute!(stdout, cursor::MoveLeft(1), terminal::Clear(ClearType::UntilNewLine)).unwrap();

				}

				KeyCode::Enter => {
				    let mut chars = command.chars();
				    let c = if chars.next() == Some('/') {
					chars.collect::<String>() + "\n"
				    } else {
					format!("m {}\n", command)
				    };

				    writer.write_all(c.as_bytes()).await.unwrap();
				    writer.flush().await.unwrap();

				    command.clear();

				    let mut stdout = stdout.lock();
				    execute!(stdout, cursor::MoveToColumn(1), terminal::Clear(ClearType::CurrentLine)).unwrap();
				}

				KeyCode::Esc => return,
				_ => (),
			    }

			    _ => (),
			}
			Some(Err(e)) => println!("Error: {:?}\r", e),
			None => break,
                    }
		}
            };
        }
    });

    // Handle TcpStream
    let mut messages = VecDeque::new();

    let stdout = stdout();
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let message = str::parse::<Commands>(&line).unwrap();
        messages.push_back(message);

        let term_rows = terminal::size().unwrap().1 as usize - 1;
        if messages.len() > term_rows {
            messages.drain(0..messages.len() - term_rows);
        }

        let mut stdout = stdout.lock();
        queue!(stdout, cursor::SavePosition, cursor::MoveTo(0, 0)).unwrap();

        for m in messages.iter() {
            queue!(stdout, terminal::Clear(ClearType::CurrentLine)).unwrap();
            write!(stdout, "{}\r\n", m).unwrap();
        }

        execute!(stdout, cursor::RestorePosition).unwrap();
    }
}
