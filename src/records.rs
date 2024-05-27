use quick_xml;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use std::fs::File;
use std::io;
use std::path::Path;

type XmlReader = quick_xml::Reader<io::BufReader<File>>;

#[derive(Debug)]
pub struct Record {
    start_date: Option<String>,
    end_date: Option<String>,
    value: Option<String>,
    r#type: Option<String>,
    source_name: Option<String>,
    source_version: Option<String>,
    device: Option<String>,
    unit: Option<String>,
    creation_date: Option<String>,
}

impl Record {
    fn from_bytes(xml_reader: &XmlReader, tag: &BytesStart) -> Result<Self, quick_xml::Error> {
        let mut record = Record::default();
        for attr_result in tag.attributes() {
            let attr = attr_result?;
            let key = std::str::from_utf8(attr.key.as_ref())?;
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

pub struct Reader {
    xml_reader: XmlReader,
    event_buf: Vec<u8>,
}

impl Reader {
    pub fn new(source: &Path) -> Result<Self, io::Error> {
        let xml_reader = Self::create_xml_reader(source)?;
        let event_buf: Vec<u8> = Vec::new();
        Ok(Self {
            xml_reader,
            event_buf,
        })
    }

    fn create_xml_reader(source: &Path) -> Result<XmlReader, io::Error> {
        let file = File::open(source)?;
        let buf_reader = io::BufReader::new(file);
        let mut xml_reader = quick_xml::Reader::from_reader(buf_reader);
        xml_reader.trim_text(true);
        Ok(xml_reader)
    }

    pub fn read_next_record(&mut self) -> Result<Option<Record>, quick_xml::Error> {
        let next_record = loop {
            match self.xml_reader.read_event_into(&mut self.event_buf) {
                Ok(Event::Start(ref tag)) if tag.name() == QName(b"Record") => {
                    let record = Record::from_bytes(&self.xml_reader, tag)?;
                    break Some(record);
                }
                Ok(Event::Eof) => break None,
                _ => continue,
            }
        };
        if next_record.is_none() {
            self.event_buf.clear();
        }
        Ok(next_record)
    }
}
