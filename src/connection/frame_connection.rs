#[allow(unused)]
use bytes::BytesMut;
use tokio::io::{BufWriter};
use tokio::net::{TcpStream};

pub struct FrameConnection {
    // write buffering
    stream: BufWriter<TcpStream>,

    // read buffering
    buffer: BytesMut
}

impl FrameConnection {
    pub fn new(socket: TcpStream) -> Self {
        FrameConnection { 
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(10*1024) 
        }
    }
}