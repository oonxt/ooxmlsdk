#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DisplayLocation {
    #[default]
    Media,
    Slide,
}
crate::__string_enum! {
    DisplayLocation { Media = "media", Slide = "slide", }
}
/// Defines the TracksInfo Class.
/// When the object is serialized out as xml, it's qualified name is p173:tracksInfo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p173:tracksInfo")]
pub struct TracksInfo {
    /// displayLoc
    /// Represents the following attribute in the schema: :displayLoc
    #[xml(attr = "displayLoc")]
    pub display_loc: DisplayLocation,
    /// _
    #[xml(child = "p173:trackLst")]
    pub track_list: Option<TrackList>,
}
/// Defines the Track Class.
/// When the object is serialized out as xml, it's qualified name is p173:track.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p173:track")]
pub struct Track {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// label
    /// Represents the following attribute in the schema: :label
    #[xml(attr = "label")]
    pub label: String,
    /// lang
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub lang: Option<String>,
    /// Embedded Picture Reference
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// Linked Picture Reference
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: Option<String>,
}
/// Defines the TrackList Class.
/// When the object is serialized out as xml, it's qualified name is p173:trackLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p173:trackLst")]
pub struct TrackList {
    /// _
    #[xml(child = "p173:track")]
    pub p173_track: Vec<Track>,
}
