use quick_xml as xml;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use std::io::{BufReader, Read};
use std::str;

type XmlReader<R> = xml::Reader<BufReader<R>>;

#[derive(Debug)]
pub struct Record {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub value: Option<String>,
    pub r#type: Option<String>,
    pub source_name: Option<String>,
    pub source_version: Option<String>,
    pub device: Option<String>,
    pub unit: Option<String>,
    pub creation_date: Option<String>,
}

impl Default for Record {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            value: None,
            r#type: None,
            source_name: None,
            source_version: None,
            device: None,
            unit: None,
            creation_date: None,
        }
    }
}

impl Record {
    fn from_tag_data<R: Read>(
        xml_reader: &XmlReader<R>,
        tag_data: &BytesStart,
    ) -> Result<Self, xml::Error> {
        let mut record = Self::default();
        for attr_result in tag_data.attributes() {
            let attr = attr_result?;
            let key = str::from_utf8(attr.key.as_ref())?;
            let val = Some(attr.decode_and_unescape_value(xml_reader)?.to_string());
            match key {
                "startDate" => record.start_date = val,
                "endDate" => record.end_date = val,
                "value" => record.value = val,
                "type" => record.r#type = val,
                "sourceName" => record.source_name = val,
                "sourceVersion" => record.source_version = val,
                "device" => record.device = val,
                "unit" => record.unit = val,
                "creationDate" => record.creation_date = val,
                _ => (),
            }
        }
        Ok(record)
    }
}

pub struct Reader<R: Read> {
    xml_reader: XmlReader<R>,
    event_buf: Vec<u8>,
}

impl<R: Read> Reader<R> {
    pub fn new(buf_reader: BufReader<R>) -> Self {
        Self {
            xml_reader: Self::create_xml_reader(buf_reader),
            event_buf: Vec::new(),
        }
    }

    fn create_xml_reader(buf_reader: BufReader<R>) -> XmlReader<R> {
        let mut xml_reader = xml::Reader::from_reader(buf_reader);
        xml_reader.trim_text(true);
        xml_reader
    }

    pub fn read_next_record(&mut self) -> Result<Option<Record>, xml::Error> {
        let next_record = loop {
            match self.xml_reader.read_event_into(&mut self.event_buf) {
                Ok(Event::Start(ref tag_data) | Event::Empty(ref tag_data)) => {
                    if tag_data.name() == QName(b"Record") {
                        let record = Record::from_tag_data(&self.xml_reader, tag_data)?;
                        break Some(record);
                    }
                }
                Ok(Event::Eof) => break None,
                _ => continue,
            }
        };
        Ok(next_record)
    }
}
