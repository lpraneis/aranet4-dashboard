use crate::handler::IoEvent;
use aranet4::history::readings::HistoryReadings;
use aranet4::readings::SensorReadings;
use aranet4::sensor::{Sensor, SensorManager};
use log::error;
use std::fmt;
use std::time::Instant;

#[derive(std::cmp::PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ConnectionStatus {
    /// The connection is idle, e.g. it hasn't been started
    Idle,
    /// We are connected to the sensor
    Connected,
}

impl fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectionStatus::Idle => write!(f, "Waiting to connect..."),
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
    cached_history: Option<HistoryReadings>,
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    is_loading: bool,
}

impl App {
    pub(crate) fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>, address: Option<String>) -> App {
        App {
            address,
            sensor: None,
            status: ConnectionStatus::Idle,
            read_time: None,
            cache: None,
            cached_history: None,
            io_tx,
            is_loading: false,
        }
    }
    pub(crate) async fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action).await {
            self.is_loading = false;
            error!("Dispatch error! : {:?}", e);
        }
    }
    pub(crate) fn loaded(&mut self) {
        self.is_loading = false;
    }
    pub(crate) fn connected(&mut self) {
        self.status = ConnectionStatus::Connected;
    }
    pub async fn connect(&mut self) -> anyhow::Result<()> {
        let sensor = SensorManager::init(self.address.clone()).await?;
        self.sensor = Some(sensor);
        Ok(())
    }
    pub async fn update_cache(&mut self) -> anyhow::Result<()> {
        self.read_time = Some(Instant::now());
        if let Some(sensor) = self.sensor.as_ref() {
            self.cache = Some(sensor.read_current_values().await?);
        }
        Ok(())
    }
    pub fn get_cached_readings(&self) -> SensorReadings {
        self.cache.as_ref().cloned().unwrap_or_default()
    }
    pub async fn update_history(&mut self) -> anyhow::Result<()> {
        if let Some(sensor) = self.sensor.as_ref() {
            self.cached_history = sensor.get_historical_data().await.ok();
        }
        Ok(())
    }
    pub fn get_cached_history(&self) -> Option<HistoryReadings> {
        self.cached_history.as_ref().cloned()
    }
}
