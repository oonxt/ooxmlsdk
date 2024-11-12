/// Application Specific File Properties.
/// When the object is serialized out as xml, it's qualified name is Properties.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Properties")]
pub struct Properties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "Template",
        child = "Manager",
        child = "Company",
        child = "Pages",
        child = "Words",
        child = "Characters",
        child = "PresentationFormat",
        child = "Lines",
        child = "Paragraphs",
        child = "Slides",
        child = "Notes",
        child = "TotalTime",
        child = "HiddenSlides",
        child = "MMClips",
        child = "ScaleCrop",
        child = "HeadingPairs",
        child = "TitlesOfParts",
        child = "LinksUpToDate",
        child = "CharactersWithSpaces",
        child = "SharedDoc",
        child = "HyperlinkBase",
        child = "HLinks",
        child = "HyperlinksChanged",
        child = "DigSig",
        child = "Application",
        child = "AppVersion",
        child = "DocSecurity",
    )]
    pub children: Vec<PropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PropertiesChildChoice {
    #[xml(tag = "Template")]
    ApTemplate(Template),
    #[xml(tag = "Manager")]
    ApManager(Manager),
    #[xml(tag = "Company")]
    ApCompany(Company),
    #[xml(tag = "Pages")]
    ApPages(Pages),
    #[xml(tag = "Words")]
    ApWords(Words),
    #[xml(tag = "Characters")]
    ApCharacters(Characters),
    #[xml(tag = "PresentationFormat")]
    ApPresentationFormat(PresentationFormat),
    #[xml(tag = "Lines")]
    ApLines(Lines),
    #[xml(tag = "Paragraphs")]
    ApParagraphs(Paragraphs),
    #[xml(tag = "Slides")]
    ApSlides(Slides),
    #[xml(tag = "Notes")]
    ApNotes(Notes),
    #[xml(tag = "TotalTime")]
    ApTotalTime(TotalTime),
    #[xml(tag = "HiddenSlides")]
    ApHiddenSlides(HiddenSlides),
    #[xml(tag = "MMClips")]
    ApMmClips(MultimediaClips),
    #[xml(tag = "ScaleCrop")]
    ApScaleCrop(ScaleCrop),
    #[xml(tag = "HeadingPairs")]
    ApHeadingPairs(HeadingPairs),
    #[xml(tag = "TitlesOfParts")]
    ApTitlesOfParts(TitlesOfParts),
    #[xml(tag = "LinksUpToDate")]
    ApLinksUpToDate(LinksUpToDate),
    #[xml(tag = "CharactersWithSpaces")]
    ApCharactersWithSpaces(CharactersWithSpaces),
    #[xml(tag = "SharedDoc")]
    ApSharedDoc(SharedDocument),
    #[xml(tag = "HyperlinkBase")]
    ApHyperlinkBase(HyperlinkBase),
    #[xml(tag = "HLinks")]
    ApHLinks(HyperlinkList),
    #[xml(tag = "HyperlinksChanged")]
    ApHyperlinksChanged(HyperlinksChanged),
    #[xml(tag = "DigSig")]
    ApDigSig(DigitalSignature),
    #[xml(tag = "Application")]
    ApApplication(Application),
    #[xml(tag = "AppVersion")]
    ApAppVersion(ApplicationVersion),
    #[xml(tag = "DocSecurity")]
    ApDocSecurity(DocumentSecurity),
}
/// Name of Document Template.
/// When the object is serialized out as xml, it's qualified name is Template.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Template")]
pub struct Template {
    #[xml(text)]
    pub child: String,
}
/// Name of Manager.
/// When the object is serialized out as xml, it's qualified name is Manager.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Manager")]
pub struct Manager {
    #[xml(text)]
    pub child: String,
}
/// Name of Company.
/// When the object is serialized out as xml, it's qualified name is Company.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Company")]
pub struct Company {
    #[xml(text)]
    pub child: String,
}
/// Intended Format of Presentation.
/// When the object is serialized out as xml, it's qualified name is PresentationFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "PresentationFormat")]
pub struct PresentationFormat {
    #[xml(text)]
    pub child: String,
}
/// Relative Hyperlink Base.
/// When the object is serialized out as xml, it's qualified name is HyperlinkBase.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "HyperlinkBase")]
pub struct HyperlinkBase {
    #[xml(text)]
    pub child: String,
}
/// Application Name.
/// When the object is serialized out as xml, it's qualified name is Application.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Application")]
pub struct Application {
    #[xml(text)]
    pub child: String,
}
/// Application Version.
/// When the object is serialized out as xml, it's qualified name is AppVersion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "AppVersion")]
pub struct ApplicationVersion {
    #[xml(text)]
    pub child: String,
}
/// Total Number of Pages.
/// When the object is serialized out as xml, it's qualified name is Pages.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Pages")]
pub struct Pages {
    #[xml(text)]
    pub child: i32,
}
/// Word Count.
/// When the object is serialized out as xml, it's qualified name is Words.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Words")]
pub struct Words {
    #[xml(text)]
    pub child: i32,
}
/// Total Number of Characters.
/// When the object is serialized out as xml, it's qualified name is Characters.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Characters")]
pub struct Characters {
    #[xml(text)]
    pub child: i32,
}
/// Number of Lines.
/// When the object is serialized out as xml, it's qualified name is Lines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Lines")]
pub struct Lines {
    #[xml(text)]
    pub child: i32,
}
/// Total Number of Paragraphs.
/// When the object is serialized out as xml, it's qualified name is Paragraphs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Paragraphs")]
pub struct Paragraphs {
    #[xml(text)]
    pub child: i32,
}
/// Slides Metadata Element.
/// When the object is serialized out as xml, it's qualified name is Slides.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Slides")]
pub struct Slides {
    #[xml(text)]
    pub child: i32,
}
/// Number of Slides Containing Notes.
/// When the object is serialized out as xml, it's qualified name is Notes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "Notes")]
pub struct Notes {
    #[xml(text)]
    pub child: i32,
}
/// Total Edit Time Metadata Element.
/// When the object is serialized out as xml, it's qualified name is TotalTime.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "TotalTime")]
pub struct TotalTime {
    #[xml(text)]
    pub child: i32,
}
/// Number of Hidden Slides.
/// When the object is serialized out as xml, it's qualified name is HiddenSlides.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "HiddenSlides")]
pub struct HiddenSlides {
    #[xml(text)]
    pub child: i32,
}
/// Total Number of Multimedia Clips.
/// When the object is serialized out as xml, it's qualified name is MMClips.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "MMClips")]
pub struct MultimediaClips {
    #[xml(text)]
    pub child: i32,
}
/// Number of Characters (With Spaces).
/// When the object is serialized out as xml, it's qualified name is CharactersWithSpaces.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "CharactersWithSpaces")]
pub struct CharactersWithSpaces {
    #[xml(text)]
    pub child: i32,
}
/// Document Security.
/// When the object is serialized out as xml, it's qualified name is DocSecurity.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "DocSecurity")]
pub struct DocumentSecurity {
    #[xml(text)]
    pub child: i32,
}
/// Thumbnail Display Mode.
/// When the object is serialized out as xml, it's qualified name is ScaleCrop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "ScaleCrop")]
pub struct ScaleCrop {
    #[xml(text)]
    pub child: bool,
}
/// Links Up-to-Date.
/// When the object is serialized out as xml, it's qualified name is LinksUpToDate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "LinksUpToDate")]
pub struct LinksUpToDate {
    #[xml(text)]
    pub child: bool,
}
/// Shared Document.
/// When the object is serialized out as xml, it's qualified name is SharedDoc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "SharedDoc")]
pub struct SharedDocument {
    #[xml(text)]
    pub child: bool,
}
/// Hyperlinks Changed.
/// When the object is serialized out as xml, it's qualified name is HyperlinksChanged.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "HyperlinksChanged")]
pub struct HyperlinksChanged {
    #[xml(text)]
    pub child: bool,
}
/// Heading Pairs.
/// When the object is serialized out as xml, it's qualified name is HeadingPairs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "HeadingPairs")]
pub struct HeadingPairs {
    ///Vector
    #[xml(child = "vt:vector")]
    pub vt_vector: crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
}
/// Hyperlink List.
/// When the object is serialized out as xml, it's qualified name is HLinks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "HLinks")]
pub struct HyperlinkList {
    ///Vector
    #[xml(child = "vt:vector")]
    pub vt_vector: crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
}
/// Defines the VectorVariantType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct VectorVariantType {}
/// Part Titles.
/// When the object is serialized out as xml, it's qualified name is TitlesOfParts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "TitlesOfParts")]
pub struct TitlesOfParts {
    ///Vector
    #[xml(child = "vt:vector")]
    pub vt_vector: crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
}
/// Digital Signature.
/// When the object is serialized out as xml, it's qualified name is DigSig.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "DigSig")]
pub struct DigitalSignature {
    ///Binary Blob
    #[xml(child = "vt:blob")]
    pub vt_blob: crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtBlob,
}
