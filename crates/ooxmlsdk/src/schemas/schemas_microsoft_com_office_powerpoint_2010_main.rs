#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionPatternValues {
    #[default]
    Diamond,
    Hexagon,
}
crate::__string_enum! {
    TransitionPatternValues { Diamond = "diamond", Hexagon = "hexagon", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionCenterDirectionTypeValues {
    #[default]
    Center,
}
crate::__string_enum! {
    TransitionCenterDirectionTypeValues { Center = "center", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionShredPatternValues {
    #[default]
    Strip,
    Rectangle,
}
crate::__string_enum! {
    TransitionShredPatternValues { Strip = "strip", Rectangle = "rectangle", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionLeftRightDirectionTypeValues {
    #[default]
    Left,
    Right,
}
crate::__string_enum! {
    TransitionLeftRightDirectionTypeValues { Left = "l", Right = "r", }
}
/// Defines the NonVisualContentPartProperties Class.
/// When the object is serialized out as xml, it's qualified name is p14:nvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
    /// _
    #[xml(child = "p14:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    /// _
    #[xml(child = "p14:cNvContentPartPr")]
    pub non_visual_ink_content_part_properties: Option<
        NonVisualInkContentPartProperties,
    >,
    /// _
    #[xml(child = "p14:nvPr")]
    pub application_non_visual_drawing_properties: ApplicationNonVisualDrawingProperties,
}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is p14:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:xfrm")]
pub struct Transform2D {
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<i32>,
    /// Horizontal Flip
    /// Represents the following attribute in the schema: :flipH
    #[xml(attr = "flipH")]
    pub horizontal_flip: Option<bool>,
    /// Vertical Flip
    /// Represents the following attribute in the schema: :flipV
    #[xml(attr = "flipV")]
    pub vertical_flip: Option<bool>,
    ///Offset
    #[xml(child = "a:off")]
    pub offset: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset,
    >,
    ///Extents
    #[xml(child = "a:ext")]
    pub extents: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents,
    >,
}
/// Defines the ExtensionListModify Class.
/// When the object is serialized out as xml, it's qualified name is p14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:extLst")]
pub struct ExtensionListModify {
    /// Modify
    /// Represents the following attribute in the schema: :mod
    #[xml(attr = "mod")]
    pub modify: Option<bool>,
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListModifyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListModifyChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
/// Defines the Media Class.
/// When the object is serialized out as xml, it's qualified name is p14:media.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:media")]
pub struct Media {
    /// Embedded Picture Reference
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: Option<String>,
    /// Linked Picture Reference
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: Option<String>,
    /// _
    #[xml(child = "p14:trim")]
    pub media_trim: Option<MediaTrim>,
    /// _
    #[xml(child = "p14:fade")]
    pub media_fade: Option<MediaFade>,
    /// _
    #[xml(child = "p14:bmkLst")]
    pub media_bookmark_list: Option<MediaBookmarkList>,
    /// _
    #[xml(child = "p14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the VortexTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:vortex.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:vortex")]
pub struct VortexTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the PanTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:pan.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:pan")]
pub struct PanTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the SideDirectionTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SideDirectionTransitionType {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the SwitchTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:switch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:switch")]
pub struct SwitchTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FlipTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:flip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:flip")]
pub struct FlipTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FerrisTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:ferris.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:ferris")]
pub struct FerrisTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the GalleryTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:gallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:gallery")]
pub struct GalleryTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the ConveyorTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:conveyor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:conveyor")]
pub struct ConveyorTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the LeftRightDirectionTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LeftRightDirectionTransitionType {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the RippleTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:ripple.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:ripple")]
pub struct RippleTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<String>,
}
/// Defines the HoneycombTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:honeycomb.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:honeycomb")]
pub struct HoneycombTransition {}
/// Defines the FlashTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:flash.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:flash")]
pub struct FlashTransition {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Defines the PrismTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:prism.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:prism")]
pub struct PrismTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
    /// isContent
    /// Represents the following attribute in the schema: :isContent
    #[xml(attr = "isContent")]
    pub is_content: Option<bool>,
    /// isInverted
    /// Represents the following attribute in the schema: :isInverted
    #[xml(attr = "isInverted")]
    pub is_inverted: Option<bool>,
}
/// Defines the DoorsTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:doors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:doors")]
pub struct DoorsTransition {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues,
    >,
}
/// Defines the WindowTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:window.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:window")]
pub struct WindowTransition {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues,
    >,
}
/// Defines the OrientationTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OrientationTransitionType {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues,
    >,
}
/// Defines the GlitterTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:glitter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:glitter")]
pub struct GlitterTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
    /// pattern
    /// Represents the following attribute in the schema: :pattern
    #[xml(attr = "pattern")]
    pub pattern: Option<TransitionPatternValues>,
}
/// Defines the WarpTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:warp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:warp")]
pub struct WarpTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
}
/// Defines the FlythroughTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:flythrough.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:flythrough")]
pub struct FlythroughTransition {
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
    /// hasBounce
    /// Represents the following attribute in the schema: :hasBounce
    #[xml(attr = "hasBounce")]
    pub has_bounce: Option<bool>,
}
/// Defines the ShredTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:shred.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:shred")]
pub struct ShredTransition {
    /// pattern
    /// Represents the following attribute in the schema: :pattern
    #[xml(attr = "pattern")]
    pub pattern: Option<TransitionShredPatternValues>,
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
}
/// Defines the RevealTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:reveal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:reveal")]
pub struct RevealTransition {
    /// thruBlk
    /// Represents the following attribute in the schema: :thruBlk
    #[xml(attr = "thruBlk")]
    pub through_black: Option<bool>,
    /// dir
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the WheelReverseTransition Class.
/// When the object is serialized out as xml, it's qualified name is p14:wheelReverse.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:wheelReverse")]
pub struct WheelReverseTransition {
    /// Spokes
    /// Represents the following attribute in the schema: :spokes
    #[xml(attr = "spokes")]
    pub spokes: Option<u32>,
}
/// Defines the BookmarkTarget Class.
/// When the object is serialized out as xml, it's qualified name is p14:bmkTgt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:bmkTgt")]
pub struct BookmarkTarget {
    /// spid
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: u32,
    /// bmkName
    /// Represents the following attribute in the schema: :bmkName
    #[xml(attr = "bmkName")]
    pub bookmark_name: String,
}
/// Defines the SectionProperties Class.
/// When the object is serialized out as xml, it's qualified name is p14:sectionPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:sectionPr")]
pub struct SectionProperties {
    /// _
    #[xml(child = "p14:section")]
    pub p14_section: Vec<SectionOld>,
}
/// Defines the SectionList Class.
/// When the object is serialized out as xml, it's qualified name is p14:sectionLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:sectionLst")]
pub struct SectionList {
    /// _
    #[xml(child = "p14:section")]
    pub p14_section: Vec<Section>,
}
/// Defines the BrowseMode Class.
/// When the object is serialized out as xml, it's qualified name is p14:browseMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:browseMode")]
pub struct BrowseMode {
    /// showStatus
    /// Represents the following attribute in the schema: :showStatus
    #[xml(attr = "showStatus")]
    pub show_status: Option<bool>,
}
/// Defines the LaserColor Class.
/// When the object is serialized out as xml, it's qualified name is p14:laserClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:laserClr")]
pub struct LaserColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<LaserColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LaserColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    ),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    ),
    #[xml(tag = "a:hslClr")]
    AHslClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor,
    ),
    #[xml(tag = "a:prstClr")]
    APrstClr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor,
    ),
}
/// Defines the DefaultImageDpi Class.
/// When the object is serialized out as xml, it's qualified name is p14:defaultImageDpi.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:defaultImageDpi")]
pub struct DefaultImageDpi {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Defines the DiscardImageEditData Class.
/// When the object is serialized out as xml, it's qualified name is p14:discardImageEditData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:discardImageEditData")]
pub struct DiscardImageEditData {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
/// Defines the ShowMediaControls Class.
/// When the object is serialized out as xml, it's qualified name is p14:showMediaCtrls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:showMediaCtrls")]
pub struct ShowMediaControls {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
/// Defines the LaserTraceList Class.
/// When the object is serialized out as xml, it's qualified name is p14:laserTraceLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:laserTraceLst")]
pub struct LaserTraceList {
    /// _
    #[xml(child = "p14:tracePtLst")]
    pub p14_trace_pt_lst: Vec<TracePointList>,
}
/// Defines the CreationId Class.
/// When the object is serialized out as xml, it's qualified name is p14:creationId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:creationId")]
pub struct CreationId {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Defines the ModificationId Class.
/// When the object is serialized out as xml, it's qualified name is p14:modId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:modId")]
pub struct ModificationId {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Defines the RandomIdType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RandomIdType {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Defines the ShowEventRecordList Class.
/// When the object is serialized out as xml, it's qualified name is p14:showEvtLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:showEvtLst")]
pub struct ShowEventRecordList {
    #[xml(
        child = "p14:triggerEvt",
        child = "p14:playEvt",
        child = "p14:stopEvt",
        child = "p14:pauseEvt",
        child = "p14:resumeEvt",
        child = "p14:seekEvt",
        child = "p14:nullEvt",
    )]
    pub children: Vec<ShowEventRecordListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShowEventRecordListChildChoice {
    #[xml(tag = "p14:triggerEvt")]
    P14TriggerEvt(TriggerEventRecord),
    #[xml(tag = "p14:playEvt")]
    P14PlayEvt(PlayEventRecord),
    #[xml(tag = "p14:stopEvt")]
    P14StopEvt(StopEventRecord),
    #[xml(tag = "p14:pauseEvt")]
    P14PauseEvt(PauseEventRecord),
    #[xml(tag = "p14:resumeEvt")]
    P14ResumeEvt(ResumeEventRecord),
    #[xml(tag = "p14:seekEvt")]
    P14SeekEvt(SeekEventRecord),
    #[xml(tag = "p14:nullEvt")]
    P14NullEvt(NullEventRecord),
}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is p14:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:cNvPr")]
pub struct NonVisualDrawingProperties {
    /// Application defined unique identifier.
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    /// Name compatible with Object Model (non-unique).
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Description of the drawing element.
    /// Represents the following attribute in the schema: :descr
    #[xml(attr = "descr")]
    pub description: Option<String>,
    /// Flag determining to show or hide this element.
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// Title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    ///Hyperlink associated with clicking or selecting the element.
    #[xml(child = "a:hlinkClick")]
    pub hyperlink_on_click: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
    >,
    ///Hyperlink associated with hovering over the element.
    #[xml(child = "a:hlinkHover")]
    pub hyperlink_on_hover: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
    >,
    ///Future extension
    #[xml(child = "a:extLst")]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualInkContentPartProperties Class.
/// When the object is serialized out as xml, it's qualified name is p14:cNvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:cNvContentPartPr")]
pub struct NonVisualInkContentPartProperties {
    /// isComment
    /// Represents the following attribute in the schema: :isComment
    #[xml(attr = "isComment")]
    pub is_comment: Option<bool>,
    /// _
    #[xml(child = "a14:cpLocks")]
    pub content_part_locks: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ContentPartLocks,
    >,
    /// _
    #[xml(child = "a14:extLst")]
    pub office_art_extension_list: Option<
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::OfficeArtExtensionList,
    >,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is p14:nvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
    /// Is a Photo Album
    /// Represents the following attribute in the schema: :isPhoto
    #[xml(attr = "isPhoto")]
    pub is_photo: Option<bool>,
    /// Is User Drawn
    /// Represents the following attribute in the schema: :userDrawn
    #[xml(attr = "userDrawn")]
    pub user_drawn: Option<bool>,
    #[xml(
        child = "p:ph",
        child = "a:audioCd",
        child = "a:wavAudioFile",
        child = "a:audioFile",
        child = "a:videoFile",
        child = "a:quickTimeFile",
        child = "p:custDataLst",
        child = "p:extLst",
    )]
    pub children: Vec<ApplicationNonVisualDrawingPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ApplicationNonVisualDrawingPropertiesChildChoice {
    #[xml(tag = "p:ph")]
    PPh(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PlaceholderShape,
    ),
    #[xml(tag = "a:audioCd")]
    AAudioCd(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AudioFromCd,
    ),
    #[xml(tag = "a:wavAudioFile")]
    AWavAudioFile(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::WaveAudioFile,
    ),
    #[xml(tag = "a:audioFile")]
    AAudioFile(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AudioFromFile,
    ),
    #[xml(tag = "a:videoFile")]
    AVideoFile(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::VideoFromFile,
    ),
    #[xml(tag = "a:quickTimeFile")]
    AQuickTimeFile(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::QuickTimeFromFile,
    ),
    #[xml(tag = "p:custDataLst")]
    PCustDataLst(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CustomerDataList,
    ),
    #[xml(tag = "p:extLst")]
    PExtLst(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ApplicationNonVisualDrawingPropertiesExtensionList,
    ),
}
/// Defines the MediaBookmark Class.
/// When the object is serialized out as xml, it's qualified name is p14:bmk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:bmk")]
pub struct MediaBookmark {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: Option<String>,
}
/// Defines the MediaTrim Class.
/// When the object is serialized out as xml, it's qualified name is p14:trim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:trim")]
pub struct MediaTrim {
    /// st
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: Option<String>,
    /// end
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: Option<String>,
}
/// Defines the MediaFade Class.
/// When the object is serialized out as xml, it's qualified name is p14:fade.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:fade")]
pub struct MediaFade {
    /// in
    /// Represents the following attribute in the schema: :in
    #[xml(attr = "in")]
    pub in_duration: Option<String>,
    /// out
    /// Represents the following attribute in the schema: :out
    #[xml(attr = "out")]
    pub out_duration: Option<String>,
}
/// Defines the MediaBookmarkList Class.
/// When the object is serialized out as xml, it's qualified name is p14:bmkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:bmkLst")]
pub struct MediaBookmarkList {
    /// _
    #[xml(child = "p14:bmk")]
    pub p14_bmk: Vec<MediaBookmark>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension),
}
/// Defines the SectionOld Class.
/// When the object is serialized out as xml, it's qualified name is p14:section.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:section")]
pub struct SectionOld {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// slideIdLst
    /// Represents the following attribute in the schema: :slideIdLst
    #[xml(attr = "slideIdLst")]
    pub slide_id_list: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "p14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the SectionSlideIdListEntry Class.
/// When the object is serialized out as xml, it's qualified name is p14:sldId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:sldId")]
pub struct SectionSlideIdListEntry {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
}
/// Defines the SectionSlideIdList Class.
/// When the object is serialized out as xml, it's qualified name is p14:sldIdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:sldIdLst")]
pub struct SectionSlideIdList {
    /// _
    #[xml(child = "p14:sldId")]
    pub p14_sld_id: Vec<SectionSlideIdListEntry>,
}
/// Defines the Section Class.
/// When the object is serialized out as xml, it's qualified name is p14:section.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:section")]
pub struct Section {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// _
    #[xml(child = "p14:sldIdLst")]
    pub section_slide_id_list: SectionSlideIdList,
    /// _
    #[xml(child = "p14:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the TracePoint Class.
/// When the object is serialized out as xml, it's qualified name is p14:tracePt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:tracePt")]
pub struct TracePoint {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub time: String,
    /// x
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x_coordinate: i32,
    /// y
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y_coordinate: i32,
}
/// Defines the TracePointList Class.
/// When the object is serialized out as xml, it's qualified name is p14:tracePtLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:tracePtLst")]
pub struct TracePointList {
    /// _
    #[xml(child = "p14:tracePt")]
    pub p14_trace_pt: Vec<TracePoint>,
}
/// Defines the TriggerEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:triggerEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:triggerEvt")]
pub struct TriggerEventRecord {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TriggerEventValues,
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
/// Defines the PlayEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:playEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:playEvt")]
pub struct PlayEventRecord {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
/// Defines the StopEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:stopEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:stopEvt")]
pub struct StopEventRecord {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
/// Defines the PauseEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:pauseEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:pauseEvt")]
pub struct PauseEventRecord {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
/// Defines the ResumeEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:resumeEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:resumeEvt")]
pub struct ResumeEventRecord {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
/// Defines the MediaPlaybackEventRecordType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MediaPlaybackEventRecordType {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
/// Defines the SeekEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:seekEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:seekEvt")]
pub struct SeekEventRecord {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
    /// seek
    /// Represents the following attribute in the schema: :seek
    #[xml(attr = "seek")]
    pub seek: String,
}
/// Defines the NullEventRecord Class.
/// When the object is serialized out as xml, it's qualified name is p14:nullEvt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p14:nullEvt")]
pub struct NullEventRecord {
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
    /// objId
    /// Represents the following attribute in the schema: :objId
    #[xml(attr = "objId")]
    pub object_id: u32,
}
