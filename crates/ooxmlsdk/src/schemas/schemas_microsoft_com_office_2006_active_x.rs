#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PersistenceValues {
    #[default]
    PersistPropertyBag,
    PersistStream,
    PersistStreamInit,
    PersistStorage,
}
crate::__string_enum! {
    PersistenceValues { PersistPropertyBag = "persistPropertyBag", PersistStream =
    "persistStream", PersistStreamInit = "persistStreamInit", PersistStorage =
    "persistStorage", }
}
/// Defines the ActiveXControlData Class.
/// When the object is serialized out as xml, it's qualified name is ax:ocx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ax:ocx")]
pub struct ActiveXControlData {
    /// classid
    /// Represents the following attribute in the schema: ax:classid
    #[xml(attr = "ax:classid")]
    pub active_x_control_class_id: String,
    /// license
    /// Represents the following attribute in the schema: ax:license
    #[xml(attr = "ax:license")]
    pub license: Option<String>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// persistence
    /// Represents the following attribute in the schema: ax:persistence
    #[xml(attr = "ax:persistence")]
    pub persistence: PersistenceValues,
    /// _
    #[xml(child = "ax:ocxPr")]
    pub ax_ocx_pr: Vec<ActiveXObjectProperty>,
}
/// Defines the ActiveXObjectProperty Class.
/// When the object is serialized out as xml, it's qualified name is ax:ocxPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ax:ocxPr")]
pub struct ActiveXObjectProperty {
    /// name
    /// Represents the following attribute in the schema: ax:name
    #[xml(attr = "ax:name")]
    pub name: String,
    /// value
    /// Represents the following attribute in the schema: ax:value
    #[xml(attr = "ax:value")]
    pub value: Option<String>,
    #[xml(child = "ax:font", child = "ax:picture")]
    pub children: Vec<ActiveXObjectPropertyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ActiveXObjectPropertyChildChoice {
    #[xml(tag = "ax:font")]
    AxFont(SharedComFont),
    #[xml(tag = "ax:picture")]
    AxPicture(SharedComPicture),
}
/// Defines the SharedComFont Class.
/// When the object is serialized out as xml, it's qualified name is ax:font.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ax:font")]
pub struct SharedComFont {
    /// persistence
    /// Represents the following attribute in the schema: ax:persistence
    #[xml(attr = "ax:persistence")]
    pub persistence: Option<PersistenceValues>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "ax:ocxPr")]
    pub ax_ocx_pr: Vec<ActiveXObjectProperty>,
}
/// Defines the SharedComPicture Class.
/// When the object is serialized out as xml, it's qualified name is ax:picture.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ax:picture")]
pub struct SharedComPicture {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
