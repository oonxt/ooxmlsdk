/// Defines the List Class.
/// When the object is serialized out as xml, it's qualified name is x12ac:list.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "x12ac:list")]
pub struct List {
    #[xml(text)]
    pub child: String,
}
