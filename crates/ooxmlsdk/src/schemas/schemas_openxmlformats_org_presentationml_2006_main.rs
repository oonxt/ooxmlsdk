#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionSlideDirectionValues {
    #[default]
    Left,
    Up,
    Right,
    Down,
}
crate::__string_enum! {
    TransitionSlideDirectionValues { Left = "l", Up = "u", Right = "r", Down = "d", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionCornerDirectionValues {
    #[default]
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}
crate::__string_enum! {
    TransitionCornerDirectionValues { LeftUp = "lu", RightUp = "ru", LeftDown = "ld",
    RightDown = "rd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionInOutDirectionValues {
    #[default]
    Out,
    In,
}
crate::__string_enum! {
    TransitionInOutDirectionValues { Out = "out", In = "in", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TransitionSpeedValues {
    #[default]
    Slow,
    Medium,
    Fast,
}
crate::__string_enum! {
    TransitionSpeedValues { Slow = "slow", Medium = "med", Fast = "fast", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum IndefiniteTimeDeclarationValues {
    #[default]
    Indefinite,
}
crate::__string_enum! {
    IndefiniteTimeDeclarationValues { Indefinite = "indefinite", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum IterateValues {
    #[default]
    Element,
    Word,
    Letter,
}
crate::__string_enum! {
    IterateValues { Element = "el", Word = "wd", Letter = "lt", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChartSubElementValues {
    #[default]
    GridLegend,
    Series,
    Category,
    PointInSeries,
    PointInCategory,
}
crate::__string_enum! {
    ChartSubElementValues { GridLegend = "gridLegend", Series = "series", Category =
    "category", PointInSeries = "ptInSeries", PointInCategory = "ptInCategory", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TriggerRuntimeNodeValues {
    #[default]
    First,
    Last,
    All,
}
crate::__string_enum! {
    TriggerRuntimeNodeValues { First = "first", Last = "last", All = "all", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeNodePresetClassValues {
    #[default]
    Entrance,
    Exit,
    Emphasis,
    Path,
    Verb,
    MediaCall,
}
crate::__string_enum! {
    TimeNodePresetClassValues { Entrance = "entr", Exit = "exit", Emphasis = "emph", Path
    = "path", Verb = "verb", MediaCall = "mediacall", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeNodeRestartValues {
    #[default]
    Always,
    WhenNotActive,
    Never,
}
crate::__string_enum! {
    TimeNodeRestartValues { Always = "always", WhenNotActive = "whenNotActive", Never =
    "never", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeNodeFillValues {
    #[default]
    Remove,
    Freeze,
    Hold,
    Transition,
}
crate::__string_enum! {
    TimeNodeFillValues { Remove = "remove", Freeze = "freeze", Hold = "hold", Transition
    = "transition", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeNodeValues {
    #[default]
    ClickEffect,
    WithEffect,
    AfterEffect,
    MainSequence,
    InteractiveSequence,
    ClickParagraph,
    WithGroup,
    AfterGroup,
    TmingRoot,
}
crate::__string_enum! {
    TimeNodeValues { ClickEffect = "clickEffect", WithEffect = "withEffect", AfterEffect
    = "afterEffect", MainSequence = "mainSeq", InteractiveSequence = "interactiveSeq",
    ClickParagraph = "clickPar", WithGroup = "withGroup", AfterGroup = "afterGroup",
    TmingRoot = "tmRoot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NextActionValues {
    #[default]
    None,
    Seek,
}
crate::__string_enum! {
    NextActionValues { None = "none", Seek = "seek", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PreviousActionValues {
    #[default]
    None,
    SkipTimed,
}
crate::__string_enum! {
    PreviousActionValues { None = "none", SkipTimed = "skipTimed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BehaviorAdditiveValues {
    #[default]
    Base,
    Sum,
    Replace,
    Multiply,
    None,
}
crate::__string_enum! {
    BehaviorAdditiveValues { Base = "base", Sum = "sum", Replace = "repl", Multiply =
    "mult", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BehaviorAccumulateValues {
    #[default]
    None,
    Always,
}
crate::__string_enum! {
    BehaviorAccumulateValues { None = "none", Always = "always", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BehaviorTransformValues {
    #[default]
    Point,
    Image,
}
crate::__string_enum! {
    BehaviorTransformValues { Point = "pt", Image = "img", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BehaviorOverrideValues {
    #[default]
    Normal,
    ChildStyle,
}
crate::__string_enum! {
    BehaviorOverrideValues { Normal = "normal", ChildStyle = "childStyle", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateBehaviorCalculateModeValues {
    #[default]
    Discrete,
    Linear,
    Formula,
}
crate::__string_enum! {
    AnimateBehaviorCalculateModeValues { Discrete = "discrete", Linear = "lin", Formula =
    "fmla", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateBehaviorValues {
    #[default]
    String,
    Number,
    Color,
}
crate::__string_enum! {
    AnimateBehaviorValues { String = "str", Number = "num", Color = "clr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateColorSpaceValues {
    #[default]
    Rgb,
    Hsl,
}
crate::__string_enum! {
    AnimateColorSpaceValues { Rgb = "rgb", Hsl = "hsl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateColorDirectionValues {
    #[default]
    Clockwise,
    CounterClockwise,
}
crate::__string_enum! {
    AnimateColorDirectionValues { Clockwise = "cw", CounterClockwise = "ccw", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateEffectTransitionValues {
    #[default]
    In,
    Out,
    None,
}
crate::__string_enum! {
    AnimateEffectTransitionValues { In = "in", Out = "out", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateMotionBehaviorOriginValues {
    #[default]
    Parent,
    Layout,
}
crate::__string_enum! {
    AnimateMotionBehaviorOriginValues { Parent = "parent", Layout = "layout", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateMotionPathEditModeValues {
    #[default]
    Relative,
    Fixed,
}
crate::__string_enum! {
    AnimateMotionPathEditModeValues { Relative = "relative", Fixed = "fixed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CommandValues {
    #[default]
    Event,
    Call,
    Verb,
}
crate::__string_enum! {
    CommandValues { Event = "evt", Call = "call", Verb = "verb", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ParagraphBuildValues {
    #[default]
    AllAtOnce,
    Paragraph,
    Custom,
    Whole,
}
crate::__string_enum! {
    ParagraphBuildValues { AllAtOnce = "allAtOnce", Paragraph = "p", Custom = "cust",
    Whole = "whole", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DiagramBuildValues {
    #[default]
    Whole,
    DepthByNode,
    DepthByBranch,
    BreadthByNode,
    BreadthByLevel,
    Clockwise,
    ClockwiseIn,
    ClockwiseOut,
    CounterClockwise,
    CounterClockwiseIn,
    CounterClockwiseOut,
    InByRing,
    OutByRing,
    Up,
    Down,
    AllAtOnce,
    Custom,
}
crate::__string_enum! {
    DiagramBuildValues { Whole = "whole", DepthByNode = "depthByNode", DepthByBranch =
    "depthByBranch", BreadthByNode = "breadthByNode", BreadthByLevel = "breadthByLvl",
    Clockwise = "cw", ClockwiseIn = "cwIn", ClockwiseOut = "cwOut", CounterClockwise =
    "ccw", CounterClockwiseIn = "ccwIn", CounterClockwiseOut = "ccwOut", InByRing =
    "inByRing", OutByRing = "outByRing", Up = "up", Down = "down", AllAtOnce =
    "allAtOnce", Custom = "cust", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleChartBuildValues {
    #[default]
    AllAtOnce,
    Series,
    Category,
    SeriesElement,
    CategoryElement,
}
crate::__string_enum! {
    OleChartBuildValues { AllAtOnce = "allAtOnce", Series = "series", Category =
    "category", SeriesElement = "seriesEl", CategoryElement = "categoryEl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeNodeMasterRelationValues {
    #[default]
    SameClick,
    NextClick,
}
crate::__string_enum! {
    TimeNodeMasterRelationValues { SameClick = "sameClick", NextClick = "nextClick", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TimeNodeSyncValues {
    #[default]
    None,
    CanSlip,
    Locked,
}
crate::__string_enum! {
    TimeNodeSyncValues { None = "none", CanSlip = "canSlip", Locked = "locked", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DirectionValues {
    #[default]
    Horizontal,
    Vertical,
}
crate::__string_enum! {
    DirectionValues { Horizontal = "horz", Vertical = "vert", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OleObjectFollowColorSchemeValues {
    #[default]
    None,
    Full,
    TextAndBackground,
}
crate::__string_enum! {
    OleObjectFollowColorSchemeValues { None = "none", Full = "full", TextAndBackground =
    "textAndBackground", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PhotoAlbumLayoutValues {
    #[default]
    FitToSlide,
    OnePic,
    TwoPic,
    FourPic,
    OnePicWithTitle,
    TwoPicWithTitle,
    FourPicWithTitle,
}
crate::__string_enum! {
    PhotoAlbumLayoutValues { FitToSlide = "fitToSlide", OnePic = "1pic", TwoPic = "2pic",
    FourPic = "4pic", OnePicWithTitle = "1picTitle", TwoPicWithTitle = "2picTitle",
    FourPicWithTitle = "4picTitle", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PhotoAlbumFrameShapeValues {
    #[default]
    FrameStyle1,
    FrameStyle2,
    FrameStyle3,
    FrameStyle4,
    FrameStyle5,
    FrameStyle6,
    FrameStyle7,
}
crate::__string_enum! {
    PhotoAlbumFrameShapeValues { FrameStyle1 = "frameStyle1", FrameStyle2 =
    "frameStyle2", FrameStyle3 = "frameStyle3", FrameStyle4 = "frameStyle4", FrameStyle5
    = "frameStyle5", FrameStyle6 = "frameStyle6", FrameStyle7 = "frameStyle7", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SlideSizeValues {
    #[default]
    Screen4x3,
    Letter,
    A4,
    Film35mm,
    Overhead,
    Banner,
    Custom,
    Ledger,
    A3,
    B4iso,
    B5iso,
    B4jis,
    B5jis,
    HagakiCard,
    Screen16x9,
    Screen16x10,
}
crate::__string_enum! {
    SlideSizeValues { Screen4x3 = "screen4x3", Letter = "letter", A4 = "a4", Film35mm =
    "35mm", Overhead = "overhead", Banner = "banner", Custom = "custom", Ledger =
    "ledger", A3 = "a3", B4iso = "b4iso", B5iso = "b5iso", B4jis = "b4jis", B5jis =
    "b5jis", HagakiCard = "hagakiCard", Screen16x9 = "screen16x9", Screen16x10 =
    "screen16x10", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CryptProviderValues {
    #[default]
    RsaAes,
    RsaFull,
    Invalid,
}
crate::__string_enum! {
    CryptProviderValues { RsaAes = "rsaAes", RsaFull = "rsaFull", Invalid = "invalid", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CryptAlgorithmClassValues {
    #[default]
    Hash,
    Invalid,
}
crate::__string_enum! {
    CryptAlgorithmClassValues { Hash = "hash", Invalid = "invalid", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CryptAlgorithmValues {
    #[default]
    TypeAny,
    Invalid,
}
crate::__string_enum! {
    CryptAlgorithmValues { TypeAny = "typeAny", Invalid = "invalid", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HtmlPublishWebBrowserSupportValues {
    #[default]
    V4,
    V3,
    V3v4,
}
crate::__string_enum! {
    HtmlPublishWebBrowserSupportValues { V4 = "v4", V3 = "v3", V3v4 = "v3v4", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WebColorValues {
    #[default]
    None,
    Browser,
    PresentationText,
    PresentationAccent,
    WhiteTextOnBlack,
    BlackTextOnWhite,
}
crate::__string_enum! {
    WebColorValues { None = "none", Browser = "browser", PresentationText =
    "presentationText", PresentationAccent = "presentationAccent", WhiteTextOnBlack =
    "whiteTextOnBlack", BlackTextOnWhite = "blackTextOnWhite", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum WebScreenSizeValues {
    #[default]
    Sz544x376,
    Sz640x480,
    Sz720x512,
    Sz800x600,
    Sz1024x768,
    Sz1152x882,
    Sz1152x900,
    Sz1280x1024,
    Sz1600x1200,
    Sz1800x1400,
    Sz1920x1200,
}
crate::__string_enum! {
    WebScreenSizeValues { Sz544x376 = "544x376", Sz640x480 = "640x480", Sz720x512 =
    "720x512", Sz800x600 = "800x600", Sz1024x768 = "1024x768", Sz1152x882 = "1152x882",
    Sz1152x900 = "1152x900", Sz1280x1024 = "1280x1024", Sz1600x1200 = "1600x1200",
    Sz1800x1400 = "1800x1400", Sz1920x1200 = "1920x1200", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PrintOutputValues {
    #[default]
    Slides,
    Handouts1,
    Handouts2,
    Handouts3,
    Handouts4,
    Handouts6,
    Handouts9,
    Notes,
    Outline,
}
crate::__string_enum! {
    PrintOutputValues { Slides = "slides", Handouts1 = "handouts1", Handouts2 =
    "handouts2", Handouts3 = "handouts3", Handouts4 = "handouts4", Handouts6 =
    "handouts6", Handouts9 = "handouts9", Notes = "notes", Outline = "outline", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PrintColorModeValues {
    #[default]
    BlackWhite,
    Gray,
    Color,
}
crate::__string_enum! {
    PrintColorModeValues { BlackWhite = "bw", Gray = "gray", Color = "clr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PlaceholderValues {
    #[default]
    Title,
    Body,
    CenteredTitle,
    SubTitle,
    DateAndTime,
    SlideNumber,
    Footer,
    Header,
    Object,
    Chart,
    Table,
    ClipArt,
    Diagram,
    Media,
    SlideImage,
    Picture,
}
crate::__string_enum! {
    PlaceholderValues { Title = "title", Body = "body", CenteredTitle = "ctrTitle",
    SubTitle = "subTitle", DateAndTime = "dt", SlideNumber = "sldNum", Footer = "ftr",
    Header = "hdr", Object = "obj", Chart = "chart", Table = "tbl", ClipArt = "clipArt",
    Diagram = "dgm", Media = "media", SlideImage = "sldImg", Picture = "pic", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PlaceholderSizeValues {
    #[default]
    Full,
    Half,
    Quarter,
}
crate::__string_enum! {
    PlaceholderSizeValues { Full = "full", Half = "half", Quarter = "quarter", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SlideLayoutValues {
    #[default]
    Title,
    Text,
    TwoColumnText,
    Table,
    TextAndChart,
    ChartAndText,
    Diagram,
    Chart,
    TextAndClipArt,
    ClipArtAndText,
    TitleOnly,
    Blank,
    TextAndObject,
    ObjectAndText,
    ObjectOnly,
    Object,
    TextAndMedia,
    MidiaAndText,
    ObjectOverText,
    TextOverObject,
    TextAndTwoObjects,
    TwoObjectsAndText,
    TwoObjectsOverText,
    FourObjects,
    VerticalText,
    ClipArtAndVerticalText,
    VerticalTitleAndText,
    VerticalTitleAndTextOverChart,
    TwoObjects,
    ObjectAndTwoObjects,
    TwoObjectsAndObject,
    Custom,
    SectionHeader,
    TwoTextAndTwoObjects,
    ObjectText,
    PictureText,
}
crate::__string_enum! {
    SlideLayoutValues { Title = "title", Text = "tx", TwoColumnText = "twoColTx", Table =
    "tbl", TextAndChart = "txAndChart", ChartAndText = "chartAndTx", Diagram = "dgm",
    Chart = "chart", TextAndClipArt = "txAndClipArt", ClipArtAndText = "clipArtAndTx",
    TitleOnly = "titleOnly", Blank = "blank", TextAndObject = "txAndObj", ObjectAndText =
    "objAndTx", ObjectOnly = "objOnly", Object = "obj", TextAndMedia = "txAndMedia",
    MidiaAndText = "mediaAndTx", ObjectOverText = "objOverTx", TextOverObject =
    "txOverObj", TextAndTwoObjects = "txAndTwoObj", TwoObjectsAndText = "twoObjAndTx",
    TwoObjectsOverText = "twoObjOverTx", FourObjects = "fourObj", VerticalText =
    "vertTx", ClipArtAndVerticalText = "clipArtAndVertTx", VerticalTitleAndText =
    "vertTitleAndTx", VerticalTitleAndTextOverChart = "vertTitleAndTxOverChart",
    TwoObjects = "twoObj", ObjectAndTwoObjects = "objAndTwoObj", TwoObjectsAndObject =
    "twoObjAndObj", Custom = "cust", SectionHeader = "secHead", TwoTextAndTwoObjects =
    "twoTxTwoObj", ObjectText = "objTx", PictureText = "picTx", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SplitterBarStateValues {
    #[default]
    Minimized,
    Restored,
    Maximized,
}
crate::__string_enum! {
    SplitterBarStateValues { Minimized = "minimized", Restored = "restored", Maximized =
    "maximized", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ViewValues {
    #[default]
    SlideView,
    SlideMasterView,
    NotesView,
    HandoutView,
    NotesMasterView,
    OutlineView,
    SlideSorterView,
    SlideThumbnailView,
}
crate::__string_enum! {
    ViewValues { SlideView = "sldView", SlideMasterView = "sldMasterView", NotesView =
    "notesView", HandoutView = "handoutView", NotesMasterView = "notesMasterView",
    OutlineView = "outlineView", SlideSorterView = "sldSorterView", SlideThumbnailView =
    "sldThumbnailView", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TriggerEventValues {
    #[default]
    None,
    OnBegin,
    OnEnd,
    Begin,
    End,
    OnClick,
    OnDoubleClick,
    OnMouseOver,
    OnMouseOut,
    OnNext,
    OnPrevious,
    OnStopAudio,
    OnMediaBookmark,
}
crate::__string_enum! {
    TriggerEventValues { None = "none", OnBegin = "onBegin", OnEnd = "onEnd", Begin =
    "begin", End = "end", OnClick = "onClick", OnDoubleClick = "onDblClick", OnMouseOver
    = "onMouseOver", OnMouseOut = "onMouseOut", OnNext = "onNext", OnPrevious = "onPrev",
    OnStopAudio = "onStopAudio", OnMediaBookmark = "onMediaBookmark", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConformanceClassValues {
    #[default]
    Strict,
    Transitional,
}
crate::__string_enum! {
    ConformanceClassValues { Strict = "strict", Transitional = "transitional", }
}
/// All Slides.
/// When the object is serialized out as xml, it's qualified name is p:sldAll.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldAll")]
pub struct SlideAll {}
/// Presenter Slide Show Mode.
/// When the object is serialized out as xml, it's qualified name is p:present.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:present")]
pub struct PresenterSlideMode {}
/// Stop Sound Action.
/// When the object is serialized out as xml, it's qualified name is p:endSnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:endSnd")]
pub struct EndSoundAction {}
/// Build As One.
/// When the object is serialized out as xml, it's qualified name is p:bldAsOne.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldAsOne")]
pub struct BuildAsOne {}
/// Slide Target.
/// When the object is serialized out as xml, it's qualified name is p:sldTgt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldTgt")]
pub struct SlideTarget {}
/// Background.
/// When the object is serialized out as xml, it's qualified name is p:bg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bg")]
pub struct BackgroundAnimation {}
/// Defines the CircleTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:circle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:circle")]
pub struct CircleTransition {}
/// Defines the DissolveTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:dissolve.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:dissolve")]
pub struct DissolveTransition {}
/// Defines the DiamondTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:diamond.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:diamond")]
pub struct DiamondTransition {}
/// Defines the NewsflashTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:newsflash.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:newsflash")]
pub struct NewsflashTransition {}
/// Defines the PlusTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:plus.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:plus")]
pub struct PlusTransition {}
/// Defines the RandomTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:random.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:random")]
pub struct RandomTransition {}
/// Defines the WedgeTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:wedge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:wedge")]
pub struct WedgeTransition {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Slide Range.
/// When the object is serialized out as xml, it's qualified name is p:sldRg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldRg")]
pub struct SlideRange {
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: u32,
    /// End
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: u32,
}
/// Character Range.
/// When the object is serialized out as xml, it's qualified name is p:charRg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:charRg")]
pub struct CharRange {
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: u32,
    /// End
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: u32,
}
/// Paragraph Text Range.
/// When the object is serialized out as xml, it's qualified name is p:pRg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:pRg")]
pub struct ParagraphIndexRange {
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: u32,
    /// End
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: u32,
}
/// Defines the IndexRangeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct IndexRangeType {
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: u32,
    /// End
    /// Represents the following attribute in the schema: :end
    #[xml(attr = "end")]
    pub end: u32,
}
/// Custom Show.
/// When the object is serialized out as xml, it's qualified name is p:custShow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:custShow")]
pub struct CustomShowReference {
    /// Custom Show Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
}
/// Extension.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct Extension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
}
/// Browse Slide Show Mode.
/// When the object is serialized out as xml, it's qualified name is p:browse.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:browse")]
pub struct BrowseSlideMode {
    /// Show Scroll Bar in Window
    /// Represents the following attribute in the schema: :showScrollbar
    #[xml(attr = "showScrollbar")]
    pub show_scrollbar: Option<bool>,
}
/// Kiosk Slide Show Mode.
/// When the object is serialized out as xml, it's qualified name is p:kiosk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:kiosk")]
pub struct KioskSlideMode {
    /// Restart Show
    /// Represents the following attribute in the schema: :restart
    #[xml(attr = "restart")]
    pub restart: Option<u32>,
}
/// Color Scheme Map.
/// When the object is serialized out as xml, it's qualified name is p:clrMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:clrMap")]
pub struct ColorMap {
    /// Background 1
    /// Represents the following attribute in the schema: :bg1
    #[xml(attr = "bg1")]
    pub background1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Text 1
    /// Represents the following attribute in the schema: :tx1
    #[xml(attr = "tx1")]
    pub text1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Background 2
    /// Represents the following attribute in the schema: :bg2
    #[xml(attr = "bg2")]
    pub background2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Text 2
    /// Represents the following attribute in the schema: :tx2
    #[xml(attr = "tx2")]
    pub text2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 1
    /// Represents the following attribute in the schema: :accent1
    #[xml(attr = "accent1")]
    pub accent1: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 2
    /// Represents the following attribute in the schema: :accent2
    #[xml(attr = "accent2")]
    pub accent2: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 3
    /// Represents the following attribute in the schema: :accent3
    #[xml(attr = "accent3")]
    pub accent3: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 4
    /// Represents the following attribute in the schema: :accent4
    #[xml(attr = "accent4")]
    pub accent4: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 5
    /// Represents the following attribute in the schema: :accent5
    #[xml(attr = "accent5")]
    pub accent5: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Accent 6
    /// Represents the following attribute in the schema: :accent6
    #[xml(attr = "accent6")]
    pub accent6: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Hyperlink
    /// Represents the following attribute in the schema: :hlink
    #[xml(attr = "hlink")]
    pub hyperlink: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// Followed Hyperlink
    /// Represents the following attribute in the schema: :folHlink
    #[xml(attr = "folHlink")]
    pub followed_hyperlink: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorSchemeIndexValues,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Color Scheme Map Override.
/// When the object is serialized out as xml, it's qualified name is p:clrMapOvr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:clrMapOvr")]
pub struct ColorMapOverride {
    #[xml(child = "a:masterClrMapping", child = "a:overrideClrMapping")]
    pub children: Vec<ColorMapOverrideChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorMapOverrideChildChoice {
    #[xml(tag = "a:masterClrMapping")]
    AMasterClrMapping(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::MasterColorMapping,
    ),
    #[xml(tag = "a:overrideClrMapping")]
    AOverrideClrMapping(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::OverrideColorMapping,
    ),
}
/// Background Properties.
/// When the object is serialized out as xml, it's qualified name is p:bgPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bgPr")]
pub struct BackgroundProperties {
    /// Shade to Title
    /// Represents the following attribute in the schema: :shadeToTitle
    #[xml(attr = "shadeToTitle")]
    pub shade_to_title: Option<bool>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "p:extLst",
    )]
    pub children: Vec<BackgroundPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundPropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill,
    ),
    #[xml(tag = "a:gradFill")]
    AGradFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill,
    ),
    #[xml(tag = "a:blipFill")]
    ABlipFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
    #[xml(tag = "p:extLst")]
    PExtLst(ExtensionList),
}
/// Background Style Reference.
/// When the object is serialized out as xml, it's qualified name is p:bgRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bgRef")]
pub struct BackgroundStyleReference {
    /// Style Matrix Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: u32,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<BackgroundStyleReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundStyleReferenceChildChoice {
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
/// Data for the Windows platform..
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct CommentPropertiesExtension {
    /// _
    #[xml(child = "p228:taskDetails")]
    pub task_details: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2022_08_main::TaskDetails,
    >,
    /// _
    #[xml(child = "p223:reactions")]
    pub reactions: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2022_03_main::Reactions,
    >,
}
/// List of Comment Authors.
/// When the object is serialized out as xml, it's qualified name is p:cmAuthorLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cmAuthorLst")]
pub struct CommentAuthorList {
    #[xml(attr = "xmlns", with = "comment_author_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "p:cmAuthor")]
    pub p_cm_author: Vec<CommentAuthor>,
}
mod comment_author_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Comment List.
/// When the object is serialized out as xml, it's qualified name is p:cmLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cmLst")]
pub struct CommentList {
    #[xml(attr = "xmlns", with = "comment_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "p:cm")]
    pub p_cm: Vec<Comment>,
}
mod comment_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Global Element for OLE Objects and Controls.
/// When the object is serialized out as xml, it's qualified name is p:oleObj.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:oleObj")]
pub struct OleObject {
    /// spid
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// showAsIcon
    /// Represents the following attribute in the schema: :showAsIcon
    #[xml(attr = "showAsIcon")]
    pub show_as_icon: Option<bool>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// imgW
    /// Represents the following attribute in the schema: :imgW
    #[xml(attr = "imgW")]
    pub image_width: Option<i32>,
    /// imgH
    /// Represents the following attribute in the schema: :imgH
    #[xml(attr = "imgH")]
    pub image_height: Option<i32>,
    /// progId
    /// Represents the following attribute in the schema: :progId
    #[xml(attr = "progId")]
    pub prog_id: Option<String>,
    #[xml(child = "p:embed", child = "p:link", child = "p:pic")]
    pub children: Vec<OleObjectChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OleObjectChildChoice {
    #[xml(tag = "p:embed")]
    PEmbed(OleObjectEmbed),
    #[xml(tag = "p:link")]
    PLink(OleObjectLink),
    #[xml(tag = "p:pic")]
    PPic(Picture),
}
/// Presentation.
/// When the object is serialized out as xml, it's qualified name is p:presentation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:presentation")]
pub struct Presentation {
    #[xml(attr = "xmlns", with = "presentation_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// serverZoom
    /// Represents the following attribute in the schema: :serverZoom
    #[xml(attr = "serverZoom")]
    pub server_zoom: Option<i32>,
    /// firstSlideNum
    /// Represents the following attribute in the schema: :firstSlideNum
    #[xml(attr = "firstSlideNum")]
    pub first_slide_num: Option<i32>,
    /// showSpecialPlsOnTitleSld
    /// Represents the following attribute in the schema: :showSpecialPlsOnTitleSld
    #[xml(attr = "showSpecialPlsOnTitleSld")]
    pub show_special_placeholder_on_title_slide: Option<bool>,
    /// rtl
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// removePersonalInfoOnSave
    /// Represents the following attribute in the schema: :removePersonalInfoOnSave
    #[xml(attr = "removePersonalInfoOnSave")]
    pub remove_personal_info_on_save: Option<bool>,
    /// compatMode
    /// Represents the following attribute in the schema: :compatMode
    #[xml(attr = "compatMode")]
    pub compatibility_mode: Option<bool>,
    /// strictFirstAndLastChars
    /// Represents the following attribute in the schema: :strictFirstAndLastChars
    #[xml(attr = "strictFirstAndLastChars")]
    pub strict_first_and_last_chars: Option<bool>,
    /// embedTrueTypeFonts
    /// Represents the following attribute in the schema: :embedTrueTypeFonts
    #[xml(attr = "embedTrueTypeFonts")]
    pub embed_true_type_fonts: Option<bool>,
    /// saveSubsetFonts
    /// Represents the following attribute in the schema: :saveSubsetFonts
    #[xml(attr = "saveSubsetFonts")]
    pub save_subset_fonts: Option<bool>,
    /// autoCompressPictures
    /// Represents the following attribute in the schema: :autoCompressPictures
    #[xml(attr = "autoCompressPictures")]
    pub auto_compress_pictures: Option<bool>,
    /// bookmarkIdSeed
    /// Represents the following attribute in the schema: :bookmarkIdSeed
    #[xml(attr = "bookmarkIdSeed")]
    pub bookmark_id_seed: Option<u32>,
    /// conformance
    /// Represents the following attribute in the schema: :conformance
    #[xml(attr = "conformance")]
    pub conformance: Option<ConformanceClassValues>,
    /// _
    #[xml(child = "p:sldMasterIdLst")]
    pub slide_master_id_list: Option<SlideMasterIdList>,
    /// _
    #[xml(child = "p:notesMasterIdLst")]
    pub notes_master_id_list: Option<NotesMasterIdList>,
    /// _
    #[xml(child = "p:handoutMasterIdLst")]
    pub handout_master_id_list: Option<HandoutMasterIdList>,
    /// _
    #[xml(child = "p:sldIdLst")]
    pub slide_id_list: Option<SlideIdList>,
    /// _
    #[xml(child = "p:sldSz")]
    pub slide_size: Option<SlideSize>,
    /// _
    #[xml(child = "p:notesSz")]
    pub notes_size: NotesSize,
    /// _
    #[xml(child = "p:embeddedFontLst")]
    pub embedded_font_list: Option<EmbeddedFontList>,
    /// _
    #[xml(child = "p:custShowLst")]
    pub custom_show_list: Option<CustomShowList>,
    /// _
    #[xml(child = "p:photoAlbum")]
    pub photo_album: Option<PhotoAlbum>,
    /// _
    #[xml(child = "p:custDataLst")]
    pub customer_data_list: Option<CustomerDataList>,
    /// _
    #[xml(child = "p:kinsoku")]
    pub kinsoku: Option<Kinsoku>,
    /// _
    #[xml(child = "p:defaultTextStyle")]
    pub default_text_style: Option<DefaultTextStyle>,
    /// _
    #[xml(child = "p:modifyVerifier")]
    pub modification_verifier: Option<ModificationVerifier>,
    /// _
    #[xml(child = "p:extLst")]
    pub presentation_extension_list: Option<PresentationExtensionList>,
}
mod presentation_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Presentation-wide Properties.
/// When the object is serialized out as xml, it's qualified name is p:presentationPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:presentationPr")]
pub struct PresentationProperties {
    #[xml(attr = "xmlns", with = "presentation_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///HTML Publishing Properties
    #[xml(child = "p:htmlPubPr")]
    pub html_publish_properties: Option<HtmlPublishProperties>,
    ///Web Properties
    #[xml(child = "p:webPr")]
    pub web_properties: Option<WebProperties>,
    /// _
    #[xml(child = "p:prnPr")]
    pub printing_properties: Option<PrintingProperties>,
    /// _
    #[xml(child = "p:showPr")]
    pub show_properties: Option<ShowProperties>,
    /// _
    #[xml(child = "p:clrMru")]
    pub color_most_recently_used: Option<ColorMostRecentlyUsed>,
    /// _
    #[xml(child = "p:extLst")]
    pub presentation_properties_extension_list: Option<
        PresentationPropertiesExtensionList,
    >,
}
mod presentation_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Presentation Slide.
/// When the object is serialized out as xml, it's qualified name is p:sld.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sld")]
pub struct Slide {
    #[xml(attr = "xmlns", with = "slide_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Show Master Shapes
    /// Represents the following attribute in the schema: :showMasterSp
    #[xml(attr = "showMasterSp")]
    pub show_master_shapes: Option<bool>,
    /// Show Master Placeholder Animations
    /// Represents the following attribute in the schema: :showMasterPhAnim
    #[xml(attr = "showMasterPhAnim")]
    pub show_master_placeholder_animations: Option<bool>,
    /// Show Slide in Slide Show
    /// Represents the following attribute in the schema: :show
    #[xml(attr = "show")]
    pub show: Option<bool>,
    #[xml(
        child = "p:cSld",
        child = "p:clrMapOvr",
        child = "p:transition",
        child = "p:timing",
        child = "p:extLst",
    )]
    pub children: Vec<SlideChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideChildChoice {
    #[xml(tag = "p:cSld")]
    PCSld(CommonSlideData),
    #[xml(tag = "p:clrMapOvr")]
    PClrMapOvr(ColorMapOverride),
    #[xml(tag = "p:transition")]
    PTransition(Transition),
    #[xml(tag = "p:timing")]
    PTiming(Timing),
    #[xml(tag = "p:extLst")]
    PExtLst(SlideExtensionList),
}
mod slide_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Slide Layout.
/// When the object is serialized out as xml, it's qualified name is p:sldLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldLayout")]
pub struct SlideLayout {
    #[xml(attr = "xmlns", with = "slide_layout_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Show Master Shapes
    /// Represents the following attribute in the schema: :showMasterSp
    #[xml(attr = "showMasterSp")]
    pub show_master_shapes: Option<bool>,
    /// Show Master Placeholder Animations
    /// Represents the following attribute in the schema: :showMasterPhAnim
    #[xml(attr = "showMasterPhAnim")]
    pub show_master_placeholder_animations: Option<bool>,
    /// matchingName
    /// Represents the following attribute in the schema: :matchingName
    #[xml(attr = "matchingName")]
    pub matching_name: Option<String>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<SlideLayoutValues>,
    /// preserve
    /// Represents the following attribute in the schema: :preserve
    #[xml(attr = "preserve")]
    pub preserve: Option<bool>,
    /// userDrawn
    /// Represents the following attribute in the schema: :userDrawn
    #[xml(attr = "userDrawn")]
    pub user_drawn: Option<bool>,
    #[xml(
        child = "p:cSld",
        child = "p:clrMapOvr",
        child = "p:transition",
        child = "p:timing",
        child = "p:hf",
        child = "p:extLst",
    )]
    pub children: Vec<SlideLayoutChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideLayoutChildChoice {
    #[xml(tag = "p:cSld")]
    PCSld(CommonSlideData),
    #[xml(tag = "p:clrMapOvr")]
    PClrMapOvr(ColorMapOverride),
    #[xml(tag = "p:transition")]
    PTransition(Transition),
    #[xml(tag = "p:timing")]
    PTiming(Timing),
    #[xml(tag = "p:hf")]
    PHf(HeaderFooter),
    #[xml(tag = "p:extLst")]
    PExtLst(SlideLayoutExtensionList),
}
mod slide_layout_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Slide Master.
/// When the object is serialized out as xml, it's qualified name is p:sldMaster.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldMaster")]
pub struct SlideMaster {
    #[xml(attr = "xmlns", with = "slide_master_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// preserve
    /// Represents the following attribute in the schema: :preserve
    #[xml(attr = "preserve")]
    pub preserve: Option<bool>,
    #[xml(
        child = "p:cSld",
        child = "p:clrMap",
        child = "p:sldLayoutIdLst",
        child = "p:transition",
        child = "p:timing",
        child = "p:hf",
        child = "p:txStyles",
        child = "p:extLst",
    )]
    pub children: Vec<SlideMasterChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideMasterChildChoice {
    #[xml(tag = "p:cSld")]
    PCSld(CommonSlideData),
    #[xml(tag = "p:clrMap")]
    PClrMap(ColorMap),
    #[xml(tag = "p:sldLayoutIdLst")]
    PSldLayoutIdLst(SlideLayoutIdList),
    #[xml(tag = "p:transition")]
    PTransition(Transition),
    #[xml(tag = "p:timing")]
    PTiming(Timing),
    #[xml(tag = "p:hf")]
    PHf(HeaderFooter),
    #[xml(tag = "p:txStyles")]
    PTxStyles(TextStyles),
    #[xml(tag = "p:extLst")]
    PExtLst(SlideMasterExtensionList),
}
mod slide_master_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Handout Master.
/// When the object is serialized out as xml, it's qualified name is p:handoutMaster.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:handoutMaster")]
pub struct HandoutMaster {
    #[xml(attr = "xmlns", with = "handout_master_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "p:cSld", child = "p:clrMap", child = "p:hf", child = "p:extLst")]
    pub children: Vec<HandoutMasterChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HandoutMasterChildChoice {
    #[xml(tag = "p:cSld")]
    PCSld(CommonSlideData),
    #[xml(tag = "p:clrMap")]
    PClrMap(ColorMap),
    #[xml(tag = "p:hf")]
    PHf(HeaderFooter),
    #[xml(tag = "p:extLst")]
    PExtLst(HandoutMasterExtensionList),
}
mod handout_master_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Notes Master.
/// When the object is serialized out as xml, it's qualified name is p:notesMaster.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesMaster")]
pub struct NotesMaster {
    #[xml(attr = "xmlns", with = "notes_master_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "p:cSld",
        child = "p:clrMap",
        child = "p:hf",
        child = "p:notesStyle",
        child = "p:extLst",
    )]
    pub children: Vec<NotesMasterChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NotesMasterChildChoice {
    #[xml(tag = "p:cSld")]
    PCSld(CommonSlideData),
    #[xml(tag = "p:clrMap")]
    PClrMap(ColorMap),
    #[xml(tag = "p:hf")]
    PHf(HeaderFooter),
    #[xml(tag = "p:notesStyle")]
    PNotesStyle(NotesStyle),
    #[xml(tag = "p:extLst")]
    PExtLst(NotesMasterExtensionList),
}
mod notes_master_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Notes Slide.
/// When the object is serialized out as xml, it's qualified name is p:notes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notes")]
pub struct NotesSlide {
    #[xml(attr = "xmlns", with = "notes_slide_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Show Master Shapes
    /// Represents the following attribute in the schema: :showMasterSp
    #[xml(attr = "showMasterSp")]
    pub show_master_shapes: Option<bool>,
    /// Show Master Placeholder Animations
    /// Represents the following attribute in the schema: :showMasterPhAnim
    #[xml(attr = "showMasterPhAnim")]
    pub show_master_placeholder_animations: Option<bool>,
    #[xml(child = "p:cSld", child = "p:clrMapOvr", child = "p:extLst")]
    pub children: Vec<NotesSlideChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NotesSlideChildChoice {
    #[xml(tag = "p:cSld")]
    PCSld(CommonSlideData),
    #[xml(tag = "p:clrMapOvr")]
    PClrMapOvr(ColorMapOverride),
    #[xml(tag = "p:extLst")]
    PExtLst(ExtensionListWithModification),
}
mod notes_slide_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Slide Synchronization Properties.
/// When the object is serialized out as xml, it's qualified name is p:sldSyncPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldSyncPr")]
pub struct SlideSyncProperties {
    #[xml(attr = "xmlns", with = "slide_sync_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Server's Slide File ID
    /// Represents the following attribute in the schema: :serverSldId
    #[xml(attr = "serverSldId")]
    pub server_slide_id: String,
    /// Server's Slide File's modification date/time
    /// Represents the following attribute in the schema: :serverSldModifiedTime
    #[xml(attr = "serverSldModifiedTime")]
    pub server_slide_modified_time: String,
    /// Client Slide Insertion date/time
    /// Represents the following attribute in the schema: :clientInsertedTime
    #[xml(attr = "clientInsertedTime")]
    pub client_inserted_time: String,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod slide_sync_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Programmable Tab List.
/// When the object is serialized out as xml, it's qualified name is p:tagLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tagLst")]
pub struct TagList {
    #[xml(attr = "xmlns", with = "tag_list_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "p:tag")]
    pub p_tag: Vec<Tag>,
}
mod tag_list_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Presentation-wide View Properties.
/// When the object is serialized out as xml, it's qualified name is p:viewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:viewPr")]
pub struct ViewProperties {
    #[xml(attr = "xmlns", with = "view_properties_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Last View
    /// Represents the following attribute in the schema: :lastView
    #[xml(attr = "lastView")]
    pub last_view: Option<ViewValues>,
    /// Show Comments
    /// Represents the following attribute in the schema: :showComments
    #[xml(attr = "showComments")]
    pub show_comments: Option<bool>,
    ///Normal View Properties
    #[xml(child = "p:normalViewPr")]
    pub normal_view_properties: Option<NormalViewProperties>,
    ///Slide View Properties
    #[xml(child = "p:slideViewPr")]
    pub slide_view_properties: Option<SlideViewProperties>,
    ///Outline View Properties
    #[xml(child = "p:outlineViewPr")]
    pub outline_view_properties: Option<OutlineViewProperties>,
    ///Notes Text View Properties
    #[xml(child = "p:notesTextViewPr")]
    pub notes_text_view_properties: Option<NotesTextViewProperties>,
    ///Slide Sorter View Properties
    #[xml(child = "p:sorterViewPr")]
    pub sorter_view_properties: Option<SorterViewProperties>,
    ///Notes View Properties
    #[xml(child = "p:notesViewPr")]
    pub notes_view_properties: Option<NotesViewProperties>,
    ///Grid Spacing
    #[xml(child = "p:gridSpacing")]
    pub grid_spacing: Option<GridSpacing>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
mod view_properties_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/presentationml/2006/main")
    }
}
/// Defines the ContentPart Class.
/// When the object is serialized out as xml, it's qualified name is p:contentPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:contentPart")]
pub struct ContentPart {
    /// bwMode
    /// Represents the following attribute in the schema: p14:bwMode
    #[xml(attr = "p14:bwMode")]
    pub p14_bw_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// _
    #[xml(child = "p14:nvContentPartPr")]
    pub non_visual_content_part_properties: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::NonVisualContentPartProperties,
    >,
    /// _
    #[xml(child = "p14:xfrm")]
    pub transform2_d: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Transform2D,
    >,
    /// _
    #[xml(child = "p14:extLst")]
    pub extension_list_modify: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ExtensionListModify,
    >,
}
/// Sound.
/// When the object is serialized out as xml, it's qualified name is p:snd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:snd")]
pub struct Sound {
    /// Embedded Audio File Relationship ID
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: String,
    /// Sound Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Recognized Built-In Sound
    /// Represents the following attribute in the schema: :builtIn
    #[xml(attr = "builtIn")]
    pub built_in: Option<bool>,
}
/// Sound Target.
/// When the object is serialized out as xml, it's qualified name is p:sndTgt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sndTgt")]
pub struct SoundTarget {
    /// Embedded Audio File Relationship ID
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: String,
    /// Sound Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Recognized Built-In Sound
    /// Represents the following attribute in the schema: :builtIn
    #[xml(attr = "builtIn")]
    pub built_in: Option<bool>,
}
/// Defines the EmbeddedWavAudioFileType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmbeddedWavAudioFileType {
    /// Embedded Audio File Relationship ID
    /// Represents the following attribute in the schema: r:embed
    #[xml(attr = "r:embed")]
    pub embed: String,
    /// Sound Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Recognized Built-In Sound
    /// Represents the following attribute in the schema: :builtIn
    #[xml(attr = "builtIn")]
    pub built_in: Option<bool>,
}
/// Start Sound Action.
/// When the object is serialized out as xml, it's qualified name is p:stSnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:stSnd")]
pub struct StartSoundAction {
    /// Loop Sound
    /// Represents the following attribute in the schema: :loop
    #[xml(attr = "loop")]
    pub r#loop: Option<bool>,
    ///Sound
    #[xml(child = "p:snd")]
    pub sound: Sound,
}
/// Time Absolute.
/// When the object is serialized out as xml, it's qualified name is p:tmAbs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tmAbs")]
pub struct TimeAbsolute {
    /// Time
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Time Percentage.
/// When the object is serialized out as xml, it's qualified name is p:tmPct.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tmPct")]
pub struct TimePercentage {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Target Element Trigger Choice.
/// When the object is serialized out as xml, it's qualified name is p:tgtEl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tgtEl")]
pub struct TargetElement {
    #[xml(
        child = "p:sldTgt",
        child = "p:sndTgt",
        child = "p:spTgt",
        child = "p:inkTgt",
        child = "p14:bmkTgt",
    )]
    pub children: Vec<TargetElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TargetElementChildChoice {
    #[xml(tag = "p:sldTgt")]
    PSldTgt(SlideTarget),
    #[xml(tag = "p:sndTgt")]
    PSndTgt(SoundTarget),
    #[xml(tag = "p:spTgt")]
    PSpTgt(ShapeTarget),
    #[xml(tag = "p:inkTgt")]
    PInkTgt(InkTarget),
    #[xml(tag = "p14:bmkTgt")]
    P14BmkTgt(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::BookmarkTarget,
    ),
}
/// Time Node.
/// When the object is serialized out as xml, it's qualified name is p:tn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tn")]
pub struct TimeNode {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: u32,
}
/// Runtime Node Trigger Choice.
/// When the object is serialized out as xml, it's qualified name is p:rtn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:rtn")]
pub struct RuntimeNodeTrigger {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: TriggerRuntimeNodeValues,
}
/// Condition.
/// When the object is serialized out as xml, it's qualified name is p:cond.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cond")]
pub struct Condition {
    /// Trigger Event
    /// Represents the following attribute in the schema: :evt
    #[xml(attr = "evt")]
    pub event: Option<TriggerEventValues>,
    /// Trigger Delay
    /// Represents the following attribute in the schema: :delay
    #[xml(attr = "delay")]
    pub delay: Option<String>,
    #[xml(child = "p:tgtEl", child = "p:tn", child = "p:rtn")]
    pub children: Vec<ConditionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ConditionChildChoice {
    #[xml(tag = "p:tgtEl")]
    PTgtEl(TargetElement),
    #[xml(tag = "p:tn")]
    PTn(TimeNode),
    #[xml(tag = "p:rtn")]
    PRtn(RuntimeNodeTrigger),
}
/// Defines the EndSync Class.
/// When the object is serialized out as xml, it's qualified name is p:endSync.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:endSync")]
pub struct EndSync {
    /// Trigger Event
    /// Represents the following attribute in the schema: :evt
    #[xml(attr = "evt")]
    pub event: Option<TriggerEventValues>,
    /// Trigger Delay
    /// Represents the following attribute in the schema: :delay
    #[xml(attr = "delay")]
    pub delay: Option<String>,
    #[xml(child = "p:tgtEl", child = "p:tn", child = "p:rtn")]
    pub children: Vec<EndSyncChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndSyncChildChoice {
    #[xml(tag = "p:tgtEl")]
    PTgtEl(TargetElement),
    #[xml(tag = "p:tn")]
    PTn(TimeNode),
    #[xml(tag = "p:rtn")]
    PRtn(RuntimeNodeTrigger),
}
/// Defines the TimeListConditionalType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeListConditionalType {
    /// Trigger Event
    /// Represents the following attribute in the schema: :evt
    #[xml(attr = "evt")]
    pub event: Option<TriggerEventValues>,
    /// Trigger Delay
    /// Represents the following attribute in the schema: :delay
    #[xml(attr = "delay")]
    pub delay: Option<String>,
    #[xml(child = "p:tgtEl", child = "p:tn", child = "p:rtn")]
    pub children: Vec<TimeListConditionalTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TimeListConditionalTypeChildChoice {
    #[xml(tag = "p:tgtEl")]
    PTgtEl(TargetElement),
    #[xml(tag = "p:tn")]
    PTn(TimeNode),
    #[xml(tag = "p:rtn")]
    PRtn(RuntimeNodeTrigger),
}
/// Parallel Time Node.
/// When the object is serialized out as xml, it's qualified name is p:par.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:par")]
pub struct ParallelTimeNode {
    ///Parallel TimeNode
    #[xml(child = "p:cTn")]
    pub common_time_node: CommonTimeNode,
}
/// Sequence Time Node.
/// When the object is serialized out as xml, it's qualified name is p:seq.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:seq")]
pub struct SequenceTimeNode {
    /// Concurrent
    /// Represents the following attribute in the schema: :concurrent
    #[xml(attr = "concurrent")]
    pub concurrent: Option<bool>,
    /// Previous Action
    /// Represents the following attribute in the schema: :prevAc
    #[xml(attr = "prevAc")]
    pub previous_action: Option<PreviousActionValues>,
    /// Next Action
    /// Represents the following attribute in the schema: :nextAc
    #[xml(attr = "nextAc")]
    pub next_action: Option<NextActionValues>,
    ///Common TimeNode Properties
    #[xml(child = "p:cTn")]
    pub common_time_node: CommonTimeNode,
    ///Previous Conditions List
    #[xml(child = "p:prevCondLst")]
    pub previous_condition_list: Option<PreviousConditionList>,
    ///Next Conditions List
    #[xml(child = "p:nextCondLst")]
    pub next_condition_list: Option<NextConditionList>,
}
/// Exclusive.
/// When the object is serialized out as xml, it's qualified name is p:excl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:excl")]
pub struct ExclusiveTimeNode {
    ///Common TimeNode Properties
    #[xml(child = "p:cTn")]
    pub common_time_node: CommonTimeNode,
}
/// Animate.
/// When the object is serialized out as xml, it's qualified name is p:anim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:anim")]
pub struct Animate {
    /// by
    /// Represents the following attribute in the schema: :by
    #[xml(attr = "by")]
    pub by: Option<String>,
    /// from
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: Option<String>,
    /// to
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: Option<String>,
    /// calcmode
    /// Represents the following attribute in the schema: :calcmode
    #[xml(attr = "calcmode")]
    pub calculation_mode: Option<AnimateBehaviorCalculateModeValues>,
    /// valueType
    /// Represents the following attribute in the schema: :valueType
    #[xml(attr = "valueType")]
    pub value_type: Option<AnimateBehaviorValues>,
    /// bounceEnd
    /// Represents the following attribute in the schema: p14:bounceEnd
    #[xml(attr = "p14:bounceEnd")]
    pub bounce_end: Option<i32>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
    /// _
    #[xml(child = "p:tavLst")]
    pub time_animate_value_list: Option<TimeAnimateValueList>,
}
/// Animate Color Behavior.
/// When the object is serialized out as xml, it's qualified name is p:animClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:animClr")]
pub struct AnimateColor {
    /// Color Space
    /// Represents the following attribute in the schema: :clrSpc
    #[xml(attr = "clrSpc")]
    pub color_space: Option<AnimateColorSpaceValues>,
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<AnimateColorDirectionValues>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
    ///By
    #[xml(child = "p:by")]
    pub by_color: Option<ByColor>,
    ///From
    #[xml(child = "p:from")]
    pub from_color: Option<FromColor>,
    ///To
    #[xml(child = "p:to")]
    pub to_color: Option<ToColor>,
}
/// Animate Effect.
/// When the object is serialized out as xml, it's qualified name is p:animEffect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:animEffect")]
pub struct AnimateEffect {
    /// Transition
    /// Represents the following attribute in the schema: :transition
    #[xml(attr = "transition")]
    pub transition: Option<AnimateEffectTransitionValues>,
    /// Filter
    /// Represents the following attribute in the schema: :filter
    #[xml(attr = "filter")]
    pub filter: Option<String>,
    /// Property List
    /// Represents the following attribute in the schema: :prLst
    #[xml(attr = "prLst")]
    pub property_list: Option<String>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
    ///Progress
    #[xml(child = "p:progress")]
    pub progress: Option<Progress>,
}
/// Animate Motion.
/// When the object is serialized out as xml, it's qualified name is p:animMotion.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:animMotion")]
pub struct AnimateMotion {
    /// origin
    /// Represents the following attribute in the schema: :origin
    #[xml(attr = "origin")]
    pub origin: Option<AnimateMotionBehaviorOriginValues>,
    /// path
    /// Represents the following attribute in the schema: :path
    #[xml(attr = "path")]
    pub path: Option<String>,
    /// pathEditMode
    /// Represents the following attribute in the schema: :pathEditMode
    #[xml(attr = "pathEditMode")]
    pub path_edit_mode: Option<AnimateMotionPathEditModeValues>,
    /// rAng
    /// Represents the following attribute in the schema: :rAng
    #[xml(attr = "rAng")]
    pub relative_angle: Option<i32>,
    /// ptsTypes
    /// Represents the following attribute in the schema: :ptsTypes
    #[xml(attr = "ptsTypes")]
    pub point_types: Option<String>,
    /// bounceEnd
    /// Represents the following attribute in the schema: p14:bounceEnd
    #[xml(attr = "p14:bounceEnd")]
    pub bounce_end: Option<i32>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
    /// _
    #[xml(child = "p:by")]
    pub by_position: Option<ByPosition>,
    /// _
    #[xml(child = "p:from")]
    pub from_position: Option<FromPosition>,
    /// _
    #[xml(child = "p:to")]
    pub to_position: Option<ToPosition>,
    /// _
    #[xml(child = "p:rCtr")]
    pub rotation_center: Option<RotationCenter>,
}
/// Animate Rotation.
/// When the object is serialized out as xml, it's qualified name is p:animRot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:animRot")]
pub struct AnimateRotation {
    /// by
    /// Represents the following attribute in the schema: :by
    #[xml(attr = "by")]
    pub by: Option<i32>,
    /// from
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: Option<i32>,
    /// to
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: Option<i32>,
    /// bounceEnd
    /// Represents the following attribute in the schema: p14:bounceEnd
    #[xml(attr = "p14:bounceEnd")]
    pub bounce_end: Option<i32>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
}
/// Animate Scale.
/// When the object is serialized out as xml, it's qualified name is p:animScale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:animScale")]
pub struct AnimateScale {
    /// zoomContents
    /// Represents the following attribute in the schema: :zoomContents
    #[xml(attr = "zoomContents")]
    pub zoom_contents: Option<bool>,
    /// bounceEnd
    /// Represents the following attribute in the schema: p14:bounceEnd
    #[xml(attr = "p14:bounceEnd")]
    pub bounce_end: Option<i32>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
    /// _
    #[xml(child = "p:by")]
    pub by_position: Option<ByPosition>,
    /// _
    #[xml(child = "p:from")]
    pub from_position: Option<FromPosition>,
    /// _
    #[xml(child = "p:to")]
    pub to_position: Option<ToPosition>,
}
/// Command.
/// When the object is serialized out as xml, it's qualified name is p:cmd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cmd")]
pub struct Command {
    /// Command Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<CommandValues>,
    /// Command
    /// Represents the following attribute in the schema: :cmd
    #[xml(attr = "cmd")]
    pub command_name: Option<String>,
    /// _
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
}
/// Set Time Node Behavior.
/// When the object is serialized out as xml, it's qualified name is p:set.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:set")]
pub struct SetBehavior {
    ///Common Behavior
    #[xml(child = "p:cBhvr")]
    pub common_behavior: CommonBehavior,
    ///To
    #[xml(child = "p:to")]
    pub to_variant_value: Option<ToVariantValue>,
}
/// Audio.
/// When the object is serialized out as xml, it's qualified name is p:audio.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:audio")]
pub struct Audio {
    /// Is Narration
    /// Represents the following attribute in the schema: :isNarration
    #[xml(attr = "isNarration")]
    pub is_narration: Option<bool>,
    ///Common Media Node Properties
    #[xml(child = "p:cMediaNode")]
    pub common_media_node: CommonMediaNode,
}
/// Video.
/// When the object is serialized out as xml, it's qualified name is p:video.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:video")]
pub struct Video {
    /// Full Screen
    /// Represents the following attribute in the schema: :fullScrn
    #[xml(attr = "fullScrn")]
    pub full_screen: Option<bool>,
    ///Common Media Node Properties
    #[xml(child = "p:cMediaNode")]
    pub common_media_node: CommonMediaNode,
}
/// Parallel TimeNode.
/// When the object is serialized out as xml, it's qualified name is p:cTn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cTn")]
pub struct CommonTimeNode {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<u32>,
    /// presetID
    /// Represents the following attribute in the schema: :presetID
    #[xml(attr = "presetID")]
    pub preset_id: Option<i32>,
    /// presetClass
    /// Represents the following attribute in the schema: :presetClass
    #[xml(attr = "presetClass")]
    pub preset_class: Option<TimeNodePresetClassValues>,
    /// presetSubtype
    /// Represents the following attribute in the schema: :presetSubtype
    #[xml(attr = "presetSubtype")]
    pub preset_subtype: Option<i32>,
    /// dur
    /// Represents the following attribute in the schema: :dur
    #[xml(attr = "dur")]
    pub duration: Option<String>,
    /// repeatCount
    /// Represents the following attribute in the schema: :repeatCount
    #[xml(attr = "repeatCount")]
    pub repeat_count: Option<String>,
    /// repeatDur
    /// Represents the following attribute in the schema: :repeatDur
    #[xml(attr = "repeatDur")]
    pub repeat_duration: Option<String>,
    /// spd
    /// Represents the following attribute in the schema: :spd
    #[xml(attr = "spd")]
    pub speed: Option<i32>,
    /// accel
    /// Represents the following attribute in the schema: :accel
    #[xml(attr = "accel")]
    pub acceleration: Option<i32>,
    /// decel
    /// Represents the following attribute in the schema: :decel
    #[xml(attr = "decel")]
    pub deceleration: Option<i32>,
    /// autoRev
    /// Represents the following attribute in the schema: :autoRev
    #[xml(attr = "autoRev")]
    pub auto_reverse: Option<bool>,
    /// restart
    /// Represents the following attribute in the schema: :restart
    #[xml(attr = "restart")]
    pub restart: Option<TimeNodeRestartValues>,
    /// fill
    /// Represents the following attribute in the schema: :fill
    #[xml(attr = "fill")]
    pub fill: Option<TimeNodeFillValues>,
    /// syncBehavior
    /// Represents the following attribute in the schema: :syncBehavior
    #[xml(attr = "syncBehavior")]
    pub sync_behavior: Option<TimeNodeSyncValues>,
    /// tmFilter
    /// Represents the following attribute in the schema: :tmFilter
    #[xml(attr = "tmFilter")]
    pub time_filter: Option<String>,
    /// evtFilter
    /// Represents the following attribute in the schema: :evtFilter
    #[xml(attr = "evtFilter")]
    pub event_filter: Option<String>,
    /// display
    /// Represents the following attribute in the schema: :display
    #[xml(attr = "display")]
    pub display: Option<bool>,
    /// masterRel
    /// Represents the following attribute in the schema: :masterRel
    #[xml(attr = "masterRel")]
    pub master_relation: Option<TimeNodeMasterRelationValues>,
    /// bldLvl
    /// Represents the following attribute in the schema: :bldLvl
    #[xml(attr = "bldLvl")]
    pub build_level: Option<i32>,
    /// grpId
    /// Represents the following attribute in the schema: :grpId
    #[xml(attr = "grpId")]
    pub group_id: Option<u32>,
    /// afterEffect
    /// Represents the following attribute in the schema: :afterEffect
    #[xml(attr = "afterEffect")]
    pub after_effect: Option<bool>,
    /// nodeType
    /// Represents the following attribute in the schema: :nodeType
    #[xml(attr = "nodeType")]
    pub node_type: Option<TimeNodeValues>,
    /// nodePh
    /// Represents the following attribute in the schema: :nodePh
    #[xml(attr = "nodePh")]
    pub node_placeholder: Option<bool>,
    /// presetBounceEnd
    /// Represents the following attribute in the schema: p14:presetBounceEnd
    #[xml(attr = "p14:presetBounceEnd")]
    pub preset_bounce_end: Option<i32>,
    /// _
    #[xml(child = "p:stCondLst")]
    pub start_condition_list: Option<StartConditionList>,
    /// _
    #[xml(child = "p:endCondLst")]
    pub end_condition_list: Option<EndConditionList>,
    /// _
    #[xml(child = "p:endSync")]
    pub end_sync: Option<EndSync>,
    /// _
    #[xml(child = "p:iterate")]
    pub iterate: Option<Iterate>,
    /// _
    #[xml(child = "p:childTnLst")]
    pub child_time_node_list: Option<ChildTimeNodeList>,
    /// _
    #[xml(child = "p:subTnLst")]
    pub sub_time_node_list: Option<SubTimeNodeList>,
}
/// Previous Conditions List.
/// When the object is serialized out as xml, it's qualified name is p:prevCondLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:prevCondLst")]
pub struct PreviousConditionList {
    #[xml(child = "p:cond")]
    pub children: Vec<PreviousConditionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousConditionListChildChoice {
    #[xml(tag = "p:cond")]
    PCond(Condition),
}
/// Next Conditions List.
/// When the object is serialized out as xml, it's qualified name is p:nextCondLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nextCondLst")]
pub struct NextConditionList {
    #[xml(child = "p:cond")]
    pub children: Vec<NextConditionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NextConditionListChildChoice {
    #[xml(tag = "p:cond")]
    PCond(Condition),
}
/// Defines the StartConditionList Class.
/// When the object is serialized out as xml, it's qualified name is p:stCondLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:stCondLst")]
pub struct StartConditionList {
    #[xml(child = "p:cond")]
    pub children: Vec<StartConditionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StartConditionListChildChoice {
    #[xml(tag = "p:cond")]
    PCond(Condition),
}
/// Defines the EndConditionList Class.
/// When the object is serialized out as xml, it's qualified name is p:endCondLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:endCondLst")]
pub struct EndConditionList {
    #[xml(child = "p:cond")]
    pub children: Vec<EndConditionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndConditionListChildChoice {
    #[xml(tag = "p:cond")]
    PCond(Condition),
}
/// Defines the TimeListTimeConditionalListType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeListTimeConditionalListType {
    #[xml(child = "p:cond")]
    pub children: Vec<TimeListTimeConditionalListTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TimeListTimeConditionalListTypeChildChoice {
    #[xml(tag = "p:cond")]
    PCond(Condition),
}
/// Attribute Name.
/// When the object is serialized out as xml, it's qualified name is p:attrName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:attrName")]
pub struct AttributeName {
    #[xml(text)]
    pub child: String,
}
/// Defines the Text Class.
/// When the object is serialized out as xml, it's qualified name is p:text.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:text")]
pub struct Text {
    #[xml(text)]
    pub child: String,
}
/// Attribute Name List.
/// When the object is serialized out as xml, it's qualified name is p:attrNameLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:attrNameLst")]
pub struct AttributeNameList {
    /// _
    #[xml(child = "p:attrName")]
    pub p_attr_name: Vec<AttributeName>,
}
/// Boolean Variant.
/// When the object is serialized out as xml, it's qualified name is p:boolVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:boolVal")]
pub struct BooleanVariantValue {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: bool,
}
/// Integer.
/// When the object is serialized out as xml, it's qualified name is p:intVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:intVal")]
pub struct IntegerVariantValue {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Float Value.
/// When the object is serialized out as xml, it's qualified name is p:fltVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:fltVal")]
pub struct FloatVariantValue {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f32,
}
/// String Value.
/// When the object is serialized out as xml, it's qualified name is p:strVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:strVal")]
pub struct StringVariantValue {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Color Value.
/// When the object is serialized out as xml, it's qualified name is p:clrVal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:clrVal")]
pub struct ColorValue {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorValueChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorValueChildChoice {
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
/// Pen Color for Slide Show.
/// When the object is serialized out as xml, it's qualified name is p:penClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:penClr")]
pub struct PenColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<PenColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PenColorChildChoice {
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
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorType {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorTypeChildChoice {
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
/// Time Animate Value.
/// When the object is serialized out as xml, it's qualified name is p:tav.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tav")]
pub struct TimeAnimateValue {
    /// Time
    /// Represents the following attribute in the schema: :tm
    #[xml(attr = "tm")]
    pub time: Option<String>,
    /// Formula
    /// Represents the following attribute in the schema: :fmla
    #[xml(attr = "fmla")]
    pub fomula: Option<String>,
    ///Value
    #[xml(child = "p:val")]
    pub variant_value: Option<VariantValue>,
}
/// RGB.
/// When the object is serialized out as xml, it's qualified name is p:rgb.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:rgb")]
pub struct RgbColor {
    /// Red
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub red: i32,
    /// Green
    /// Represents the following attribute in the schema: :g
    #[xml(attr = "g")]
    pub green: i32,
    /// Blue
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub blue: i32,
}
/// HSL.
/// When the object is serialized out as xml, it's qualified name is p:hsl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:hsl")]
pub struct HslColor {
    /// Hue
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub hue: i32,
    /// Saturation
    /// Represents the following attribute in the schema: :s
    #[xml(attr = "s")]
    pub saturation: i32,
    /// Lightness
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub lightness: i32,
}
/// Defines the CommonBehavior Class.
/// When the object is serialized out as xml, it's qualified name is p:cBhvr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cBhvr")]
pub struct CommonBehavior {
    /// Additive
    /// Represents the following attribute in the schema: :additive
    #[xml(attr = "additive")]
    pub additive: Option<BehaviorAdditiveValues>,
    /// Accumulate
    /// Represents the following attribute in the schema: :accumulate
    #[xml(attr = "accumulate")]
    pub accumulate: Option<BehaviorAccumulateValues>,
    /// Transform Type
    /// Represents the following attribute in the schema: :xfrmType
    #[xml(attr = "xfrmType")]
    pub transform_type: Option<BehaviorTransformValues>,
    /// From
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: Option<String>,
    /// To
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: Option<String>,
    /// By
    /// Represents the following attribute in the schema: :by
    #[xml(attr = "by")]
    pub by: Option<String>,
    /// Runtime Context
    /// Represents the following attribute in the schema: :rctx
    #[xml(attr = "rctx")]
    pub runtime_context: Option<String>,
    /// Override
    /// Represents the following attribute in the schema: :override
    #[xml(attr = "override")]
    pub r#override: Option<BehaviorOverrideValues>,
    /// _
    #[xml(child = "p:cTn")]
    pub common_time_node: CommonTimeNode,
    ///Target Element
    #[xml(child = "p:tgtEl")]
    pub target_element: TargetElement,
    ///Attribute Name List
    #[xml(child = "p:attrNameLst")]
    pub attribute_name_list: Option<AttributeNameList>,
}
/// Progress.
/// When the object is serialized out as xml, it's qualified name is p:progress.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:progress")]
pub struct Progress {
    #[xml(child = "p:fltVal")]
    pub children: Vec<ProgressChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ProgressChildChoice {
    #[xml(tag = "p:fltVal")]
    PFltVal(FloatVariantValue),
}
/// To.
/// When the object is serialized out as xml, it's qualified name is p:to.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:to")]
pub struct ToVariantValue {
    #[xml(
        child = "p:boolVal",
        child = "p:intVal",
        child = "p:fltVal",
        child = "p:strVal",
        child = "p:clrVal",
    )]
    pub children: Vec<ToVariantValueChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ToVariantValueChildChoice {
    #[xml(tag = "p:boolVal")]
    PBoolVal(BooleanVariantValue),
    #[xml(tag = "p:intVal")]
    PIntVal(IntegerVariantValue),
    #[xml(tag = "p:fltVal")]
    PFltVal(FloatVariantValue),
    #[xml(tag = "p:strVal")]
    PStrVal(StringVariantValue),
    #[xml(tag = "p:clrVal")]
    PClrVal(ColorValue),
}
/// Value.
/// When the object is serialized out as xml, it's qualified name is p:val.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:val")]
pub struct VariantValue {
    #[xml(
        child = "p:boolVal",
        child = "p:intVal",
        child = "p:fltVal",
        child = "p:strVal",
        child = "p:clrVal",
    )]
    pub children: Vec<VariantValueChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum VariantValueChildChoice {
    #[xml(tag = "p:boolVal")]
    PBoolVal(BooleanVariantValue),
    #[xml(tag = "p:intVal")]
    PIntVal(IntegerVariantValue),
    #[xml(tag = "p:fltVal")]
    PFltVal(FloatVariantValue),
    #[xml(tag = "p:strVal")]
    PStrVal(StringVariantValue),
    #[xml(tag = "p:clrVal")]
    PClrVal(ColorValue),
}
/// Defines the TimeListAnimationVariantType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeListAnimationVariantType {
    #[xml(
        child = "p:boolVal",
        child = "p:intVal",
        child = "p:fltVal",
        child = "p:strVal",
        child = "p:clrVal",
    )]
    pub children: Vec<TimeListAnimationVariantTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TimeListAnimationVariantTypeChildChoice {
    #[xml(tag = "p:boolVal")]
    PBoolVal(BooleanVariantValue),
    #[xml(tag = "p:intVal")]
    PIntVal(IntegerVariantValue),
    #[xml(tag = "p:fltVal")]
    PFltVal(FloatVariantValue),
    #[xml(tag = "p:strVal")]
    PStrVal(StringVariantValue),
    #[xml(tag = "p:clrVal")]
    PClrVal(ColorValue),
}
/// Common Media Node Properties.
/// When the object is serialized out as xml, it's qualified name is p:cMediaNode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cMediaNode")]
pub struct CommonMediaNode {
    /// Volume
    /// Represents the following attribute in the schema: :vol
    #[xml(attr = "vol")]
    pub volume: Option<i32>,
    /// Mute
    /// Represents the following attribute in the schema: :mute
    #[xml(attr = "mute")]
    pub mute: Option<bool>,
    /// Number of Slides
    /// Represents the following attribute in the schema: :numSld
    #[xml(attr = "numSld")]
    pub slide_count: Option<u32>,
    /// Show When Stopped
    /// Represents the following attribute in the schema: :showWhenStopped
    #[xml(attr = "showWhenStopped")]
    pub show_when_stopped: Option<bool>,
    ///Common Time Node Properties
    #[xml(child = "p:cTn")]
    pub common_time_node: CommonTimeNode,
    /// _
    #[xml(child = "p:tgtEl")]
    pub target_element: TargetElement,
}
/// Time Node List.
/// When the object is serialized out as xml, it's qualified name is p:tnLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tnLst")]
pub struct TimeNodeList {
    /// _
    #[xml(child = "p:par")]
    pub parallel_time_node: ParallelTimeNode,
}
/// Template Effects.
/// When the object is serialized out as xml, it's qualified name is p:tmpl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tmpl")]
pub struct Template {
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<u32>,
    ///Time Node List
    #[xml(child = "p:tnLst")]
    pub time_node_list: TimeNodeList,
}
/// Template effects.
/// When the object is serialized out as xml, it's qualified name is p:tmplLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tmplLst")]
pub struct TemplateList {
    /// _
    #[xml(child = "p:tmpl")]
    pub p_tmpl: Vec<Template>,
}
/// Build Sub Elements.
/// When the object is serialized out as xml, it's qualified name is p:bldSub.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldSub")]
pub struct BuildSubElement {
    #[xml(child = "a:bldDgm", child = "a:bldChart")]
    pub children: Vec<BuildSubElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BuildSubElementChildChoice {
    #[xml(tag = "a:bldDgm")]
    ABldDgm(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BuildDiagram,
    ),
    #[xml(tag = "a:bldChart")]
    ABldChart(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BuildChart,
    ),
}
/// Build Paragraph.
/// When the object is serialized out as xml, it's qualified name is p:bldP.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldP")]
pub struct BuildParagraph {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
    /// Group ID
    /// Represents the following attribute in the schema: :grpId
    #[xml(attr = "grpId")]
    pub group_id: u32,
    /// Expand UI
    /// Represents the following attribute in the schema: :uiExpand
    #[xml(attr = "uiExpand")]
    pub ui_expand: Option<bool>,
    /// Build Types
    /// Represents the following attribute in the schema: :build
    #[xml(attr = "build")]
    pub build: Option<ParagraphBuildValues>,
    /// Build Level
    /// Represents the following attribute in the schema: :bldLvl
    #[xml(attr = "bldLvl")]
    pub build_level: Option<u32>,
    /// Animate Background
    /// Represents the following attribute in the schema: :animBg
    #[xml(attr = "animBg")]
    pub animate_background: Option<bool>,
    /// Auto Update Animation Background
    /// Represents the following attribute in the schema: :autoUpdateAnimBg
    #[xml(attr = "autoUpdateAnimBg")]
    pub auto_animate_background: Option<bool>,
    /// Reverse
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub reverse: Option<bool>,
    /// Auto Advance Time
    /// Represents the following attribute in the schema: :advAuto
    #[xml(attr = "advAuto")]
    pub auto_advance: Option<String>,
    ///Template effects
    #[xml(child = "p:tmplLst")]
    pub template_list: Option<TemplateList>,
}
/// Build Diagram.
/// When the object is serialized out as xml, it's qualified name is p:bldDgm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldDgm")]
pub struct BuildDiagram {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
    /// Group ID
    /// Represents the following attribute in the schema: :grpId
    #[xml(attr = "grpId")]
    pub group_id: u32,
    /// Expand UI
    /// Represents the following attribute in the schema: :uiExpand
    #[xml(attr = "uiExpand")]
    pub ui_expand: Option<bool>,
    /// Diagram Build Types
    /// Represents the following attribute in the schema: :bld
    #[xml(attr = "bld")]
    pub build: Option<DiagramBuildValues>,
}
/// Build OLE Chart.
/// When the object is serialized out as xml, it's qualified name is p:bldOleChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldOleChart")]
pub struct BuildOleChart {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
    /// Group ID
    /// Represents the following attribute in the schema: :grpId
    #[xml(attr = "grpId")]
    pub group_id: u32,
    /// Expand UI
    /// Represents the following attribute in the schema: :uiExpand
    #[xml(attr = "uiExpand")]
    pub ui_expand: Option<bool>,
    /// Build
    /// Represents the following attribute in the schema: :bld
    #[xml(attr = "bld")]
    pub build: Option<OleChartBuildValues>,
    /// Animate Background
    /// Represents the following attribute in the schema: :animBg
    #[xml(attr = "animBg")]
    pub animate_background: Option<bool>,
}
/// Build Graphics.
/// When the object is serialized out as xml, it's qualified name is p:bldGraphic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldGraphic")]
pub struct BuildGraphics {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
    /// Group ID
    /// Represents the following attribute in the schema: :grpId
    #[xml(attr = "grpId")]
    pub group_id: u32,
    /// Expand UI
    /// Represents the following attribute in the schema: :uiExpand
    #[xml(attr = "uiExpand")]
    pub ui_expand: Option<bool>,
    #[xml(child = "p:bldAsOne", child = "p:bldSub")]
    pub children: Vec<BuildGraphicsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BuildGraphicsChildChoice {
    #[xml(tag = "p:bldAsOne")]
    PBldAsOne(BuildAsOne),
    #[xml(tag = "p:bldSub")]
    PBldSub(BuildSubElement),
}
/// Build List.
/// When the object is serialized out as xml, it's qualified name is p:bldLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bldLst")]
pub struct BuildList {
    #[xml(
        child = "p:bldP",
        child = "p:bldDgm",
        child = "p:bldOleChart",
        child = "p:bldGraphic",
    )]
    pub children: Vec<BuildListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BuildListChildChoice {
    #[xml(tag = "p:bldP")]
    PBldP(BuildParagraph),
    #[xml(tag = "p:bldDgm")]
    PBldDgm(BuildDiagram),
    #[xml(tag = "p:bldOleChart")]
    PBldOleChart(BuildOleChart),
    #[xml(tag = "p:bldGraphic")]
    PBldGraphic(BuildGraphics),
}
/// Defines the ExtensionListWithModification Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct ExtensionListWithModification {
    /// Modify
    /// Represents the following attribute in the schema: :mod
    #[xml(attr = "mod")]
    pub modify: Option<bool>,
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListWithModificationChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListWithModificationChildChoice {
    #[xml(tag = "p:ext")]
    PExt(Extension),
}
/// By.
/// When the object is serialized out as xml, it's qualified name is p:by.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:by")]
pub struct ByColor {
    #[xml(child = "p:rgb", child = "p:hsl")]
    pub children: Vec<ByColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ByColorChildChoice {
    #[xml(tag = "p:rgb")]
    PRgb(RgbColor),
    #[xml(tag = "p:hsl")]
    PHsl(HslColor),
}
/// From.
/// When the object is serialized out as xml, it's qualified name is p:from.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:from")]
pub struct FromColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<FromColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FromColorChildChoice {
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
/// To.
/// When the object is serialized out as xml, it's qualified name is p:to.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:to")]
pub struct ToColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ToColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ToColorChildChoice {
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
/// Defines the Color3Type Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Color3Type {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Color3TypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Color3TypeChildChoice {
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
/// Presentation Slide.
/// When the object is serialized out as xml, it's qualified name is p:sld.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sld")]
pub struct SlideListEntry {
    /// Relationship ID
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Customer Data.
/// When the object is serialized out as xml, it's qualified name is p:custData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:custData")]
pub struct CustomerData {
    /// Relationship ID
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Customer Data Tags.
/// When the object is serialized out as xml, it's qualified name is p:tags.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tags")]
pub struct CustomerDataTags {
    /// Relationship ID
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Comment Author.
/// When the object is serialized out as xml, it's qualified name is p:cmAuthor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cmAuthor")]
pub struct CommentAuthor {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// initials
    /// Represents the following attribute in the schema: :initials
    #[xml(attr = "initials")]
    pub initials: String,
    /// lastIdx
    /// Represents the following attribute in the schema: :lastIdx
    #[xml(attr = "lastIdx")]
    pub last_index: u32,
    /// clrIdx
    /// Represents the following attribute in the schema: :clrIdx
    #[xml(attr = "clrIdx")]
    pub color_index: u32,
    /// _
    #[xml(child = "p:extLst")]
    pub comment_author_extension_list: Option<CommentAuthorExtensionList>,
}
/// Comment.
/// When the object is serialized out as xml, it's qualified name is p:cm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cm")]
pub struct Comment {
    /// authorId
    /// Represents the following attribute in the schema: :authorId
    #[xml(attr = "authorId")]
    pub author_id: u32,
    /// dt
    /// Represents the following attribute in the schema: :dt
    #[xml(attr = "dt")]
    pub date_time: Option<String>,
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: u32,
    /// _
    #[xml(child = "p:pos")]
    pub position: Position,
    /// _
    #[xml(child = "p:text")]
    pub text: Text,
    /// _
    #[xml(child = "p:extLst")]
    pub comment_extension_list: Option<CommentExtensionList>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct ExtensionList {
    #[xml(child = "p:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "p:ext")]
    PExt(Extension),
}
/// Embedded Control.
/// When the object is serialized out as xml, it's qualified name is p:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:control")]
pub struct Control {
    /// spid
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// showAsIcon
    /// Represents the following attribute in the schema: :showAsIcon
    #[xml(attr = "showAsIcon")]
    pub show_as_icon: Option<bool>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// imgW
    /// Represents the following attribute in the schema: :imgW
    #[xml(attr = "imgW")]
    pub image_width: Option<i32>,
    /// imgH
    /// Represents the following attribute in the schema: :imgH
    #[xml(attr = "imgH")]
    pub image_height: Option<i32>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
    /// _
    #[xml(child = "p:pic")]
    pub picture: Option<Picture>,
}
/// Slide ID.
/// When the object is serialized out as xml, it's qualified name is p:sldId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldId")]
pub struct SlideId {
    /// Slide Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: String,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Slide Master ID.
/// When the object is serialized out as xml, it's qualified name is p:sldMasterId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldMasterId")]
pub struct SlideMasterId {
    /// Slide Master Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<u32>,
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: String,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Notes Master ID.
/// When the object is serialized out as xml, it's qualified name is p:notesMasterId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesMasterId")]
pub struct NotesMasterId {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Handout Master ID.
/// When the object is serialized out as xml, it's qualified name is p:handoutMasterId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:handoutMasterId")]
pub struct HandoutMasterId {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Embedded Font Name.
/// When the object is serialized out as xml, it's qualified name is p:font.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:font")]
pub struct Font {
    /// Text Typeface
    /// Represents the following attribute in the schema: :typeface
    #[xml(attr = "typeface")]
    pub typeface: Option<String>,
    /// Panose Setting
    /// Represents the following attribute in the schema: :panose
    #[xml(attr = "panose")]
    pub panose: Option<String>,
    /// Similar Font Family
    /// Represents the following attribute in the schema: :pitchFamily
    #[xml(attr = "pitchFamily")]
    pub pitch_family: Option<u8>,
    /// Similar Character Set
    /// Represents the following attribute in the schema: :charset
    #[xml(attr = "charset")]
    pub character_set: Option<u8>,
}
/// Regular Embedded Font.
/// When the object is serialized out as xml, it's qualified name is p:regular.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:regular")]
pub struct RegularFont {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Bold Embedded Font.
/// When the object is serialized out as xml, it's qualified name is p:bold.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bold")]
pub struct BoldFont {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Italic Embedded Font.
/// When the object is serialized out as xml, it's qualified name is p:italic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:italic")]
pub struct ItalicFont {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Bold Italic Embedded Font.
/// When the object is serialized out as xml, it's qualified name is p:boldItalic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:boldItalic")]
pub struct BoldItalicFont {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the EmbeddedFontDataIdType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmbeddedFontDataIdType {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Embedded Font.
/// When the object is serialized out as xml, it's qualified name is p:embeddedFont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:embeddedFont")]
pub struct EmbeddedFont {
    ///Embedded Font Name
    #[xml(child = "p:font")]
    pub font: Font,
    ///Regular Embedded Font
    #[xml(child = "p:regular")]
    pub regular_font: Option<RegularFont>,
    ///Bold Embedded Font
    #[xml(child = "p:bold")]
    pub bold_font: Option<BoldFont>,
    ///Italic Embedded Font
    #[xml(child = "p:italic")]
    pub italic_font: Option<ItalicFont>,
    ///Bold Italic Embedded Font
    #[xml(child = "p:boldItalic")]
    pub bold_italic_font: Option<BoldItalicFont>,
}
/// List of Presentation Slides.
/// When the object is serialized out as xml, it's qualified name is p:sldLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldLst")]
pub struct SlideList {
    /// _
    #[xml(child = "p:sld")]
    pub p_sld: Vec<SlideListEntry>,
}
/// Custom Show.
/// When the object is serialized out as xml, it's qualified name is p:custShow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:custShow")]
pub struct CustomShow {
    /// Custom Show Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Custom Show ID
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: u32,
    ///List of Presentation Slides
    #[xml(child = "p:sldLst")]
    pub slide_list: SlideList,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is p:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cNvPr")]
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
/// Non-Visual Drawing Properties for a Shape.
/// When the object is serialized out as xml, it's qualified name is p:cNvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
    /// Text Box
    /// Represents the following attribute in the schema: :txBox
    #[xml(attr = "txBox")]
    pub text_box: Option<bool>,
    ///Shape Locks
    #[xml(child = "a:spLocks")]
    pub shape_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Application Non-Visual Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is p:nvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nvPr")]
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
    PPh(PlaceholderShape),
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
    PCustDataLst(CustomerDataList),
    #[xml(tag = "p:extLst")]
    PExtLst(ApplicationNonVisualDrawingPropertiesExtensionList),
}
/// Non-Visual Properties for a Shape.
/// When the object is serialized out as xml, it's qualified name is p:nvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nvSpPr")]
pub struct NonVisualShapeProperties {
    ///Non-Visual Drawing Properties
    #[xml(child = "p:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Drawing Properties for a Shape
    #[xml(child = "p:cNvSpPr")]
    pub non_visual_shape_drawing_properties: NonVisualShapeDrawingProperties,
    ///Application Non-Visual Drawing Properties
    #[xml(child = "p:nvPr")]
    pub application_non_visual_drawing_properties: ApplicationNonVisualDrawingProperties,
}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is p:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:spPr")]
pub struct ShapeProperties {
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    #[xml(
        child = "a:xfrm",
        child = "a:custGeom",
        child = "a:prstGeom",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:ln",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:extLst",
    )]
    pub children: Vec<ShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapePropertiesChildChoice {
    #[xml(tag = "a:xfrm")]
    AXfrm(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Transform2D),
    #[xml(tag = "a:custGeom")]
    ACustGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomGeometry,
    ),
    #[xml(tag = "a:prstGeom")]
    APrstGeom(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetGeometry,
    ),
    #[xml(tag = "a:noFill")]
    ANoFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill,
    ),
    #[xml(tag = "a:gradFill")]
    AGradFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill,
    ),
    #[xml(tag = "a:blipFill")]
    ABlipFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:grpFill")]
    AGrpFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill),
    #[xml(tag = "a:ln")]
    ALn(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Outline),
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapePropertiesExtensionList,
    ),
}
/// Shape Style.
/// When the object is serialized out as xml, it's qualified name is p:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:style")]
pub struct ShapeStyle {
    /// _
    #[xml(child = "a:lnRef")]
    pub line_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineReference,
    /// _
    #[xml(child = "a:fillRef")]
    pub fill_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillReference,
    /// _
    #[xml(child = "a:effectRef")]
    pub effect_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectReference,
    ///Font Reference
    #[xml(child = "a:fontRef")]
    pub font_reference: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontReference,
}
/// Shape Text Body.
/// When the object is serialized out as xml, it's qualified name is p:txBody.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:txBody")]
pub struct TextBody {
    ///Body Properties
    #[xml(child = "a:bodyPr")]
    pub body_properties: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BodyProperties,
    ///Text List Styles
    #[xml(child = "a:lstStyle")]
    pub list_style: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ListStyle,
    >,
    /// _
    #[xml(child = "a:p")]
    pub a_p: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Paragraph,
    >,
}
/// Non-Visual Connector Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is p:cNvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
    ///Connection Shape Locks
    #[xml(child = "a:cxnSpLocks")]
    pub connection_shape_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ConnectionShapeLocks,
    >,
    ///Connection Start
    #[xml(child = "a:stCxn")]
    pub start_connection: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::StartConnection,
    >,
    ///Connection End
    #[xml(child = "a:endCxn")]
    pub end_connection: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EndConnection,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Non-Visual Properties for a Connection Shape.
/// When the object is serialized out as xml, it's qualified name is p:nvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
    ///Non-Visual Drawing Properties
    #[xml(child = "p:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Connector Shape Drawing Properties
    #[xml(child = "p:cNvCxnSpPr")]
    pub non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
    ///Application Non-Visual Drawing Properties
    #[xml(child = "p:nvPr")]
    pub application_non_visual_drawing_properties: ApplicationNonVisualDrawingProperties,
}
/// Non-Visual Picture Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is p:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    /// preferRelativeResize
    /// Represents the following attribute in the schema: :preferRelativeResize
    #[xml(attr = "preferRelativeResize")]
    pub prefer_relative_resize: Option<bool>,
    /// _
    #[xml(child = "a:picLocks")]
    pub picture_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_picture_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualPicturePropertiesExtensionList,
    >,
}
/// Non-Visual Properties for a Picture.
/// When the object is serialized out as xml, it's qualified name is p:nvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nvPicPr")]
pub struct NonVisualPictureProperties {
    /// _
    #[xml(child = "p:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Picture Drawing Properties
    #[xml(child = "p:cNvPicPr")]
    pub non_visual_picture_drawing_properties: NonVisualPictureDrawingProperties,
    /// _
    #[xml(child = "p:nvPr")]
    pub application_non_visual_drawing_properties: ApplicationNonVisualDrawingProperties,
}
/// Picture Fill.
/// When the object is serialized out as xml, it's qualified name is p:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:blipFill")]
pub struct BlipFill {
    /// DPI Setting
    /// Represents the following attribute in the schema: :dpi
    #[xml(attr = "dpi")]
    pub dpi: Option<u32>,
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
    #[xml(child = "a:blip", child = "a:srcRect", child = "a:tile", child = "a:stretch")]
    pub children: Vec<BlipFillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipFillChildChoice {
    #[xml(tag = "a:blip")]
    ABlip(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blip),
    #[xml(tag = "a:srcRect")]
    ASrcRect(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SourceRectangle,
    ),
    #[xml(tag = "a:tile")]
    ATile(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Tile),
    #[xml(tag = "a:stretch")]
    AStretch(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Stretch),
}
/// Non-Visual Graphic Frame Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is p:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
    ///Graphic Frame Locks
    #[xml(child = "a:graphicFrameLocks")]
    pub graphic_frame_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GraphicFrameLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Non-Visual Properties for a Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is p:nvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
    ///Non-Visual Drawing Properties
    #[xml(child = "p:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Graphic Frame Drawing Properties
    #[xml(child = "p:cNvGraphicFramePr")]
    pub non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
    ///Application Non-Visual Drawing Properties
    #[xml(child = "p:nvPr")]
    pub application_non_visual_drawing_properties: ApplicationNonVisualDrawingProperties,
}
/// 2D Transform for Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is p:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:xfrm")]
pub struct Transform {
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
/// Non-Visual Group Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is p:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
    /// _
    #[xml(child = "a:grpSpLocks")]
    pub group_shape_locks: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupShapeLocks,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Slide Master Title Text Style.
/// When the object is serialized out as xml, it's qualified name is p:titleStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:titleStyle")]
pub struct TitleStyle {
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Slide Master Body Text Style.
/// When the object is serialized out as xml, it's qualified name is p:bodyStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bodyStyle")]
pub struct BodyStyle {
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Slide Master Other Text Style.
/// When the object is serialized out as xml, it's qualified name is p:otherStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:otherStyle")]
pub struct OtherStyle {
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the DefaultTextStyle Class.
/// When the object is serialized out as xml, it's qualified name is p:defaultTextStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:defaultTextStyle")]
pub struct DefaultTextStyle {
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the NotesStyle Class.
/// When the object is serialized out as xml, it's qualified name is p:notesStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesStyle")]
pub struct NotesStyle {
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultParagraphProperties,
    >,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level1ParagraphProperties,
    >,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level2ParagraphProperties,
    >,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level3ParagraphProperties,
    >,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level4ParagraphProperties,
    >,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level5ParagraphProperties,
    >,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level6ParagraphProperties,
    >,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level7ParagraphProperties,
    >,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level8ParagraphProperties,
    >,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Level9ParagraphProperties,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Defines the TextListStyleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextListStyleType {}
/// Slide Layout Id.
/// When the object is serialized out as xml, it's qualified name is p:sldLayoutId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldLayoutId")]
pub struct SlideLayoutId {
    /// ID Tag
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<u32>,
    /// ID Tag
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: String,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Common slide data for notes slides.
/// When the object is serialized out as xml, it's qualified name is p:cSld.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cSld")]
pub struct CommonSlideData {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    ///Slide Background
    #[xml(child = "p:bg")]
    pub background: Option<Background>,
    ///Shape Tree
    #[xml(child = "p:spTree")]
    pub shape_tree: ShapeTree,
    ///Customer Data List
    #[xml(child = "p:custDataLst")]
    pub customer_data_list: Option<CustomerDataList>,
    ///List of controls
    #[xml(child = "p:controls")]
    pub control_list: Option<ControlList>,
    /// _
    #[xml(child = "p:extLst")]
    pub common_slide_data_extension_list: Option<CommonSlideDataExtensionList>,
}
/// Programmable Extensibility Tag.
/// When the object is serialized out as xml, it's qualified name is p:tag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tag")]
pub struct Tag {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Normal View Restored Left Properties.
/// When the object is serialized out as xml, it's qualified name is p:restoredLeft.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:restoredLeft")]
pub struct RestoredLeft {
    /// Normal View Dimension Size
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub size: i32,
    /// Auto Adjust Normal View
    /// Represents the following attribute in the schema: :autoAdjust
    #[xml(attr = "autoAdjust")]
    pub auto_adjust: Option<bool>,
}
/// Normal View Restored Top Properties.
/// When the object is serialized out as xml, it's qualified name is p:restoredTop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:restoredTop")]
pub struct RestoredTop {
    /// Normal View Dimension Size
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub size: i32,
    /// Auto Adjust Normal View
    /// Represents the following attribute in the schema: :autoAdjust
    #[xml(attr = "autoAdjust")]
    pub auto_adjust: Option<bool>,
}
/// Defines the NormalViewPortionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NormalViewPortionType {
    /// Normal View Dimension Size
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub size: i32,
    /// Auto Adjust Normal View
    /// Represents the following attribute in the schema: :autoAdjust
    #[xml(attr = "autoAdjust")]
    pub auto_adjust: Option<bool>,
}
/// View Scale.
/// When the object is serialized out as xml, it's qualified name is p:scale.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:scale")]
pub struct ScaleFactor {
    ///Horizontal Ratio
    #[xml(child = "a:sx")]
    pub scale_x: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ScaleX,
    ///Vertical Ratio
    #[xml(child = "a:sy")]
    pub scale_y: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ScaleY,
}
/// View Origin.
/// When the object is serialized out as xml, it's qualified name is p:origin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:origin")]
pub struct Origin {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the Position Class.
/// When the object is serialized out as xml, it's qualified name is p:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:pos")]
pub struct Position {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the Point2DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Point2DType {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Base properties for Notes View.
/// When the object is serialized out as xml, it's qualified name is p:cViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cViewPr")]
pub struct CommonViewProperties {
    /// Variable Scale
    /// Represents the following attribute in the schema: :varScale
    #[xml(attr = "varScale")]
    pub variable_scale: Option<bool>,
    ///View Scale
    #[xml(child = "p:scale")]
    pub scale_factor: ScaleFactor,
    ///View Origin
    #[xml(child = "p:origin")]
    pub origin: Origin,
}
/// Presentation Slide.
/// When the object is serialized out as xml, it's qualified name is p:sld.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sld")]
pub struct OutlineViewSlideListEntry {
    /// Relationship Identifier
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    /// Collapsed
    /// Represents the following attribute in the schema: :collapse
    #[xml(attr = "collapse")]
    pub collapse: Option<bool>,
}
/// List of Presentation Slides.
/// When the object is serialized out as xml, it's qualified name is p:sldLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldLst")]
pub struct OutlineViewSlideList {
    /// _
    #[xml(child = "p:sld")]
    pub p_sld: Vec<OutlineViewSlideListEntry>,
}
/// A Guide.
/// When the object is serialized out as xml, it's qualified name is p:guide.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:guide")]
pub struct Guide {
    /// Guide Orientation
    /// Represents the following attribute in the schema: :orient
    #[xml(attr = "orient")]
    pub orientation: Option<DirectionValues>,
    /// Guide Position
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub position: Option<i32>,
}
/// List of Guides.
/// When the object is serialized out as xml, it's qualified name is p:guideLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:guideLst")]
pub struct GuideList {
    /// _
    #[xml(child = "p:guide")]
    pub p_guide: Vec<Guide>,
}
/// Defines the CommonSlideViewProperties Class.
/// When the object is serialized out as xml, it's qualified name is p:cSldViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cSldViewPr")]
pub struct CommonSlideViewProperties {
    /// Snap Objects to Grid
    /// Represents the following attribute in the schema: :snapToGrid
    #[xml(attr = "snapToGrid")]
    pub snap_to_grid: Option<bool>,
    /// Snap Objects to Objects
    /// Represents the following attribute in the schema: :snapToObjects
    #[xml(attr = "snapToObjects")]
    pub snap_to_objects: Option<bool>,
    /// Show Guides in View
    /// Represents the following attribute in the schema: :showGuides
    #[xml(attr = "showGuides")]
    pub show_guides: Option<bool>,
    ///Base properties for Slide View
    #[xml(child = "p:cViewPr")]
    pub common_view_properties: CommonViewProperties,
    ///List of Guides
    #[xml(child = "p:guideLst")]
    pub guide_list: Option<GuideList>,
}
/// Normal View Properties.
/// When the object is serialized out as xml, it's qualified name is p:normalViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:normalViewPr")]
pub struct NormalViewProperties {
    /// Show Outline Icons in Normal View
    /// Represents the following attribute in the schema: :showOutlineIcons
    #[xml(attr = "showOutlineIcons")]
    pub show_outline_icons: Option<bool>,
    /// Snap Vertical Splitter
    /// Represents the following attribute in the schema: :snapVertSplitter
    #[xml(attr = "snapVertSplitter")]
    pub snap_vertical_splitter: Option<bool>,
    /// State of the Vertical Splitter Bar
    /// Represents the following attribute in the schema: :vertBarState
    #[xml(attr = "vertBarState")]
    pub vertical_bar_state: Option<SplitterBarStateValues>,
    /// State of the Horizontal Splitter Bar
    /// Represents the following attribute in the schema: :horzBarState
    #[xml(attr = "horzBarState")]
    pub horizontal_bar_state: Option<SplitterBarStateValues>,
    /// Prefer Single View
    /// Represents the following attribute in the schema: :preferSingleView
    #[xml(attr = "preferSingleView")]
    pub prefer_single_view: Option<bool>,
    ///Normal View Restored Left Properties
    #[xml(child = "p:restoredLeft")]
    pub restored_left: RestoredLeft,
    ///Normal View Restored Top Properties
    #[xml(child = "p:restoredTop")]
    pub restored_top: RestoredTop,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Slide View Properties.
/// When the object is serialized out as xml, it's qualified name is p:slideViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:slideViewPr")]
pub struct SlideViewProperties {
    /// _
    #[xml(child = "p:cSldViewPr")]
    pub common_slide_view_properties: CommonSlideViewProperties,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Outline View Properties.
/// When the object is serialized out as xml, it's qualified name is p:outlineViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:outlineViewPr")]
pub struct OutlineViewProperties {
    ///Common View Properties
    #[xml(child = "p:cViewPr")]
    pub common_view_properties: CommonViewProperties,
    ///List of Presentation Slides
    #[xml(child = "p:sldLst")]
    pub outline_view_slide_list: Option<OutlineViewSlideList>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Notes Text View Properties.
/// When the object is serialized out as xml, it's qualified name is p:notesTextViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesTextViewPr")]
pub struct NotesTextViewProperties {
    ///Base properties for Notes View
    #[xml(child = "p:cViewPr")]
    pub common_view_properties: CommonViewProperties,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Slide Sorter View Properties.
/// When the object is serialized out as xml, it's qualified name is p:sorterViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sorterViewPr")]
pub struct SorterViewProperties {
    /// Show Formatting
    /// Represents the following attribute in the schema: :showFormatting
    #[xml(attr = "showFormatting")]
    pub show_formatting: Option<bool>,
    ///Base properties for Slide Sorter View
    #[xml(child = "p:cViewPr")]
    pub common_view_properties: CommonViewProperties,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Notes View Properties.
/// When the object is serialized out as xml, it's qualified name is p:notesViewPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesViewPr")]
pub struct NotesViewProperties {
    ///Common Slide View Properties
    #[xml(child = "p:cSldViewPr")]
    pub common_slide_view_properties: CommonSlideViewProperties,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Grid Spacing.
/// When the object is serialized out as xml, it's qualified name is p:gridSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:gridSpacing")]
pub struct GridSpacing {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i32,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i32,
}
/// Defines the NotesSize Class.
/// When the object is serialized out as xml, it's qualified name is p:notesSz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesSz")]
pub struct NotesSize {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i32,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i32,
}
/// Defines the PositiveSize2DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PositiveSize2DType {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i32,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i32,
}
/// Defines the SlideExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct SlideExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "p14:laserTraceLst",
        child = "p14:showEvtLst",
        child = "p188:commentRel",
    )]
    pub children: Vec<SlideExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideExtensionChildChoice {
    #[xml(tag = "p14:laserTraceLst")]
    P14LaserTraceLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::LaserTraceList,
    ),
    #[xml(tag = "p14:showEvtLst")]
    P14ShowEvtLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShowEventRecordList,
    ),
    #[xml(tag = "p188:commentRel")]
    P188CommentRel(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentRelationship,
    ),
}
/// Defines the CommonSlideDataExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct CommonSlideDataExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p14:creationId")]
    pub children: Vec<CommonSlideDataExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommonSlideDataExtensionChildChoice {
    #[xml(tag = "p14:creationId")]
    P14CreationId(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::CreationId,
    ),
}
/// Defines the ShowPropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct ShowPropertiesExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "p14:browseMode",
        child = "p14:laserClr",
        child = "p14:showMediaCtrls",
    )]
    pub children: Vec<ShowPropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShowPropertiesExtensionChildChoice {
    #[xml(tag = "p14:browseMode")]
    P14BrowseMode(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::BrowseMode,
    ),
    #[xml(tag = "p14:laserClr")]
    P14LaserClr(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::LaserColor,
    ),
    #[xml(tag = "p14:showMediaCtrls")]
    P14ShowMediaCtrls(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShowMediaControls,
    ),
}
/// Defines the Picture Class.
/// When the object is serialized out as xml, it's qualified name is p:pic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:pic")]
pub struct Picture {
    ///Non-Visual Properties for a Picture
    #[xml(child = "p:nvPicPr")]
    pub non_visual_picture_properties: NonVisualPictureProperties,
    ///Picture Fill
    #[xml(child = "p:blipFill")]
    pub blip_fill: BlipFill,
    /// _
    #[xml(child = "p:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "p:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the OleObjectEmbed Class.
/// When the object is serialized out as xml, it's qualified name is p:embed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:embed")]
pub struct OleObjectEmbed {
    /// Color Scheme Properties for OLE Object
    /// Represents the following attribute in the schema: :followColorScheme
    #[xml(attr = "followColorScheme")]
    pub follow_color_scheme: Option<OleObjectFollowColorSchemeValues>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the OleObjectLink Class.
/// When the object is serialized out as xml, it's qualified name is p:link.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:link")]
pub struct OleObjectLink {
    /// Update Linked OLE Objects Automatically
    /// Represents the following attribute in the schema: :updateAutomatic
    #[xml(attr = "updateAutomatic")]
    pub auto_update: Option<bool>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Slide Transition.
/// When the object is serialized out as xml, it's qualified name is p:transition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:transition")]
pub struct Transition {
    /// spd
    /// Represents the following attribute in the schema: :spd
    #[xml(attr = "spd")]
    pub speed: Option<TransitionSpeedValues>,
    /// dur
    /// Represents the following attribute in the schema: p14:dur
    #[xml(attr = "p14:dur")]
    pub duration: Option<String>,
    /// Specifies whether a mouse click will advance the slide.
    /// Represents the following attribute in the schema: :advClick
    #[xml(attr = "advClick")]
    pub advance_on_click: Option<bool>,
    /// advTm
    /// Represents the following attribute in the schema: :advTm
    #[xml(attr = "advTm")]
    pub advance_after_time: Option<String>,
    #[xml(
        child = "p:blinds",
        child = "p:checker",
        child = "p:circle",
        child = "p:dissolve",
        child = "p:comb",
        child = "p:cover",
        child = "p:cut",
        child = "p:diamond",
        child = "p:fade",
        child = "p:newsflash",
        child = "p:plus",
        child = "p:pull",
        child = "p:push",
        child = "p:random",
        child = "p:randomBar",
        child = "p:split",
        child = "p:strips",
        child = "p:wedge",
        child = "p:wheel",
        child = "p:wipe",
        child = "p:zoom",
        child = "p14:flash",
        child = "p14:vortex",
        child = "p14:switch",
        child = "p14:flip",
        child = "p14:ripple",
        child = "p14:glitter",
        child = "p14:honeycomb",
        child = "p14:prism",
        child = "p14:doors",
        child = "p14:window",
        child = "p14:shred",
        child = "p14:ferris",
        child = "p14:flythrough",
        child = "p14:warp",
        child = "p14:gallery",
        child = "p14:conveyor",
        child = "p14:pan",
        child = "p14:reveal",
        child = "p14:wheelReverse",
        child = "p15:prstTrans",
        child = "p:sndAc",
        child = "p:extLst",
    )]
    pub children: Vec<TransitionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TransitionChildChoice {
    #[xml(tag = "p:blinds")]
    PBlinds(BlindsTransition),
    #[xml(tag = "p:checker")]
    PChecker(CheckerTransition),
    #[xml(tag = "p:circle")]
    PCircle(CircleTransition),
    #[xml(tag = "p:dissolve")]
    PDissolve(DissolveTransition),
    #[xml(tag = "p:comb")]
    PComb(CombTransition),
    #[xml(tag = "p:cover")]
    PCover(CoverTransition),
    #[xml(tag = "p:cut")]
    PCut(CutTransition),
    #[xml(tag = "p:diamond")]
    PDiamond(DiamondTransition),
    #[xml(tag = "p:fade")]
    PFade(FadeTransition),
    #[xml(tag = "p:newsflash")]
    PNewsflash(NewsflashTransition),
    #[xml(tag = "p:plus")]
    PPlus(PlusTransition),
    #[xml(tag = "p:pull")]
    PPull(PullTransition),
    #[xml(tag = "p:push")]
    PPush(PushTransition),
    #[xml(tag = "p:random")]
    PRandom(RandomTransition),
    #[xml(tag = "p:randomBar")]
    PRandomBar(RandomBarTransition),
    #[xml(tag = "p:split")]
    PSplit(SplitTransition),
    #[xml(tag = "p:strips")]
    PStrips(StripsTransition),
    #[xml(tag = "p:wedge")]
    PWedge(WedgeTransition),
    #[xml(tag = "p:wheel")]
    PWheel(WheelTransition),
    #[xml(tag = "p:wipe")]
    PWipe(WipeTransition),
    #[xml(tag = "p:zoom")]
    PZoom(ZoomTransition),
    #[xml(tag = "p14:flash")]
    P14Flash(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlashTransition,
    ),
    #[xml(tag = "p14:vortex")]
    P14Vortex(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::VortexTransition,
    ),
    #[xml(tag = "p14:switch")]
    P14Switch(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SwitchTransition,
    ),
    #[xml(tag = "p14:flip")]
    P14Flip(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlipTransition,
    ),
    #[xml(tag = "p14:ripple")]
    P14Ripple(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::RippleTransition,
    ),
    #[xml(tag = "p14:glitter")]
    P14Glitter(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::GlitterTransition,
    ),
    #[xml(tag = "p14:honeycomb")]
    P14Honeycomb(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::HoneycombTransition,
    ),
    #[xml(tag = "p14:prism")]
    P14Prism(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::PrismTransition,
    ),
    #[xml(tag = "p14:doors")]
    P14Doors(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DoorsTransition,
    ),
    #[xml(tag = "p14:window")]
    P14Window(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WindowTransition,
    ),
    #[xml(tag = "p14:shred")]
    P14Shred(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShredTransition,
    ),
    #[xml(tag = "p14:ferris")]
    P14Ferris(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FerrisTransition,
    ),
    #[xml(tag = "p14:flythrough")]
    P14Flythrough(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlythroughTransition,
    ),
    #[xml(tag = "p14:warp")]
    P14Warp(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WarpTransition,
    ),
    #[xml(tag = "p14:gallery")]
    P14Gallery(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::GalleryTransition,
    ),
    #[xml(tag = "p14:conveyor")]
    P14Conveyor(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ConveyorTransition,
    ),
    #[xml(tag = "p14:pan")]
    P14Pan(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::PanTransition,
    ),
    #[xml(tag = "p14:reveal")]
    P14Reveal(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::RevealTransition,
    ),
    #[xml(tag = "p14:wheelReverse")]
    P14WheelReverse(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WheelReverseTransition,
    ),
    #[xml(tag = "p15:prstTrans")]
    P15PrstTrans(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::PresetTransition,
    ),
    #[xml(tag = "p:sndAc")]
    PSndAc(SoundAction),
    #[xml(tag = "p:extLst")]
    PExtLst(ExtensionListWithModification),
}
/// Slide Timing Information for a Slide.
/// When the object is serialized out as xml, it's qualified name is p:timing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:timing")]
pub struct Timing {
    /// _
    #[xml(child = "p:tnLst")]
    pub time_node_list: Option<TimeNodeList>,
    ///Build List
    #[xml(child = "p:bldLst")]
    pub build_list: Option<BuildList>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct SlideExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<SlideExtension>,
}
/// Slide Background.
/// When the object is serialized out as xml, it's qualified name is p:bg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:bg")]
pub struct Background {
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    #[xml(child = "p:bgPr", child = "p:bgRef")]
    pub children: Vec<BackgroundChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundChildChoice {
    #[xml(tag = "p:bgPr")]
    PBgPr(BackgroundProperties),
    #[xml(tag = "p:bgRef")]
    PBgRef(BackgroundStyleReference),
}
/// Shape Tree.
/// When the object is serialized out as xml, it's qualified name is p:spTree.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:spTree")]
pub struct ShapeTree {
    #[xml(
        child = "p:nvGrpSpPr",
        child = "p:grpSpPr",
        child = "p:sp",
        child = "p:grpSp",
        child = "p:graphicFrame",
        child = "p:cxnSp",
        child = "p:pic",
        child = "p:contentPart",
        child = "p:extLst",
    )]
    pub children: Vec<ShapeTreeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeTreeChildChoice {
    #[xml(tag = "p:nvGrpSpPr")]
    PNvGrpSpPr(NonVisualGroupShapeProperties),
    #[xml(tag = "p:grpSpPr")]
    PGrpSpPr(GroupShapeProperties),
    #[xml(tag = "p:sp")]
    PSp(Shape),
    #[xml(tag = "p:grpSp")]
    PGrpSp(GroupShape),
    #[xml(tag = "p:graphicFrame")]
    PGraphicFrame(GraphicFrame),
    #[xml(tag = "p:cxnSp")]
    PCxnSp(ConnectionShape),
    #[xml(tag = "p:pic")]
    PPic(Picture),
    #[xml(tag = "p:contentPart")]
    PContentPart(ContentPart),
    #[xml(tag = "p:extLst")]
    PExtLst(ExtensionListWithModification),
}
/// Group Shape.
/// When the object is serialized out as xml, it's qualified name is p:grpSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:grpSp")]
pub struct GroupShape {
    #[xml(
        child = "p:nvGrpSpPr",
        child = "p:grpSpPr",
        child = "p:sp",
        child = "p:grpSp",
        child = "p:graphicFrame",
        child = "p:cxnSp",
        child = "p:pic",
        child = "p:contentPart",
        child = "p:extLst",
    )]
    pub children: Vec<GroupShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapeChildChoice {
    #[xml(tag = "p:nvGrpSpPr")]
    PNvGrpSpPr(NonVisualGroupShapeProperties),
    #[xml(tag = "p:grpSpPr")]
    PGrpSpPr(GroupShapeProperties),
    #[xml(tag = "p:sp")]
    PSp(Shape),
    #[xml(tag = "p:grpSp")]
    PGrpSp(GroupShape),
    #[xml(tag = "p:graphicFrame")]
    PGraphicFrame(GraphicFrame),
    #[xml(tag = "p:cxnSp")]
    PCxnSp(ConnectionShape),
    #[xml(tag = "p:pic")]
    PPic(Picture),
    #[xml(tag = "p:contentPart")]
    PContentPart(ContentPart),
    #[xml(tag = "p:extLst")]
    PExtLst(ExtensionListWithModification),
}
/// Defines the GroupShapeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct GroupShapeType {}
/// Customer Data List.
/// When the object is serialized out as xml, it's qualified name is p:custDataLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:custDataLst")]
pub struct CustomerDataList {
    /// _
    #[xml(child = "p:custData")]
    pub p_cust_data: Vec<CustomerData>,
    /// _
    #[xml(child = "p:tags")]
    pub p_tags: Option<CustomerDataTags>,
}
/// List of controls.
/// When the object is serialized out as xml, it's qualified name is p:controls.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:controls")]
pub struct ControlList {
    /// _
    #[xml(child = "p:control")]
    pub p_control: Vec<Control>,
}
/// Defines the CommonSlideDataExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct CommonSlideDataExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<CommonSlideDataExtension>,
}
/// Non-Visual Properties for a Group Shape.
/// When the object is serialized out as xml, it's qualified name is p:nvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
    ///Non-visual Drawing Properties
    #[xml(child = "p:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Group Shape Drawing Properties
    #[xml(child = "p:cNvGrpSpPr")]
    pub non_visual_group_shape_drawing_properties: NonVisualGroupShapeDrawingProperties,
    ///Non-Visual Properties
    #[xml(child = "p:nvPr")]
    pub application_non_visual_drawing_properties: ApplicationNonVisualDrawingProperties,
}
/// Group Shape Properties.
/// When the object is serialized out as xml, it's qualified name is p:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:grpSpPr")]
pub struct GroupShapeProperties {
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    #[xml(
        child = "a:xfrm",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:extLst",
    )]
    pub children: Vec<GroupShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapePropertiesChildChoice {
    #[xml(tag = "a:xfrm")]
    AXfrm(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TransformGroup,
    ),
    #[xml(tag = "a:noFill")]
    ANoFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill,
    ),
    #[xml(tag = "a:gradFill")]
    AGradFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill,
    ),
    #[xml(tag = "a:blipFill")]
    ABlipFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:grpFill")]
    AGrpFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectList,
    ),
    #[xml(tag = "a:effectDag")]
    AEffectDag(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::EffectDag,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Shape.
/// When the object is serialized out as xml, it's qualified name is p:sp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sp")]
pub struct Shape {
    /// Use Background Fill
    /// Represents the following attribute in the schema: :useBgFill
    #[xml(attr = "useBgFill")]
    pub use_background_fill: Option<bool>,
    ///Non-Visual Properties for a Shape
    #[xml(child = "p:nvSpPr")]
    pub non_visual_shape_properties: NonVisualShapeProperties,
    /// _
    #[xml(child = "p:spPr")]
    pub shape_properties: ShapeProperties,
    ///Shape Style
    #[xml(child = "p:style")]
    pub shape_style: Option<ShapeStyle>,
    ///Shape Text Body
    #[xml(child = "p:txBody")]
    pub text_body: Option<TextBody>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is p:graphicFrame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:graphicFrame")]
pub struct GraphicFrame {
    ///Non-Visual Properties for a Graphic Frame
    #[xml(child = "p:nvGraphicFramePr")]
    pub non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    ///2D Transform for Graphic Frame
    #[xml(child = "p:xfrm")]
    pub transform: Transform,
    /// _
    #[xml(child = "a:graphic")]
    pub graphic: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Graphic,
    ///Extension List with Modification Flag
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Connection Shape.
/// When the object is serialized out as xml, it's qualified name is p:cxnSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cxnSp")]
pub struct ConnectionShape {
    ///Non-Visual Properties for a Connection Shape
    #[xml(child = "p:nvCxnSpPr")]
    pub non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
    ///Shape Properties
    #[xml(child = "p:spPr")]
    pub shape_properties: ShapeProperties,
    ///Connector Shape Style
    #[xml(child = "p:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ShowPropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct ShowPropertiesExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<ShowPropertiesExtension>,
}
/// Shape Target.
/// When the object is serialized out as xml, it's qualified name is p:spTgt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:spTgt")]
pub struct ShapeTarget {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
    #[xml(
        child = "p:bg",
        child = "p:subSp",
        child = "p:oleChartEl",
        child = "p:txEl",
        child = "p:graphicEl",
    )]
    pub children: Vec<ShapeTargetChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeTargetChildChoice {
    #[xml(tag = "p:bg")]
    PBg(BackgroundAnimation),
    #[xml(tag = "p:subSp")]
    PSubSp(SubShape),
    #[xml(tag = "p:oleChartEl")]
    POleChartEl(OleChartElement),
    #[xml(tag = "p:txEl")]
    PTxEl(TextElement),
    #[xml(tag = "p:graphicEl")]
    PGraphicEl(GraphicElement),
}
/// Ink Target.
/// When the object is serialized out as xml, it's qualified name is p:inkTgt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:inkTgt")]
pub struct InkTarget {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
}
/// Subshape.
/// When the object is serialized out as xml, it's qualified name is p:subSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:subSp")]
pub struct SubShape {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
}
/// Defines the TimeListSubShapeIdType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeListSubShapeIdType {
    /// Shape ID
    /// Represents the following attribute in the schema: :spid
    #[xml(attr = "spid")]
    pub shape_id: String,
}
/// Defines the CommentAuthorExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct CommentAuthorExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p15:presenceInfo")]
    pub children: Vec<CommentAuthorExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommentAuthorExtensionChildChoice {
    #[xml(tag = "p15:presenceInfo")]
    P15PresenceInfo(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::PresenceInfo,
    ),
}
/// Defines the CommentExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct CommentExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p15:threadingInfo")]
    pub children: Vec<CommentExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommentExtensionChildChoice {
    #[xml(tag = "p15:threadingInfo")]
    P15ThreadingInfo(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::ThreadingInfo,
    ),
}
/// Defines the SlideLayoutExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct SlideLayoutExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p15:sldGuideLst")]
    pub children: Vec<SlideLayoutExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideLayoutExtensionChildChoice {
    #[xml(tag = "p15:sldGuideLst")]
    P15SldGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    ),
}
/// Defines the SlideMasterExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct SlideMasterExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p15:sldGuideLst")]
    pub children: Vec<SlideMasterExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SlideMasterExtensionChildChoice {
    #[xml(tag = "p15:sldGuideLst")]
    P15SldGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    ),
}
/// Defines the HandoutMasterExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct HandoutMasterExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p15:sldGuideLst")]
    pub children: Vec<HandoutMasterExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HandoutMasterExtensionChildChoice {
    #[xml(tag = "p15:sldGuideLst")]
    P15SldGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    ),
}
/// Defines the NotesMasterExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct NotesMasterExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p15:sldGuideLst")]
    pub children: Vec<NotesMasterExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NotesMasterExtensionChildChoice {
    #[xml(tag = "p15:sldGuideLst")]
    P15SldGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    ),
}
/// Placeholder Shape.
/// When the object is serialized out as xml, it's qualified name is p:ph.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ph")]
pub struct PlaceholderShape {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<PlaceholderValues>,
    /// orient
    /// Represents the following attribute in the schema: :orient
    #[xml(attr = "orient")]
    pub orientation: Option<DirectionValues>,
    /// sz
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub size: Option<PlaceholderSizeValues>,
    /// idx
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: Option<u32>,
    /// hasCustomPrompt
    /// Represents the following attribute in the schema: :hasCustomPrompt
    #[xml(attr = "hasCustomPrompt")]
    pub has_custom_prompt: Option<bool>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct ApplicationNonVisualDrawingPropertiesExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<ApplicationNonVisualDrawingPropertiesExtension>,
}
/// Defines the ApplicationNonVisualDrawingPropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct ApplicationNonVisualDrawingPropertiesExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "p14:media", child = "p14:modId")]
    pub children: Vec<ApplicationNonVisualDrawingPropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ApplicationNonVisualDrawingPropertiesExtensionChildChoice {
    #[xml(tag = "p14:media")]
    P14Media(crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Media),
    #[xml(tag = "p14:modId")]
    P14ModId(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ModificationId,
    ),
}
/// Defines the Iterate Class.
/// When the object is serialized out as xml, it's qualified name is p:iterate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:iterate")]
pub struct Iterate {
    /// Iterate Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<IterateValues>,
    /// Backwards
    /// Represents the following attribute in the schema: :backwards
    #[xml(attr = "backwards")]
    pub backwards: Option<bool>,
    #[xml(child = "p:tmAbs", child = "p:tmPct")]
    pub children: Vec<IterateChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum IterateChildChoice {
    #[xml(tag = "p:tmAbs")]
    PTmAbs(TimeAbsolute),
    #[xml(tag = "p:tmPct")]
    PTmPct(TimePercentage),
}
/// Defines the ChildTimeNodeList Class.
/// When the object is serialized out as xml, it's qualified name is p:childTnLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:childTnLst")]
pub struct ChildTimeNodeList {
    #[xml(
        child = "p:par",
        child = "p:seq",
        child = "p:excl",
        child = "p:anim",
        child = "p:animClr",
        child = "p:animEffect",
        child = "p:animMotion",
        child = "p:animRot",
        child = "p:animScale",
        child = "p:cmd",
        child = "p:set",
        child = "p:audio",
        child = "p:video",
    )]
    pub children: Vec<ChildTimeNodeListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ChildTimeNodeListChildChoice {
    #[xml(tag = "p:par")]
    PPar(ParallelTimeNode),
    #[xml(tag = "p:seq")]
    PSeq(SequenceTimeNode),
    #[xml(tag = "p:excl")]
    PExcl(ExclusiveTimeNode),
    #[xml(tag = "p:anim")]
    PAnim(Animate),
    #[xml(tag = "p:animClr")]
    PAnimClr(AnimateColor),
    #[xml(tag = "p:animEffect")]
    PAnimEffect(AnimateEffect),
    #[xml(tag = "p:animMotion")]
    PAnimMotion(AnimateMotion),
    #[xml(tag = "p:animRot")]
    PAnimRot(AnimateRotation),
    #[xml(tag = "p:animScale")]
    PAnimScale(AnimateScale),
    #[xml(tag = "p:cmd")]
    PCmd(Command),
    #[xml(tag = "p:set")]
    PSet(SetBehavior),
    #[xml(tag = "p:audio")]
    PAudio(Audio),
    #[xml(tag = "p:video")]
    PVideo(Video),
}
/// Defines the SubTimeNodeList Class.
/// When the object is serialized out as xml, it's qualified name is p:subTnLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:subTnLst")]
pub struct SubTimeNodeList {
    #[xml(
        child = "p:par",
        child = "p:seq",
        child = "p:excl",
        child = "p:anim",
        child = "p:animClr",
        child = "p:animEffect",
        child = "p:animMotion",
        child = "p:animRot",
        child = "p:animScale",
        child = "p:cmd",
        child = "p:set",
        child = "p:audio",
        child = "p:video",
    )]
    pub children: Vec<SubTimeNodeListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SubTimeNodeListChildChoice {
    #[xml(tag = "p:par")]
    PPar(ParallelTimeNode),
    #[xml(tag = "p:seq")]
    PSeq(SequenceTimeNode),
    #[xml(tag = "p:excl")]
    PExcl(ExclusiveTimeNode),
    #[xml(tag = "p:anim")]
    PAnim(Animate),
    #[xml(tag = "p:animClr")]
    PAnimClr(AnimateColor),
    #[xml(tag = "p:animEffect")]
    PAnimEffect(AnimateEffect),
    #[xml(tag = "p:animMotion")]
    PAnimMotion(AnimateMotion),
    #[xml(tag = "p:animRot")]
    PAnimRot(AnimateRotation),
    #[xml(tag = "p:animScale")]
    PAnimScale(AnimateScale),
    #[xml(tag = "p:cmd")]
    PCmd(Command),
    #[xml(tag = "p:set")]
    PSet(SetBehavior),
    #[xml(tag = "p:audio")]
    PAudio(Audio),
    #[xml(tag = "p:video")]
    PVideo(Video),
}
/// Defines the TimeTypeListType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeTypeListType {
    #[xml(
        child = "p:par",
        child = "p:seq",
        child = "p:excl",
        child = "p:anim",
        child = "p:animClr",
        child = "p:animEffect",
        child = "p:animMotion",
        child = "p:animRot",
        child = "p:animScale",
        child = "p:cmd",
        child = "p:set",
        child = "p:audio",
        child = "p:video",
    )]
    pub children: Vec<TimeTypeListTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TimeTypeListTypeChildChoice {
    #[xml(tag = "p:par")]
    PPar(ParallelTimeNode),
    #[xml(tag = "p:seq")]
    PSeq(SequenceTimeNode),
    #[xml(tag = "p:excl")]
    PExcl(ExclusiveTimeNode),
    #[xml(tag = "p:anim")]
    PAnim(Animate),
    #[xml(tag = "p:animClr")]
    PAnimClr(AnimateColor),
    #[xml(tag = "p:animEffect")]
    PAnimEffect(AnimateEffect),
    #[xml(tag = "p:animMotion")]
    PAnimMotion(AnimateMotion),
    #[xml(tag = "p:animRot")]
    PAnimRot(AnimateRotation),
    #[xml(tag = "p:animScale")]
    PAnimScale(AnimateScale),
    #[xml(tag = "p:cmd")]
    PCmd(Command),
    #[xml(tag = "p:set")]
    PSet(SetBehavior),
    #[xml(tag = "p:audio")]
    PAudio(Audio),
    #[xml(tag = "p:video")]
    PVideo(Video),
}
/// Defines the TimeAnimateValueList Class.
/// When the object is serialized out as xml, it's qualified name is p:tavLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:tavLst")]
pub struct TimeAnimateValueList {
    /// _
    #[xml(child = "p:tav")]
    pub p_tav: Vec<TimeAnimateValue>,
}
/// Defines the ByPosition Class.
/// When the object is serialized out as xml, it's qualified name is p:by.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:by")]
pub struct ByPosition {
    /// X coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the FromPosition Class.
/// When the object is serialized out as xml, it's qualified name is p:from.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:from")]
pub struct FromPosition {
    /// X coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the ToPosition Class.
/// When the object is serialized out as xml, it's qualified name is p:to.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:to")]
pub struct ToPosition {
    /// X coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the RotationCenter Class.
/// When the object is serialized out as xml, it's qualified name is p:rCtr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:rCtr")]
pub struct RotationCenter {
    /// X coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the TimeListType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TimeListType {
    /// X coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i32,
    /// Y coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i32,
}
/// Defines the CommentAuthorExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct CommentAuthorExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<CommentAuthorExtension>,
}
/// Defines the CommentExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct CommentExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<CommentExtension>,
}
/// Defines the SlideMasterIdList Class.
/// When the object is serialized out as xml, it's qualified name is p:sldMasterIdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldMasterIdLst")]
pub struct SlideMasterIdList {
    /// _
    #[xml(child = "p:sldMasterId")]
    pub p_sld_master_id: Vec<SlideMasterId>,
}
/// Defines the NotesMasterIdList Class.
/// When the object is serialized out as xml, it's qualified name is p:notesMasterIdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:notesMasterIdLst")]
pub struct NotesMasterIdList {
    ///Notes Master ID
    #[xml(child = "p:notesMasterId")]
    pub notes_master_id: Option<NotesMasterId>,
}
/// Defines the HandoutMasterIdList Class.
/// When the object is serialized out as xml, it's qualified name is p:handoutMasterIdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:handoutMasterIdLst")]
pub struct HandoutMasterIdList {
    ///Handout Master ID
    #[xml(child = "p:handoutMasterId")]
    pub handout_master_id: Option<HandoutMasterId>,
}
/// Defines the SlideIdList Class.
/// When the object is serialized out as xml, it's qualified name is p:sldIdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldIdLst")]
pub struct SlideIdList {
    /// _
    #[xml(child = "p:sldId")]
    pub p_sld_id: Vec<SlideId>,
}
/// Defines the SlideSize Class.
/// When the object is serialized out as xml, it's qualified name is p:sldSz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldSz")]
pub struct SlideSize {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i32,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i32,
    /// Type of Size
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<SlideSizeValues>,
}
/// Defines the EmbeddedFontList Class.
/// When the object is serialized out as xml, it's qualified name is p:embeddedFontLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:embeddedFontLst")]
pub struct EmbeddedFontList {
    /// _
    #[xml(child = "p:embeddedFont")]
    pub p_embedded_font: Vec<EmbeddedFont>,
}
/// Defines the CustomShowList Class.
/// When the object is serialized out as xml, it's qualified name is p:custShowLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:custShowLst")]
pub struct CustomShowList {
    /// _
    #[xml(child = "p:custShow")]
    pub p_cust_show: Vec<CustomShow>,
}
/// Defines the PhotoAlbum Class.
/// When the object is serialized out as xml, it's qualified name is p:photoAlbum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:photoAlbum")]
pub struct PhotoAlbum {
    /// Black and White
    /// Represents the following attribute in the schema: :bw
    #[xml(attr = "bw")]
    pub black_white: Option<bool>,
    /// Show/Hide Captions
    /// Represents the following attribute in the schema: :showCaptions
    #[xml(attr = "showCaptions")]
    pub show_captions: Option<bool>,
    /// Photo Album Layout
    /// Represents the following attribute in the schema: :layout
    #[xml(attr = "layout")]
    pub layout: Option<PhotoAlbumLayoutValues>,
    /// Frame Type
    /// Represents the following attribute in the schema: :frame
    #[xml(attr = "frame")]
    pub frame: Option<PhotoAlbumFrameShapeValues>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the Kinsoku Class.
/// When the object is serialized out as xml, it's qualified name is p:kinsoku.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:kinsoku")]
pub struct Kinsoku {
    /// Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Invalid Kinsoku Start Characters
    /// Represents the following attribute in the schema: :invalStChars
    #[xml(attr = "invalStChars")]
    pub invalid_start_chars: String,
    /// Invalid Kinsoku End Characters
    /// Represents the following attribute in the schema: :invalEndChars
    #[xml(attr = "invalEndChars")]
    pub invalid_end_chars: String,
}
/// Defines the ModificationVerifier Class.
/// When the object is serialized out as xml, it's qualified name is p:modifyVerifier.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:modifyVerifier")]
pub struct ModificationVerifier {
    /// Cryptographic Provider Type
    /// Represents the following attribute in the schema: :cryptProviderType
    #[xml(attr = "cryptProviderType")]
    pub cryptographic_provider_type: CryptProviderValues,
    /// Cryptographic Algorithm Class
    /// Represents the following attribute in the schema: :cryptAlgorithmClass
    #[xml(attr = "cryptAlgorithmClass")]
    pub cryptographic_algorithm_class: CryptAlgorithmClassValues,
    /// Cryptographic Algorithm Type
    /// Represents the following attribute in the schema: :cryptAlgorithmType
    #[xml(attr = "cryptAlgorithmType")]
    pub cryptographic_algorithm_type: CryptAlgorithmValues,
    /// Cryptographic Hashing Algorithm
    /// Represents the following attribute in the schema: :cryptAlgorithmSid
    #[xml(attr = "cryptAlgorithmSid")]
    pub cryptographic_algorithm_sid: u32,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: :spinCount
    #[xml(attr = "spinCount")]
    pub spin_count: u32,
    /// Salt for Password Verifier
    /// Represents the following attribute in the schema: :saltData
    #[xml(attr = "saltData")]
    pub salt_data: String,
    /// Password Hash
    /// Represents the following attribute in the schema: :hashData
    #[xml(attr = "hashData")]
    pub hash_data: String,
    /// Cryptographic Provider
    /// Represents the following attribute in the schema: :cryptProvider
    #[xml(attr = "cryptProvider")]
    pub cryptographic_provider: Option<String>,
    /// Cryptographic Algorithm Extensibility
    /// Represents the following attribute in the schema: :algIdExt
    #[xml(attr = "algIdExt")]
    pub extended_cryptographic_algorithm: Option<u32>,
    /// Algorithm Extensibility Source
    /// Represents the following attribute in the schema: :algIdExtSource
    #[xml(attr = "algIdExtSource")]
    pub extended_cryptographic_algorithm_source: Option<String>,
    /// Cryptographic Provider Type Extensibility
    /// Represents the following attribute in the schema: :cryptProviderTypeExt
    #[xml(attr = "cryptProviderTypeExt")]
    pub cryptographic_provider_type_extensibility: Option<u32>,
    /// Provider Type Extensibility Source
    /// Represents the following attribute in the schema: :cryptProviderTypeExtSource
    #[xml(attr = "cryptProviderTypeExtSource")]
    pub cryptographic_provider_type_extensibility_source: Option<String>,
    /// algorithmName
    /// Represents the following attribute in the schema: :algorithmName
    #[xml(attr = "algorithmName")]
    pub algorithm_name: Option<String>,
    /// hashValue
    /// Represents the following attribute in the schema: :hashValue
    #[xml(attr = "hashValue")]
    pub hash_value: Option<String>,
    /// saltValue
    /// Represents the following attribute in the schema: :saltValue
    #[xml(attr = "saltValue")]
    pub salt_value: Option<String>,
    /// spinValue
    /// Represents the following attribute in the schema: :spinValue
    #[xml(attr = "spinValue")]
    pub spin_value: Option<u32>,
}
/// Defines the PresentationExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct PresentationExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<PresentationExtension>,
}
/// Defines the PresentationExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct PresentationExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "p14:sectionPr",
        child = "p14:sectionLst",
        child = "p15:sldGuideLst",
        child = "p15:notesGuideLst",
    )]
    pub children: Vec<PresentationExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PresentationExtensionChildChoice {
    #[xml(tag = "p14:sectionPr")]
    P14SectionPr(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SectionProperties,
    ),
    #[xml(tag = "p14:sectionLst")]
    P14SectionLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SectionList,
    ),
    #[xml(tag = "p15:sldGuideLst")]
    P15SldGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    ),
    #[xml(tag = "p15:notesGuideLst")]
    P15NotesGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::NotesGuideList,
    ),
}
/// HTML Publishing Properties.
/// When the object is serialized out as xml, it's qualified name is p:htmlPubPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:htmlPubPr")]
pub struct HtmlPublishProperties {
    /// Show Speaker Notes
    /// Represents the following attribute in the schema: :showSpeakerNotes
    #[xml(attr = "showSpeakerNotes")]
    pub show_speaker_notes: Option<bool>,
    /// Browser Support Target
    /// Represents the following attribute in the schema: :pubBrowser
    #[xml(attr = "pubBrowser")]
    pub target_browser: Option<HtmlPublishWebBrowserSupportValues>,
    /// Publish Path
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
    #[xml(
        child = "p:sldAll",
        child = "p:sldRg",
        child = "p:custShow",
        child = "p:extLst",
    )]
    pub children: Vec<HtmlPublishPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HtmlPublishPropertiesChildChoice {
    #[xml(tag = "p:sldAll")]
    PSldAll(SlideAll),
    #[xml(tag = "p:sldRg")]
    PSldRg(SlideRange),
    #[xml(tag = "p:custShow")]
    PCustShow(CustomShowReference),
    #[xml(tag = "p:extLst")]
    PExtLst(ExtensionList),
}
/// Web Properties.
/// When the object is serialized out as xml, it's qualified name is p:webPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:webPr")]
pub struct WebProperties {
    /// Show animation in HTML output
    /// Represents the following attribute in the schema: :showAnimation
    #[xml(attr = "showAnimation")]
    pub show_animation: Option<bool>,
    /// Resize graphics in HTML output
    /// Represents the following attribute in the schema: :resizeGraphics
    #[xml(attr = "resizeGraphics")]
    pub resize_graphics: Option<bool>,
    /// Allow PNG in HTML output
    /// Represents the following attribute in the schema: :allowPng
    #[xml(attr = "allowPng")]
    pub allow_png: Option<bool>,
    /// Rely on VML for HTML output
    /// Represents the following attribute in the schema: :relyOnVml
    #[xml(attr = "relyOnVml")]
    pub rely_on_vml: Option<bool>,
    /// Organize HTML output in folders
    /// Represents the following attribute in the schema: :organizeInFolders
    #[xml(attr = "organizeInFolders")]
    pub organize_in_folders: Option<bool>,
    /// Use long file names in HTML output
    /// Represents the following attribute in the schema: :useLongFilenames
    #[xml(attr = "useLongFilenames")]
    pub use_long_filenames: Option<bool>,
    /// Image size for HTML output
    /// Represents the following attribute in the schema: :imgSz
    #[xml(attr = "imgSz")]
    pub image_size: Option<WebScreenSizeValues>,
    /// Encoding for HTML output
    /// Represents the following attribute in the schema: :encoding
    #[xml(attr = "encoding")]
    pub encoding: Option<String>,
    /// Slide Navigation Colors for HTML output
    /// Represents the following attribute in the schema: :clr
    #[xml(attr = "clr")]
    pub color: Option<WebColorValues>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the PrintingProperties Class.
/// When the object is serialized out as xml, it's qualified name is p:prnPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:prnPr")]
pub struct PrintingProperties {
    /// Print Output
    /// Represents the following attribute in the schema: :prnWhat
    #[xml(attr = "prnWhat")]
    pub print_what: Option<PrintOutputValues>,
    /// Print Color Mode
    /// Represents the following attribute in the schema: :clrMode
    #[xml(attr = "clrMode")]
    pub color_mode: Option<PrintColorModeValues>,
    /// Print Hidden Slides
    /// Represents the following attribute in the schema: :hiddenSlides
    #[xml(attr = "hiddenSlides")]
    pub hidden_slides: Option<bool>,
    /// Scale to Fit Paper when printing
    /// Represents the following attribute in the schema: :scaleToFitPaper
    #[xml(attr = "scaleToFitPaper")]
    pub scale_to_fit_paper: Option<bool>,
    /// Frame slides when printing
    /// Represents the following attribute in the schema: :frameSlides
    #[xml(attr = "frameSlides")]
    pub frame_slides: Option<bool>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ShowProperties Class.
/// When the object is serialized out as xml, it's qualified name is p:showPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:showPr")]
pub struct ShowProperties {
    /// Loop Slide Show
    /// Represents the following attribute in the schema: :loop
    #[xml(attr = "loop")]
    pub r#loop: Option<bool>,
    /// Show Narration in Slide Show
    /// Represents the following attribute in the schema: :showNarration
    #[xml(attr = "showNarration")]
    pub show_narration: Option<bool>,
    /// Show Animation in Slide Show
    /// Represents the following attribute in the schema: :showAnimation
    #[xml(attr = "showAnimation")]
    pub show_animation: Option<bool>,
    /// Use Timings in Slide Show
    /// Represents the following attribute in the schema: :useTimings
    #[xml(attr = "useTimings")]
    pub use_timings: Option<bool>,
    #[xml(
        child = "p:present",
        child = "p:browse",
        child = "p:kiosk",
        child = "p:sldAll",
        child = "p:sldRg",
        child = "p:custShow",
        child = "p:penClr",
        child = "p:extLst",
    )]
    pub children: Vec<ShowPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShowPropertiesChildChoice {
    #[xml(tag = "p:present")]
    PPresent(PresenterSlideMode),
    #[xml(tag = "p:browse")]
    PBrowse(BrowseSlideMode),
    #[xml(tag = "p:kiosk")]
    PKiosk(KioskSlideMode),
    #[xml(tag = "p:sldAll")]
    PSldAll(SlideAll),
    #[xml(tag = "p:sldRg")]
    PSldRg(SlideRange),
    #[xml(tag = "p:custShow")]
    PCustShow(CustomShowReference),
    #[xml(tag = "p:penClr")]
    PPenClr(PenColor),
    #[xml(tag = "p:extLst")]
    PExtLst(ShowPropertiesExtensionList),
}
/// Defines the ColorMostRecentlyUsed Class.
/// When the object is serialized out as xml, it's qualified name is p:clrMru.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:clrMru")]
pub struct ColorMostRecentlyUsed {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorMostRecentlyUsedChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorMostRecentlyUsedChildChoice {
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
/// Defines the PresentationPropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct PresentationPropertiesExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<PresentationPropertiesExtension>,
}
/// Defines the PresentationPropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct PresentationPropertiesExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "p14:discardImageEditData",
        child = "p14:defaultImageDpi",
        child = "a14:m",
        child = "p15:chartTrackingRefBased",
    )]
    pub children: Vec<PresentationPropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PresentationPropertiesExtensionChildChoice {
    #[xml(tag = "p14:discardImageEditData")]
    P14DiscardImageEditData(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DiscardImageEditData,
    ),
    #[xml(tag = "p14:defaultImageDpi")]
    P14DefaultImageDpi(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DefaultImageDpi,
    ),
    #[xml(tag = "a14:m")]
    A14M(crate::schemas::schemas_microsoft_com_office_drawing_2010_main::TextMath),
    #[xml(tag = "p15:chartTrackingRefBased")]
    P15ChartTrackingRefBased(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::ChartTrackingReferenceBased,
    ),
}
/// Defines the HeaderFooter Class.
/// When the object is serialized out as xml, it's qualified name is p:hf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:hf")]
pub struct HeaderFooter {
    /// Slide Number Placeholder
    /// Represents the following attribute in the schema: :sldNum
    #[xml(attr = "sldNum")]
    pub slide_number: Option<bool>,
    /// Header Placeholder
    /// Represents the following attribute in the schema: :hdr
    #[xml(attr = "hdr")]
    pub header: Option<bool>,
    /// Footer Placeholder
    /// Represents the following attribute in the schema: :ftr
    #[xml(attr = "ftr")]
    pub footer: Option<bool>,
    /// Date/Time Placeholder
    /// Represents the following attribute in the schema: :dt
    #[xml(attr = "dt")]
    pub date_time: Option<bool>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list_with_modification: Option<ExtensionListWithModification>,
}
/// Defines the SlideLayoutExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct SlideLayoutExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<SlideLayoutExtension>,
}
/// Defines the SlideLayoutIdList Class.
/// When the object is serialized out as xml, it's qualified name is p:sldLayoutIdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sldLayoutIdLst")]
pub struct SlideLayoutIdList {
    /// _
    #[xml(child = "p:sldLayoutId")]
    pub p_sld_layout_id: Vec<SlideLayoutId>,
}
/// Defines the TextStyles Class.
/// When the object is serialized out as xml, it's qualified name is p:txStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:txStyles")]
pub struct TextStyles {
    ///Slide Master Title Text Style
    #[xml(child = "p:titleStyle")]
    pub title_style: Option<TitleStyle>,
    ///Slide Master Body Text Style
    #[xml(child = "p:bodyStyle")]
    pub body_style: Option<BodyStyle>,
    ///Slide Master Other Text Style
    #[xml(child = "p:otherStyle")]
    pub other_style: Option<OtherStyle>,
    /// _
    #[xml(child = "p:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the SlideMasterExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct SlideMasterExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<SlideMasterExtension>,
}
/// Defines the HandoutMasterExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct HandoutMasterExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<HandoutMasterExtension>,
}
/// Defines the NotesMasterExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is p:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:extLst")]
pub struct NotesMasterExtensionList {
    /// _
    #[xml(child = "p:ext")]
    pub p_ext: Vec<NotesMasterExtension>,
}
/// OLE Chart Element.
/// When the object is serialized out as xml, it's qualified name is p:oleChartEl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:oleChartEl")]
pub struct OleChartElement {
    /// Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ChartSubElementValues,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<u32>,
}
/// Text Element.
/// When the object is serialized out as xml, it's qualified name is p:txEl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:txEl")]
pub struct TextElement {
    #[xml(child = "p:charRg", child = "p:pRg")]
    pub children: Vec<TextElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextElementChildChoice {
    #[xml(tag = "p:charRg")]
    PCharRg(CharRange),
    #[xml(tag = "p:pRg")]
    PPRg(ParagraphIndexRange),
}
/// Graphic Element.
/// When the object is serialized out as xml, it's qualified name is p:graphicEl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:graphicEl")]
pub struct GraphicElement {
    #[xml(child = "a:dgm", child = "a:chart")]
    pub children: Vec<GraphicElementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GraphicElementChildChoice {
    #[xml(tag = "a:dgm")]
    ADgm(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Diagram),
    #[xml(tag = "a:chart")]
    AChart(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Chart),
}
/// Defines the BlindsTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:blinds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:blinds")]
pub struct BlindsTransition {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<DirectionValues>,
}
/// Defines the CheckerTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:checker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:checker")]
pub struct CheckerTransition {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<DirectionValues>,
}
/// Defines the CombTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:comb.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:comb")]
pub struct CombTransition {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<DirectionValues>,
}
/// Defines the RandomBarTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:randomBar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:randomBar")]
pub struct RandomBarTransition {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<DirectionValues>,
}
/// Defines the OrientationTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OrientationTransitionType {
    /// Transition Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<DirectionValues>,
}
/// Defines the CoverTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:cover.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cover")]
pub struct CoverTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<String>,
}
/// Defines the PullTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:pull.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:pull")]
pub struct PullTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<String>,
}
/// Defines the EightDirectionTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EightDirectionTransitionType {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<String>,
}
/// Defines the CutTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:cut.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:cut")]
pub struct CutTransition {
    /// Transition Through Black
    /// Represents the following attribute in the schema: :thruBlk
    #[xml(attr = "thruBlk")]
    pub through_black: Option<bool>,
}
/// Defines the FadeTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:fade.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:fade")]
pub struct FadeTransition {
    /// Transition Through Black
    /// Represents the following attribute in the schema: :thruBlk
    #[xml(attr = "thruBlk")]
    pub through_black: Option<bool>,
}
/// Defines the OptionalBlackTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OptionalBlackTransitionType {
    /// Transition Through Black
    /// Represents the following attribute in the schema: :thruBlk
    #[xml(attr = "thruBlk")]
    pub through_black: Option<bool>,
}
/// Defines the PushTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:push.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:push")]
pub struct PushTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the WipeTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:wipe.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:wipe")]
pub struct WipeTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the SideDirectionTransitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SideDirectionTransitionType {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionSlideDirectionValues>,
}
/// Defines the SplitTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:split.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:split")]
pub struct SplitTransition {
    /// Orientation
    /// Represents the following attribute in the schema: :orient
    #[xml(attr = "orient")]
    pub orientation: Option<DirectionValues>,
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the StripsTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:strips.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:strips")]
pub struct StripsTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionCornerDirectionValues>,
}
/// Defines the WheelTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:wheel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:wheel")]
pub struct WheelTransition {
    /// Spokes
    /// Represents the following attribute in the schema: :spokes
    #[xml(attr = "spokes")]
    pub spokes: Option<u32>,
}
/// Defines the ZoomTransition Class.
/// When the object is serialized out as xml, it's qualified name is p:zoom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:zoom")]
pub struct ZoomTransition {
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<TransitionInOutDirectionValues>,
}
/// Defines the SoundAction Class.
/// When the object is serialized out as xml, it's qualified name is p:sndAc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:sndAc")]
pub struct SoundAction {
    #[xml(child = "p:stSnd", child = "p:endSnd")]
    pub children: Vec<SoundActionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SoundActionChildChoice {
    #[xml(tag = "p:stSnd")]
    PStSnd(StartSoundAction),
    #[xml(tag = "p:endSnd")]
    PEndSnd(EndSoundAction),
}
/// Defines the PlaceholderExtension Class.
/// When the object is serialized out as xml, it's qualified name is p:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "p:ext")]
pub struct PlaceholderExtension {
    /// _
    #[xml(child = "p232:phTypeExt")]
    pub placeholder_type_extension: Option<
        crate::schemas::schemas_microsoft_com_office_powerpoint_2023_02_main::PlaceholderTypeExtension,
    >,
}
