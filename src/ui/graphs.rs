use aranet4::history::readings::HistoryReadings;
use chrono::Duration;
use tui::{
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
};

pub(crate) struct DataGrapher<'a> {
    pub co2_chart: Chart<'a>,
    pub humidity_chart: Chart<'a>,
    pub pressure_chart: Chart<'a>,
    pub temperature_chart: Chart<'a>,
}

pub(crate) struct WrappedData {
    pub temperature: Vec<(f64, f64)>,
    pub humidity: Vec<(f64, f64)>,
    pub co2: Vec<(f64, f64)>,
    pub pressure: Vec<(f64, f64)>,
}

fn make_timescale<T>(data: &[T]) -> Vec<(f64, f64)>
where
    T: Copy + Into<f64>,
{
    (0..)
        .zip(data.iter())
        .map(|(x, y)| (x as f64, (*y).into()))
        .collect()
}

impl<'a> DataGrapher<'a> {
    pub fn wrap_data(readings: HistoryReadings) -> WrappedData {
        WrappedData {
            temperature: make_timescale(&readings.temperature),
            humidity: make_timescale(&readings.humidity),
            co2: make_timescale(&readings.co2),
            pressure: make_timescale(&readings.pressure),
        }
    }
    pub fn new(bounds: [f64; 2], data: &'a WrappedData) -> Self {
        let co2_data = Dataset::default()
            .name("Co2")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .graph_type(tui::widgets::GraphType::Line)
            .data(&data.co2);
        let humid_data = Dataset::default()
            .name("Humidity")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .graph_type(tui::widgets::GraphType::Line)
            .data(&data.humidity);
        let pressure_data = Dataset::default()
            .name("Pressure")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Green))
            .graph_type(tui::widgets::GraphType::Line)
            .data(&data.pressure);
        let temp_data = Dataset::default()
            .name("Temperature")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Blue))
            .graph_type(tui::widgets::GraphType::Line)
            .data(&data.temperature);

        let time_labels = calculate_time_labels();

        let time_axis = Axis::default()
            .title("Time")
            .style(Style::default())
            .bounds(bounds)
            .labels(time_labels);

        Self {
            co2_chart: make_co2_chart(co2_data).x_axis(time_axis.clone()),
            humidity_chart: make_humidity_chart(humid_data).x_axis(time_axis.clone()),
            pressure_chart: make_pressure_chart(pressure_data).x_axis(time_axis.clone()),
            temperature_chart: make_temperature_data(temp_data).x_axis(time_axis.clone()),
        }
    }
}
fn make_co2_chart(data: Dataset) -> Chart {
    let chart = Chart::new(vec![data])
        .block(Block::default().borders(Borders::all()))
        .y_axis(
            Axis::default()
                .title("CO2 ( ppm )")
                .style(Style::default())
                .bounds([0.0, 2000.0])
                .labels(vec![
                    Span::styled("0", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("500", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("1000", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("1500", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("2000", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    chart
}
fn make_humidity_chart(data: Dataset) -> Chart {
    let chart = Chart::new(vec![data])
        .block(Block::default().borders(Borders::all()))
        .y_axis(
            Axis::default()
                .title("Humidity %")
                .style(Style::default())
                .bounds([30.0, 80.0])
                .labels(vec![
                    Span::styled("30%", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("40%", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("50%", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("60%", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("70%", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("80%", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    chart
}
fn make_pressure_chart(data: Dataset) -> Chart {
    let chart = Chart::new(vec![data])
        .block(Block::default().borders(Borders::all()))
        .y_axis(
            Axis::default()
                .title("Pressure")
                .style(Style::default())
                .bounds([900.0, 1000.0])
                .labels(vec![
                    Span::styled("900", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("925", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("950", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("975", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("1000", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    chart
}
fn make_temperature_data(data: Dataset) -> Chart {
    let chart = Chart::new(vec![data])
        .block(Block::default().borders(Borders::all()))
        .y_axis(
            Axis::default()
                .title("Temperature")
                .style(Style::default())
                .bounds([50.0, 90.0])
                .labels(vec![
                    Span::styled("50F", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("60F", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("70F", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("80F", Style::default().add_modifier(Modifier::BOLD)),
                    Span::styled("90F", Style::default().add_modifier(Modifier::BOLD)),
                ]),
        );
    chart
}

fn calculate_time_labels() -> Vec<Span<'static>> {
    let now = chrono::Utc::now();
    let four = now
        .checked_sub_signed(Duration::hours(4))
        .map(|x| x.to_rfc2822())
        .unwrap_or_else(|| "4 hours ago".to_string());
    let eight = now
        .checked_sub_signed(Duration::hours(8))
        .map(|x| x.to_rfc2822())
        .unwrap_or_else(|| "8 hours ago".to_string());

    vec![
        Span::styled(eight, Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(four, Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("now", Style::default().add_modifier(Modifier::BOLD)),
    ]
}
