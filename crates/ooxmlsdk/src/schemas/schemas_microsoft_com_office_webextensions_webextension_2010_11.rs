/// Defines the WebExtension Class.
/// When the object is serialized out as xml, it's qualified name is we:webextension.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:webextension")]
pub struct WebExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// frozen
    /// Represents the following attribute in the schema: :frozen
    #[xml(attr = "frozen")]
    pub frozen: Option<bool>,
    /// _
    #[xml(child = "we:reference")]
    pub web_extension_store_reference: WebExtensionStoreReference,
    /// _
    #[xml(child = "we:alternateReferences")]
    pub web_extension_reference_list: Option<WebExtensionReferenceList>,
    /// _
    #[xml(child = "we:properties")]
    pub web_extension_property_bag: WebExtensionPropertyBag,
    /// _
    #[xml(child = "we:bindings")]
    pub web_extension_binding_list: WebExtensionBindingList,
    /// _
    #[xml(child = "we:snapshot")]
    pub snapshot: Option<Snapshot>,
    /// _
    #[xml(child = "we:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReference Class.
/// When the object is serialized out as xml, it's qualified name is we:webextensionref.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:webextensionref")]
pub struct WebExtensionReference {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the WebExtensionProperty Class.
/// When the object is serialized out as xml, it's qualified name is we:property.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:property")]
pub struct WebExtensionProperty {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// value
    /// Represents the following attribute in the schema: :value
    #[xml(attr = "value")]
    pub value: String,
}
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is we:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Defines the WebExtensionBinding Class.
/// When the object is serialized out as xml, it's qualified name is we:binding.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:binding")]
pub struct WebExtensionBinding {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// appref
    /// Represents the following attribute in the schema: :appref
    #[xml(attr = "appref")]
    pub app_reference: String,
    /// _
    #[xml(child = "we:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionStoreReference Class.
/// When the object is serialized out as xml, it's qualified name is we:reference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:reference")]
pub struct WebExtensionStoreReference {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// version
    /// Represents the following attribute in the schema: :version
    #[xml(attr = "version")]
    pub version: String,
    /// store
    /// Represents the following attribute in the schema: :store
    #[xml(attr = "store")]
    pub store: Option<String>,
    /// storeType
    /// Represents the following attribute in the schema: :storeType
    #[xml(attr = "storeType")]
    pub store_type: Option<String>,
    /// _
    #[xml(child = "we:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReferenceList Class.
/// When the object is serialized out as xml, it's qualified name is we:alternateReferences.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:alternateReferences")]
pub struct WebExtensionReferenceList {
    /// _
    #[xml(child = "we:reference")]
    pub we_reference: Vec<WebExtensionStoreReference>,
}
/// Defines the WebExtensionPropertyBag Class.
/// When the object is serialized out as xml, it's qualified name is we:properties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:properties")]
pub struct WebExtensionPropertyBag {
    /// _
    #[xml(child = "we:property")]
    pub we_property: Vec<WebExtensionProperty>,
}
/// Defines the WebExtensionBindingList Class.
/// When the object is serialized out as xml, it's qualified name is we:bindings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:bindings")]
pub struct WebExtensionBindingList {
    /// _
    #[xml(child = "we:binding")]
    pub we_binding: Vec<WebExtensionBinding>,
}
/// Defines the Snapshot Class.
/// When the object is serialized out as xml, it's qualified name is we:snapshot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "we:snapshot")]
pub struct Snapshot {
    /// Embedded Picture Reference
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// Linked Picture Reference
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: Option<String>,
    /// Compression state for blips.
    /// Represents the following attribute in the schema: :cstate
    #[xml(attr = "cstate")]
    pub compression_state: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues,
    >,
    #[xml(
        child = "a:alphaBiLevel",
        child = "a:alphaCeiling",
        child = "a:alphaFloor",
        child = "a:alphaInv",
        child = "a:alphaMod",
        child = "a:alphaModFix",
        child = "a:alphaRepl",
        child = "a:biLevel",
        child = "a:blur",
        child = "a:clrChange",
        child = "a:clrRepl",
        child = "a:duotone",
        child = "a:fillOverlay",
        child = "a:grayscl",
        child = "a:hsl",
        child = "a:lum",
        child = "a:tint",
        child = "a:extLst",
    )]
    pub children: Vec<SnapshotChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SnapshotChildChoice {
    #[xml(tag = "a:alphaBiLevel")]
    AAlphaBiLevel(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaBiLevel,
    ),
    #[xml(tag = "a:alphaCeiling")]
    AAlphaCeiling(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaCeiling,
    ),
    #[xml(tag = "a:alphaFloor")]
    AAlphaFloor(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaFloor,
    ),
    #[xml(tag = "a:alphaInv")]
    AAlphaInv(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaInverse,
    ),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationEffect,
    ),
    #[xml(tag = "a:alphaModFix")]
    AAlphaModFix(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationFixed,
    ),
    #[xml(tag = "a:alphaRepl")]
    AAlphaRepl(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaReplace,
    ),
    #[xml(tag = "a:biLevel")]
    ABiLevel(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BiLevel),
    #[xml(tag = "a:blur")]
    ABlur(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blur),
    #[xml(tag = "a:clrChange")]
    AClrChange(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorChange,
    ),
    #[xml(tag = "a:clrRepl")]
    AClrRepl(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorReplacement,
    ),
    #[xml(tag = "a:duotone")]
    ADuotone(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Duotone),
    #[xml(tag = "a:fillOverlay")]
    AFillOverlay(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillOverlay,
    ),
    #[xml(tag = "a:grayscl")]
    AGrayscl(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Grayscale),
    #[xml(tag = "a:hsl")]
    AHsl(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hsl),
    #[xml(tag = "a:lum")]
    ALum(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceEffect,
    ),
    #[xml(tag = "a:tint")]
    ATint(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TintEffect),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipExtensionList,
    ),
}
