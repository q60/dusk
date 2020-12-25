use std::f64::consts::PI;

fn deg_to_rad(angle_deg: f64) -> f64 {
  PI * angle_deg / 180_f64
}

fn rad_to_deg(angle_rad: f64) -> f64 {
  180_f64 * angle_rad / PI
}

fn mean_ecliptic_obliquity(t: f64) -> f64 {
  let seconds: f64 = 21.448 - t * (46.8150 + t * (0.00_059 - t * (0.001_813)));
  23_f64 + (26_f64 + (seconds / 60_f64)) / 60_f64 // in degrees
}

fn mean_sun_lon(t: f64) -> f64 {
  let mut lon: f64 = 280.46_646 + t * (36_000.76983 + 0.0_003_032 * t);
  while lon as i64 > 360 {
    lon -= 360_f64;
  }
  while lon < 0_f64 {
    lon += 360_f64;
  }
  lon // in degrees
}

fn obliquity_correction(t: f64) -> f64 {
  let e0:    f64 = mean_ecliptic_obliquity(t);
  let omega: f64 = 125.04 - 1934.136 * t;
  e0 + 0.00_256 * deg_to_rad(omega).cos()
}

fn earth_orbit_eccentricity(t: f64) -> f64 {
  0.016_708_634 - t * (0.000_042_037 + 0.0_000_001_267 * t)
}

fn mean_sun_anomaly(t: f64) -> f64 {
  357.52_911 + t * (35_999.05_029 - 0.0_001_537 * t)
}

fn sun_center_equation(t: f64) -> f64 {
  let m:     f64 = mean_sun_anomaly(t);
  let mrad:  f64 = deg_to_rad(m);
  let sinm:  f64 = mrad.sin();
  let sin2m: f64 = (2_f64 * mrad).sin();
  let sin3m: f64 = (3_f64 * mrad).sin();
  sinm * (1.914_602 - t * (0.004_817 + 0.000_014 * t)) + sin2m * (0.019_993 - 0.000_101 * t) + sin3m * 0.000_289
}

fn time_equation(t: f64) -> f64 {
  let epsilon: f64 = obliquity_correction(t);
  let l0:      f64 = mean_sun_lon(t);
  let e:       f64 = earth_orbit_eccentricity(t);
  let m:       f64 = mean_sun_anomaly(t);
  let y:       f64 = ((deg_to_rad(epsilon) / 2_f64).tan()).powi(2);
  let sin2l0:  f64 = (2. * deg_to_rad(l0)).sin();
  let cos2l0:  f64 = (2. * deg_to_rad(l0)).cos();
  let sin4l0:  f64 = (4. * deg_to_rad(l0)).sin();
  let sinm:    f64 = (deg_to_rad(m)).sin();
  let sin2m:   f64 = (2. * deg_to_rad(m)).sin();
  rad_to_deg(y * sin2l0 - 2_f64 * e * sinm + 4_f64 * e * y * sinm * cos2l0 - 0.5 * y * y * sin4l0 - 1.25 * e * e * sin2m) * 4_f64
}

fn julian_cent(jd: f64) -> f64 {
  (jd - 2_451_545_f64) / 36_525_f64
}

fn true_sun_lon(t: f64) -> f64 {
  mean_sun_lon(t) + sun_center_equation(t)
}

fn apparent_sun_lon(t: f64) -> f64 {
  let o:      f64 = true_sun_lon(t);
  let omega:  f64 = 125.04 - 1934.136 * t;
  o - 0.00569 - 0.00478 * deg_to_rad(omega).sin()
}

fn sun_declination(t: f64) -> f64 {
  let e:      f64 = obliquity_correction(t);
  let lambda: f64 = apparent_sun_lon(t);
  let sint:   f64 = (deg_to_rad(e) * deg_to_rad(lambda).sin()).sin();
  rad_to_deg(sint.asin())
}

fn sunrise_hour_angle(lat: f64, decl: f64) -> f64 {
  let lat_rad:  f64 = deg_to_rad(lat);
  let decl_rad: f64 = deg_to_rad(decl);
  (deg_to_rad(90.833).cos() / (lat_rad.cos() * decl_rad.cos()) - lat_rad.tan() * decl_rad.tan()).acos()
}

fn sunset_hour_angle(lat: f64, decl: f64) -> f64 {
  -sunrise_hour_angle(lat, decl)
}

fn jd(mut yy: u64, mut mm: u64, dd: u64) -> f64 {
  if mm <= 2 {
    yy -= 1;
    mm += 12;
  }
  let a: f64 = (yy as f64 / 100_f64).floor();
  let b: f64 = 2_f64 - a + (a as f64 / 4_f64).floor();
  (365.25 * (yy as f64 + 4716_f64)).floor() + (30.6001 * (mm as f64 + 1_f64)).floor() + (dd as f64) + (b as f64) - 1524.5
}

fn jd_from_julian_cent(t: f64) -> f64 {
  t * 36525_f64 + 2451545_f64
}

fn utc_time(jd: f64, lat: f64, lon: f64, mode: &str) -> f64 {
  let t:         f64 = julian_cent(jd);
  let decl:      f64 = sun_declination(t);
  let ha:        f64;
  if mode == "sunrise" {
    ha = sunrise_hour_angle(lat, decl);
  } else if mode == "sunset" {
    ha = sunset_hour_angle(lat, decl);
  } else {
    ha = sunrise_hour_angle(lat, decl);
  }
  let delta:     f64 = lon - rad_to_deg(ha);
  let time_diff: f64 = 4_f64 * delta;
  let new_time:  f64 = julian_cent(jd_from_julian_cent(t));
  720_f64 + time_diff - time_equation(new_time)
}

fn main() {
  let year:  u64;
  let month: u64;
  let day:   u64;
  let latitude:  f64 = 51.67_166;
  let longitude: f64 = -39.21_055;
  match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
    Ok(unixtime) => {
      let days: f64 = unixtime.as_secs_f64() / 3600_f64 / 24_f64; // 1609459200_f64
      year   = (days / 365.25).floor() as u64 + 1970;
      month  = (days % 365.25 / 30.437).ceil() as u64;
      day    = (days % 30.437).ceil() as u64;
    }
    Err(_) => {
      year   = 1970;
      month  = 1;
      day    = 1;
    }
  }
  let sunrise_time: i64 = utc_time(jd(year, month, day), latitude, longitude, "sunrise") as i64 * 60 + 10_800;
  let sunset_time:  i64 = utc_time(jd(year, month, day), latitude, longitude, "sunset") as i64 * 60 + 10_800;
  println!(
    "\x1B[94mat\x1B[0m ({}, {}) \x1B[95mon\x1B[0m {}.{}.{}:",
    latitude,
    longitude,
    day,
    month,
    year
  );
  println!(
    " \x1B[93m-\x1B[0m sunrise: {}:{}",
    sunrise_time % 86_400 / 3600,
    sunrise_time % 3600 / 60,
  );
  println!(
    " \x1B[93m-\x1B[0m sunset: {}:{}",
    sunset_time % 86_400 / 3600,
    sunset_time % 3600 / 60,
  );
}
