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
                println!("User connected at {}", name);

                let id = users.add(User::new(name, writer));
                response.send(id).unwrap();
            }

            InternalCommand::Disconnect { id } => {
                let name = &users.get(id).get_name();

                println!("User {:?} disconnected", name);

                users.remove(id);
            }

            InternalCommand::UserCommand { id, command } => {
                let user = users.get(id);
                let name = &user.get_name();

                match command {
                    UserCommand::Message(message) => {
                        println!("User {:?} sent {:?}", name, message);

                        let message = format!("m {} {}\n", name, message);

                        users.send_to_all(message.as_bytes());
                    }

                    UserCommand::Name(newname) => {
                        if !newname.contains(' ') {
                            println!("User {:?} changed name to {:?}", name, newname);

                            user.set_name(newname);
                        }
                    }
                }
            }
        }
    }
}
