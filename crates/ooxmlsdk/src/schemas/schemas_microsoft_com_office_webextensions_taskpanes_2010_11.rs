/// Defines the Taskpanes Class.
/// When the object is serialized out as xml, it's qualified name is wetp:taskpanes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wetp:taskpanes")]
pub struct Taskpanes {
    #[xml(attr = "xmlns", with = "taskpanes_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "wetp:taskpane")]
    pub wetp_taskpane: Vec<WebExtensionTaskpane>,
}
mod taskpanes_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/webextensions/taskpanes/2010/11")
    }
}
/// Defines the WebExtensionPartReference Class.
/// When the object is serialized out as xml, it's qualified name is wetp:webextensionref.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wetp:webextensionref")]
pub struct WebExtensionPartReference {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is wetp:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wetp:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the WebExtensionTaskpane Class.
/// When the object is serialized out as xml, it's qualified name is wetp:taskpane.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "wetp:taskpane")]
pub struct WebExtensionTaskpane {
    /// dockstate
    /// Represents the following attribute in the schema: :dockstate
    #[xml(attr = "dockstate")]
    pub dock_state: String,
    /// visibility
    /// Represents the following attribute in the schema: :visibility
    #[xml(attr = "visibility")]
    pub visibility: bool,
    /// width
    /// Represents the following attribute in the schema: :width
    #[xml(attr = "width")]
    pub width: f64,
    /// row
    /// Represents the following attribute in the schema: :row
    #[xml(attr = "row")]
    pub row: u32,
    /// locked
    /// Represents the following attribute in the schema: :locked
    #[xml(attr = "locked")]
    pub locked: Option<bool>,
    /// _
    #[xml(child = "wetp:webextensionref")]
    pub web_extension_part_reference: WebExtensionPartReference,
    /// _
    #[xml(child = "wetp:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
