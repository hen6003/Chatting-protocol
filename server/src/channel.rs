use crate::{commands::*, users::*};

use tokio::sync::mpsc;

pub async fn start_channel(mut rx: mpsc::Receiver<InternalCommand>) {
    let mut users = Users::new();

    while let Some(res) = rx.recv().await {
        match res {
            InternalCommand::Connect {
                name,
                writer,
                response,
            } => {
                println!("User connected as {:?}", name);

                users.send_to_all(format!("c {}\n", name).as_bytes());

                let id = users.add(name, writer);
                response.send(id).unwrap();
            }

            InternalCommand::Disconnect { id } => {
                let name = users.get_name(id).unwrap().to_owned();

                println!("User {:?} disconnected", name);

                users.remove(id);
                users.send_to_all(format!("d {}\n", name).as_bytes());
            }

            InternalCommand::UserCommand { id, command } => {
                let name = users.get_name(id).unwrap().to_owned();

                match command {
                    UserCommand::Message(message) => {
                        if message.len() > 0 {
                            println!("User {:?} sent {:?}", name, message);

                            users.send_to_all(format!("m {} {}\n", name, message).as_bytes());
                        }
                    }

                    UserCommand::Name(newname) => {
                        if !newname.contains(' ') {
                            println!("User {:?} changed name to {:?}", name, newname);

                            users.send_to_all(format!("r {} {}\n", name, newname).as_bytes());
                            users.set_name(id, newname).unwrap();
                        }
                    }
                }
            }
        }
    }
}
