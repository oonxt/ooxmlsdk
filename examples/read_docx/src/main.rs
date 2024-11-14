#![recursion_limit = "1024"]

use std::fs;
use hard_xml::XmlWrite;
use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn main() {
    let docx = WordprocessingDocument::new("examples/read_docx/samples/demo.docx").unwrap();

    // println!(
    //     "{:?}",
    //     docx.main_document_part.root_element.to_string()
    // );
    docx.save("examples/read_docx/samples/1_out.docx").unwrap();

    let file = fs::File::open("examples/read_docx/samples/demo.docx").unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    zip.extract("examples/read_docx/samples/1_out").unwrap();

    let file = fs::File::open("examples/read_docx/samples/1_out.docx").unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    zip.extract("examples/read_docx/samples/11_out_out").unwrap();
}
