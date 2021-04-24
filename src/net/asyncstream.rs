//! Wrapper for tokio Framed

use crate::error::Result;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio_util::codec::{Framed, LinesCodec};

#[derive(Debug)]
pub(crate) struct AsyncStream {
    frame: Framed<TcpStream, LinesCodec>,
}

impl AsyncStream {
    pub(crate) async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        let frame = Framed::new(stream, LinesCodec::new());
        Ok(Self { frame })
    }

    // Example
    pub(crate) async fn send_hello(self: &mut Self) -> Result<()> {
        let mut stream = self.frame.get_mut();
        stream.try_write(b"Hello")?;
        Ok(())
    }
}
