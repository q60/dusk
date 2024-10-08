use argh::FromArgs;
use chrono::{Datelike, Timelike};
use std::f64::consts::PI;

#[derive(FromArgs)]
/// civil sunrise and sunset time fetcher
struct Args {
    /// latitude in degrees
    #[argh(option)]
    latitude: f64,

    /// longitude in degrees
    #[argh(option)]
    longitude: f64,

    /// wether to show only the times
    #[argh(switch, short = 's')]
    simple: bool,
}

fn main() {
    let Args {
        latitude,
        longitude,
        simple,
    } = argh::from_env();

    let time = chrono::Local::now();
    let hours_since_year = 24 * time.ordinal0() + time.hour() - 12;
    let timezone_offset = time.offset().local_minus_utc() as f64 / 60.;
    let days = if time.naive_utc().date().leap_year() {
        366.
    } else {
        365.
    };

    let latitude_deg: f64 = latitude;
    let longitude_deg: f64 = longitude;
    let latitude_rad = latitude_deg.to_radians();

    // fractional year in radians
    let g = 2. * PI / days * hours_since_year as f64 / 24.;

    // equation of time in minutes
    let eqtime = 229.18
        * (0.000075 + 0.001868 * g.cos()
            - 0.032077 * g.sin()
            - 0.014615 * (2. * g).cos()
            - 0.040849 * (2. * g).sin());

    // solar declination angle in radians
    let decl = 0.006918 - 0.399912 * g.cos() + 0.070257 * g.sin() - 0.006758 * (2. * g).cos()
        + 0.000907 * (2. * g).sin()
        - 0.002697 * (3. * g).cos()
        + 0.00148 * (3. * g).sin();

    // hour angle in radians
    let ha = (90.833_f64.to_radians().cos() / (latitude_rad.cos() * decl.cos())
        - latitude_rad.tan() * decl.tan())
    .acos();

    let base = 720. - 4. * longitude_deg - eqtime + timezone_offset;
    let offset = 4. * ha.to_degrees();

    // sunrise time in minutes
    let sunrise = base - offset;
    let sunrise_hour = (sunrise / 60.) as i32;
    let sunrise_minutes = (sunrise % 60.) as i32;

    // sunset time in minutes
    let sunset = base + offset;
    let sunset_hour = (sunset / 60.) as i32;
    let sunset_minutes = (sunset % 60.) as i32;

    if !simple {
        println!(
            "\x1B[94mat\x1B[0m ({}, {}) \x1B[95mon\x1B[0m {}:",
            latitude_deg,
            longitude_deg,
            time.format("%d.%m.%Y")
        );
        println!(
            " \x1B[93m-\x1B[0m sunrise: {:0>2}:{:0>2}",
            sunrise_hour, sunrise_minutes,
        );
        println!(
            " \x1B[93m-\x1B[0m sunset: {:0>2}:{:0>2}",
            sunset_hour, sunset_minutes,
        );
    } else {
        println!("{:0>2}:{:0>2}", sunrise_hour, sunrise_minutes,);
        println!("{:0>2}:{:0>2}", sunset_hour, sunset_minutes,);
    }
}
