use crate::{error::Error, APP_CONFIG};
use std::{
    fmt::{self, Debug, Display},
    time::Duration,
};
use tokio::{sync::mpsc, task::JoinHandle, time::error::Elapsed};

#[allow(dead_code)]
pub struct ChannelReceiver {
    receiver: mpsc::Receiver<TxMessage>,
    next_id: u32,
}

#[derive(Debug)]
pub enum TxMessage {
    RunTask { timestamp: String },
}

impl fmt::Display for TxMessage {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl ChannelReceiver {
    pub fn new(receiver: mpsc::Receiver<TxMessage>) -> Self {
        ChannelReceiver {
            receiver,
            next_id: 0,
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        tracing::info!("\n\r --> run(): Blocking for next message.");

        while let Some(msg) = self.receiver.recv().await {
            match msg {
                TxMessage::RunTask { timestamp: _ } => {
                    unimplemented!()
                }
            };
        }

        Ok(())
    }
}
