use crate::app::App;
use std::sync::Arc;
use tokio::sync::Mutex;
pub enum IoEvent {
    Connect,
    GetCurrentData,
}

pub struct Handler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl Handler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Handler {
        Handler { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        //TODO
        match io_event {
            IoEvent::Connect => {
                self.connect().await;
            }
            IoEvent::GetCurrentData => {
                self.get_current_data().await;
            }
        }
        let mut app = self.app.lock().await;
        app.loaded();
    }

    pub async fn connect(&self) {
        let mut app = self.app.lock().await;
        app.connect().await;
        app.connected();
        app.update_cache().await;
    }
    pub async fn get_current_data(&self) {}
}
