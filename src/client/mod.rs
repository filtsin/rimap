//! Client api

use crate::error::Result;
use crate::net::asyncstream::AsyncStream;
use tokio::net::ToSocketAddrs;

#[derive(Debug)]
pub struct Client {
    stream: AsyncStream,
}

impl Client {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let stream = AsyncStream::connect(addr).await?;
        Ok(Self { stream })
    }
}
