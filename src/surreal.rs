use std::error::Error;
use crate::types::errors::*;
use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{ future, pin_mut, StreamExt };
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::protocol::Message};
use url::Url;


async fn tin() {
    let url = Url::parse(&*"").unwrap();
    let (stdin_tx, stdin_rx) = unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(url).await.expect("sfadsfsd");
    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_stdin(tx: UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}

#[allow(unused)]
pub struct Surreal<'a> {
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    url: Option<Url>,
    token: Option<&'a str>,
    // pinger: Option<Pinger>,
    //attempted: Option<futures::Future<Box<dyn ()>>>,

    auth_err: AuthenticationError,
    permission_err: PermissionError,
    record_err: RecordError,
    live_err: LiveError
}

impl<'a> Surreal<'a> {
    pub fn new(url: Option<Url>, token: Option<&'a str>) -> Self {        
        let surreal = Surreal { ws: None, url: url.clone(), token, auth_err: AuthenticationError, permission_err: PermissionError, record_err: RecordError, live_err: LiveError };
        
        surreal
    }

    pub async fn connect(&mut self, url: &Url) -> Result<(), Box<dyn Error>> {
        let (ws_stream, mut socket) = connect_async(url).await?;
        self.ws = Some(ws_stream);

        Ok(())
    }
}