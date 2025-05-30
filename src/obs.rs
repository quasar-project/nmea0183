use crate::common;
use crate::datetime::Time;
use crate::Source;

#[derive(Debug, PartialEq, Clone)]
pub struct Dim3 {
  pub x: i16,
  pub y: i16,
  pub z: i16,
}

impl<'a> Dim3 {
  pub fn parse(fields: &mut core::str::Split<'a, char>) -> Result<Option<Self>, &'static str> {
    let x = common::parse_i16(fields.next())?;
    let y = common::parse_i16(fields.next())?;
    let z = common::parse_i16(fields.next())?;

    if let (Some(x), Some(y), Some(z)) = (x, y, z) {
      return Ok(Some(Dim3 { x, y, z }));
    } else {
      return Ok(None);
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct OBS {
  /// Navigational system.
  pub source: Source,
  pub time: Time,
  pub pressure: f32,
  pub temperature: f32,
  pub gyroscope: Dim3,
  pub accelerometer: Dim3,
  pub compass: Dim3,
}

impl OBS {
  pub(crate) fn parse<'a>(
    source: Source,
    fields: &mut core::str::Split<'a, char>,
  ) -> Result<Option<Self>, &'static str> {
    let time = Time::parse_from_hhmmss(fields.next())?;
    let pressure = common::parse_f32(fields.next())?;
    let temperature = common::parse_f32(fields.next())?;
    let gyroscope = Dim3::parse(fields)?;
    let accelerometer = Dim3::parse(fields)?;
    let compass = Dim3::parse(fields)?;

    if let (
      Some(time),
      Some(pressure),
      Some(temperature),
      Some(gyroscope),
      Some(accelerometer),
      Some(compass),
    ) = (
      time,
      pressure,
      temperature,
      gyroscope,
      accelerometer,
      compass,
    ) {
      Ok(Some(OBS {
        source,
        time,
        pressure,
        temperature,
        gyroscope,
        accelerometer,
        compass,
      }))
    } else {
      Ok(None)
    }
  }
}
