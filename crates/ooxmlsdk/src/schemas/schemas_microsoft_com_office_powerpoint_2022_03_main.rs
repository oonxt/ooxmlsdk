/// Defines the Reactions Class.
/// When the object is serialized out as xml, it's qualified name is p223:reactions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p223:reactions")]
pub struct Reactions {
    /// _
    #[xml(child = "p223:rxn")]
    pub p223_rxn: Vec<Reaction>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p223:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p223:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
/// Defines the ReactionInstance Class.
/// When the object is serialized out as xml, it's qualified name is p223:instance.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p223:instance")]
pub struct ReactionInstance {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: String,
    /// _
    #[xml(child = "p223:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Reaction Class.
/// When the object is serialized out as xml, it's qualified name is p223:rxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p223:rxn")]
pub struct Reaction {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// _
    #[xml(child = "p223:instance")]
    pub p223_instance: Vec<ReactionInstance>,
}
