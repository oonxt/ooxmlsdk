/// Defines the CommentAuthorMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:cmAuthorMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:cmAuthorMkLst")]
pub struct CommentAuthorMonikerList {}
/// Defines the CommentMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:cmMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:cmMkLst")]
pub struct CommentMonikerList {}
/// Defines the StringTagMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:tagMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:tagMkLst")]
pub struct StringTagMonikerList {}
/// Defines the CustomShowMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:custShowMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:custShowMkLst")]
pub struct CustomShowMonikerList {}
/// Defines the DocumentMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:docMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:docMkLst")]
pub struct DocumentMonikerList {}
/// Defines the SectionMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sectionMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sectionMkLst")]
pub struct SectionMonikerList {}
/// Defines the SlideBaseMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sldBaseMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sldBaseMkLst")]
pub struct SlideBaseMonikerList {}
/// Defines the SlideLayoutMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sldLayoutMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sldLayoutMkLst")]
pub struct SlideLayoutMonikerList {}
/// Defines the MainMasterMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sldMasterMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sldMasterMkLst")]
pub struct MainMasterMonikerList {}
/// Defines the SlideMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sldMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sldMkLst")]
pub struct SlideMonikerList {
    #[xml(child = "pc:docMk", child = "pc:sldMk")]
    pub children: Vec<SlideMonikerListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideMonikerListChildChoice {
    #[xml(tag = "pc:docMk")]
    PcDocMk(DocumentMoniker),
    #[xml(tag = "pc:sldMk")]
    PcSldMk(SlideMoniker),
}
/// Defines the SlidePosMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sldPosMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sldPosMkLst")]
pub struct SlidePosMonikerList {}
/// Defines the NotesMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:notesMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:notesMkLst")]
pub struct NotesMonikerList {}
/// Defines the NotesTextMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:notesTxtMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:notesTxtMkLst")]
pub struct NotesTextMonikerList {}
/// Defines the NotesMasterMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:notesMasterMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:notesMasterMkLst")]
pub struct NotesMasterMonikerList {}
/// Defines the HandoutMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:handoutMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:handoutMkLst")]
pub struct HandoutMonikerList {}
/// Defines the AnimEffectMkLstAnimationEffectMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:animEffectMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:animEffectMkLst")]
pub struct AnimEffectMkLstAnimationEffectMonikerList {}
/// Defines the AnimEffectParentMkLstAnimationEffectMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:animEffectParentMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:animEffectParentMkLst")]
pub struct AnimEffectParentMkLstAnimationEffectMonikerList {}
/// Defines the OpenXmlAnimationEffectMonikerListElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlAnimationEffectMonikerListElement {}
/// Defines the OsfTaskPaneAppMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:tkAppMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:tkAppMkLst")]
pub struct OsfTaskPaneAppMonikerList {}
/// Defines the SummaryZoomMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:tocMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:tocMkLst")]
pub struct SummaryZoomMonikerList {}
/// Defines the SectionLinkObjMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:sectionLnkObjMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sectionLnkObjMkLst")]
pub struct SectionLinkObjMonikerList {}
/// Defines the DesignerTagMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:designTagMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:designTagMkLst")]
pub struct DesignerTagMonikerList {}
/// Defines the CustomXmlPartMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is pc:cXmlMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:cXmlMkLst")]
pub struct CustomXmlPartMonikerList {}
/// Defines the DocumentMoniker Class.
/// When the object is serialized out as xml, it's qualified name is pc:docMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:docMk")]
pub struct DocumentMoniker {}
/// Defines the SlideMoniker Class.
/// When the object is serialized out as xml, it's qualified name is pc:sldMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "pc:sldMk")]
pub struct SlideMoniker {
    /// cId
    /// Represents the following attribute in the schema: :cId
    #[xml(attr = "cId")]
    pub c_id: Option<i32>,
    /// sldId
    /// Represents the following attribute in the schema: :sldId
    #[xml(attr = "sldId")]
    pub sld_id: i32,
}
