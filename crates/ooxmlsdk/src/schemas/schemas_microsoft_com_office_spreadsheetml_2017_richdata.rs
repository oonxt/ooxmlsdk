#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RichValueFallbackType {
    #[default]
    B,
    N,
    E,
    S,
}
crate::__string_enum! {
    RichValueFallbackType { B = "b", N = "n", E = "e", S = "s", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RichValueValueType {
    #[default]
    D,
    I,
    B,
    E,
    S,
    R,
    A,
    Spb,
}
crate::__string_enum! {
    RichValueValueType { D = "d", I = "i", B = "b", E = "e", S = "s", R = "r", A = "a",
    Spb = "spb", }
}
/// Defines the RichValueBlock Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:rvb.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:rvb")]
pub struct RichValueBlock {
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub i: i32,
}
/// Defines the RichValueData Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:rvData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:rvData")]
pub struct RichValueData {
    #[xml(attr = "xmlns", with = "rich_value_data_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd:rv")]
    pub xlrd_rv: Vec<RichValue>,
    /// _
    #[xml(child = "xlrd:extLst")]
    pub xlrd_ext_lst: Option<ExtensionList>,
}
mod rich_value_data_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2017/richdata")
    }
}
/// Defines the RichValueStructures Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:rvStructures.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:rvStructures")]
pub struct RichValueStructures {
    #[xml(attr = "xmlns", with = "rich_value_structures_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// count
    /// Represents the following attribute in the schema: :count
    #[xml(attr = "count")]
    pub count: i32,
    /// _
    #[xml(child = "xlrd:s")]
    pub xlrd_s: Vec<RichValueStructure>,
    /// _
    #[xml(child = "xlrd:extLst")]
    pub xlrd_ext_lst: Option<ExtensionList>,
}
mod rich_value_structures_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/spreadsheetml/2017/richdata")
    }
}
/// Defines the RichValue Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:rv.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:rv")]
pub struct RichValue {
    /// s
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub s: i32,
    /// _
    #[xml(child = "xlrd:fb")]
    pub rich_value_fallback: Option<RichValueFallback>,
    /// _
    #[xml(child = "xlrd:v")]
    pub xlrd_v: Vec<Value>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the RichValueFallback Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:fb.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:fb")]
pub struct RichValueFallback {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<RichValueFallbackType>,
    #[xml(text)]
    pub child: String,
}
/// Defines the Value Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:v")]
pub struct Value {
    #[xml(text)]
    pub child: String,
}
/// Defines the RichValueStructure Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:s.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:s")]
pub struct RichValueStructure {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: String,
    /// _
    #[xml(child = "xlrd:k")]
    pub xlrd_k: Vec<Key>,
}
/// Defines the Key Class.
/// When the object is serialized out as xml, it's qualified name is xlrd:k.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xlrd:k")]
pub struct Key {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<RichValueValueType>,
}
