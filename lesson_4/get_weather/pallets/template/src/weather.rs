
use codec::{Decode, Encode};
use serde::{Deserialize, Deserializer};
use frame_support::inherent::Vec;
use log;

#[derive(Deserialize, Encode, Decode)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Deserialize, Encode, Decode)]
pub struct Weather {
    // details: Details
    details: Vec<Details>
}

#[derive(Deserialize, Encode, Decode)]
pub struct Details {
    id: i32,
    #[serde(deserialize_with = "de_string_to_bytes")]
    main: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    description: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    icon: Vec<u8>,
}

#[derive(Deserialize, Encode, Decode)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: u32,
    pub grnd_level: u32,
}

#[derive(Deserialize, Encode, Decode)]
pub struct Wind {
    speed: f64,
    deg: u32,
    gust: u32,
}

#[derive(Deserialize, Encode, Decode)]
pub struct Clouds {
    all: u32,
}

#[derive(Deserialize, Encode, Decode)]
pub struct Sys {
    _type: u32,
    id: u32,
    #[serde(deserialize_with = "de_string_to_bytes")]
    country: Vec<u8>,
    sunrise: u64,
    sunset: u64
}


// #[derive(Deserialize, Encode, Decode)]
// pub struct All {
//     coord: Coord,
//     weather: Weather,
//     base: Vec<u8>,
//     main: Main,
// }

// #[derive(Deserialize, Encode, Decode)]
// pub struct All {
//     #[serde(deserialize_with = "de_coord_to_bytes")]
//     coord: Coord,
//     #[serde(deserialize_with = "de_weather_to_bytes")]
//     weather: Weather,
//     #[serde(deserialize_with = "de_string_to_bytes")]
//     base: Vec<u8>,
//     #[serde(deserialize_with = "de_main_to_bytes")]
//     main: Main,
//     visibility: u64,
//     #[serde(deserialize_with = "de_wind_to_bytes")]
//     wind: Wind,
//     #[serde(deserialize_with = "de_clouds_to_bytes")]
//     clouds: Clouds,
//     dt: u64,
//     #[serde(deserialize_with = "de_sys_to_bytes")]
//     sys: Sys,
//     timezone: u64,
//     id: u64,
//     #[serde(deserialize_with = "de_string_to_bytes")]
//     name: Vec<u8>,
//     cod: u64
// }

#[derive(Deserialize, Encode, Decode)]
pub struct All {
    #[serde(deserialize_with = "de_main_to_bytes")]
    pub main: Main,
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}

// pub fn de_coord_to_bytes<'de, D>(de: D) -> Result<Coord, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let s: Coord = Deserialize::deserialize(de)?;
//     log::info!("coord: {:?}", s);
//
//     Ok(s)
// }
//
// pub fn de_weather_to_bytes<'de, D>(de: D) -> Result<Weather, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let s: Weather = Deserialize::deserialize(de)?;
//     log::info!("weather: {:?}", s);
//     Ok(s)
// }

pub fn de_main_to_bytes<'de, D>(de: D) -> Result<Main, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Main = Deserialize::deserialize(de)?;
    log::info!("main: {:?}", s);
    Ok(s)
}

// pub fn de_wind_to_bytes<'de, D>(de: D) -> Result<Wind, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let s: Wind = Deserialize::deserialize(de)?;
//     log::info!("wind: {:?}", s);
//     Ok(s)
// }
//
// pub fn de_clouds_to_bytes<'de, D>(de: D) -> Result<Clouds, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let s: Clouds = Deserialize::deserialize(de)?;
//     log::info!("clouds: {:?}", s);
//     Ok(s)
// }
//
// pub fn de_sys_to_bytes<'de, D>(de: D) -> Result<Sys, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     let s: Sys = Deserialize::deserialize(de)?;
//     log::info!("sys: {:?}", s);
//     Ok(s)
// }


use core::{convert::TryInto, fmt};
// impl fmt::Debug for All {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(
//             f,
//             // "{{ coord: {}, weather: {}, base: {}, main: {} }}",
//             "{{ coord: {:?}, weather: none, base: {:?}, main: {:?}, \
//              visibility: {:?}, wind: {:?}, clouds: {:?}, dt: {:?}, sys: {:?}, \
//              timezone: {:?}, id: {:?}, name: {:?}, cod: {:?} }}",
//             &self.coord,
//             // &self.weather,
//             &self.base,
//             &self.main,
//             &self.visibility,
//             &self.wind,
//             &self.clouds,
//             &self.dt,
//             &self.sys,
//             &self.timezone,
//             &self.id,
//             &self.name,
//             &self.cod,
//         )
//     }
// }
impl fmt::Debug for All {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ main: {:?} }}",
            &self.main,
        )
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ lon: {}, lat: {} }}",
            &self.lon,
            &self.lat
        )
    }
}

impl fmt::Debug for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ details: {:?} }}",
            &self.details,
        )
    }
}

impl fmt::Debug for Details {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ id: {}, main: {}, description: {}, icon: {} }}",
            &self.id,
            sp_std::str::from_utf8(&self.main).map_err(|_| fmt::Error)?,
            sp_std::str::from_utf8(&self.description).map_err(|_| fmt::Error)?,
            sp_std::str::from_utf8(&self.icon).map_err(|_| fmt::Error)?,
        )
    }
}

impl fmt::Debug for Main {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ temp: {}, feels_like: {}, temp_min: {}, temp_max: {}, pressure: {}, humidity: {}, sea_level: {}, grnd_level: {} }}",
            &self.temp,
            &self.feels_like,
            &self.temp_min,
            &self.temp_max,
            &self.pressure,
            &self.humidity,
            &self.sea_level,
            &self.grnd_level,
        )
    }
}

impl fmt::Debug for Wind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ speed: {}, deg: {}, gust: {} }}",
            &self.speed,
            &self.deg,
            &self.gust,
        )
    }
}

impl fmt::Debug for Clouds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ all: {} }}",
            &self.all,
        )
    }
}

impl fmt::Debug for Sys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ type: {}, id: {}, country: {}, sunrise: {}, sunrise: {} }}",
            &self._type,
            &self.id,
            sp_std::str::from_utf8(&self.country).map_err(|_| fmt::Error)?,
            &self.sunrise,
            &self.sunset,
        )
    }
}
