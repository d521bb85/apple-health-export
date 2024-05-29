use apple_health_export_reader::Reader;
use std::io::{self, BufReader, Cursor};

fn read_test_export() -> BufReader<Cursor<&'static [u8]>> {
    let bytes: &'static [u8] = include_bytes!("./test_export.xml");
    let cursor = Cursor::new(bytes);
    BufReader::new(cursor)
}

fn collect_values<R: io::Read>(reader: &mut Reader<R>) -> String {
    let mut values = Vec::new();
    loop {
        match reader.read_next_record().unwrap() {
            Some(record) => values.push(record.value.unwrap_or("".to_string())),
            None => break
        }
    }
    values.join(",")
}

#[test]
fn test_unconditional_reading() {
    let buf_reader = read_test_export();
    let mut reader = Reader::new(buf_reader);
    assert_eq!(collect_values(&mut reader), "1,2,3,4,5,6,7,8,9,10");
}
