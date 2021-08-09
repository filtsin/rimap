//!
use crate::error::Result;
use crate::tag::{Tag, TagGenerator};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use log::trace;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::{
    net::{TcpStream, ToSocketAddrs},
    task::JoinHandle,
};
use tokio_util::codec::{Decoder, Framed, LinesCodec};

/// An async tcp stream.
/// The `ImapConnection` serves to register a request to the server
/// and receive a response.
///
/// All requests for the server saved as ([Tag](Tag) - Channel) match.
/// If the tagged response was received, we'll get Channel by the `Tag`
/// and send parsed response into it.
///
/// We can wait multiple responses at one time in multiple threads
/// because we can analyze received tag and find the corresponding Channel.
pub(crate) struct ImapConnection {
    sink: SplitSink<Framed<TcpStream, LinesCodec>, String>,
    subscriptions: Arc<HashMap<Tag, Sender<String>>>,
    generator: TagGenerator,
    // Cancel background listener future
    cancel: JoinHandle<()>,
}

impl ImapConnection {
    async fn connect<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let connection = TcpStream::connect(addr).await?;

        let frame = LinesCodec::default().framed(connection);

        let (sink, mut stream) = frame.split();

        let subscriptions = Arc::new(HashMap::new());

        let future = tokio::spawn(async move {
            loop {
                let buf = stream.next().await.unwrap();
                trace!("S: {:?}", buf);

                // Get in subs sender half by tag
            }
        });

        Ok(Self {
            sink,
            subscriptions,
            generator: TagGenerator::default(),
            cancel: future,
        })
    }

    async fn send(&mut self, data: String) {
        // Save subscription by tag to subscriptions map
        self.sink.send(data).await.unwrap()
    }
}
