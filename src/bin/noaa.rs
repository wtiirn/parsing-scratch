use serde::Deserialize;
use serde_xml_rs;

// Taken from the examples section of the NOAA website
// https://opendap.co-ops.nos.noaa.gov/axis/webservices/highlowtidepred/samples/response.xml
const RESPONSE: &'static str = include_str!("./noaa_response.xml");

#[derive(Debug, Deserialize)]
struct Envelope {
    #[serde(rename = "Body")]
    body: Body,
}

#[derive(Debug, Deserialize)]
struct Body {
    #[serde(rename = "HighLowAndMetadata")]
    predictions: HighLowAndMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "data")]
struct TideData {
    time: String,
    pred: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "item")]
struct Item {
    date: String,
    data: Vec<TideData>,
}

#[derive(Debug, Deserialize)]
struct HighLowValues {
    #[serde(rename = "item")]
    values: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct HighLowAndMetadata {
    stationId: u32,
    stationName: String,
    latitude: f64,
    longitude: f64,
    timeZone: String,
    #[serde(rename = "unit")]
    unit_name: String,
    #[serde(rename = "HighLowValues")]
    values: HighLowValues,
}

fn main() {
    let s: Envelope = serde_xml_rs::from_str(RESPONSE).unwrap();
    dbg!(s);
}
