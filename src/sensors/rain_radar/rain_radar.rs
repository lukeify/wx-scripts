use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::{DateTime, FixedOffset, SecondsFormat, Timelike, Utc};
use crate::sensors::rain_radar::range::Range;
use crate::sensors::sensor_trait::SensorTrait;

pub struct RainRadar {}

impl SensorTrait for RainRadar {
    fn monitor() {
        rr_monitor();
    }
}

#[tokio::main]
async fn rr_monitor() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = "2023-12-24T22:37:00+13:00";

    create_directory_for_image(Range::Close, timestamp);

    let response = reqwest::get(rain_radar_url(Range::Close, timestamp))
        .await?
        .bytes()
        .await?;

    let mut path = directory_for_image(Range::Close, timestamp).join(filename_for_image(timestamp));
    path.set_extension("gif");

    let mut img_file = std::fs::File::create(path)?;
    img_file.write_all(&response)?;

    let timestamp = current_metservice_compatible_timestamp();

    println!("{}", timestamp);
    Ok(())
}

fn rain_radar_url(rain_radar_range: Range, timestamp: &str) -> String {
    String::from(
        "https://www.metservice.com/publicData/rainRadar/image/Otago/".to_owned() +
            rain_radar_range.to_string().as_str() +
            "/" +
            timestamp
    )
}

fn date_from_timestamp(timestamp: &str) -> String {
    String::from(timestamp.split('T').next().unwrap())
}

fn current_metservice_compatible_timestamp() -> String {
    let converted_now: DateTime<FixedOffset> = Utc::now().with_timezone(
        &FixedOffset::east_opt(13 * 3600).unwrap()
    );

    String::from(
        converted_now
            .with_second(0)
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

// fs handling

fn filename_for_image(timestamp: &str) -> String {
    timestamp.replace(":", "")
}

fn directory_for_image(rain_radar_range: Range, timestamp: &str) -> PathBuf {
    Path::new("images")
        .join(rain_radar_range.to_string())
        .join(date_from_timestamp(timestamp))
}

fn create_directory_for_image(rain_radar_range: Range, timestamp: &str) {
    let dir_path = directory_for_image(rain_radar_range, timestamp);

    match std::fs::create_dir_all(&dir_path) {
        Ok(_) => println!("Directories created! `{}`", dir_path.display()),
        Err(err) => println!("Err creating directories! {}", err)
    }
}
