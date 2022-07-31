use crate::app::App;
use log::{error, info};
use std::{sync::Arc, time::Duration};

#[derive(Debug)]
// TODO: Add additional IO Events
pub(crate) enum IoEvent {
    Connect,
}

pub(crate) async fn run_handler(
    app: Arc<tokio::sync::Mutex<App>>,
    mut io_rx: tokio::sync::mpsc::Receiver<IoEvent>,
) {
    let handler = Handler::new(app);
    let mut update_interval = tokio::time::interval(Duration::from_secs(120));
    update_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        tokio::select! {
            Some(io_event) = io_rx.recv() => {
                handler.handle_io_event(io_event).await;
            }
            _ = update_interval.tick() => {
                let _ = handler.update().await;
            }
        }
    }
}

pub(crate) struct Handler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl Handler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    pub(crate) async fn handle_io_event(&self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Connect => self.connect().await,
        };
        if let Err(err) = result {
            error!("Error: {:?}", err);
        }
        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn connect(&self) -> anyhow::Result<()> {
        info!("Connecting...");
        let mut app = self.app.lock().await;
        app.connect().await?;
        app.connected();
        info!("Updating cache...");
        app.update_cache().await
    }
    pub(crate) async fn update(&self) -> anyhow::Result<()> {
        info!("Updating cache...");
        let mut app = self.app.lock().await;
        app.update_cache().await
    }
}
