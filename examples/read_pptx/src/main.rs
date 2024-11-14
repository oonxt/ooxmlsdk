#![recursion_limit = "1024"]

use std::fs;
use ooxmlsdk::parts::presentation_document::PresentationDocument;
use hard_xml::{XmlWrite, XmlRead};
use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main::{Presentation, SlideMasterId};

fn main() {
  let pptx = PresentationDocument::new("examples/read_pptx/samples/demo.pptx").unwrap();

  let file = fs::File::open("examples/read_pptx/samples/demo.pptx").unwrap();
  let mut zip = zip::ZipArchive::new(file).unwrap();
  zip.extract("examples/read_pptx/samples/1_out").unwrap();

  pptx.save("examples/read_pptx/samples/out_1.pptx").unwrap();
  let file = fs::File::open("examples/read_pptx/samples/out_1.pptx").unwrap();
  let mut zip = zip::ZipArchive::new(file).unwrap();
  zip.extract("examples/read_pptx/samples/1_out_out").unwrap();
  // let s = fs::read_to_string("examples/read_pptx/samples/1_out/ppt/presentation.xml").unwrap();
  // Presentation::from_str(&s).unwrap();
  // let s = r#"<p:sldMasterId id="2147483684" r:id="rId1"/>"#;
  // SlideMasterId::from_str(s).unwrap();
}
