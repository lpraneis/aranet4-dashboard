use aranet4::{Sensor, SensorManager};
#[derive(std::cmp::PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ConnectionStatus {
    Idle,
    Connected,
    Connecting,
    ConnectionFailed,
}
pub struct App {
    address: Option<String>,
    sensor: Option<Sensor>,
    status: ConnectionStatus,
}

impl App {
    pub fn new(address: Option<String>) -> App {
        App {
            address,
            sensor: None,
            status: ConnectionStatus::Idle,
        }
    }
    pub async fn connect(&mut self) {
        self.sensor = SensorManager::init(self.address.take()).await;
    }
    pub fn on_tick(&mut self) {}
    pub async fn init(&mut self) {
        self.connect().await;
    }
    pub fn status(&self) -> ConnectionStatus {
        self.status
    }
}
