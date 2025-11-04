
use serde::Deserialize;

#[allow(unused)]
#[derive(Deserialize)]
///Base class for physical entities like inputs and outputs.
pub struct DeviceEntity {
    #[serde(rename = "@token")]
    pub token:ReferenceToken
}
#[allow(unused)]
#[derive(Deserialize)]
///Completely undocumented "Token" value. Fuck you onvif foundation.
pub struct ReferenceToken(String);
#[allow(unused)]
#[derive(Deserialize)]
///User readable name. Length up to 64 characters.
pub struct Name(String);
#[allow(unused)]
#[derive(Deserialize)]
///Rectangle defined by lower left corner position and size. Units are pixel.
pub struct IntRectangle {
    #[serde(rename = "@x")]
    x:i16,
    #[serde(rename = "@y")]
    y:i16,
    #[serde(rename = "@width")]
    width:i16,
    #[serde(rename = "@height")]
    height:i16
}
#[allow(unused)]
#[derive(Deserialize)]
///Range of a rectangle. The rectangle itself is defined by lower left corner position and size. Units are pixel.
pub struct IntRectangleRange {

}
