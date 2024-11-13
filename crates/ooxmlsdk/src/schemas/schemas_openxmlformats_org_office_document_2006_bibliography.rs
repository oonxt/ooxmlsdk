#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DataSourceValues {
    #[default]
    ArticleInAPeriodical,
    Book,
    BookSection,
    JournalArticle,
    ConferenceProceedings,
    Report,
    SoundRecording,
    Performance,
    Art,
    DocumentFromInternetSite,
    InternetSite,
    Film,
    Interview,
    Patent,
    ElectronicSource,
    Case,
    Miscellaneous,
}
crate::__string_enum! {
    DataSourceValues { ArticleInAPeriodical = "articleInAPeriodical", Book = "book",
    BookSection = "bookSection", JournalArticle = "journalArticle", ConferenceProceedings
    = "conferenceProceedings", Report = "report", SoundRecording = "soundRecording",
    Performance = "performance", Art = "art", DocumentFromInternetSite =
    "documentFromInternetSite", InternetSite = "internetSite", Film = "film", Interview =
    "interview", Patent = "patent", ElectronicSource = "electronicSource", Case = "case",
    Miscellaneous = "misc", }
}
/// Sources.
/// When the object is serialized out as xml, it's qualified name is b:Sources.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Sources")]
pub struct Sources {
    #[xml(attr = "xmlns", with = "sources_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Selected Style
    /// Represents the following attribute in the schema: :SelectedStyle
    #[xml(attr = "SelectedStyle")]
    pub selected_style: Option<String>,
    /// Documentation Style Name
    /// Represents the following attribute in the schema: :StyleName
    #[xml(attr = "StyleName")]
    pub style_name: Option<String>,
    /// Uniform Resource Identifier
    /// Represents the following attribute in the schema: :URI
    #[xml(attr = "URI")]
    pub uri: Option<String>,
    /// _
    #[xml(child = "b:Source")]
    pub b_source: Vec<Source>,
}
mod sources_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/officeDocument/2006/bibliography")
    }
}
/// Person.
/// When the object is serialized out as xml, it's qualified name is b:Person.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Person")]
pub struct Person {
    /// _
    #[xml(child = "b:Last")]
    pub b_last: Vec<Last>,
    /// _
    #[xml(child = "b:First")]
    pub b_first: Vec<First>,
    /// _
    #[xml(child = "b:Middle")]
    pub b_middle: Vec<Middle>,
}
/// Person's Last, or Family, Name.
/// When the object is serialized out as xml, it's qualified name is b:Last.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Last")]
pub struct Last {
    #[xml(text)]
    pub child: String,
}
/// Person's First, or Given, Name.
/// When the object is serialized out as xml, it's qualified name is b:First.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:First")]
pub struct First {
    #[xml(text)]
    pub child: String,
}
/// Person's Middle, or Other, Name.
/// When the object is serialized out as xml, it's qualified name is b:Middle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Middle")]
pub struct Middle {
    #[xml(text)]
    pub child: String,
}
/// Corporate Author.
/// When the object is serialized out as xml, it's qualified name is b:Corporate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Corporate")]
pub struct Corporate {
    #[xml(text)]
    pub child: String,
}
/// Abbreviated Case Number.
/// When the object is serialized out as xml, it's qualified name is b:AbbreviatedCaseNumber.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:AbbreviatedCaseNumber")]
pub struct AbbreviatedCaseNumber {
    #[xml(text)]
    pub child: String,
}
/// Album Title.
/// When the object is serialized out as xml, it's qualified name is b:AlbumTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:AlbumTitle")]
pub struct AlbumTitle {
    #[xml(text)]
    pub child: String,
}
/// Book Title.
/// When the object is serialized out as xml, it's qualified name is b:BookTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:BookTitle")]
pub struct BookTitle {
    #[xml(text)]
    pub child: String,
}
/// Broadcaster.
/// When the object is serialized out as xml, it's qualified name is b:Broadcaster.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Broadcaster")]
pub struct Broadcaster {
    #[xml(text)]
    pub child: String,
}
/// Broadcast Title.
/// When the object is serialized out as xml, it's qualified name is b:BroadcastTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:BroadcastTitle")]
pub struct BroadcastTitle {
    #[xml(text)]
    pub child: String,
}
/// Case Number.
/// When the object is serialized out as xml, it's qualified name is b:CaseNumber.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:CaseNumber")]
pub struct CaseNumber {
    #[xml(text)]
    pub child: String,
}
/// Chapter Number.
/// When the object is serialized out as xml, it's qualified name is b:ChapterNumber.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:ChapterNumber")]
pub struct ChapterNumber {
    #[xml(text)]
    pub child: String,
}
/// City.
/// When the object is serialized out as xml, it's qualified name is b:City.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:City")]
pub struct City {
    #[xml(text)]
    pub child: String,
}
/// Comments.
/// When the object is serialized out as xml, it's qualified name is b:Comments.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Comments")]
pub struct Comments {
    #[xml(text)]
    pub child: String,
}
/// Conference or Proceedings Name.
/// When the object is serialized out as xml, it's qualified name is b:ConferenceName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:ConferenceName")]
pub struct ConferenceName {
    #[xml(text)]
    pub child: String,
}
/// Country or Region.
/// When the object is serialized out as xml, it's qualified name is b:CountryRegion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:CountryRegion")]
pub struct CountryRegion {
    #[xml(text)]
    pub child: String,
}
/// Court.
/// When the object is serialized out as xml, it's qualified name is b:Court.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Court")]
pub struct Court {
    #[xml(text)]
    pub child: String,
}
/// Day.
/// When the object is serialized out as xml, it's qualified name is b:Day.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Day")]
pub struct Day {
    #[xml(text)]
    pub child: String,
}
/// Day Accessed.
/// When the object is serialized out as xml, it's qualified name is b:DayAccessed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:DayAccessed")]
pub struct DayAccessed {
    #[xml(text)]
    pub child: String,
}
/// Department.
/// When the object is serialized out as xml, it's qualified name is b:Department.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Department")]
pub struct Department {
    #[xml(text)]
    pub child: String,
}
/// Distributor.
/// When the object is serialized out as xml, it's qualified name is b:Distributor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Distributor")]
pub struct Distributor {
    #[xml(text)]
    pub child: String,
}
/// Editor.
/// When the object is serialized out as xml, it's qualified name is b:Edition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Edition")]
pub struct Edition {
    #[xml(text)]
    pub child: String,
}
/// GUID.
/// When the object is serialized out as xml, it's qualified name is b:Guid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Guid")]
pub struct GuidString {
    #[xml(text)]
    pub child: String,
}
/// Institution.
/// When the object is serialized out as xml, it's qualified name is b:Institution.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Institution")]
pub struct Institution {
    #[xml(text)]
    pub child: String,
}
/// Internet Site Title.
/// When the object is serialized out as xml, it's qualified name is b:InternetSiteTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:InternetSiteTitle")]
pub struct InternetSiteTitle {
    #[xml(text)]
    pub child: String,
}
/// Issue.
/// When the object is serialized out as xml, it's qualified name is b:Issue.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Issue")]
pub struct Issue {
    #[xml(text)]
    pub child: String,
}
/// Journal Name.
/// When the object is serialized out as xml, it's qualified name is b:JournalName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:JournalName")]
pub struct JournalName {
    #[xml(text)]
    pub child: String,
}
/// Locale ID.
/// When the object is serialized out as xml, it's qualified name is b:LCID.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:LCID")]
pub struct LcId {
    #[xml(text)]
    pub child: String,
}
/// Medium.
/// When the object is serialized out as xml, it's qualified name is b:Medium.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Medium")]
pub struct Medium {
    #[xml(text)]
    pub child: String,
}
/// Month.
/// When the object is serialized out as xml, it's qualified name is b:Month.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Month")]
pub struct Month {
    #[xml(text)]
    pub child: String,
}
/// Month Accessed.
/// When the object is serialized out as xml, it's qualified name is b:MonthAccessed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:MonthAccessed")]
pub struct MonthAccessed {
    #[xml(text)]
    pub child: String,
}
/// Number of Volumes.
/// When the object is serialized out as xml, it's qualified name is b:NumberVolumes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:NumberVolumes")]
pub struct NumberVolumes {
    #[xml(text)]
    pub child: String,
}
/// Pages.
/// When the object is serialized out as xml, it's qualified name is b:Pages.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Pages")]
pub struct Pages {
    #[xml(text)]
    pub child: String,
}
/// Patent Number.
/// When the object is serialized out as xml, it's qualified name is b:PatentNumber.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:PatentNumber")]
pub struct PatentNumber {
    #[xml(text)]
    pub child: String,
}
/// Periodical Title.
/// When the object is serialized out as xml, it's qualified name is b:PeriodicalTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:PeriodicalTitle")]
pub struct PeriodicalTitle {
    #[xml(text)]
    pub child: String,
}
/// Production Company.
/// When the object is serialized out as xml, it's qualified name is b:ProductionCompany.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:ProductionCompany")]
pub struct ProductionCompany {
    #[xml(text)]
    pub child: String,
}
/// Publication Title.
/// When the object is serialized out as xml, it's qualified name is b:PublicationTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:PublicationTitle")]
pub struct PublicationTitle {
    #[xml(text)]
    pub child: String,
}
/// Publisher.
/// When the object is serialized out as xml, it's qualified name is b:Publisher.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Publisher")]
pub struct Publisher {
    #[xml(text)]
    pub child: String,
}
/// Recording Number.
/// When the object is serialized out as xml, it's qualified name is b:RecordingNumber.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:RecordingNumber")]
pub struct RecordingNumber {
    #[xml(text)]
    pub child: String,
}
/// Reference Order.
/// When the object is serialized out as xml, it's qualified name is b:RefOrder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:RefOrder")]
pub struct ReferenceOrder {
    #[xml(text)]
    pub child: String,
}
/// Reporter.
/// When the object is serialized out as xml, it's qualified name is b:Reporter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Reporter")]
pub struct Reporter {
    #[xml(text)]
    pub child: String,
}
/// Short Title.
/// When the object is serialized out as xml, it's qualified name is b:ShortTitle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:ShortTitle")]
pub struct ShortTitle {
    #[xml(text)]
    pub child: String,
}
/// Standard Number.
/// When the object is serialized out as xml, it's qualified name is b:StandardNumber.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:StandardNumber")]
pub struct StandardNumber {
    #[xml(text)]
    pub child: String,
}
/// State or Province.
/// When the object is serialized out as xml, it's qualified name is b:StateProvince.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:StateProvince")]
pub struct StateProvince {
    #[xml(text)]
    pub child: String,
}
/// Station.
/// When the object is serialized out as xml, it's qualified name is b:Station.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Station")]
pub struct Station {
    #[xml(text)]
    pub child: String,
}
/// Tag.
/// When the object is serialized out as xml, it's qualified name is b:Tag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Tag")]
pub struct Tag {
    #[xml(text)]
    pub child: String,
}
/// Theater.
/// When the object is serialized out as xml, it's qualified name is b:Theater.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Theater")]
pub struct Theater {
    #[xml(text)]
    pub child: String,
}
/// Thesis Type.
/// When the object is serialized out as xml, it's qualified name is b:ThesisType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:ThesisType")]
pub struct ThesisType {
    #[xml(text)]
    pub child: String,
}
/// Title.
/// When the object is serialized out as xml, it's qualified name is b:Title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Title")]
pub struct Title {
    #[xml(text)]
    pub child: String,
}
/// Type.
/// When the object is serialized out as xml, it's qualified name is b:Type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Type")]
pub struct PatentType {
    #[xml(text)]
    pub child: String,
}
/// URL.
/// When the object is serialized out as xml, it's qualified name is b:URL.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:URL")]
pub struct UrlString {
    #[xml(text)]
    pub child: String,
}
/// Version.
/// When the object is serialized out as xml, it's qualified name is b:Version.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Version")]
pub struct Version {
    #[xml(text)]
    pub child: String,
}
/// Volume.
/// When the object is serialized out as xml, it's qualified name is b:Volume.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Volume")]
pub struct Volume {
    #[xml(text)]
    pub child: String,
}
/// Year.
/// When the object is serialized out as xml, it's qualified name is b:Year.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Year")]
pub struct Year {
    #[xml(text)]
    pub child: String,
}
/// Year Accessed.
/// When the object is serialized out as xml, it's qualified name is b:YearAccessed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:YearAccessed")]
pub struct YearAccessed {
    #[xml(text)]
    pub child: String,
}
/// Name List.
/// When the object is serialized out as xml, it's qualified name is b:NameList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:NameList")]
pub struct NameList {
    /// _
    #[xml(child = "b:Person")]
    pub b_person: Vec<Person>,
}
/// Artist.
/// When the object is serialized out as xml, it's qualified name is b:Artist.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Artist")]
pub struct Artist {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Book Author.
/// When the object is serialized out as xml, it's qualified name is b:BookAuthor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:BookAuthor")]
pub struct BookAuthor {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Compiler.
/// When the object is serialized out as xml, it's qualified name is b:Compiler.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Compiler")]
pub struct Compiler {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Composer.
/// When the object is serialized out as xml, it's qualified name is b:Composer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Composer")]
pub struct Composer {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Conductor.
/// When the object is serialized out as xml, it's qualified name is b:Conductor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Conductor")]
pub struct Conductor {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Counsel.
/// When the object is serialized out as xml, it's qualified name is b:Counsel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Counsel")]
pub struct Counsel {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Director.
/// When the object is serialized out as xml, it's qualified name is b:Director.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Director")]
pub struct Director {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Editor.
/// When the object is serialized out as xml, it's qualified name is b:Editor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Editor")]
pub struct Editor {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Interviewee.
/// When the object is serialized out as xml, it's qualified name is b:Interviewee.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Interviewee")]
pub struct Interviewee {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Interviewer.
/// When the object is serialized out as xml, it's qualified name is b:Interviewer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Interviewer")]
pub struct Interviewer {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Inventor.
/// When the object is serialized out as xml, it's qualified name is b:Inventor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Inventor")]
pub struct Inventor {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Producer Name.
/// When the object is serialized out as xml, it's qualified name is b:ProducerName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:ProducerName")]
pub struct ProducerName {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Translator.
/// When the object is serialized out as xml, it's qualified name is b:Translator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Translator")]
pub struct Translator {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Writer.
/// When the object is serialized out as xml, it's qualified name is b:Writer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Writer")]
pub struct Writer {
    ///Name List
    #[xml(child = "b:NameList")]
    pub name_list: NameList,
}
/// Defines the NameType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NameType {}
/// Author.
/// When the object is serialized out as xml, it's qualified name is b:Author.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Author")]
pub struct Author {
    #[xml(child = "b:NameList", child = "b:Corporate")]
    pub children: Vec<AuthorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AuthorChildChoice {
    #[xml(tag = "b:NameList")]
    BNameList(NameList),
    #[xml(tag = "b:Corporate")]
    BCorporate(Corporate),
}
/// Performer.
/// When the object is serialized out as xml, it's qualified name is b:Performer.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Performer")]
pub struct Performer {
    #[xml(child = "b:NameList", child = "b:Corporate")]
    pub children: Vec<PerformerChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PerformerChildChoice {
    #[xml(tag = "b:NameList")]
    BNameList(NameList),
    #[xml(tag = "b:Corporate")]
    BCorporate(Corporate),
}
/// Defines the NameOrCorporateType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NameOrCorporateType {
    #[xml(child = "b:NameList", child = "b:Corporate")]
    pub children: Vec<NameOrCorporateTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NameOrCorporateTypeChildChoice {
    #[xml(tag = "b:NameList")]
    BNameList(NameList),
    #[xml(tag = "b:Corporate")]
    BCorporate(Corporate),
}
/// Contributors List.
/// When the object is serialized out as xml, it's qualified name is b:Author.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Author")]
pub struct AuthorList {
    #[xml(
        child = "b:Artist",
        child = "b:Author",
        child = "b:BookAuthor",
        child = "b:Compiler",
        child = "b:Composer",
        child = "b:Conductor",
        child = "b:Counsel",
        child = "b:Director",
        child = "b:Editor",
        child = "b:Interviewee",
        child = "b:Interviewer",
        child = "b:Inventor",
        child = "b:Performer",
        child = "b:ProducerName",
        child = "b:Translator",
        child = "b:Writer",
    )]
    pub children: Vec<AuthorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AuthorListChildChoice {
    #[xml(tag = "b:Artist")]
    BArtist(Artist),
    #[xml(tag = "b:Author")]
    BAuthor(Author),
    #[xml(tag = "b:BookAuthor")]
    BBookAuthor(BookAuthor),
    #[xml(tag = "b:Compiler")]
    BCompiler(Compiler),
    #[xml(tag = "b:Composer")]
    BComposer(Composer),
    #[xml(tag = "b:Conductor")]
    BConductor(Conductor),
    #[xml(tag = "b:Counsel")]
    BCounsel(Counsel),
    #[xml(tag = "b:Director")]
    BDirector(Director),
    #[xml(tag = "b:Editor")]
    BEditor(Editor),
    #[xml(tag = "b:Interviewee")]
    BInterviewee(Interviewee),
    #[xml(tag = "b:Interviewer")]
    BInterviewer(Interviewer),
    #[xml(tag = "b:Inventor")]
    BInventor(Inventor),
    #[xml(tag = "b:Performer")]
    BPerformer(Performer),
    #[xml(tag = "b:ProducerName")]
    BProducerName(ProducerName),
    #[xml(tag = "b:Translator")]
    BTranslator(Translator),
    #[xml(tag = "b:Writer")]
    BWriter(Writer),
}
/// Source Type.
/// When the object is serialized out as xml, it's qualified name is b:SourceType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:SourceType")]
pub struct SourceType {
    #[xml(attr = "b:SourceType")]
    pub child: Option<DataSourceValues>,
}
/// Source.
/// When the object is serialized out as xml, it's qualified name is b:Source.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "b:Source")]
pub struct Source {
    #[xml(
        child = "b:AbbreviatedCaseNumber",
        child = "b:AlbumTitle",
        child = "b:Author",
        child = "b:BookTitle",
        child = "b:Broadcaster",
        child = "b:BroadcastTitle",
        child = "b:CaseNumber",
        child = "b:ChapterNumber",
        child = "b:City",
        child = "b:Comments",
        child = "b:ConferenceName",
        child = "b:CountryRegion",
        child = "b:Court",
        child = "b:Day",
        child = "b:DayAccessed",
        child = "b:Department",
        child = "b:Distributor",
        child = "b:Edition",
        child = "b:Guid",
        child = "b:Institution",
        child = "b:InternetSiteTitle",
        child = "b:Issue",
        child = "b:JournalName",
        child = "b:LCID",
        child = "b:Medium",
        child = "b:Month",
        child = "b:MonthAccessed",
        child = "b:NumberVolumes",
        child = "b:Pages",
        child = "b:PatentNumber",
        child = "b:PeriodicalTitle",
        child = "b:ProductionCompany",
        child = "b:PublicationTitle",
        child = "b:Publisher",
        child = "b:RecordingNumber",
        child = "b:RefOrder",
        child = "b:Reporter",
        child = "b:SourceType",
        child = "b:ShortTitle",
        child = "b:StandardNumber",
        child = "b:StateProvince",
        child = "b:Station",
        child = "b:Tag",
        child = "b:Theater",
        child = "b:ThesisType",
        child = "b:Title",
        child = "b:Type",
        child = "b:URL",
        child = "b:Version",
        child = "b:Volume",
        child = "b:Year",
        child = "b:YearAccessed",
    )]
    pub children: Vec<SourceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SourceChildChoice {
    #[xml(tag = "b:AbbreviatedCaseNumber")]
    BAbbreviatedCaseNumber(AbbreviatedCaseNumber),
    #[xml(tag = "b:AlbumTitle")]
    BAlbumTitle(AlbumTitle),
    #[xml(tag = "b:Author")]
    BAuthor(AuthorList),
    #[xml(tag = "b:BookTitle")]
    BBookTitle(BookTitle),
    #[xml(tag = "b:Broadcaster")]
    BBroadcaster(Broadcaster),
    #[xml(tag = "b:BroadcastTitle")]
    BBroadcastTitle(BroadcastTitle),
    #[xml(tag = "b:CaseNumber")]
    BCaseNumber(CaseNumber),
    #[xml(tag = "b:ChapterNumber")]
    BChapterNumber(ChapterNumber),
    #[xml(tag = "b:City")]
    BCity(City),
    #[xml(tag = "b:Comments")]
    BComments(Comments),
    #[xml(tag = "b:ConferenceName")]
    BConferenceName(ConferenceName),
    #[xml(tag = "b:CountryRegion")]
    BCountryRegion(CountryRegion),
    #[xml(tag = "b:Court")]
    BCourt(Court),
    #[xml(tag = "b:Day")]
    BDay(Day),
    #[xml(tag = "b:DayAccessed")]
    BDayAccessed(DayAccessed),
    #[xml(tag = "b:Department")]
    BDepartment(Department),
    #[xml(tag = "b:Distributor")]
    BDistributor(Distributor),
    #[xml(tag = "b:Edition")]
    BEdition(Edition),
    #[xml(tag = "b:Guid")]
    BGuid(GuidString),
    #[xml(tag = "b:Institution")]
    BInstitution(Institution),
    #[xml(tag = "b:InternetSiteTitle")]
    BInternetSiteTitle(InternetSiteTitle),
    #[xml(tag = "b:Issue")]
    BIssue(Issue),
    #[xml(tag = "b:JournalName")]
    BJournalName(JournalName),
    #[xml(tag = "b:LCID")]
    BLcid(LcId),
    #[xml(tag = "b:Medium")]
    BMedium(Medium),
    #[xml(tag = "b:Month")]
    BMonth(Month),
    #[xml(tag = "b:MonthAccessed")]
    BMonthAccessed(MonthAccessed),
    #[xml(tag = "b:NumberVolumes")]
    BNumberVolumes(NumberVolumes),
    #[xml(tag = "b:Pages")]
    BPages(Pages),
    #[xml(tag = "b:PatentNumber")]
    BPatentNumber(PatentNumber),
    #[xml(tag = "b:PeriodicalTitle")]
    BPeriodicalTitle(PeriodicalTitle),
    #[xml(tag = "b:ProductionCompany")]
    BProductionCompany(ProductionCompany),
    #[xml(tag = "b:PublicationTitle")]
    BPublicationTitle(PublicationTitle),
    #[xml(tag = "b:Publisher")]
    BPublisher(Publisher),
    #[xml(tag = "b:RecordingNumber")]
    BRecordingNumber(RecordingNumber),
    #[xml(tag = "b:RefOrder")]
    BRefOrder(ReferenceOrder),
    #[xml(tag = "b:Reporter")]
    BReporter(Reporter),
    #[xml(tag = "b:SourceType")]
    BSourceType(SourceType),
    #[xml(tag = "b:ShortTitle")]
    BShortTitle(ShortTitle),
    #[xml(tag = "b:StandardNumber")]
    BStandardNumber(StandardNumber),
    #[xml(tag = "b:StateProvince")]
    BStateProvince(StateProvince),
    #[xml(tag = "b:Station")]
    BStation(Station),
    #[xml(tag = "b:Tag")]
    BTag(Tag),
    #[xml(tag = "b:Theater")]
    BTheater(Theater),
    #[xml(tag = "b:ThesisType")]
    BThesisType(ThesisType),
    #[xml(tag = "b:Title")]
    BTitle(Title),
    #[xml(tag = "b:Type")]
    BType(PatentType),
    #[xml(tag = "b:URL")]
    BUrl(UrlString),
    #[xml(tag = "b:Version")]
    BVersion(Version),
    #[xml(tag = "b:Volume")]
    BVolume(Volume),
    #[xml(tag = "b:Year")]
    BYear(Year),
    #[xml(tag = "b:YearAccessed")]
    BYearAccessed(YearAccessed),
}
