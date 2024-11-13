#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SdtAppearance {
    #[default]
    BoundingBox,
    Tags,
    Hidden,
}
crate::__string_enum! {
    SdtAppearance { BoundingBox = "boundingBox", Tags = "tags", Hidden = "hidden", }
}
/// Defines the Color Class.
/// When the object is serialized out as xml, it's qualified name is w15:color.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:color")]
pub struct Color {
    /// Run Content Color
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
    /// Run Content Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ThemeColorValues,
    >,
    /// Run Content Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Run Content Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
}
/// Defines the DataBinding Class.
/// When the object is serialized out as xml, it's qualified name is w15:dataBinding.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:dataBinding")]
pub struct DataBinding {
    /// XML Namespace Prefix Mappings
    /// Represents the following attribute in the schema: w:prefixMappings
    #[xml(attr = "w:prefixMappings")]
    pub prefix_mappings: Option<String>,
    /// XPath
    /// Represents the following attribute in the schema: w:xpath
    #[xml(attr = "w:xpath")]
    pub x_path: String,
    /// Custom XML Data Storage ID
    /// Represents the following attribute in the schema: w:storeItemID
    #[xml(attr = "w:storeItemID")]
    pub store_item_id: String,
}
/// Defines the Appearance Class.
/// When the object is serialized out as xml, it's qualified name is w15:appearance.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:appearance")]
pub struct Appearance {
    /// val
    /// Represents the following attribute in the schema: w15:val
    #[xml(attr = "w15:val")]
    pub val: Option<SdtAppearance>,
}
/// Defines the CommentsEx Class.
/// When the object is serialized out as xml, it's qualified name is w15:commentsEx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:commentsEx")]
pub struct CommentsEx {
    #[xml(attr = "xmlns", with = "comments_ex_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w15:commentEx")]
    pub w15_comment_ex: Vec<CommentEx>,
}
mod comments_ex_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/word/2012/wordml")
    }
}
/// Defines the People Class.
/// When the object is serialized out as xml, it's qualified name is w15:people.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:people")]
pub struct People {
    #[xml(attr = "xmlns", with = "people_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w15:person")]
    pub w15_person: Vec<Person>,
}
mod people_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.microsoft.com/office/word/2012/wordml")
    }
}
/// Defines the SdtRepeatedSection Class.
/// When the object is serialized out as xml, it's qualified name is w15:repeatingSection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:repeatingSection")]
pub struct SdtRepeatedSection {
    /// _
    #[xml(child = "w15:sectionTitle")]
    pub section_title: Option<SectionTitle>,
    /// _
    #[xml(child = "w15:doNotAllowInsertDeleteSection")]
    pub do_not_allow_insert_delete_section: Option<DoNotAllowInsertDeleteSection>,
}
/// Defines the SdtRepeatedSectionItem Class.
/// When the object is serialized out as xml, it's qualified name is w15:repeatingSectionItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:repeatingSectionItem")]
pub struct SdtRepeatedSectionItem {}
/// Defines the ChartTrackingRefBased Class.
/// When the object is serialized out as xml, it's qualified name is w15:chartTrackingRefBased.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:chartTrackingRefBased")]
pub struct ChartTrackingRefBased {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DefaultCollapsed Class.
/// When the object is serialized out as xml, it's qualified name is w15:collapsed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:collapsed")]
pub struct DefaultCollapsed {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the WebExtensionLinked Class.
/// When the object is serialized out as xml, it's qualified name is w15:webExtensionLinked.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:webExtensionLinked")]
pub struct WebExtensionLinked {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the WebExtensionCreated Class.
/// When the object is serialized out as xml, it's qualified name is w15:webExtensionCreated.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:webExtensionCreated")]
pub struct WebExtensionCreated {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DoNotAllowInsertDeleteSection Class.
/// When the object is serialized out as xml, it's qualified name is w15:doNotAllowInsertDeleteSection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:doNotAllowInsertDeleteSection")]
pub struct DoNotAllowInsertDeleteSection {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the OnOffType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OnOffType {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the PersistentDocumentId Class.
/// When the object is serialized out as xml, it's qualified name is w15:docId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:docId")]
pub struct PersistentDocumentId {
    /// val
    /// Represents the following attribute in the schema: w15:val
    #[xml(attr = "w15:val")]
    pub w15_val: Option<String>,
}
/// Defines the FootnoteColumns Class.
/// When the object is serialized out as xml, it's qualified name is w15:footnoteColumns.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:footnoteColumns")]
pub struct FootnoteColumns {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the CommentEx Class.
/// When the object is serialized out as xml, it's qualified name is w15:commentEx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:commentEx")]
pub struct CommentEx {
    /// paraId
    /// Represents the following attribute in the schema: w15:paraId
    #[xml(attr = "w15:paraId")]
    pub para_id: String,
    /// paraIdParent
    /// Represents the following attribute in the schema: w15:paraIdParent
    #[xml(attr = "w15:paraIdParent")]
    pub para_id_parent: Option<String>,
    /// done
    /// Represents the following attribute in the schema: w15:done
    #[xml(attr = "w15:done")]
    pub done: Option<bool>,
}
/// Defines the Person Class.
/// When the object is serialized out as xml, it's qualified name is w15:person.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:person")]
pub struct Person {
    /// author
    /// Represents the following attribute in the schema: w15:author
    #[xml(attr = "w15:author")]
    pub author: String,
    /// _
    #[xml(child = "w15:presenceInfo")]
    pub presence_info: Option<PresenceInfo>,
}
/// Defines the PresenceInfo Class.
/// When the object is serialized out as xml, it's qualified name is w15:presenceInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:presenceInfo")]
pub struct PresenceInfo {
    /// providerId
    /// Represents the following attribute in the schema: w15:providerId
    #[xml(attr = "w15:providerId")]
    pub w15_provider_id: String,
    /// userId
    /// Represents the following attribute in the schema: w15:userId
    #[xml(attr = "w15:userId")]
    pub w15_user_id: String,
}
/// Defines the SectionTitle Class.
/// When the object is serialized out as xml, it's qualified name is w15:sectionTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w15:sectionTitle")]
pub struct SectionTitle {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
