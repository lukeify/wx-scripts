mod rain_radar_range;
mod rain_radar_location;

use std::io::Write;
use std::path::{Path, PathBuf};
use crate::rain_radar_range::RainRadarRange;
use crate::rain_radar_location::RainRadarLocation;
use chrono::{DateTime, Utc, FixedOffset, SecondsFormat, Timelike};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let timestamp = "2023-12-31T11:52:00+13:00";

    create_directory_for_image(
        RainRadarLocation::Otago,
        RainRadarRange::Close,
        timestamp
    );

    let response = reqwest::get(rain_radar_url(RainRadarLocation::Otago, RainRadarRange::Close, timestamp))
        .await?
        .bytes()
        .await?;

    let mut path = directory_for_image(
        RainRadarLocation::Otago,
        RainRadarRange::Close,
        timestamp
    ).join(filename_for_image(timestamp));
    path.set_extension("gif");

    let mut img_file = std::fs::File::create(path)?;
    img_file.write_all(&response)?;

    let timestamp = current_metservice_compatible_timestamp();

    println!("{}", timestamp);
    Ok(())
}


// urls

fn rain_radar_url(location: RainRadarLocation, range: RainRadarRange, timestamp: &str) -> String {
    let location = location.to_string();
    let range = range.to_string();

    let url_segments = vec![
        "https://www.metservice.com/publicData/rainRadar/image",
        &location,
        &range,
        timestamp
    ];

    String::from(url_segments.join("/"))
}

// timestamps and naming

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

fn directory_for_image(location: RainRadarLocation, range: RainRadarRange, timestamp: &str) -> PathBuf {
    Path::new("images")
        .join(location.to_string())
        .join(range.to_string())
        .join(date_from_timestamp(timestamp))
}

fn create_directory_for_image(location: RainRadarLocation, range: RainRadarRange, timestamp: &str) {
    let dir_path = directory_for_image(location, range, timestamp);

    match std::fs::create_dir_all(&dir_path) {
        Ok(_) => println!("Directories created! `{}`", dir_path.display()),
        Err(err) => println!("Err creating directories! {}", err)
    }
}