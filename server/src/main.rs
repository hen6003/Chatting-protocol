mod channel;
mod commands;
mod users;

use commands::*;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:6078").await.unwrap();
    let (chan_tx, chan_rx) = mpsc::channel::<InternalCommand>(100);

    // Start channel manager thread
    tokio::spawn(channel::start_channel(chan_rx));

    println!("Started channel on {}", listener.local_addr().unwrap());

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();

        let chan_tx = chan_tx.clone();

        tokio::spawn(async move {
            process(socket, chan_tx).await;
        });
    }
}

async fn process(socket: TcpStream, chan_tx: mpsc::Sender<InternalCommand>) {
    let (reader, writer) = socket.into_split();
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    let (resp_tx, resp_rx) = oneshot::channel();

    chan_tx
        .send(InternalCommand::Connect {
            name: writer.peer_addr().unwrap().to_string(),
            writer,
            response: resp_tx,
        })
        .await
        .unwrap();

    let id = resp_rx.await.unwrap();

    while let Ok(Some(line)) = lines.next_line().await {
        let command = str::parse::<UserCommand>(&line);

        if let Ok(command) = command {
            chan_tx
                .send(InternalCommand::UserCommand { id, command })
                .await
                .unwrap();
        } else {
            println!("{:?}", command);
        }
    }

    chan_tx
        .send(InternalCommand::Disconnect { id })
        .await
        .unwrap();
}
