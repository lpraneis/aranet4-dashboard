use crate::handler::IoEvent;
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
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    is_loading: bool,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>, address: Option<String>) -> App {
        App {
            address,
            sensor: None,
            status: ConnectionStatus::Idle,
            read_time: None,
            cache: Some(SensorReadings::empty()),
            io_tx,
            is_loading: false,
        }
    }
    pub async fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            eprintln!("Error from dispatch: {}", e);
        }
    }
    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
    pub fn connected(&mut self) {
        eprintln!("Connected!");
        self.status = ConnectionStatus::Connected;
    }
    pub fn status(&self) -> ConnectionStatus {
        self.status
    }
    pub async fn connect(&mut self) {
        match SensorManager::init(self.address.clone()).await {
            Ok(sensor) => self.sensor = Some(sensor),
            Err(_) => panic!("Cannot connect"),
        }
    }
    pub async fn update_cache(&mut self) {
        eprintln!("Updating cache...");
        self.read_time = Some(Instant::now());
        self.cache = self
            .sensor
            .as_ref()
            .unwrap()
            .read_current_values()
            .await
            .ok();
    }
    pub fn get_cached_readings(&self) -> SensorReadings {
        if let Some(s) = &self.cache {
            return s.clone();
        }
        SensorReadings::empty()
    }
}
