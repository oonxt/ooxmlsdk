/// Ink Annotation Flag.
/// When the object is serialized out as xml, it's qualified name is pvml:iscomment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pvml:iscomment")]
pub struct InkAnnotationFlag {}
/// VML Diagram Text.
/// When the object is serialized out as xml, it's qualified name is pvml:textdata.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pvml:textdata")]
pub struct TextData {
    /// Text Reference
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
}
