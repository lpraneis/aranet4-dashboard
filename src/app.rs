use aranet4::{Sensor, SensorManager, SensorReadings};
use std::fmt;
use std::time::{Duration, Instant};
#[derive(std::cmp::PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ConnectionStatus {
    /// The connection is idle, e.g. it hasn't been started
    Idle,
    /// We are connected to the sensor
    Connected,
    /// We are connecting to the sensor
    Connecting,
    /// The connection to the sensor failed
    ConnectionFailed,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionStatus::Idle => write!(f, "Waiting to connect..."),
            ConnectionStatus::ConnectionFailed => write!(f, "Connection Failed"),
            ConnectionStatus::Connecting => write!(f, "Connecting..."),
            ConnectionStatus::Connected => write!(f, "Connected to Sensor"),
        }
    }
}
pub struct App {
    address: Option<String>,
    sensor: Option<Sensor>,
    status: ConnectionStatus,
    read_time: Option<Instant>,
    cache: Option<SensorReadings>,
}

impl App {
    pub fn new(address: Option<String>) -> App {
        App {
            address,
            sensor: None,
            status: ConnectionStatus::Idle,
            read_time: None,
            cache: Some(SensorReadings::empty()),
        }
    }
    pub async fn connect(&mut self) {
        self.sensor = SensorManager::init(self.address.take()).await;
    }
    pub async fn on_tick(&mut self) {}
    pub async fn init(&mut self) {
        self.connect().await;
        match &self.sensor {
            Some(_) => self.status = ConnectionStatus::Connected,
            None => self.status = ConnectionStatus::ConnectionFailed,
        }
    }
    pub fn status(&self) -> ConnectionStatus {
        self.status
    }
    pub async fn update_cache(&mut self) {
        eprintln!("Updating cache...");
        self.read_time = Some(Instant::now());
        self.cache = self.sensor.as_ref().unwrap().read_current_values().await;
    }

    pub fn get_cached_status(&self) -> SensorReadings {
        if let Some(s) = &self.cache {
            return s.clone();
        }
        SensorReadings::empty()
    }
}
