use std::io::BufRead;

use anyhow::Result;
use quick_xml::{
    de::Deserializer,
    events::{BytesStart, Event},
    Reader, Writer,
};
use serde::{Deserialize, Serialize};

use crate::defaults::SJC_GOLD_BASE_URL;

use super::DataProvider;

const FILTER_CITY: &str = "Hồ Chí Minh";
const FILTER_TYPE: &str = "Vàng SJC";
const MULTIPLIER: f32 = 1000000.0;

pub struct SjcDataProvider {
    base_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct City {
    #[serde(rename = "@name")]
    name: String,
    item: Option<Vec<Item>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "@type")]
    r#type: String,
    #[serde(rename = "@buy")]
    buy: f32,
    #[serde(rename = "@sell")]
    sell: f32,
}

impl DataProvider for SjcDataProvider {
    fn new() -> Self {
        return SjcDataProvider {
            base_url: SJC_GOLD_BASE_URL.to_string(),
        };
    }

    async fn fetch_asset_price(&self, _: Option<String>) -> Result<f32> {
        let resp = reqwest::get(self.base_url.clone()).await?.text().await?;

        // println!("{:#?}", resp);
        let mut reader = Reader::from_str(&resp);
        reader.trim_text(true);

        // Setup buffer
        let mut buf = Vec::new();
        let mut junk_buf: Vec<u8> = Vec::new();

        // Setup tuple response: (buy, sell)
        let mut gold_price = Item {
            r#type: "SJC".to_string(), // Hardcode
            buy: 0.0,
            sell: 0.0,
        };

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,

                Ok(Event::Start(e)) => match e.name().as_ref() {
                    b"city" => {
                        // Load city into byte, then convert to str
                        let city_bytes =
                            read_to_end_into_buffer(&mut reader, &e, &mut junk_buf).unwrap();
                        let city_str = std::str::from_utf8(&city_bytes).unwrap();
                        // print!("{}\n---x---\n", city_str);

                        // Map struct City object
                        let mut deserializer = Deserializer::from_str(city_str);
                        let city = City::deserialize(&mut deserializer).unwrap();
                        if city.name == FILTER_CITY {
                            // print!("{:?}\n", city);
                            if let Some(items) = city.item {
                                for i in items {
                                    if i.r#type.contains(FILTER_TYPE) {
                                        gold_price.buy = i.buy * MULTIPLIER;
                                        gold_price.sell = i.sell * MULTIPLIER;
                                    }
                                }
                            }
                            break;
                        }
                    }
                    _ => (),
                },

                // There are several other `Event`s we do not consider here
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
            junk_buf.clear();
        }

        Ok(gold_price.sell)
    }
}

fn read_to_end_into_buffer<R: BufRead>(
    reader: &mut Reader<R>,
    start_tag: &BytesStart,
    junk_buf: &mut Vec<u8>,
) -> Result<Vec<u8>, quick_xml::Error> {
    let mut depth = 0;
    let mut output_buf: Vec<u8> = Vec::new();
    let mut w = Writer::new(&mut output_buf);
    let tag_name = start_tag.name();
    w.write_event(Event::Start(start_tag.clone()))?;
    loop {
        junk_buf.clear();
        let event = reader.read_event_into(junk_buf)?;
        w.write_event(&event)?;

        match event {
            Event::Start(e) if e.name() == tag_name => depth += 1,
            Event::End(e) if e.name() == tag_name => {
                if depth == 0 {
                    return Ok(output_buf);
                }
                depth -= 1;
            }
            Event::Eof => {
                panic!("oh no")
            }
            _ => {}
        }
    }
}
