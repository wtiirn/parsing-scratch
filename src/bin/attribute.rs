use serde::Deserialize;
use serde_xml_rs;

const RESPONSE: &'static str = r#"
<HighLowValues>
    <item date="01/27/2009">
        <data>
            <time>01:36</time>
            <pred>-0.2</pred>
            <type>L</type>
        </data>
        <data>
            <time>08:32</time>
            <pred>4.6</pred>
            <type>H</type>
        </data>
    </item>
    <item date="01/27/2009">
        <data>
            <time>01:36</time>
            <pred>-0.2</pred>
            <type>L</type>
        </data>
    </item>
</HighLowValues>
"#;

#[derive(Debug, Deserialize)]
#[serde(rename="data")]
struct Data {
    time: String,
    pred: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename="item")]
struct Item {
    date: String,
    #[serde(rename="data")]
    data: Vec<Data>,
}

#[derive(Debug, Deserialize)]
#[serde(rename="HighLowValues")]
struct Response {
    #[serde(rename="item")]
    high_low_values: Vec<Item>,
}

fn main() {
    let s: Response = serde_xml_rs::from_str(RESPONSE).unwrap();
    dbg!(s);
}
