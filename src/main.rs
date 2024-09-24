use chrono::{Datelike, Timelike};
use std::f64::consts::PI;

fn main() {
    let time = chrono::Local::now();

    let timezone_offset = time.offset().local_minus_utc() as f64 / 60.;
    let year = time.year();
    let day_of_the_year = time.ordinal() as f64;
    let month = time.month();
    let day = time.day();
    let hour = time.hour() as f64;
    let days = if time.naive_utc().date().leap_year() {
        366
    } else {
        365
    };

    let latitude_deg: f64 = 51.671667;
    let longitude_deg: f64 = 39.210556;
    let latitude_rad = latitude_deg.to_radians();

    // fractional year in radians
    let g = 2. * PI / (days as f64) * (day_of_the_year - 1. + (hour - 12.) / 24.);

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

    // sunrise time in minutes
    let sunrise = (720. - 4. * (longitude_deg + ha.to_degrees()) - eqtime) + timezone_offset;
    let sunrise_hour = (sunrise / 60.) as i32;
    let sunrise_minutes = (sunrise - (sunrise_hour as f64) * 60.) as i32;

    // sunset time in minutes
    let sunset = (720. - 4. * (longitude_deg - ha.to_degrees()) - eqtime) + timezone_offset;
    let sunset_hour = (sunset / 60.) as i32;
    let sunset_minutes = (sunset - (sunset_hour as f64) * 60.) as i32;

    println!(
        "\x1B[94mat\x1B[0m ({}, {}) \x1B[95mon\x1B[0m {:0>2}.{:0>2}.{}:",
        latitude_deg, longitude_deg, day, month, year
    );
    println!(
        " \x1B[93m-\x1B[0m sunrise: {:0>2}:{:0>2}",
        sunrise_hour, sunrise_minutes,
    );
    println!(
        " \x1B[93m-\x1B[0m sunset: {:0>2}:{:0>2}",
        sunset_hour, sunset_minutes,
    );
}
