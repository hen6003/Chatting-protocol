use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum Commands {
    Message { sender: String, message: String },
    UserConnected { name: String },
    UserDisconnected { name: String },
    UserRenamed { oldname: String, newname: String },
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

                "c" | "connect" => Ok(Self::UserConnected {
                    name: b.to_string(),
                }),
                "d" | "disconnect" => Ok(Self::UserDisconnected {
                    name: b.to_string(),
                }),

                "r" | "rename" => {
                    let (a, b) = b.split_once(' ').ok_or(())?;

                    Ok(Self::UserRenamed {
                        oldname: a.to_string(),
                        newname: b.to_string(),
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
            Self::UserConnected { name } => write!(f, "! {} connected", name),
            Self::UserDisconnected { name } => write!(f, "! {} disconnected", name),
            Self::UserRenamed { oldname, newname } => {
                write!(f, "! {} changed names to {}", oldname, newname)
            }
        }
    }
}
