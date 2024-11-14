#![recursion_limit = "1024"]

use std::fs;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use hard_xml::{XmlRead, XmlWrite};
fn main() {
  let xlsx = SpreadsheetDocument::new("examples/read_xlsx/samples/1.xlsx").unwrap();

  let file = fs::File::open("examples/read_xlsx/samples/1.xlsx").unwrap();
  let mut zip = zip::ZipArchive::new(file).unwrap();
  zip.extract("examples/read_xlsx/samples/1_out").unwrap();

  xlsx.save("examples/read_xlsx/samples/1_out.xlsx").unwrap();
  let file = fs::File::open("examples/read_xlsx/samples/1_out.xlsx").unwrap();
  let mut zip = zip::ZipArchive::new(file).unwrap();
  zip.extract("examples/read_xlsx/samples/1_out_out").unwrap();
}
