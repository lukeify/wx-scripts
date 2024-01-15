use crate::sensors::rain_radar::{Location, RainRadarArgs, Range};
use crate::sensors::SensorTrait;
use chrono::{FixedOffset, SecondsFormat, Timelike, Utc};
use std::io::Write;
use std::path::{Path, PathBuf};

// TODO: Understand lifetimes
pub struct RainRadarSensor<'a> {
    pub(crate) args: &'a RainRadarArgs,
}

// TODO: Understand lifetime references
impl SensorTrait for RainRadarSensor<'_> {
    fn monitor(&self) {
        match fetch_rain_radar_image(self.args) {
            Ok(()) => println!("Ok!"),
            Err(_) => println!("Error!"),
        }
    }
}

#[tokio::main]
async fn fetch_rain_radar_image(
    args: &RainRadarArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = "2024-01-03T13:07:00+13:00";

    create_directory_for_image(args.location, args.range, timestamp);

    let response = reqwest::get(rain_radar_url(args.location, args.range, timestamp))
        .await?
        .bytes()
        .await?;

    let mut path = directory_for_image(args.location, args.range, timestamp)
        .join(filename_for_image(timestamp));
    path.set_extension("gif");

    let mut img_file = std::fs::File::create(path)?;
    img_file.write_all(&response)?;

    println!("{}", current_metservice_compatible_timestamp());
    Ok(())
}

fn rain_radar_url(location: Location, range: Range, timestamp: &str) -> String {
    let location = location.to_string();
    let range = range.to_string();

    [
        "https://www.metservice.com/publicData/rainRadar/image",
        &location,
        &range,
        timestamp,
    ]
    .join("/")
}

fn date_from_timestamp(timestamp: &str) -> String {
    String::from(timestamp.split('T').next().unwrap())
}

// fn metservice_compatible_timestamp(minutes_before_now: u8) -> DateTime<FixedOffset> {}

fn current_metservice_compatible_timestamp() -> String {
    Utc::now()
        .with_timezone(&FixedOffset::east_opt(13 * 3600).unwrap())
        .with_second(0)
        .unwrap()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

// fs handling

fn filename_for_image(timestamp: &str) -> String {
    timestamp.replace(':', "")
}

fn directory_for_image(location: Location, range: Range, timestamp: &str) -> PathBuf {
    Path::new("images")
        .join(location.to_string())
        .join(range.to_string())
        .join(date_from_timestamp(timestamp))
}

fn create_directory_for_image(location: Location, range: Range, timestamp: &str) {
    let dir_path = directory_for_image(location, range, timestamp);

    match std::fs::create_dir_all(&dir_path) {
        Ok(()) => println!("Directories created! `{}`", dir_path.display()),
        Err(err) => println!("Err creating directories! {err}"),
    }
}

// fn most_recent_image() -> DateTime<FixedOffset> {}
