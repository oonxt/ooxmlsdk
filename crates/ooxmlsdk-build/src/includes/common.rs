use std::{
  io::BufRead,
  num::{ParseFloatError, ParseIntError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SdkError {
  #[error("xml error")]
  XmlError(#[from] hard_xml::XmlError),
  #[error("ParseIntError")]
  ParseIntError(#[from] ParseIntError),
  #[error("ParseFloatError")]
  ParseFloatError(#[from] ParseFloatError),
  #[error("StdFmtError")]
  StdFmtError(#[from] std::fmt::Error),
  #[error("StdIoError")]
  StdIoError(#[from] std::io::Error),
  #[error("ZipError")]
  ZipError(#[from] zip::result::ZipError),
  #[error("mismatch error (expected {expected:?}, found {found:?})")]
  MismatchError { expected: String, found: String },
  #[error("`{0}` common error")]
  CommonError(String),
  #[error("unknown error")]
  UnknownError,
}


pub fn resolve_zip_file_path(path: &str) -> String {
  let mut stack = Vec::new();

  for component in path.split('/') {
    match component {
      "" | "." => {
        // Ignore empty components and current directory symbol
      }
      ".." => {
        // Go up one directory if possible
        stack.pop();
      }
      _ => {
        // Add the component to the path
        stack.push(component);
      }
    }
  }
  // Join the components back into a path
  stack.join("/")
}

pub fn read_string(file: &mut zip::read::ZipFile) -> Result<String, SdkError> {
  let mut buffer = String::new();
  file.read_to_string(&mut buffer)?;
  Ok(buffer)
}