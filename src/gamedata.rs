use std::fmt;
use std::io::{Read, Write};

use base64ct::{Base64, Encoding};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Unexpected, Visitor};

#[derive(Debug)]
pub struct GameData {
    pub version: u8,
    pub car_body: CarBody,
    pub car_color: CarColor,
    pub data: String,
}

#[derive(Debug)]
pub enum CarBody {
    Standard,
    HotRod,
    Mini,
    SurfMobile,
    SuperSport,
}

#[derive(Debug)]
pub enum CarColor {
    Red,
    Orange,
    Yellow,
    LightGreen,
    Green,
    LightBlue,
    Blue,
    Purple,
    Pink,
    Black,
}

impl<'de> Deserialize<'de> for CarBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CarBodyVisitor;

        impl<'de> Visitor<'de> for CarBodyVisitor {
            type Value = CarBody;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between 0 and 4")
            }

            fn visit_i64<E>(self, value: i64) -> Result<CarBody, E>
            where
                E: de::Error,
            {
                self.visit_u64(value as u64)
            }

            fn visit_u64<E>(self, value: u64) -> Result<CarBody, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(CarBody::Standard),
                    1 => Ok(CarBody::HotRod),
                    2 => Ok(CarBody::Mini),
                    3 => Ok(CarBody::SurfMobile),
                    4 => Ok(CarBody::SuperSport),
                    _ => Err(de::Error::invalid_value(Unexpected::Unsigned(value), &self)),
                }
            }
        }

        deserializer.deserialize_u64(CarBodyVisitor)
    }
}

impl<'de> Deserialize<'de> for CarColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CarColorVisitor;

        impl<'de> Visitor<'de> for CarColorVisitor {
            type Value = CarColor;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an integer between 0 and 9")
            }

            fn visit_i64<E>(self, value: i64) -> Result<CarColor, E>
            where
                E: de::Error,
            {
                self.visit_u64(value as u64)
            }

            fn visit_u64<E>(self, value: u64) -> Result<CarColor, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(CarColor::Red),
                    1 => Ok(CarColor::Orange),
                    2 => Ok(CarColor::Yellow),
                    3 => Ok(CarColor::LightGreen),
                    4 => Ok(CarColor::Green),
                    5 => Ok(CarColor::LightBlue),
                    6 => Ok(CarColor::Blue),
                    7 => Ok(CarColor::Purple),
                    8 => Ok(CarColor::Pink),
                    9 => Ok(CarColor::Black),
                    _ => Err(de::Error::invalid_value(Unexpected::Unsigned(value), &self)),
                }
            }
        }

        deserializer.deserialize_u64(CarColorVisitor)
    }
}
impl<'de> Deserialize<'de> for GameData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        //for some reason rust's base64 crate doesn't read \n properly
        let base64_str: String = String::deserialize(deserializer)?.replace("\n", "");

        let decoded_bytes = Base64::decode_vec(&base64_str).map_err(de::Error::custom)?;

        let mut decoder = ZlibDecoder::new(&decoded_bytes[..]);
        let mut decompressed_bytes = Vec::new();
        decoder
            .read_to_end(&mut decompressed_bytes)
            .map_err(de::Error::custom)?;

        let uncompressed_string =
            String::from_utf8(decompressed_bytes).map_err(de::Error::custom)?;

        let parts: Vec<&str> = uncompressed_string.split(',').collect();
        if parts.len() != 4 {
            return Err(de::Error::custom(
                "Invalid number of fields in uncompressed string",
            ));
        }

        let game_data = GameData {
            version: parts[0].parse().map_err(de::Error::custom)?,
            car_body: serde_json::from_str(parts[1]).map_err(de::Error::custom)?,
            car_color: serde_json::from_str(parts[2]).map_err(de::Error::custom)?,
            data: parts[3].to_string(),
        };

        Ok(game_data)
    }
}

impl Serialize for CarBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let value = match self {
            CarBody::Standard => 0,
            CarBody::HotRod => 1,
            CarBody::Mini => 2,
            CarBody::SurfMobile => 3,
            CarBody::SuperSport => 4,
        };
        serializer.serialize_u64(value)
    }
}

// Implement Serialize for CarColor
impl Serialize for CarColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let value = match self {
            CarColor::Red => 0,
            CarColor::Orange => 1,
            CarColor::Yellow => 2,
            CarColor::LightGreen => 3,
            CarColor::Green => 4,
            CarColor::LightBlue => 5,
            CarColor::Blue => 6,
            CarColor::Purple => 7,
            CarColor::Pink => 8,
            CarColor::Black => 9,
        };
        serializer.serialize_u64(value)
    }
}

impl Serialize for GameData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // Step 1: Create a string representation of the struct
        let data_str = format!("{},{},{},{}",
                               self.version,
                               serde_json::to_string(&self.car_body).map_err(serde::ser::Error::custom)?,
                               serde_json::to_string(&self.car_color).map_err(serde::ser::Error::custom)?,
                               self.data
        );

        // Step 2: Compress the string using zlib
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data_str.as_bytes()).map_err(serde::ser::Error::custom)?;
        let compressed_bytes = encoder.finish().map_err(serde::ser::Error::custom)?;

        // Step 3: Encode the compressed bytes to base64
        let base64_str = Base64::encode_string(&compressed_bytes);

        // Step 4: Serialize the base64 string
        serializer.serialize_str(&base64_str)
    }
}
