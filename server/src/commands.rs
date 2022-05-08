// Possible commands to send to master thread
use std::str::FromStr;
use tokio::{net::tcp::OwnedWriteHalf, sync::oneshot};

#[derive(Debug)]
pub enum InternalCommand {
    Connect {
        name: String,
        writer: OwnedWriteHalf,
        response: oneshot::Sender<usize>,
    },
    Disconnect {
        id: usize,
    },
    UserCommand {
        id: usize,
        command: UserCommand,
    },
}

#[derive(Debug)]
pub enum UserCommand {
    Message(String),
    Name(String),
}

impl FromStr for UserCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((a, b)) => match a {
                "m" | "msg" => Ok(Self::Message(b.to_string())),
                "n" | "name" => Ok(Self::Name(b.to_string())),

                _ => Err(()),
            },

            None => Err(()),
        }
    }
}
