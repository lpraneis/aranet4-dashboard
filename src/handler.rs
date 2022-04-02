use crate::app::App;
use log::{error, info};
use std::sync::Arc;
pub enum IoEvent {
    Connect,
    GetCurrentData,
}

pub struct Handler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl Handler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        //TODO
        let result = match io_event {
            IoEvent::Connect => self.connect().await,
            IoEvent::GetCurrentData => self.get_current_data().await,
        };
        if let Err(err) = result {
            error!("Error: {:?}", err);
        }
        let mut app = self.app.lock().await;
        app.loaded();
    }

    pub async fn connect(&self) -> Result<(), ()> {
        info!("Connecting...");
        let mut app = self.app.lock().await;
        app.connect().await;
        app.connected();
        info!("Updating cache...");
        app.update_cache().await;
        Ok(())
    }
    pub async fn get_current_data(&self) -> Result<(), ()> {
        Ok(())
    }
}
