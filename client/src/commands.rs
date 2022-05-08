use std::{fmt, str::FromStr};

pub enum Commands {
    Message { sender: String, message: String },
}

impl FromStr for Commands {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s.split_once(' ') {
            Some((a, b)) => match a {
                "m" | "msg" => {
                    let (a, b) = b.split_once(' ').ok_or(())?;

                    Ok(Self::Message {
                        sender: a.to_string(),
                        message: b.to_string(),
                    })
                }

                _ => Err(()),
            },

            None => Err(()),
        }
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message { sender, message } => write!(f, "{}: {}", sender, message),
        }
    }
}
