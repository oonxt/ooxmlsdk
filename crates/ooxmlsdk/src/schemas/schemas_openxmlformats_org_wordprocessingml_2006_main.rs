#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OnOffOnlyValues {
    #[default]
    On,
    Off,
}
crate::__string_enum! {
    OnOffOnlyValues { On = "on", Off = "off", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HighlightColorValues {
    #[default]
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    Yellow,
    White,
    DarkBlue,
    DarkCyan,
    DarkGreen,
    DarkMagenta,
    DarkRed,
    DarkYellow,
    DarkGray,
    LightGray,
    None,
}
crate::__string_enum! {
    HighlightColorValues { Black = "black", Blue = "blue", Cyan = "cyan", Green =
    "green", Magenta = "magenta", Red = "red", Yellow = "yellow", White = "white",
    DarkBlue = "darkBlue", DarkCyan = "darkCyan", DarkGreen = "darkGreen", DarkMagenta =
    "darkMagenta", DarkRed = "darkRed", DarkYellow = "darkYellow", DarkGray = "darkGray",
    LightGray = "lightGray", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AutomaticColorValues {
    #[default]
    Auto,
}
crate::__string_enum! {
    AutomaticColorValues { Auto = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum UnderlineValues {
    #[default]
    Single,
    Words,
    Double,
    Thick,
    Dotted,
    DottedHeavy,
    Dash,
    DashedHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DashDotHeavy,
    DotDotDash,
    DashDotDotHeavy,
    Wave,
    WavyHeavy,
    WavyDouble,
    None,
}
crate::__string_enum! {
    UnderlineValues { Single = "single", Words = "words", Double = "double", Thick =
    "thick", Dotted = "dotted", DottedHeavy = "dottedHeavy", Dash = "dash", DashedHeavy =
    "dashedHeavy", DashLong = "dashLong", DashLongHeavy = "dashLongHeavy", DotDash =
    "dotDash", DashDotHeavy = "dashDotHeavy", DotDotDash = "dotDotDash", DashDotDotHeavy
    = "dashDotDotHeavy", Wave = "wave", WavyHeavy = "wavyHeavy", WavyDouble =
    "wavyDouble", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextEffectValues {
    #[default]
    BlinkBackground,
    Lights,
    AntsBlack,
    AntsRed,
    Shimmer,
    Sparkle,
    None,
}
crate::__string_enum! {
    TextEffectValues { BlinkBackground = "blinkBackground", Lights = "lights", AntsBlack
    = "antsBlack", AntsRed = "antsRed", Shimmer = "shimmer", Sparkle = "sparkle", None =
    "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalPositionValues {
    #[default]
    Baseline,
    Superscript,
    Subscript,
}
crate::__string_enum! {
    VerticalPositionValues { Baseline = "baseline", Superscript = "superscript",
    Subscript = "subscript", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EmphasisMarkValues {
    #[default]
    None,
    Dot,
    Comma,
    Circle,
    UnderDot,
}
crate::__string_enum! {
    EmphasisMarkValues { None = "none", Dot = "dot", Comma = "comma", Circle = "circle",
    UnderDot = "underDot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CombineBracketValues {
    #[default]
    None,
    Round,
    Square,
    Angle,
    Curly,
}
crate::__string_enum! {
    CombineBracketValues { None = "none", Round = "round", Square = "square", Angle =
    "angle", Curly = "curly", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
    Inside,
    Outside,
}
crate::__string_enum! {
    HorizontalAlignmentValues { Left = "left", Center = "center", Right = "right", Inside
    = "inside", Outside = "outside", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAlignmentValues {
    #[default]
    Inline,
    Top,
    Center,
    Bottom,
    Inside,
    Outside,
}
crate::__string_enum! {
    VerticalAlignmentValues { Inline = "inline", Top = "top", Center = "center", Bottom =
    "bottom", Inside = "inside", Outside = "outside", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HeightRuleValues {
    #[default]
    Auto,
    Exact,
    AtLeast,
}
crate::__string_enum! {
    HeightRuleValues { Auto = "auto", Exact = "exact", AtLeast = "atLeast", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextWrappingValues {
    #[default]
    Auto,
    NotBeside,
    Around,
    Tight,
    Through,
    None,
}
crate::__string_enum! {
    TextWrappingValues { Auto = "auto", NotBeside = "notBeside", Around = "around", Tight
    = "tight", Through = "through", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAnchorValues {
    #[default]
    Text,
    Margin,
    Page,
}
crate::__string_enum! {
    VerticalAnchorValues { Text = "text", Margin = "margin", Page = "page", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAnchorValues {
    #[default]
    Text,
    Margin,
    Page,
}
crate::__string_enum! {
    HorizontalAnchorValues { Text = "text", Margin = "margin", Page = "page", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DropCapLocationValues {
    #[default]
    None,
    Drop,
    Margin,
}
crate::__string_enum! {
    DropCapLocationValues { None = "none", Drop = "drop", Margin = "margin", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabStopLeaderCharValues {
    #[default]
    None,
    Dot,
    Hyphen,
    Underscore,
    Heavy,
    MiddleDot,
}
crate::__string_enum! {
    TabStopLeaderCharValues { None = "none", Dot = "dot", Hyphen = "hyphen", Underscore =
    "underscore", Heavy = "heavy", MiddleDot = "middleDot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LineSpacingRuleValues {
    #[default]
    Auto,
    Exact,
    AtLeast,
}
crate::__string_enum! {
    LineSpacingRuleValues { Auto = "auto", Exact = "exact", AtLeast = "atLeast", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableRowAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    TableRowAlignmentValues { Left = "left", Center = "center", Right = "right", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ViewValues {
    #[default]
    None,
    Print,
    Outline,
    MasterPages,
    Normal,
    Web,
}
crate::__string_enum! {
    ViewValues { None = "none", Print = "print", Outline = "outline", MasterPages =
    "masterPages", Normal = "normal", Web = "web", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetZoomValues {
    #[default]
    None,
    FullPage,
    BestFit,
    TextFit,
}
crate::__string_enum! {
    PresetZoomValues { None = "none", FullPage = "fullPage", BestFit = "bestFit", TextFit
    = "textFit", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ProofingStateValues {
    #[default]
    Clean,
    Dirty,
}
crate::__string_enum! {
    ProofingStateValues { Clean = "clean", Dirty = "dirty", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocumentTypeValues {
    #[default]
    NotSpecified,
    Letter,
    Email,
}
crate::__string_enum! {
    DocumentTypeValues { NotSpecified = "notSpecified", Letter = "letter", Email =
    "eMail", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocumentProtectionValues {
    #[default]
    None,
    ReadOnly,
    Comments,
    TrackedChanges,
    Forms,
}
crate::__string_enum! {
    DocumentProtectionValues { None = "none", ReadOnly = "readOnly", Comments =
    "comments", TrackedChanges = "trackedChanges", Forms = "forms", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MailMergeDocumentValues {
    #[default]
    Catalog,
    Envelope,
    MailingLabel,
    FormLetter,
    Email,
    Fax,
}
crate::__string_enum! {
    MailMergeDocumentValues { Catalog = "catalog", Envelope = "envelopes", MailingLabel =
    "mailingLabels", FormLetter = "formLetters", Email = "email", Fax = "fax", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MailMergeDataValues {
    #[default]
    TextFile,
    Database,
    Spreadsheet,
    Query,
    Odbc,
    Native,
}
crate::__string_enum! {
    MailMergeDataValues { TextFile = "textFile", Database = "database", Spreadsheet =
    "spreadsheet", Query = "query", Odbc = "odbc", Native = "native", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MailMergeDestinationValues {
    #[default]
    NewDocument,
    Printer,
    Email,
    Fax,
}
crate::__string_enum! {
    MailMergeDestinationValues { NewDocument = "newDocument", Printer = "printer", Email
    = "email", Fax = "fax", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MailMergeOdsoFieldValues {
    #[default]
    Null,
    DbColumn,
}
crate::__string_enum! {
    MailMergeOdsoFieldValues { Null = "null", DbColumn = "dbColumn", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalTextAlignmentValues {
    #[default]
    Top,
    Center,
    Baseline,
    Bottom,
    Auto,
}
crate::__string_enum! {
    VerticalTextAlignmentValues { Top = "top", Center = "center", Baseline = "baseline",
    Bottom = "bottom", Auto = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DisplacedByCustomXmlValues {
    #[default]
    Next,
    Previous,
}
crate::__string_enum! {
    DisplacedByCustomXmlValues { Next = "next", Previous = "prev", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalMergeRevisionValues {
    #[default]
    Continue,
    Restart,
}
crate::__string_enum! {
    VerticalMergeRevisionValues { Continue = "cont", Restart = "rest", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextBoxTightWrapValues {
    #[default]
    None,
    AllLines,
    FirstAndLastLine,
    FirstLineOnly,
    LastLineOnly,
}
crate::__string_enum! {
    TextBoxTightWrapValues { None = "none", AllLines = "allLines", FirstAndLastLine =
    "firstAndLastLine", FirstLineOnly = "firstLineOnly", LastLineOnly = "lastLineOnly", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FieldCharValues {
    #[default]
    Begin,
    Separate,
    End,
}
crate::__string_enum! {
    FieldCharValues { Begin = "begin", Separate = "separate", End = "end", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InfoTextValues {
    #[default]
    Text,
    AutoText,
}
crate::__string_enum! {
    InfoTextValues { Text = "text", AutoText = "autoText", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextBoxFormFieldValues {
    #[default]
    Regular,
    Number,
    Date,
    CurrentTime,
    CurrentDate,
    Calculated,
}
crate::__string_enum! {
    TextBoxFormFieldValues { Regular = "regular", Number = "number", Date = "date",
    CurrentTime = "currentTime", CurrentDate = "currentDate", Calculated = "calculated",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SectionMarkValues {
    #[default]
    NextPage,
    NextColumn,
    Continuous,
    EvenPage,
    OddPage,
}
crate::__string_enum! {
    SectionMarkValues { NextPage = "nextPage", NextColumn = "nextColumn", Continuous =
    "continuous", EvenPage = "evenPage", OddPage = "oddPage", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageOrientationValues {
    #[default]
    Portrait,
    Landscape,
}
crate::__string_enum! {
    PageOrientationValues { Portrait = "portrait", Landscape = "landscape", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageBorderZOrderValues {
    #[default]
    Front,
    Back,
}
crate::__string_enum! {
    PageBorderZOrderValues { Front = "front", Back = "back", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageBorderDisplayValues {
    #[default]
    AllPages,
    FirstPage,
    NotFirstPage,
}
crate::__string_enum! {
    PageBorderDisplayValues { AllPages = "allPages", FirstPage = "firstPage",
    NotFirstPage = "notFirstPage", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageBorderOffsetValues {
    #[default]
    Page,
    Text,
}
crate::__string_enum! {
    PageBorderOffsetValues { Page = "page", Text = "text", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChapterSeparatorValues {
    #[default]
    Hyphen,
    Period,
    Colon,
    EmDash,
    EnDash,
}
crate::__string_enum! {
    ChapterSeparatorValues { Hyphen = "hyphen", Period = "period", Colon = "colon",
    EmDash = "emDash", EnDash = "enDash", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LineNumberRestartValues {
    #[default]
    NewPage,
    NewSection,
    Continuous,
}
crate::__string_enum! {
    LineNumberRestartValues { NewPage = "newPage", NewSection = "newSection", Continuous
    = "continuous", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalJustificationValues {
    #[default]
    Top,
    Center,
    Both,
    Bottom,
}
crate::__string_enum! {
    VerticalJustificationValues { Top = "top", Center = "center", Both = "both", Bottom =
    "bottom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableVerticalAlignmentValues {
    #[default]
    Top,
    Center,
    Bottom,
}
crate::__string_enum! {
    TableVerticalAlignmentValues { Top = "top", Center = "center", Bottom = "bottom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocGridValues {
    #[default]
    Default,
    Lines,
    LinesAndChars,
    SnapToChars,
}
crate::__string_enum! {
    DocGridValues { Default = "default", Lines = "lines", LinesAndChars =
    "linesAndChars", SnapToChars = "snapToChars", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HeaderFooterValues {
    #[default]
    Even,
    Default,
    First,
}
crate::__string_enum! {
    HeaderFooterValues { Even = "even", Default = "default", First = "first", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FootnoteEndnoteValues {
    #[default]
    Normal,
    Separator,
    ContinuationSeparator,
    ContinuationNotice,
}
crate::__string_enum! {
    FootnoteEndnoteValues { Normal = "normal", Separator = "separator",
    ContinuationSeparator = "continuationSeparator", ContinuationNotice =
    "continuationNotice", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BreakValues {
    #[default]
    Page,
    Column,
    TextWrapping,
}
crate::__string_enum! {
    BreakValues { Page = "page", Column = "column", TextWrapping = "textWrapping", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BreakTextRestartLocationValues {
    #[default]
    None,
    Left,
    Right,
    All,
}
crate::__string_enum! {
    BreakTextRestartLocationValues { None = "none", Left = "left", Right = "right", All =
    "all", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AbsolutePositionTabAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    AbsolutePositionTabAlignmentValues { Left = "left", Center = "center", Right =
    "right", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AbsolutePositionTabPositioningBaseValues {
    #[default]
    Margin,
    Indent,
}
crate::__string_enum! {
    AbsolutePositionTabPositioningBaseValues { Margin = "margin", Indent = "indent", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AbsolutePositionTabLeaderCharValues {
    #[default]
    None,
    Dot,
    Hyphen,
    Underscore,
    MiddleDot,
}
crate::__string_enum! {
    AbsolutePositionTabLeaderCharValues { None = "none", Dot = "dot", Hyphen = "hyphen",
    Underscore = "underscore", MiddleDot = "middleDot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ProofingErrorValues {
    #[default]
    SpellStart,
    SpellEnd,
    GrammarStart,
    GrammarEnd,
}
crate::__string_enum! {
    ProofingErrorValues { SpellStart = "spellStart", SpellEnd = "spellEnd", GrammarStart
    = "gramStart", GrammarEnd = "gramEnd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RangePermissionEditingGroupValues {
    #[default]
    None,
    Everyone,
    Administrators,
    Contributors,
    Editors,
    Owners,
    Current,
}
crate::__string_enum! {
    RangePermissionEditingGroupValues { None = "none", Everyone = "everyone",
    Administrators = "administrators", Contributors = "contributors", Editors =
    "editors", Owners = "owners", Current = "current", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FontTypeHintValues {
    #[default]
    Default,
    EastAsia,
    ComplexScript,
}
crate::__string_enum! {
    FontTypeHintValues { Default = "default", EastAsia = "eastAsia", ComplexScript =
    "cs", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ThemeFontValues {
    #[default]
    MajorEastAsia,
    MajorBidi,
    MajorAscii,
    MajorHighAnsi,
    MinorEastAsia,
    MinorBidi,
    MinorAscii,
    MinorHighAnsi,
}
crate::__string_enum! {
    ThemeFontValues { MajorEastAsia = "majorEastAsia", MajorBidi = "majorBidi",
    MajorAscii = "majorAscii", MajorHighAnsi = "majorHAnsi", MinorEastAsia =
    "minorEastAsia", MinorBidi = "minorBidi", MinorAscii = "minorAscii", MinorHighAnsi =
    "minorHAnsi", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RubyAlignValues {
    #[default]
    Center,
    DistributeLetter,
    DistributeSpace,
    Left,
    Right,
    RightVertical,
}
crate::__string_enum! {
    RubyAlignValues { Center = "center", DistributeLetter = "distributeLetter",
    DistributeSpace = "distributeSpace", Left = "left", Right = "right", RightVertical =
    "rightVertical", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LockingValues {
    #[default]
    SdtLocked,
    ContentLocked,
    Unlocked,
    SdtContentLocked,
}
crate::__string_enum! {
    LockingValues { SdtLocked = "sdtLocked", ContentLocked = "contentLocked", Unlocked =
    "unlocked", SdtContentLocked = "sdtContentLocked", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DateFormatValues {
    #[default]
    Text,
    Date,
    DateTime,
}
crate::__string_enum! {
    DateFormatValues { Text = "text", Date = "date", DateTime = "dateTime", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableWidthUnitValues {
    #[default]
    Nil,
    Pct,
    Dxa,
    Auto,
}
crate::__string_enum! {
    TableWidthUnitValues { Nil = "nil", Pct = "pct", Dxa = "dxa", Auto = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableWidthValues {
    #[default]
    Nil,
    Dxa,
}
crate::__string_enum! {
    TableWidthValues { Nil = "nil", Dxa = "dxa", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MergedCellValues {
    #[default]
    Continue,
    Restart,
}
crate::__string_enum! {
    MergedCellValues { Continue = "continue", Restart = "restart", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableLayoutValues {
    #[default]
    Fixed,
    Autofit,
}
crate::__string_enum! {
    TableLayoutValues { Fixed = "fixed", Autofit = "autofit", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableOverlapValues {
    #[default]
    Never,
    Overlap,
}
crate::__string_enum! {
    TableOverlapValues { Never = "never", Overlap = "overlap", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FootnotePositionValues {
    #[default]
    PageBottom,
    BeneathText,
    SectionEnd,
}
crate::__string_enum! {
    FootnotePositionValues { PageBottom = "pageBottom", BeneathText = "beneathText",
    SectionEnd = "sectEnd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EndnotePositionValues {
    #[default]
    SectionEnd,
    DocumentEnd,
}
crate::__string_enum! {
    EndnotePositionValues { SectionEnd = "sectEnd", DocumentEnd = "docEnd", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RestartNumberValues {
    #[default]
    Continuous,
    EachSection,
    EachPage,
}
crate::__string_enum! {
    RestartNumberValues { Continuous = "continuous", EachSection = "eachSect", EachPage =
    "eachPage", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MailMergeSourceValues {
    #[default]
    Database,
    AddressBook,
    Document1,
    Document2,
    Text,
    Email,
    Native,
    Legacy,
    Master,
}
crate::__string_enum! {
    MailMergeSourceValues { Database = "database", AddressBook = "addressBook", Document1
    = "document1", Document2 = "document2", Text = "text", Email = "email", Native =
    "native", Legacy = "legacy", Master = "master", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TargetScreenSizeValues {
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
    Sz1800x1440,
    Sz1920x1200,
}
crate::__string_enum! {
    TargetScreenSizeValues { Sz544x376 = "544x376", Sz640x480 = "640x480", Sz720x512 =
    "720x512", Sz800x600 = "800x600", Sz1024x768 = "1024x768", Sz1152x882 = "1152x882",
    Sz1152x900 = "1152x900", Sz1280x1024 = "1280x1024", Sz1600x1200 = "1600x1200",
    Sz1800x1440 = "1800x1440", Sz1920x1200 = "1920x1200", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CharacterSpacingValues {
    #[default]
    DoNotCompress,
    CompressPunctuation,
    CompressPunctuationAndJapaneseKana,
}
crate::__string_enum! {
    CharacterSpacingValues { DoNotCompress = "doNotCompress", CompressPunctuation =
    "compressPunctuation", CompressPunctuationAndJapaneseKana =
    "compressPunctuationAndJapaneseKana", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ColorSchemeIndexValues {
    #[default]
    Dark1,
    Light1,
    Dark2,
    Light2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hyperlink,
    FollowedHyperlink,
}
crate::__string_enum! {
    ColorSchemeIndexValues { Dark1 = "dark1", Light1 = "light1", Dark2 = "dark2", Light2
    = "light2", Accent1 = "accent1", Accent2 = "accent2", Accent3 = "accent3", Accent4 =
    "accent4", Accent5 = "accent5", Accent6 = "accent6", Hyperlink = "hyperlink",
    FollowedHyperlink = "followedHyperlink", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FrameScrollbarVisibilityValues {
    #[default]
    On,
    Off,
    Auto,
}
crate::__string_enum! {
    FrameScrollbarVisibilityValues { On = "on", Off = "off", Auto = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FrameLayoutValues {
    #[default]
    Rows,
    Columns,
    None,
}
crate::__string_enum! {
    FrameLayoutValues { Rows = "rows", Columns = "cols", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LevelSuffixValues {
    #[default]
    Tab,
    Space,
    Nothing,
}
crate::__string_enum! {
    LevelSuffixValues { Tab = "tab", Space = "space", Nothing = "nothing", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MultiLevelValues {
    #[default]
    SingleLevel,
    Multilevel,
    HybridMultilevel,
}
crate::__string_enum! {
    MultiLevelValues { SingleLevel = "singleLevel", Multilevel = "multilevel",
    HybridMultilevel = "hybridMultilevel", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TableStyleOverrideValues {
    #[default]
    WholeTable,
    FirstRow,
    LastRow,
    FirstColumn,
    LastColumn,
    Band1Vertical,
    Band2Vertical,
    Band1Horizontal,
    Band2Horizontal,
    NorthEastCell,
    NorthWestCell,
    SouthEastCell,
    SouthWestCell,
}
crate::__string_enum! {
    TableStyleOverrideValues { WholeTable = "wholeTable", FirstRow = "firstRow", LastRow
    = "lastRow", FirstColumn = "firstCol", LastColumn = "lastCol", Band1Vertical =
    "band1Vert", Band2Vertical = "band2Vert", Band1Horizontal = "band1Horz",
    Band2Horizontal = "band2Horz", NorthEastCell = "neCell", NorthWestCell = "nwCell",
    SouthEastCell = "seCell", SouthWestCell = "swCell", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StyleValues {
    #[default]
    Paragraph,
    Character,
    Table,
    Numbering,
}
crate::__string_enum! {
    StyleValues { Paragraph = "paragraph", Character = "character", Table = "table",
    Numbering = "numbering", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FontFamilyValues {
    #[default]
    Decorative,
    Modern,
    Roman,
    Script,
    Swiss,
    Auto,
}
crate::__string_enum! {
    FontFamilyValues { Decorative = "decorative", Modern = "modern", Roman = "roman",
    Script = "script", Swiss = "swiss", Auto = "auto", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FontPitchValues {
    #[default]
    Fixed,
    Variable,
    Default,
}
crate::__string_enum! {
    FontPitchValues { Fixed = "fixed", Variable = "variable", Default = "default", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ThemeColorValues {
    #[default]
    Dark1,
    Light1,
    Dark2,
    Light2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hyperlink,
    FollowedHyperlink,
    None,
    Background1,
    Text1,
    Background2,
    Text2,
}
crate::__string_enum! {
    ThemeColorValues { Dark1 = "dark1", Light1 = "light1", Dark2 = "dark2", Light2 =
    "light2", Accent1 = "accent1", Accent2 = "accent2", Accent3 = "accent3", Accent4 =
    "accent4", Accent5 = "accent5", Accent6 = "accent6", Hyperlink = "hyperlink",
    FollowedHyperlink = "followedHyperlink", None = "none", Background1 = "background1",
    Text1 = "text1", Background2 = "background2", Text2 = "text2", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocPartBehaviorValues {
    #[default]
    Content,
    Paragraph,
    Page,
}
crate::__string_enum! {
    DocPartBehaviorValues { Content = "content", Paragraph = "p", Page = "pg", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocPartValues {
    #[default]
    None,
    Normal,
    AutoExp,
    Toolbar,
    Speller,
    FormField,
    SdtPlaceholder,
}
crate::__string_enum! {
    DocPartValues { None = "none", Normal = "normal", AutoExp = "autoExp", Toolbar =
    "toolbar", Speller = "speller", FormField = "formFld", SdtPlaceholder = "bbPlcHdr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocPartGalleryValues {
    #[default]
    Placeholder,
    Any,
    Default,
    DocumentPart,
    CoverPage,
    Equation,
    Footer,
    Header,
    PageNumber,
    Table,
    WaterMark,
    AutoText,
    TextBox,
    PageNumberTop,
    PageNumberBottom,
    PageNumberMargins,
    TableOfContents,
    Bibliography,
    CustomQuickParts,
    CustomCoverPage,
    CustomEquation,
    CustomFooter,
    CustomHeaders,
    CustomPageNumber,
    CustomTable,
    CustomWatermark,
    CustomAutoText,
    CustomTextBox,
    CustomPageNumberTop,
    CustomPageNumberBottom,
    CustomPageNumberMargin,
    CustomTableOfContents,
    CustomBibliography,
    Custom1,
    Custom2,
    Custom3,
    Custom4,
    Custom5,
}
crate::__string_enum! {
    DocPartGalleryValues { Placeholder = "placeholder", Any = "any", Default = "default",
    DocumentPart = "docParts", CoverPage = "coverPg", Equation = "eq", Footer = "ftrs",
    Header = "hdrs", PageNumber = "pgNum", Table = "tbls", WaterMark = "watermarks",
    AutoText = "autoTxt", TextBox = "txtBox", PageNumberTop = "pgNumT", PageNumberBottom
    = "pgNumB", PageNumberMargins = "pgNumMargins", TableOfContents = "tblOfContents",
    Bibliography = "bib", CustomQuickParts = "custQuickParts", CustomCoverPage =
    "custCoverPg", CustomEquation = "custEq", CustomFooter = "custFtrs", CustomHeaders =
    "custHdrs", CustomPageNumber = "custPgNum", CustomTable = "custTbls", CustomWatermark
    = "custWatermarks", CustomAutoText = "custAutoTxt", CustomTextBox = "custTxtBox",
    CustomPageNumberTop = "custPgNumT", CustomPageNumberBottom = "custPgNumB",
    CustomPageNumberMargin = "custPgNumMargins", CustomTableOfContents =
    "custTblOfContents", CustomBibliography = "custBib", Custom1 = "custom1", Custom2 =
    "custom2", Custom3 = "custom3", Custom4 = "custom4", Custom5 = "custom5", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CaptionPositionValues {
    #[default]
    Above,
    Below,
}
crate::__string_enum! {
    CaptionPositionValues { Above = "above", Below = "below", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LevelJustificationValues {
    #[default]
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    LevelJustificationValues { Left = "left", Center = "center", Right = "right", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ShadingPatternValues {
    #[default]
    Nil,
    Clear,
    Solid,
    HorizontalStripe,
    VerticalStripe,
    ReverseDiagonalStripe,
    DiagonalStripe,
    HorizontalCross,
    DiagonalCross,
    ThinHorizontalStripe,
    ThinVerticalStripe,
    ThinReverseDiagonalStripe,
    ThinDiagonalStripe,
    ThinHorizontalCross,
    ThinDiagonalCross,
    Percent5,
    Percent10,
    Percent12,
    Percent15,
    Percent20,
    Percent25,
    Percent30,
    Percent35,
    Percent37,
    Percent40,
    Percent45,
    Percent50,
    Percent55,
    Percent60,
    Percent62,
    Percent65,
    Percent70,
    Percent75,
    Percent80,
    Percent85,
    Percent87,
    Percent90,
    Percent95,
}
crate::__string_enum! {
    ShadingPatternValues { Nil = "nil", Clear = "clear", Solid = "solid",
    HorizontalStripe = "horzStripe", VerticalStripe = "vertStripe", ReverseDiagonalStripe
    = "reverseDiagStripe", DiagonalStripe = "diagStripe", HorizontalCross = "horzCross",
    DiagonalCross = "diagCross", ThinHorizontalStripe = "thinHorzStripe",
    ThinVerticalStripe = "thinVertStripe", ThinReverseDiagonalStripe =
    "thinReverseDiagStripe", ThinDiagonalStripe = "thinDiagStripe", ThinHorizontalCross =
    "thinHorzCross", ThinDiagonalCross = "thinDiagCross", Percent5 = "pct5", Percent10 =
    "pct10", Percent12 = "pct12", Percent15 = "pct15", Percent20 = "pct20", Percent25 =
    "pct25", Percent30 = "pct30", Percent35 = "pct35", Percent37 = "pct37", Percent40 =
    "pct40", Percent45 = "pct45", Percent50 = "pct50", Percent55 = "pct55", Percent60 =
    "pct60", Percent62 = "pct62", Percent65 = "pct65", Percent70 = "pct70", Percent75 =
    "pct75", Percent80 = "pct80", Percent85 = "pct85", Percent87 = "pct87", Percent90 =
    "pct90", Percent95 = "pct95", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StylePaneSortMethodsValues {
    #[default]
    Zero,
    Name,
    One,
    Priority,
    Two,
    Font,
    Three,
    BasedOn,
    Four,
    Type,
    Five,
    Default,
}
crate::__string_enum! {
    StylePaneSortMethodsValues { Zero = "0000", Name = "name", One = "0001", Priority =
    "priority", Two = "0002", Font = "font", Three = "0003", BasedOn = "basedOn", Four =
    "0004", Type = "type", Five = "0005", Default = "default", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DirectionValues {
    #[default]
    Ltr,
    Rtl,
}
crate::__string_enum! {
    DirectionValues { Ltr = "ltr", Rtl = "rtl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CalendarValues {
    #[default]
    Gregorian,
    Hijri,
    Umalqura,
    Hebrew,
    Taiwan,
    Japan,
    Thai,
    Korea,
    Saka,
    GregorianTransliteratedEnglish,
    GregorianTransliteratedFrench,
    GregorianUs,
    GregorianMeFrench,
    GregorianArabic,
    None,
}
crate::__string_enum! {
    CalendarValues { Gregorian = "gregorian", Hijri = "hijri", Umalqura = "umalqura",
    Hebrew = "hebrew", Taiwan = "taiwan", Japan = "japan", Thai = "thai", Korea =
    "korea", Saka = "saka", GregorianTransliteratedEnglish = "gregorianXlitEnglish",
    GregorianTransliteratedFrench = "gregorianXlitFrench", GregorianUs = "gregorianUs",
    GregorianMeFrench = "gregorianMeFrench", GregorianArabic = "gregorianArabic", None =
    "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NumberFormatValues {
    #[default]
    Decimal,
    UpperRoman,
    LowerRoman,
    UpperLetter,
    LowerLetter,
    Ordinal,
    CardinalText,
    OrdinalText,
    Hex,
    Chicago,
    IdeographDigital,
    JapaneseCounting,
    Aiueo,
    Iroha,
    DecimalFullWidth,
    DecimalHalfWidth,
    JapaneseLegal,
    JapaneseDigitalTenThousand,
    DecimalEnclosedCircle,
    DecimalFullWidth2,
    AiueoFullWidth,
    IrohaFullWidth,
    DecimalZero,
    Bullet,
    Ganada,
    Chosung,
    DecimalEnclosedFullstop,
    DecimalEnclosedParen,
    DecimalEnclosedCircleChinese,
    IdeographEnclosedCircle,
    IdeographTraditional,
    IdeographZodiac,
    IdeographZodiacTraditional,
    TaiwaneseCounting,
    IdeographLegalTraditional,
    TaiwaneseCountingThousand,
    TaiwaneseDigital,
    ChineseCounting,
    ChineseLegalSimplified,
    ChineseCountingThousand,
    KoreanDigital,
    KoreanCounting,
    KoreanLegal,
    KoreanDigital2,
    VietnameseCounting,
    RussianLower,
    RussianUpper,
    None,
    NumberInDash,
    Hebrew1,
    Hebrew2,
    ArabicAlpha,
    ArabicAbjad,
    HindiVowels,
    HindiConsonants,
    HindiNumbers,
    HindiCounting,
    ThaiLetters,
    ThaiNumbers,
    ThaiCounting,
    BahtText,
    DollarText,
    Custom,
}
crate::__string_enum! {
    NumberFormatValues { Decimal = "decimal", UpperRoman = "upperRoman", LowerRoman =
    "lowerRoman", UpperLetter = "upperLetter", LowerLetter = "lowerLetter", Ordinal =
    "ordinal", CardinalText = "cardinalText", OrdinalText = "ordinalText", Hex = "hex",
    Chicago = "chicago", IdeographDigital = "ideographDigital", JapaneseCounting =
    "japaneseCounting", Aiueo = "aiueo", Iroha = "iroha", DecimalFullWidth =
    "decimalFullWidth", DecimalHalfWidth = "decimalHalfWidth", JapaneseLegal =
    "japaneseLegal", JapaneseDigitalTenThousand = "japaneseDigitalTenThousand",
    DecimalEnclosedCircle = "decimalEnclosedCircle", DecimalFullWidth2 =
    "decimalFullWidth2", AiueoFullWidth = "aiueoFullWidth", IrohaFullWidth =
    "irohaFullWidth", DecimalZero = "decimalZero", Bullet = "bullet", Ganada = "ganada",
    Chosung = "chosung", DecimalEnclosedFullstop = "decimalEnclosedFullstop",
    DecimalEnclosedParen = "decimalEnclosedParen", DecimalEnclosedCircleChinese =
    "decimalEnclosedCircleChinese", IdeographEnclosedCircle = "ideographEnclosedCircle",
    IdeographTraditional = "ideographTraditional", IdeographZodiac = "ideographZodiac",
    IdeographZodiacTraditional = "ideographZodiacTraditional", TaiwaneseCounting =
    "taiwaneseCounting", IdeographLegalTraditional = "ideographLegalTraditional",
    TaiwaneseCountingThousand = "taiwaneseCountingThousand", TaiwaneseDigital =
    "taiwaneseDigital", ChineseCounting = "chineseCounting", ChineseLegalSimplified =
    "chineseLegalSimplified", ChineseCountingThousand = "chineseCountingThousand",
    KoreanDigital = "koreanDigital", KoreanCounting = "koreanCounting", KoreanLegal =
    "koreanLegal", KoreanDigital2 = "koreanDigital2", VietnameseCounting =
    "vietnameseCounting", RussianLower = "russianLower", RussianUpper = "russianUpper",
    None = "none", NumberInDash = "numberInDash", Hebrew1 = "hebrew1", Hebrew2 =
    "hebrew2", ArabicAlpha = "arabicAlpha", ArabicAbjad = "arabicAbjad", HindiVowels =
    "hindiVowels", HindiConsonants = "hindiConsonants", HindiNumbers = "hindiNumbers",
    HindiCounting = "hindiCounting", ThaiLetters = "thaiLetters", ThaiNumbers =
    "thaiNumbers", ThaiCounting = "thaiCounting", BahtText = "bahtText", DollarText =
    "dollarText", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextDirectionValues {
    #[default]
    LefToRightTopToBottom,
    LeftToRightTopToBottom2010,
    TopToBottomRightToLeft,
    TopToBottomRightToLeft2010,
    BottomToTopLeftToRight,
    BottomToTopLeftToRight2010,
    LefttoRightTopToBottomRotated,
    LeftToRightTopToBottomRotated2010,
    TopToBottomRightToLeftRotated,
    TopToBottomRightToLeftRotated2010,
    TopToBottomLeftToRightRotated,
    TopToBottomLeftToRightRotated2010,
}
crate::__string_enum! {
    TextDirectionValues { LefToRightTopToBottom = "lrTb", LeftToRightTopToBottom2010 =
    "tb", TopToBottomRightToLeft = "tbRl", TopToBottomRightToLeft2010 = "rl",
    BottomToTopLeftToRight = "btLr", BottomToTopLeftToRight2010 = "lr",
    LefttoRightTopToBottomRotated = "lrTbV", LeftToRightTopToBottomRotated2010 = "tbV",
    TopToBottomRightToLeftRotated = "tbRlV", TopToBottomRightToLeftRotated2010 = "rlV",
    TopToBottomLeftToRightRotated = "tbLrV", TopToBottomLeftToRightRotated2010 = "lrV", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CryptAlgorithmValues {
    #[default]
    TypeAny,
    Custom,
}
crate::__string_enum! {
    CryptAlgorithmValues { TypeAny = "typeAny", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CryptAlgorithmClassValues {
    #[default]
    Hash,
    Custom,
}
crate::__string_enum! {
    CryptAlgorithmClassValues { Hash = "hash", Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CryptProviderValues {
    #[default]
    RsaAdvancedEncryptionStandard,
    RsaFull,
    Custom,
}
crate::__string_enum! {
    CryptProviderValues { RsaAdvancedEncryptionStandard = "rsaAes", RsaFull = "rsaFull",
    Custom = "custom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum JustificationValues {
    #[default]
    Left,
    Start,
    Center,
    Right,
    End,
    Both,
    MediumKashida,
    Distribute,
    NumTab,
    HighKashida,
    LowKashida,
    ThaiDistribute,
}
crate::__string_enum! {
    JustificationValues { Left = "left", Start = "start", Center = "center", Right =
    "right", End = "end", Both = "both", MediumKashida = "mediumKashida", Distribute =
    "distribute", NumTab = "numTab", HighKashida = "highKashida", LowKashida =
    "lowKashida", ThaiDistribute = "thaiDistribute", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabStopValues {
    #[default]
    Clear,
    Left,
    Start,
    Center,
    Right,
    End,
    Decimal,
    Bar,
    Number,
}
crate::__string_enum! {
    TabStopValues { Clear = "clear", Left = "left", Start = "start", Center = "center",
    Right = "right", End = "end", Decimal = "decimal", Bar = "bar", Number = "num", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BorderValues {
    #[default]
    Nil,
    None,
    Single,
    Thick,
    Double,
    Dotted,
    Dashed,
    DotDash,
    DotDotDash,
    Triple,
    ThinThickSmallGap,
    ThickThinSmallGap,
    ThinThickThinSmallGap,
    ThinThickMediumGap,
    ThickThinMediumGap,
    ThinThickThinMediumGap,
    ThinThickLargeGap,
    ThickThinLargeGap,
    ThinThickThinLargeGap,
    Wave,
    DoubleWave,
    DashSmallGap,
    DashDotStroked,
    ThreeDEmboss,
    ThreeDEngrave,
    Outset,
    Inset,
    Apples,
    ArchedScallops,
    BabyPacifier,
    BabyRattle,
    Balloons3Colors,
    BalloonsHotAir,
    BasicBlackDashes,
    BasicBlackDots,
    BasicBlackSquares,
    BasicThinLines,
    BasicWhiteDashes,
    BasicWhiteDots,
    BasicWhiteSquares,
    BasicWideInline,
    BasicWideMidline,
    BasicWideOutline,
    Bats,
    Birds,
    BirdsFlight,
    Cabins,
    CakeSlice,
    CandyCorn,
    CelticKnotwork,
    CertificateBanner,
    ChainLink,
    ChampagneBottle,
    CheckedBarBlack,
    CheckedBarColor,
    Checkered,
    ChristmasTree,
    CirclesLines,
    CirclesRectangles,
    ClassicalWave,
    Clocks,
    Compass,
    Confetti,
    ConfettiGrays,
    ConfettiOutline,
    ConfettiStreamers,
    ConfettiWhite,
    CornerTriangles,
    CouponCutoutDashes,
    CouponCutoutDots,
    CrazyMaze,
    CreaturesButterfly,
    CreaturesFish,
    CreaturesInsects,
    CreaturesLadyBug,
    CrossStitch,
    Cup,
    DecoArch,
    DecoArchColor,
    DecoBlocks,
    DiamondsGray,
    DoubleD,
    DoubleDiamonds,
    Earth1,
    Earth2,
    EclipsingSquares1,
    EclipsingSquares2,
    EggsBlack,
    Fans,
    Film,
    Firecrackers,
    FlowersBlockPrint,
    FlowersDaisies,
    FlowersModern1,
    FlowersModern2,
    FlowersPansy,
    FlowersRedRose,
    FlowersRoses,
    FlowersTeacup,
    FlowersTiny,
    Gems,
    GingerbreadMan,
    Gradient,
    Handmade1,
    Handmade2,
    HeartBalloon,
    HeartGray,
    Hearts,
    HeebieJeebies,
    Holly,
    HouseFunky,
    Hypnotic,
    IceCreamCones,
    LightBulb,
    Lightning1,
    Lightning2,
    MapPins,
    MapleLeaf,
    MapleMuffins,
    Marquee,
    MarqueeToothed,
    Moons,
    Mosaic,
    MusicNotes,
    Northwest,
    Ovals,
    Packages,
    PalmsBlack,
    PalmsColor,
    PaperClips,
    Papyrus,
    PartyFavor,
    PartyGlass,
    Pencils,
    People,
    PeopleWaving,
    PeopleHats,
    Poinsettias,
    PostageStamp,
    Pumpkin1,
    PushPinNote2,
    PushPinNote1,
    Pyramids,
    PyramidsAbove,
    Quadrants,
    Rings,
    Safari,
    Sawtooth,
    SawtoothGray,
    ScaredCat,
    Seattle,
    ShadowedSquares,
    SharksTeeth,
    ShorebirdTracks,
    Skyrocket,
    SnowflakeFancy,
    Snowflakes,
    Sombrero,
    Southwest,
    Stars,
    StarsTop,
    Stars3d,
    StarsBlack,
    StarsShadowed,
    Sun,
    Swirligig,
    TornPaper,
    TornPaperBlack,
    Trees,
    TriangleParty,
    Triangles,
    Tribal1,
    Tribal2,
    Tribal3,
    Tribal4,
    Tribal5,
    Tribal6,
    Triangle1,
    Triangle2,
    TriangleCircle1,
    TriangleCircle2,
    Shapes1,
    Shapes2,
    TwistedLines1,
    TwistedLines2,
    Vine,
    Waveline,
    WeavingAngles,
    WeavingBraid,
    WeavingRibbon,
    WeavingStrips,
    WhiteFlowers,
    Woodwork,
    XIllusions,
    ZanyTriangles,
    ZigZag,
    ZigZagStitch,
}
crate::__string_enum! {
    BorderValues { Nil = "nil", None = "none", Single = "single", Thick = "thick", Double
    = "double", Dotted = "dotted", Dashed = "dashed", DotDash = "dotDash", DotDotDash =
    "dotDotDash", Triple = "triple", ThinThickSmallGap = "thinThickSmallGap",
    ThickThinSmallGap = "thickThinSmallGap", ThinThickThinSmallGap =
    "thinThickThinSmallGap", ThinThickMediumGap = "thinThickMediumGap",
    ThickThinMediumGap = "thickThinMediumGap", ThinThickThinMediumGap =
    "thinThickThinMediumGap", ThinThickLargeGap = "thinThickLargeGap", ThickThinLargeGap
    = "thickThinLargeGap", ThinThickThinLargeGap = "thinThickThinLargeGap", Wave =
    "wave", DoubleWave = "doubleWave", DashSmallGap = "dashSmallGap", DashDotStroked =
    "dashDotStroked", ThreeDEmboss = "threeDEmboss", ThreeDEngrave = "threeDEngrave",
    Outset = "outset", Inset = "inset", Apples = "apples", ArchedScallops =
    "archedScallops", BabyPacifier = "babyPacifier", BabyRattle = "babyRattle",
    Balloons3Colors = "balloons3Colors", BalloonsHotAir = "balloonsHotAir",
    BasicBlackDashes = "basicBlackDashes", BasicBlackDots = "basicBlackDots",
    BasicBlackSquares = "basicBlackSquares", BasicThinLines = "basicThinLines",
    BasicWhiteDashes = "basicWhiteDashes", BasicWhiteDots = "basicWhiteDots",
    BasicWhiteSquares = "basicWhiteSquares", BasicWideInline = "basicWideInline",
    BasicWideMidline = "basicWideMidline", BasicWideOutline = "basicWideOutline", Bats =
    "bats", Birds = "birds", BirdsFlight = "birdsFlight", Cabins = "cabins", CakeSlice =
    "cakeSlice", CandyCorn = "candyCorn", CelticKnotwork = "celticKnotwork",
    CertificateBanner = "certificateBanner", ChainLink = "chainLink", ChampagneBottle =
    "champagneBottle", CheckedBarBlack = "checkedBarBlack", CheckedBarColor =
    "checkedBarColor", Checkered = "checkered", ChristmasTree = "christmasTree",
    CirclesLines = "circlesLines", CirclesRectangles = "circlesRectangles", ClassicalWave
    = "classicalWave", Clocks = "clocks", Compass = "compass", Confetti = "confetti",
    ConfettiGrays = "confettiGrays", ConfettiOutline = "confettiOutline",
    ConfettiStreamers = "confettiStreamers", ConfettiWhite = "confettiWhite",
    CornerTriangles = "cornerTriangles", CouponCutoutDashes = "couponCutoutDashes",
    CouponCutoutDots = "couponCutoutDots", CrazyMaze = "crazyMaze", CreaturesButterfly =
    "creaturesButterfly", CreaturesFish = "creaturesFish", CreaturesInsects =
    "creaturesInsects", CreaturesLadyBug = "creaturesLadyBug", CrossStitch =
    "crossStitch", Cup = "cup", DecoArch = "decoArch", DecoArchColor = "decoArchColor",
    DecoBlocks = "decoBlocks", DiamondsGray = "diamondsGray", DoubleD = "doubleD",
    DoubleDiamonds = "doubleDiamonds", Earth1 = "earth1", Earth2 = "earth2",
    EclipsingSquares1 = "eclipsingSquares1", EclipsingSquares2 = "eclipsingSquares2",
    EggsBlack = "eggsBlack", Fans = "fans", Film = "film", Firecrackers = "firecrackers",
    FlowersBlockPrint = "flowersBlockPrint", FlowersDaisies = "flowersDaisies",
    FlowersModern1 = "flowersModern1", FlowersModern2 = "flowersModern2", FlowersPansy =
    "flowersPansy", FlowersRedRose = "flowersRedRose", FlowersRoses = "flowersRoses",
    FlowersTeacup = "flowersTeacup", FlowersTiny = "flowersTiny", Gems = "gems",
    GingerbreadMan = "gingerbreadMan", Gradient = "gradient", Handmade1 = "handmade1",
    Handmade2 = "handmade2", HeartBalloon = "heartBalloon", HeartGray = "heartGray",
    Hearts = "hearts", HeebieJeebies = "heebieJeebies", Holly = "holly", HouseFunky =
    "houseFunky", Hypnotic = "hypnotic", IceCreamCones = "iceCreamCones", LightBulb =
    "lightBulb", Lightning1 = "lightning1", Lightning2 = "lightning2", MapPins =
    "mapPins", MapleLeaf = "mapleLeaf", MapleMuffins = "mapleMuffins", Marquee =
    "marquee", MarqueeToothed = "marqueeToothed", Moons = "moons", Mosaic = "mosaic",
    MusicNotes = "musicNotes", Northwest = "northwest", Ovals = "ovals", Packages =
    "packages", PalmsBlack = "palmsBlack", PalmsColor = "palmsColor", PaperClips =
    "paperClips", Papyrus = "papyrus", PartyFavor = "partyFavor", PartyGlass =
    "partyGlass", Pencils = "pencils", People = "people", PeopleWaving = "peopleWaving",
    PeopleHats = "peopleHats", Poinsettias = "poinsettias", PostageStamp =
    "postageStamp", Pumpkin1 = "pumpkin1", PushPinNote2 = "pushPinNote2", PushPinNote1 =
    "pushPinNote1", Pyramids = "pyramids", PyramidsAbove = "pyramidsAbove", Quadrants =
    "quadrants", Rings = "rings", Safari = "safari", Sawtooth = "sawtooth", SawtoothGray
    = "sawtoothGray", ScaredCat = "scaredCat", Seattle = "seattle", ShadowedSquares =
    "shadowedSquares", SharksTeeth = "sharksTeeth", ShorebirdTracks = "shorebirdTracks",
    Skyrocket = "skyrocket", SnowflakeFancy = "snowflakeFancy", Snowflakes =
    "snowflakes", Sombrero = "sombrero", Southwest = "southwest", Stars = "stars",
    StarsTop = "starsTop", Stars3d = "stars3d", StarsBlack = "starsBlack", StarsShadowed
    = "starsShadowed", Sun = "sun", Swirligig = "swirligig", TornPaper = "tornPaper",
    TornPaperBlack = "tornPaperBlack", Trees = "trees", TriangleParty = "triangleParty",
    Triangles = "triangles", Tribal1 = "tribal1", Tribal2 = "tribal2", Tribal3 =
    "tribal3", Tribal4 = "tribal4", Tribal5 = "tribal5", Tribal6 = "tribal6", Triangle1 =
    "triangle1", Triangle2 = "triangle2", TriangleCircle1 = "triangleCircle1",
    TriangleCircle2 = "triangleCircle2", Shapes1 = "shapes1", Shapes2 = "shapes2",
    TwistedLines1 = "twistedLines1", TwistedLines2 = "twistedLines2", Vine = "vine",
    Waveline = "waveline", WeavingAngles = "weavingAngles", WeavingBraid =
    "weavingBraid", WeavingRibbon = "weavingRibbon", WeavingStrips = "weavingStrips",
    WhiteFlowers = "whiteFlowers", Woodwork = "woodwork", XIllusions = "xIllusions",
    ZanyTriangles = "zanyTriangles", ZigZag = "zigZag", ZigZagStitch = "zigZagStitch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DocumentConformance {
    #[default]
    Transitional,
    Strict,
}
crate::__string_enum! {
    DocumentConformance { Transitional = "transitional", Strict = "strict", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StrictCharacterSet {
    #[default]
    ChsAnsi,
    ChsMacFfn,
    ChsShiftJis,
    ChsHangeul,
    ChsJohab,
    ChsGb2312,
    ChsChinese5,
    ChsGreek,
    ChsTurkish,
    ChsVietnamese,
    ChsHebrew,
    ChsArabic,
    ChsBaltic,
    ChsRussian,
    ChsThai,
    ChsEastEurope,
}
crate::__string_enum! {
    StrictCharacterSet { ChsAnsi = "iso88591", ChsMacFfn = "macintosh", ChsShiftJis =
    "shiftJis", ChsHangeul = "ksC56011987", ChsJohab = "ksC56011992", ChsGb2312 = "gbk",
    ChsChinese5 = "big5", ChsGreek = "windows1253", ChsTurkish = "iso88599",
    ChsVietnamese = "windows1258", ChsHebrew = "windows1255", ChsArabic = "windows1256",
    ChsBaltic = "windows1257", ChsRussian = "windows1251", ChsThai = "windows874",
    ChsEastEurope = "windows1250", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ObjectDrawAspect {
    #[default]
    Content,
    Icon,
}
crate::__string_enum! {
    ObjectDrawAspect { Content = "content", Icon = "icon", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ObjectUpdateMode {
    #[default]
    Always,
    OnCall,
}
crate::__string_enum! {
    ObjectUpdateMode { Always = "always", OnCall = "onCall", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CompatSettingNameValues {
    #[default]
    CompatibilityMode,
    OverrideTableStyleFontSizeAndJustification,
    EnableOpenTypeFeatures,
    DoNotFlipMirrorIndents,
    DifferentiateMultirowTableHeaders,
    UseWord2013TrackBottomHyphenation,
    AllowHyphenationAtTrackBottom,
    AllowTextAfterFloatingTableBreak,
}
crate::__string_enum! {
    CompatSettingNameValues { CompatibilityMode = "compatibilityMode",
    OverrideTableStyleFontSizeAndJustification =
    "overrideTableStyleFontSizeAndJustification", EnableOpenTypeFeatures =
    "enableOpenTypeFeatures", DoNotFlipMirrorIndents = "doNotFlipMirrorIndents",
    DifferentiateMultirowTableHeaders = "differentiateMultirowTableHeaders",
    UseWord2013TrackBottomHyphenation = "useWord2013TrackBottomHyphenation",
    AllowHyphenationAtTrackBottom = "allowHyphenationAtTrackBottom",
    AllowTextAfterFloatingTableBreak = "allowTextAfterFloatingTableBreak", }
}
/// Table Cell Insertion.
/// When the object is serialized out as xml, it's qualified name is w:cellIns.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cellIns")]
pub struct CellInsertion {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Table Cell Deletion.
/// When the object is serialized out as xml, it's qualified name is w:cellDel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cellDel")]
pub struct CellDeletion {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlInsRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlInsRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlInsRangeStart")]
pub struct CustomXmlInsRangeStart {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlDelRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlDelRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlDelRangeStart")]
pub struct CustomXmlDelRangeStart {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlMoveFromRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlMoveFromRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlMoveFromRangeStart")]
pub struct CustomXmlMoveFromRangeStart {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlMoveToRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlMoveToRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlMoveToRangeStart")]
pub struct CustomXmlMoveToRangeStart {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Inserted Paragraph.
/// When the object is serialized out as xml, it's qualified name is w:ins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ins")]
pub struct Inserted {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Deleted Paragraph.
/// When the object is serialized out as xml, it's qualified name is w:del.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:del")]
pub struct Deleted {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Move Source Paragraph.
/// When the object is serialized out as xml, it's qualified name is w:moveFrom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveFrom")]
pub struct MoveFrom {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Move Destination Paragraph.
/// When the object is serialized out as xml, it's qualified name is w:moveTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveTo")]
pub struct MoveTo {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the TrackChangeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TrackChangeType {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Vertically Merged/Split Table Cells.
/// When the object is serialized out as xml, it's qualified name is w:cellMerge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cellMerge")]
pub struct CellMerge {
    /// vMerge
    /// Represents the following attribute in the schema: w:vMerge
    #[xml(attr = "w:vMerge")]
    pub vertical_merge: Option<VerticalMergeRevisionValues>,
    /// vMergeOrig
    /// Represents the following attribute in the schema: w:vMergeOrig
    #[xml(attr = "w:vMergeOrig")]
    pub vertical_merge_original: Option<VerticalMergeRevisionValues>,
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the BookmarkStart Class.
/// When the object is serialized out as xml, it's qualified name is w:bookmarkStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bookmarkStart")]
pub struct BookmarkStart {
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// colFirst
    /// Represents the following attribute in the schema: w:colFirst
    #[xml(attr = "w:colFirst")]
    pub column_first: Option<i32>,
    /// colLast
    /// Represents the following attribute in the schema: w:colLast
    #[xml(attr = "w:colLast")]
    pub column_last: Option<i32>,
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the BookmarkEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:bookmarkEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bookmarkEnd")]
pub struct BookmarkEnd {
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CommentRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:commentRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:commentRangeStart")]
pub struct CommentRangeStart {
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CommentRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:commentRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:commentRangeEnd")]
pub struct CommentRangeEnd {
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MoveFromRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:moveFromRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveFromRangeEnd")]
pub struct MoveFromRangeEnd {
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MoveToRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:moveToRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveToRangeEnd")]
pub struct MoveToRangeEnd {
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MarkupRangeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MarkupRangeType {
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MoveFromRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:moveFromRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveFromRangeStart")]
pub struct MoveFromRangeStart {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: String,
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// colFirst
    /// Represents the following attribute in the schema: w:colFirst
    #[xml(attr = "w:colFirst")]
    pub column_first: Option<i32>,
    /// colLast
    /// Represents the following attribute in the schema: w:colLast
    #[xml(attr = "w:colLast")]
    pub column_last: Option<i32>,
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MoveToRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w:moveToRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveToRangeStart")]
pub struct MoveToRangeStart {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: String,
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// colFirst
    /// Represents the following attribute in the schema: w:colFirst
    #[xml(attr = "w:colFirst")]
    pub column_first: Option<i32>,
    /// colLast
    /// Represents the following attribute in the schema: w:colLast
    #[xml(attr = "w:colLast")]
    pub column_last: Option<i32>,
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MoveBookmarkType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MoveBookmarkType {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: String,
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// colFirst
    /// Represents the following attribute in the schema: w:colFirst
    #[xml(attr = "w:colFirst")]
    pub column_first: Option<i32>,
    /// colLast
    /// Represents the following attribute in the schema: w:colLast
    #[xml(attr = "w:colLast")]
    pub column_last: Option<i32>,
    /// displacedByCustomXml
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlInsRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlInsRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlInsRangeEnd")]
pub struct CustomXmlInsRangeEnd {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlDelRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlDelRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlDelRangeEnd")]
pub struct CustomXmlDelRangeEnd {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlMoveFromRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlMoveFromRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlMoveFromRangeEnd")]
pub struct CustomXmlMoveFromRangeEnd {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlMoveToRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:customXmlMoveToRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlMoveToRangeEnd")]
pub struct CustomXmlMoveToRangeEnd {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Comment Content Reference Mark.
/// When the object is serialized out as xml, it's qualified name is w:commentReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:commentReference")]
pub struct CommentReference {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the MarkupType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MarkupType {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the ParagraphStyleId Class.
/// When the object is serialized out as xml, it's qualified name is w:pStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pStyle")]
pub struct ParagraphStyleId {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Date Display Mask.
/// When the object is serialized out as xml, it's qualified name is w:dateFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dateFormat")]
pub struct DateFormat {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Document Part Gallery Filter.
/// When the object is serialized out as xml, it's qualified name is w:docPartGallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartGallery")]
pub struct DocPartGallery {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Document Part Category Filter.
/// When the object is serialized out as xml, it's qualified name is w:docPartCategory.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartCategory")]
pub struct DocPartCategory {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Document Part Reference.
/// When the object is serialized out as xml, it's qualified name is w:docPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPart")]
pub struct DocPartReference {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Custom XML Element Placeholder Text.
/// When the object is serialized out as xml, it's qualified name is w:placeholder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:placeholder")]
pub struct CustomXmlPlaceholder {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the TableCaption Class.
/// When the object is serialized out as xml, it's qualified name is w:tblCaption.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblCaption")]
pub struct TableCaption {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the TableDescription Class.
/// When the object is serialized out as xml, it's qualified name is w:tblDescription.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblDescription")]
pub struct TableDescription {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Data Source Name for Column.
/// When the object is serialized out as xml, it's qualified name is w:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:name")]
pub struct Name {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Predefined Merge Field Name.
/// When the object is serialized out as xml, it's qualified name is w:mappedName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mappedName")]
pub struct MappedName {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// UDL Connection String.
/// When the object is serialized out as xml, it's qualified name is w:udl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:udl")]
pub struct UdlConnectionString {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Data Source Table Name.
/// When the object is serialized out as xml, it's qualified name is w:table.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:table")]
pub struct DataSourceTableName {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Data Source Connection String.
/// When the object is serialized out as xml, it's qualified name is w:connectString.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:connectString")]
pub struct ConnectString {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Query For Data Source Records To Merge.
/// When the object is serialized out as xml, it's qualified name is w:query.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:query")]
pub struct Query {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Column Containing E-mail Address.
/// When the object is serialized out as xml, it's qualified name is w:addressFieldName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:addressFieldName")]
pub struct AddressFieldName {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Merged E-mail or Fax Subject Line.
/// When the object is serialized out as xml, it's qualified name is w:mailSubject.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mailSubject")]
pub struct MailSubject {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Frame Size.
/// When the object is serialized out as xml, it's qualified name is w:sz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sz")]
pub struct FrameSize {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Associated Paragraph Style Name.
/// When the object is serialized out as xml, it's qualified name is w:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:style")]
pub struct StyleId {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Description for Entry.
/// When the object is serialized out as xml, it's qualified name is w:description.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:description")]
pub struct Description {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the SdtAlias Class.
/// When the object is serialized out as xml, it's qualified name is w:alias.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:alias")]
pub struct SdtAlias {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the Tag Class.
/// When the object is serialized out as xml, it's qualified name is w:tag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tag")]
pub struct Tag {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Attached Custom XML Schema.
/// When the object is serialized out as xml, it's qualified name is w:attachedSchema.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:attachedSchema")]
pub struct AttachedSchema {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Radix Point for Field Code Evaluation.
/// When the object is serialized out as xml, it's qualified name is w:decimalSymbol.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:decimalSymbol")]
pub struct DecimalSymbol {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// List Separator for Field Code Evaluation.
/// When the object is serialized out as xml, it's qualified name is w:listSeparator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:listSeparator")]
pub struct ListSeparator {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the WebPageEncoding Class.
/// When the object is serialized out as xml, it's qualified name is w:encoding.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:encoding")]
pub struct WebPageEncoding {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the AltName Class.
/// When the object is serialized out as xml, it's qualified name is w:altName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:altName")]
pub struct AltName {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the StringType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StringType {
    /// String Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the KeepNext Class.
/// When the object is serialized out as xml, it's qualified name is w:keepNext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:keepNext")]
pub struct KeepNext {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the KeepLines Class.
/// When the object is serialized out as xml, it's qualified name is w:keepLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:keepLines")]
pub struct KeepLines {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the PageBreakBefore Class.
/// When the object is serialized out as xml, it's qualified name is w:pageBreakBefore.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pageBreakBefore")]
pub struct PageBreakBefore {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the WidowControl Class.
/// When the object is serialized out as xml, it's qualified name is w:widowControl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:widowControl")]
pub struct WidowControl {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the SuppressLineNumbers Class.
/// When the object is serialized out as xml, it's qualified name is w:suppressLineNumbers.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressLineNumbers")]
pub struct SuppressLineNumbers {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the SuppressAutoHyphens Class.
/// When the object is serialized out as xml, it's qualified name is w:suppressAutoHyphens.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressAutoHyphens")]
pub struct SuppressAutoHyphens {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Kinsoku Class.
/// When the object is serialized out as xml, it's qualified name is w:kinsoku.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:kinsoku")]
pub struct Kinsoku {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the WordWrap Class.
/// When the object is serialized out as xml, it's qualified name is w:wordWrap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:wordWrap")]
pub struct WordWrap {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the OverflowPunctuation Class.
/// When the object is serialized out as xml, it's qualified name is w:overflowPunct.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:overflowPunct")]
pub struct OverflowPunctuation {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the TopLinePunctuation Class.
/// When the object is serialized out as xml, it's qualified name is w:topLinePunct.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:topLinePunct")]
pub struct TopLinePunctuation {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the AutoSpaceDE Class.
/// When the object is serialized out as xml, it's qualified name is w:autoSpaceDE.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoSpaceDE")]
pub struct AutoSpaceDe {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the AutoSpaceDN Class.
/// When the object is serialized out as xml, it's qualified name is w:autoSpaceDN.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoSpaceDN")]
pub struct AutoSpaceDn {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the BiDi Class.
/// When the object is serialized out as xml, it's qualified name is w:bidi.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bidi")]
pub struct BiDi {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the AdjustRightIndent Class.
/// When the object is serialized out as xml, it's qualified name is w:adjustRightInd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:adjustRightInd")]
pub struct AdjustRightIndent {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the SnapToGrid Class.
/// When the object is serialized out as xml, it's qualified name is w:snapToGrid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:snapToGrid")]
pub struct SnapToGrid {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the ContextualSpacing Class.
/// When the object is serialized out as xml, it's qualified name is w:contextualSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:contextualSpacing")]
pub struct ContextualSpacing {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the MirrorIndents Class.
/// When the object is serialized out as xml, it's qualified name is w:mirrorIndents.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mirrorIndents")]
pub struct MirrorIndents {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the SuppressOverlap Class.
/// When the object is serialized out as xml, it's qualified name is w:suppressOverlap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressOverlap")]
pub struct SuppressOverlap {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Bold Class.
/// When the object is serialized out as xml, it's qualified name is w:b.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:b")]
pub struct Bold {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the BoldComplexScript Class.
/// When the object is serialized out as xml, it's qualified name is w:bCs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bCs")]
pub struct BoldComplexScript {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Italic Class.
/// When the object is serialized out as xml, it's qualified name is w:i.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:i")]
pub struct Italic {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the ItalicComplexScript Class.
/// When the object is serialized out as xml, it's qualified name is w:iCs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:iCs")]
pub struct ItalicComplexScript {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Caps Class.
/// When the object is serialized out as xml, it's qualified name is w:caps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:caps")]
pub struct Caps {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the SmallCaps Class.
/// When the object is serialized out as xml, it's qualified name is w:smallCaps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:smallCaps")]
pub struct SmallCaps {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Strike Class.
/// When the object is serialized out as xml, it's qualified name is w:strike.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:strike")]
pub struct Strike {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DoubleStrike Class.
/// When the object is serialized out as xml, it's qualified name is w:dstrike.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dstrike")]
pub struct DoubleStrike {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Outline Class.
/// When the object is serialized out as xml, it's qualified name is w:outline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:outline")]
pub struct Outline {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Shadow Class.
/// When the object is serialized out as xml, it's qualified name is w:shadow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:shadow")]
pub struct Shadow {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Emboss Class.
/// When the object is serialized out as xml, it's qualified name is w:emboss.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:emboss")]
pub struct Emboss {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Imprint Class.
/// When the object is serialized out as xml, it's qualified name is w:imprint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:imprint")]
pub struct Imprint {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the NoProof Class.
/// When the object is serialized out as xml, it's qualified name is w:noProof.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noProof")]
pub struct NoProof {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Vanish Class.
/// When the object is serialized out as xml, it's qualified name is w:vanish.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:vanish")]
pub struct Vanish {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the WebHidden Class.
/// When the object is serialized out as xml, it's qualified name is w:webHidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:webHidden")]
pub struct WebHidden {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the RightToLeftText Class.
/// When the object is serialized out as xml, it's qualified name is w:rtl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rtl")]
pub struct RightToLeftText {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the ComplexScript Class.
/// When the object is serialized out as xml, it's qualified name is w:cs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cs")]
pub struct ComplexScript {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the SpecVanish Class.
/// When the object is serialized out as xml, it's qualified name is w:specVanish.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:specVanish")]
pub struct SpecVanish {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the OfficeMath Class.
/// When the object is serialized out as xml, it's qualified name is w:oMath.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:oMath")]
pub struct OfficeMath {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the Hidden Class.
/// When the object is serialized out as xml, it's qualified name is w:hidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hidden")]
pub struct Hidden {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the FormProtection Class.
/// When the object is serialized out as xml, it's qualified name is w:formProt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:formProt")]
pub struct FormProtection {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the NoEndnote Class.
/// When the object is serialized out as xml, it's qualified name is w:noEndnote.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noEndnote")]
pub struct NoEndnote {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the TitlePage Class.
/// When the object is serialized out as xml, it's qualified name is w:titlePg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:titlePg")]
pub struct TitlePage {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the GutterOnRight Class.
/// When the object is serialized out as xml, it's qualified name is w:rtlGutter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rtlGutter")]
pub struct GutterOnRight {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Form Field Enabled.
/// When the object is serialized out as xml, it's qualified name is w:enabled.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:enabled")]
pub struct Enabled {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Recalculate Fields When Current Field Is Modified.
/// When the object is serialized out as xml, it's qualified name is w:calcOnExit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:calcOnExit")]
pub struct CalculateOnExit {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Automatically Size Form Field.
/// When the object is serialized out as xml, it's qualified name is w:sizeAuto.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sizeAuto")]
pub struct AutomaticallySizeFormField {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Default Checkbox Form Field State.
/// When the object is serialized out as xml, it's qualified name is w:default.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:default")]
pub struct DefaultCheckBoxFormFieldState {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Checkbox Form Field State.
/// When the object is serialized out as xml, it's qualified name is w:checked.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:checked")]
pub struct Checked {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Keep Source Formatting on Import.
/// When the object is serialized out as xml, it's qualified name is w:matchSrc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:matchSrc")]
pub struct MatchSource {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Invalidated Field Cache.
/// When the object is serialized out as xml, it's qualified name is w:dirty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dirty")]
pub struct Dirty {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Built-In Document Part.
/// When the object is serialized out as xml, it's qualified name is w:docPartUnique.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartUnique")]
pub struct DocPartUnique {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Record Is Included in Mail Merge.
/// When the object is serialized out as xml, it's qualified name is w:active.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:active")]
pub struct Active {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Country/Region-Based Address Field Ordering.
/// When the object is serialized out as xml, it's qualified name is w:dynamicAddress.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dynamicAddress")]
pub struct DynamicAddress {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// First Row of Data Source Contains Column Names.
/// When the object is serialized out as xml, it's qualified name is w:fHdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fHdr")]
pub struct FirstRowHeader {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Query Contains Link to External Query File.
/// When the object is serialized out as xml, it's qualified name is w:linkToQuery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:linkToQuery")]
pub struct LinkToQuery {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Remove Blank Lines from Merged Documents.
/// When the object is serialized out as xml, it's qualified name is w:doNotSuppressBlankLines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotSuppressBlankLines")]
pub struct DoNotSuppressBlankLines {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Merged Document To E-Mail Attachment.
/// When the object is serialized out as xml, it's qualified name is w:mailAsAttachment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mailAsAttachment")]
pub struct MailAsAttachment {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// View Merged Data Within Document.
/// When the object is serialized out as xml, it's qualified name is w:viewMergedData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:viewMergedData")]
pub struct ViewMergedData {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Display All Levels Using Arabic Numerals.
/// When the object is serialized out as xml, it's qualified name is w:isLgl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:isLgl")]
pub struct IsLegalNumberingStyle {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Data for HTML blockquote Element.
/// When the object is serialized out as xml, it's qualified name is w:blockQuote.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:blockQuote")]
pub struct BlockQuote {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Data for HTML body Element.
/// When the object is serialized out as xml, it's qualified name is w:bodyDiv.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bodyDiv")]
pub struct BodyDiv {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Simplified Rules For Table Border Conflicts.
/// When the object is serialized out as xml, it's qualified name is w:useSingleBorderforContiguousCells.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useSingleBorderforContiguousCells")]
pub struct UseSingleBorderForContiguousCells {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate WordPerfect 6.x Paragraph Justification.
/// When the object is serialized out as xml, it's qualified name is w:wpJustification.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:wpJustification")]
pub struct WordPerfectJustification {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Create Custom Tab Stop for Hanging Indent.
/// When the object is serialized out as xml, it's qualified name is w:noTabHangInd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noTabHangInd")]
pub struct NoTabHangIndent {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Add Leading Between Lines of Text.
/// When the object is serialized out as xml, it's qualified name is w:noLeading.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noLeading")]
pub struct NoLeading {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Add Additional Space Below Baseline For Underlined East Asian Text.
/// When the object is serialized out as xml, it's qualified name is w:spaceForUL.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:spaceForUL")]
pub struct SpaceForUnderline {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Balance Text Columns within a Section.
/// When the object is serialized out as xml, it's qualified name is w:noColumnBalance.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noColumnBalance")]
pub struct NoColumnBalance {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Balance Single Byte and Double Byte Characters.
/// When the object is serialized out as xml, it's qualified name is w:balanceSingleByteDoubleByteWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:balanceSingleByteDoubleByteWidth")]
pub struct BalanceSingleByteDoubleByteWidth {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Center Content on Lines With Exact Line Height.
/// When the object is serialized out as xml, it's qualified name is w:noExtraLineSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noExtraLineSpacing")]
pub struct NoExtraLineSpacing {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Convert Backslash To Yen Sign When Entered.
/// When the object is serialized out as xml, it's qualified name is w:doNotLeaveBackslashAlone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotLeaveBackslashAlone")]
pub struct DoNotLeaveBackslashAlone {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Underline All Trailing Spaces.
/// When the object is serialized out as xml, it's qualified name is w:ulTrailSpace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ulTrailSpace")]
pub struct UnderlineTrailingSpaces {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Don't Justify Lines Ending in Soft Line Break.
/// When the object is serialized out as xml, it's qualified name is w:doNotExpandShiftReturn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotExpandShiftReturn")]
pub struct DoNotExpandShiftReturn {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Only Expand/Condense Text By Whole Points.
/// When the object is serialized out as xml, it's qualified name is w:spacingInWholePoints.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:spacingInWholePoints")]
pub struct SpacingInWholePoints {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 6.0 Line Wrapping for East Asian Text.
/// When the object is serialized out as xml, it's qualified name is w:lineWrapLikeWord6.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lineWrapLikeWord6")]
pub struct LineWrapLikeWord6 {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Print Body Text before Header/Footer Contents.
/// When the object is serialized out as xml, it's qualified name is w:printBodyTextBeforeHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printBodyTextBeforeHeader")]
pub struct PrintBodyTextBeforeHeader {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Print Colors as Black And White without Dithering.
/// When the object is serialized out as xml, it's qualified name is w:printColBlack.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printColBlack")]
pub struct PrintColorBlackWhite {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Space width.
/// When the object is serialized out as xml, it's qualified name is w:wpSpaceWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:wpSpaceWidth")]
pub struct WordPerfectSpaceWidth {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Display Page/Column Breaks Present in Frames.
/// When the object is serialized out as xml, it's qualified name is w:showBreaksInFrames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:showBreaksInFrames")]
pub struct ShowBreaksInFrames {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Increase Priority Of Font Size During Font Substitution.
/// When the object is serialized out as xml, it's qualified name is w:subFontBySize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:subFontBySize")]
pub struct SubFontBySize {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Exact Line Height for Last Line on Page.
/// When the object is serialized out as xml, it's qualified name is w:suppressBottomSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressBottomSpacing")]
pub struct SuppressBottomSpacing {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Minimum and Exact Line Height for First Line on Page.
/// When the object is serialized out as xml, it's qualified name is w:suppressTopSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressTopSpacing")]
pub struct SuppressTopSpacing {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Minimum Line Height for First Line on Page.
/// When the object is serialized out as xml, it's qualified name is w:suppressSpacingAtTopOfPage.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressSpacingAtTopOfPage")]
pub struct SuppressSpacingAtTopOfPage {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate WordPerfect 5.x Line Spacing.
/// When the object is serialized out as xml, it's qualified name is w:suppressTopSpacingWP.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressTopSpacingWP")]
pub struct SuppressTopSpacingWordPerfect {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Use Space Before On First Line After a Page Break.
/// When the object is serialized out as xml, it's qualified name is w:suppressSpBfAfterPgBrk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suppressSpBfAfterPgBrk")]
pub struct SuppressSpacingBeforeAfterPageBreak {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Swap Paragraph Borders on Odd Numbered Pages.
/// When the object is serialized out as xml, it's qualified name is w:swapBordersFacingPages.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:swapBordersFacingPages")]
pub struct SwapBordersFacingPages {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Treat Backslash Quotation Delimiter as Two Quotation Marks.
/// When the object is serialized out as xml, it's qualified name is w:convMailMergeEsc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:convMailMergeEsc")]
pub struct ConvertMailMergeEscape {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate WordPerfect 6.x Font Height Calculation.
/// When the object is serialized out as xml, it's qualified name is w:truncateFontHeightsLikeWP6.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:truncateFontHeightsLikeWP6")]
pub struct TruncateFontHeightsLikeWordPerfect {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 5.x for the Macintosh Small Caps Formatting.
/// When the object is serialized out as xml, it's qualified name is w:mwSmallCaps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mwSmallCaps")]
pub struct MacWordSmallCaps {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Printer Metrics To Display Documents.
/// When the object is serialized out as xml, it's qualified name is w:usePrinterMetrics.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:usePrinterMetrics")]
pub struct UsePrinterMetrics {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Suppress Paragraph Borders Next To Frames.
/// When the object is serialized out as xml, it's qualified name is w:doNotSuppressParagraphBorders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotSuppressParagraphBorders")]
pub struct DoNotSuppressParagraphBorders {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Line Wrap Trailing Spaces.
/// When the object is serialized out as xml, it's qualified name is w:wrapTrailSpaces.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:wrapTrailSpaces")]
pub struct WrapTrailSpaces {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 6.x/95/97 Footnote Placement.
/// When the object is serialized out as xml, it's qualified name is w:footnoteLayoutLikeWW8.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnoteLayoutLikeWW8")]
pub struct FootnoteLayoutLikeWord8 {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 97 Text Wrapping Around Floating Objects.
/// When the object is serialized out as xml, it's qualified name is w:shapeLayoutLikeWW8.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:shapeLayoutLikeWW8")]
pub struct ShapeLayoutLikeWord8 {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Align Table Rows Independently.
/// When the object is serialized out as xml, it's qualified name is w:alignTablesRowByRow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:alignTablesRowByRow")]
pub struct AlignTablesRowByRow {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Width of Last Tab Stop When Aligning Paragraph If It Is Not Left Aligned.
/// When the object is serialized out as xml, it's qualified name is w:forgetLastTabAlignment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:forgetLastTabAlignment")]
pub struct ForgetLastTabAlignment {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Add Document Grid Line Pitch To Lines in Table Cells.
/// When the object is serialized out as xml, it's qualified name is w:adjustLineHeightInTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:adjustLineHeightInTable")]
pub struct AdjustLineHeightInTable {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 95 Full-Width Character Spacing.
/// When the object is serialized out as xml, it's qualified name is w:autoSpaceLikeWord95.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoSpaceLikeWord95")]
pub struct AutoSpaceLikeWord95 {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Increase Line Height for Raised/Lowered Text.
/// When the object is serialized out as xml, it's qualified name is w:noSpaceRaiseLower.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noSpaceRaiseLower")]
pub struct NoSpaceRaiseLower {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Fixed Paragraph Spacing for HTML Auto Setting.
/// When the object is serialized out as xml, it's qualified name is w:doNotUseHTMLParagraphAutoSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotUseHTMLParagraphAutoSpacing")]
pub struct DoNotUseHtmlParagraphAutoSpacing {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Space Before Table When Deciding If Table Should Wrap Floating Object.
/// When the object is serialized out as xml, it's qualified name is w:layoutRawTableWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:layoutRawTableWidth")]
pub struct LayoutRawTableWidth {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Allow Table Rows to Wrap Inline Objects Independently.
/// When the object is serialized out as xml, it's qualified name is w:layoutTableRowsApart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:layoutTableRowsApart")]
pub struct LayoutTableRowsApart {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 97 East Asian Line Breaking.
/// When the object is serialized out as xml, it's qualified name is w:useWord97LineBreakRules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useWord97LineBreakRules")]
pub struct UseWord97LineBreakRules {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Allow Floating Tables To Break Across Pages.
/// When the object is serialized out as xml, it's qualified name is w:doNotBreakWrappedTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotBreakWrappedTables")]
pub struct DoNotBreakWrappedTables {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Snap to Document Grid in Table Cells with Objects.
/// When the object is serialized out as xml, it's qualified name is w:doNotSnapToGridInCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotSnapToGridInCell")]
pub struct DoNotSnapToGridInCell {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Select Field When First or Last Character Is Selected.
/// When the object is serialized out as xml, it's qualified name is w:selectFldWithFirstOrLastChar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:selectFldWithFirstOrLastChar")]
pub struct SelectFieldWithFirstOrLastChar {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Legacy Ethiopic and Amharic Line Breaking Rules.
/// When the object is serialized out as xml, it's qualified name is w:applyBreakingRules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:applyBreakingRules")]
pub struct ApplyBreakingRules {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Allow Hanging Punctuation With Character Grid.
/// When the object is serialized out as xml, it's qualified name is w:doNotWrapTextWithPunct.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotWrapTextWithPunct")]
pub struct DoNotWrapTextWithPunctuation {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Compress Compressible Characters When Using Document Grid.
/// When the object is serialized out as xml, it's qualified name is w:doNotUseEastAsianBreakRules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotUseEastAsianBreakRules")]
pub struct DoNotUseEastAsianBreakRules {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Emulate Word 2002 Table Style Rules.
/// When the object is serialized out as xml, it's qualified name is w:useWord2002TableStyleRules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useWord2002TableStyleRules")]
pub struct UseWord2002TableStyleRules {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Allow Tables to AutoFit Into Page Margins.
/// When the object is serialized out as xml, it's qualified name is w:growAutofit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:growAutofit")]
pub struct GrowAutofit {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Bypass East Asian/Complex Script Layout Code.
/// When the object is serialized out as xml, it's qualified name is w:useFELayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useFELayout")]
pub struct UseFarEastLayout {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Automatically Apply List Paragraph Style To Bulleted/Numbered Text.
/// When the object is serialized out as xml, it's qualified name is w:useNormalStyleForList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useNormalStyleForList")]
pub struct UseNormalStyleForList {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Hanging Indent When Creating Tab Stop After Numbering.
/// When the object is serialized out as xml, it's qualified name is w:doNotUseIndentAsNumberingTabStop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotUseIndentAsNumberingTabStop")]
pub struct DoNotUseIndentAsNumberingTabStop {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Alternate Set of East Asian Line Breaking Rules.
/// When the object is serialized out as xml, it's qualified name is w:useAltKinsokuLineBreakRules.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useAltKinsokuLineBreakRules")]
pub struct UseAltKinsokuLineBreakRules {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Allow Contextual Spacing of Paragraphs in Tables.
/// When the object is serialized out as xml, it's qualified name is w:allowSpaceOfSameStyleInTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:allowSpaceOfSameStyleInTable")]
pub struct AllowSpaceOfSameStyleInTable {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Ignore Floating Objects When Calculating Paragraph Indentation.
/// When the object is serialized out as xml, it's qualified name is w:doNotSuppressIndentation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotSuppressIndentation")]
pub struct DoNotSuppressIndentation {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not AutoFit Tables To Fit Next To Wrapped Objects.
/// When the object is serialized out as xml, it's qualified name is w:doNotAutofitConstrainedTables.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotAutofitConstrainedTables")]
pub struct DoNotAutofitConstrainedTables {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Allow Table Columns To Exceed Preferred Widths of Constituent Cells.
/// When the object is serialized out as xml, it's qualified name is w:autofitToFirstFixedWidthCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autofitToFirstFixedWidthCell")]
pub struct AutofitToFirstFixedWidthCell {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Underline Following Character Following Numbering.
/// When the object is serialized out as xml, it's qualified name is w:underlineTabInNumList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:underlineTabInNumList")]
pub struct UnderlineTabInNumberingList {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Always Use Fixed Width for Hangul Characters.
/// When the object is serialized out as xml, it's qualified name is w:displayHangulFixedWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:displayHangulFixedWidth")]
pub struct DisplayHangulFixedWidth {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Always Move Paragraph Mark to Page after a Page Break.
/// When the object is serialized out as xml, it's qualified name is w:splitPgBreakAndParaMark.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:splitPgBreakAndParaMark")]
pub struct SplitPageBreakAndParagraphMark {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Don't Vertically Align Cells Containing Floating Objects.
/// When the object is serialized out as xml, it's qualified name is w:doNotVertAlignCellWithSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotVertAlignCellWithSp")]
pub struct DoNotVerticallyAlignCellWithShape {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Don't Break Table Rows Around Floating Tables.
/// When the object is serialized out as xml, it's qualified name is w:doNotBreakConstrainedForcedTable.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotBreakConstrainedForcedTable")]
pub struct DoNotBreakConstrainedForcedTable {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Vertical Alignment in Textboxes.
/// When the object is serialized out as xml, it's qualified name is w:doNotVertAlignInTxbx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotVertAlignInTxbx")]
pub struct DoNotVerticallyAlignInTextBox {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use ANSI Kerning Pairs from Fonts.
/// When the object is serialized out as xml, it's qualified name is w:useAnsiKerningPairs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useAnsiKerningPairs")]
pub struct UseAnsiKerningPairs {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Cached Paragraph Information for Column Balancing.
/// When the object is serialized out as xml, it's qualified name is w:cachedColBalance.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cachedColBalance")]
pub struct CachedColumnBalance {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the ShowingPlaceholder Class.
/// When the object is serialized out as xml, it's qualified name is w:showingPlcHdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:showingPlcHdr")]
pub struct ShowingPlaceholder {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the TemporarySdt Class.
/// When the object is serialized out as xml, it's qualified name is w:temporary.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:temporary")]
pub struct TemporarySdt {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Remove Personal Information from Document Properties.
/// When the object is serialized out as xml, it's qualified name is w:removePersonalInformation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:removePersonalInformation")]
pub struct RemovePersonalInformation {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Remove Date and Time from Annotations.
/// When the object is serialized out as xml, it's qualified name is w:removeDateAndTime.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:removeDateAndTime")]
pub struct RemoveDateAndTime {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Display Visual Boundary For Header/Footer or Between Pages.
/// When the object is serialized out as xml, it's qualified name is w:doNotDisplayPageBoundaries.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotDisplayPageBoundaries")]
pub struct DoNotDisplayPageBoundaries {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Display Background Objects When Displaying Document.
/// When the object is serialized out as xml, it's qualified name is w:displayBackgroundShape.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:displayBackgroundShape")]
pub struct DisplayBackgroundShape {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Print PostScript Codes With Document Text.
/// When the object is serialized out as xml, it's qualified name is w:printPostScriptOverText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printPostScriptOverText")]
pub struct PrintPostScriptOverText {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Print Fractional Character Widths.
/// When the object is serialized out as xml, it's qualified name is w:printFractionalCharacterWidth.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printFractionalCharacterWidth")]
pub struct PrintFractionalCharacterWidth {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Only Print Form Field Content.
/// When the object is serialized out as xml, it's qualified name is w:printFormsData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printFormsData")]
pub struct PrintFormsData {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Embed TrueType Fonts.
/// When the object is serialized out as xml, it's qualified name is w:embedTrueTypeFonts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:embedTrueTypeFonts")]
pub struct EmbedTrueTypeFonts {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Embed Common System Fonts.
/// When the object is serialized out as xml, it's qualified name is w:embedSystemFonts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:embedSystemFonts")]
pub struct EmbedSystemFonts {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Subset Fonts When Embedding.
/// When the object is serialized out as xml, it's qualified name is w:saveSubsetFonts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:saveSubsetFonts")]
pub struct SaveSubsetFonts {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Only Save Form Field Content.
/// When the object is serialized out as xml, it's qualified name is w:saveFormsData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:saveFormsData")]
pub struct SaveFormsData {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Mirror Page Margins.
/// When the object is serialized out as xml, it's qualified name is w:mirrorMargins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mirrorMargins")]
pub struct MirrorMargins {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Align Paragraph and Table Borders with Page Border.
/// When the object is serialized out as xml, it's qualified name is w:alignBordersAndEdges.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:alignBordersAndEdges")]
pub struct AlignBorderAndEdges {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Page Border Excludes Header.
/// When the object is serialized out as xml, it's qualified name is w:bordersDoNotSurroundHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bordersDoNotSurroundHeader")]
pub struct BordersDoNotSurroundHeader {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Page Border Excludes Footer.
/// When the object is serialized out as xml, it's qualified name is w:bordersDoNotSurroundFooter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bordersDoNotSurroundFooter")]
pub struct BordersDoNotSurroundFooter {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Position Gutter At Top of Page.
/// When the object is serialized out as xml, it's qualified name is w:gutterAtTop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:gutterAtTop")]
pub struct GutterAtTop {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Display Visual Indication of Spelling Errors.
/// When the object is serialized out as xml, it's qualified name is w:hideSpellingErrors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hideSpellingErrors")]
pub struct HideSpellingErrors {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Display Visual Indication of Grammatical Errors.
/// When the object is serialized out as xml, it's qualified name is w:hideGrammaticalErrors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hideGrammaticalErrors")]
pub struct HideGrammaticalErrors {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Structured Document Tag Placeholder Text Should be Resaved.
/// When the object is serialized out as xml, it's qualified name is w:formsDesign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:formsDesign")]
pub struct FormsDesign {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Automatically Update Styles From Document Template.
/// When the object is serialized out as xml, it's qualified name is w:linkStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:linkStyles")]
pub struct LinkStyles {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Track Revisions to Document.
/// When the object is serialized out as xml, it's qualified name is w:trackRevisions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:trackRevisions")]
pub struct TrackRevisions {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Use Move Syntax When Tracking Revisions.
/// When the object is serialized out as xml, it's qualified name is w:doNotTrackMoves.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotTrackMoves")]
pub struct DoNotTrackMoves {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Track Formatting Revisions When Tracking Revisions.
/// When the object is serialized out as xml, it's qualified name is w:doNotTrackFormatting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotTrackFormatting")]
pub struct DoNotTrackFormatting {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Allow Automatic Formatting to Override Formatting Protection Settings.
/// When the object is serialized out as xml, it's qualified name is w:autoFormatOverride.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoFormatOverride")]
pub struct AutoFormatOverride {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Prevent Modification of Themes Part.
/// When the object is serialized out as xml, it's qualified name is w:styleLockTheme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:styleLockTheme")]
pub struct StyleLockThemesPart {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Prevent Replacement of Styles Part.
/// When the object is serialized out as xml, it's qualified name is w:styleLockQFSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:styleLockQFSet")]
pub struct StyleLockStylesPart {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Automatically Hyphenate Document Contents When Displayed.
/// When the object is serialized out as xml, it's qualified name is w:autoHyphenation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoHyphenation")]
pub struct AutoHyphenation {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Hyphenate Words in ALL CAPITAL LETTERS.
/// When the object is serialized out as xml, it's qualified name is w:doNotHyphenateCaps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotHyphenateCaps")]
pub struct DoNotHyphenateCaps {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Show E-Mail Message Header.
/// When the object is serialized out as xml, it's qualified name is w:showEnvelope.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:showEnvelope")]
pub struct ShowEnvelope {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Different Even/Odd Page Headers and Footers.
/// When the object is serialized out as xml, it's qualified name is w:evenAndOddHeaders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:evenAndOddHeaders")]
pub struct EvenAndOddHeaders {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Reverse Book Fold Printing.
/// When the object is serialized out as xml, it's qualified name is w:bookFoldRevPrinting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bookFoldRevPrinting")]
pub struct BookFoldReversePrinting {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Book Fold Printing.
/// When the object is serialized out as xml, it's qualified name is w:bookFoldPrinting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bookFoldPrinting")]
pub struct BookFoldPrinting {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Use Margins for Drawing Grid Origin.
/// When the object is serialized out as xml, it's qualified name is w:doNotUseMarginsForDrawingGridOrigin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotUseMarginsForDrawingGridOrigin")]
pub struct DoNotUseMarginsForDrawingGridOrigin {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Show Visual Indicator For Form Fields.
/// When the object is serialized out as xml, it's qualified name is w:doNotShadeFormData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotShadeFormData")]
pub struct DoNotShadeFormData {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Never Kern Punctuation Characters.
/// When the object is serialized out as xml, it's qualified name is w:noPunctuationKerning.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noPunctuationKerning")]
pub struct NoPunctuationKerning {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Print Two Pages Per Sheet.
/// When the object is serialized out as xml, it's qualified name is w:printTwoOnOne.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printTwoOnOne")]
pub struct PrintTwoOnOne {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Strict Kinsoku Rules for Japanese Text.
/// When the object is serialized out as xml, it's qualified name is w:strictFirstAndLastChars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:strictFirstAndLastChars")]
pub struct StrictFirstAndLastChars {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Generate Thumbnail For Document On Save.
/// When the object is serialized out as xml, it's qualified name is w:savePreviewPicture.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:savePreviewPicture")]
pub struct SavePreviewPicture {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Validate Custom XML Markup Against Schemas.
/// When the object is serialized out as xml, it's qualified name is w:doNotValidateAgainstSchema.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotValidateAgainstSchema")]
pub struct DoNotValidateAgainstSchema {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Allow Saving Document As XML File When Custom XML Markup Is Invalid.
/// When the object is serialized out as xml, it's qualified name is w:saveInvalidXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:saveInvalidXml")]
pub struct SaveInvalidXml {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Ignore Mixed Content When Validating Custom XML Markup.
/// When the object is serialized out as xml, it's qualified name is w:ignoreMixedContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ignoreMixedContent")]
pub struct IgnoreMixedContent {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Use Custom XML Element Names as Default Placeholder Text.
/// When the object is serialized out as xml, it's qualified name is w:alwaysShowPlaceholderText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:alwaysShowPlaceholderText")]
pub struct AlwaysShowPlaceholderText {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Show Visual Indicator For Invalid Custom XML Markup.
/// When the object is serialized out as xml, it's qualified name is w:doNotDemarcateInvalidXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotDemarcateInvalidXml")]
pub struct DoNotDemarcateInvalidXml {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Only Save Custom XML Markup.
/// When the object is serialized out as xml, it's qualified name is w:saveXmlDataOnly.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:saveXmlDataOnly")]
pub struct SaveXmlDataOnly {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Save Document as XML File through Custom XSL Transform.
/// When the object is serialized out as xml, it's qualified name is w:useXSLTWhenSaving.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:useXSLTWhenSaving")]
pub struct UseXsltWhenSaving {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Show Visual Indicators for Custom XML Markup Start/End Locations.
/// When the object is serialized out as xml, it's qualified name is w:showXMLTags.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:showXMLTags")]
pub struct ShowXmlTags {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Mark Custom XML Elements With No Namespace As Invalid.
/// When the object is serialized out as xml, it's qualified name is w:alwaysMergeEmptyNamespace.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:alwaysMergeEmptyNamespace")]
pub struct AlwaysMergeEmptyNamespace {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Automatically Recalculate Fields on Open.
/// When the object is serialized out as xml, it's qualified name is w:updateFields.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:updateFields")]
pub struct UpdateFieldsOnOpen {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Disable Features Incompatible With Earlier Word Processing Formats.
/// When the object is serialized out as xml, it's qualified name is w:uiCompat97To2003.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:uiCompat97To2003")]
pub struct UiCompatibleWith97To2003 {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Include Content in Text Boxes, Footnotes, and Endnotes in Document Statistics.
/// When the object is serialized out as xml, it's qualified name is w:doNotIncludeSubdocsInStats.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotIncludeSubdocsInStats")]
pub struct DoNotIncludeSubdocsInStats {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Do Not Automatically Compress Images.
/// When the object is serialized out as xml, it's qualified name is w:doNotAutoCompressPictures.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotAutoCompressPictures")]
pub struct DoNotAutoCompressPictures {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the OptimizeForBrowser Class.
/// When the object is serialized out as xml, it's qualified name is w:optimizeForBrowser.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:optimizeForBrowser")]
pub struct OptimizeForBrowser {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the RelyOnVML Class.
/// When the object is serialized out as xml, it's qualified name is w:relyOnVML.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:relyOnVML")]
pub struct RelyOnVml {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the AllowPNG Class.
/// When the object is serialized out as xml, it's qualified name is w:allowPNG.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:allowPNG")]
pub struct AllowPng {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DoNotRelyOnCSS Class.
/// When the object is serialized out as xml, it's qualified name is w:doNotRelyOnCSS.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotRelyOnCSS")]
pub struct DoNotRelyOnCss {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DoNotSaveAsSingleFile Class.
/// When the object is serialized out as xml, it's qualified name is w:doNotSaveAsSingleFile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotSaveAsSingleFile")]
pub struct DoNotSaveAsSingleFile {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DoNotOrganizeInFolder Class.
/// When the object is serialized out as xml, it's qualified name is w:doNotOrganizeInFolder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotOrganizeInFolder")]
pub struct DoNotOrganizeInFolder {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the DoNotUseLongFileNames Class.
/// When the object is serialized out as xml, it's qualified name is w:doNotUseLongFileNames.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:doNotUseLongFileNames")]
pub struct DoNotUseLongFileNames {
    /// On/Off Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<bool>,
}
/// Defines the NotTrueType Class.
/// When the object is serialized out as xml, it's qualified name is w:notTrueType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:notTrueType")]
pub struct NotTrueType {
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
/// Defines the FrameProperties Class.
/// When the object is serialized out as xml, it's qualified name is w:framePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:framePr")]
pub struct FrameProperties {
    /// Drop Cap Frame
    /// Represents the following attribute in the schema: w:dropCap
    #[xml(attr = "w:dropCap")]
    pub drop_cap: Option<DropCapLocationValues>,
    /// Drop Cap Vertical Height in Lines
    /// Represents the following attribute in the schema: w:lines
    #[xml(attr = "w:lines")]
    pub lines: Option<i32>,
    /// Frame Width
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Frame Height
    /// Represents the following attribute in the schema: w:h
    #[xml(attr = "w:h")]
    pub height: Option<i32>,
    /// Vertical Frame Padding
    /// Represents the following attribute in the schema: w:vSpace
    #[xml(attr = "w:vSpace")]
    pub vertical_space: Option<String>,
    /// Horizontal Frame Padding
    /// Represents the following attribute in the schema: w:hSpace
    #[xml(attr = "w:hSpace")]
    pub horizontal_space: Option<String>,
    /// Text Wrapping Around Frame
    /// Represents the following attribute in the schema: w:wrap
    #[xml(attr = "w:wrap")]
    pub wrap: Option<TextWrappingValues>,
    /// Frame Horizontal Positioning Base
    /// Represents the following attribute in the schema: w:hAnchor
    #[xml(attr = "w:hAnchor")]
    pub horizontal_position: Option<HorizontalAnchorValues>,
    /// Frame Vertical Positioning Base
    /// Represents the following attribute in the schema: w:vAnchor
    #[xml(attr = "w:vAnchor")]
    pub vertical_position: Option<VerticalAnchorValues>,
    /// Absolute Horizontal Position
    /// Represents the following attribute in the schema: w:x
    #[xml(attr = "w:x")]
    pub x: Option<String>,
    /// Relative Horizontal Position
    /// Represents the following attribute in the schema: w:xAlign
    #[xml(attr = "w:xAlign")]
    pub x_align: Option<HorizontalAlignmentValues>,
    /// Absolute Vertical Position
    /// Represents the following attribute in the schema: w:y
    #[xml(attr = "w:y")]
    pub y: Option<String>,
    /// Relative Vertical Position
    /// Represents the following attribute in the schema: w:yAlign
    #[xml(attr = "w:yAlign")]
    pub y_align: Option<VerticalAlignmentValues>,
    /// Frame Height Type
    /// Represents the following attribute in the schema: w:hRule
    #[xml(attr = "w:hRule")]
    pub height_type: Option<HeightRuleValues>,
    /// Lock Frame Anchor to Paragraph
    /// Represents the following attribute in the schema: w:anchorLock
    #[xml(attr = "w:anchorLock")]
    pub anchor_lock: Option<bool>,
}
/// Defines the NumberingProperties Class.
/// When the object is serialized out as xml, it's qualified name is w:numPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numPr")]
pub struct NumberingProperties {
    ///Numbering Level Reference
    #[xml(child = "w:ilvl")]
    pub numbering_level_reference: Option<NumberingLevelReference>,
    ///Numbering Definition Instance Reference
    #[xml(child = "w:numId")]
    pub numbering_id: Option<NumberingId>,
    ///Previous Paragraph Numbering Properties
    #[xml(child = "w:numberingChange")]
    pub numbering_change: Option<NumberingChange>,
    ///Inserted Numbering Properties
    #[xml(child = "w:ins")]
    pub inserted: Option<Inserted>,
}
/// Defines the ParagraphBorders Class.
/// When the object is serialized out as xml, it's qualified name is w:pBdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pBdr")]
pub struct ParagraphBorders {
    ///Paragraph Border Above Identical Paragraphs
    #[xml(child = "w:top")]
    pub top_border: Option<TopBorder>,
    ///Left Paragraph Border
    #[xml(child = "w:left")]
    pub left_border: Option<LeftBorder>,
    ///Paragraph Border Between Identical Paragraphs
    #[xml(child = "w:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Right Paragraph Border
    #[xml(child = "w:right")]
    pub right_border: Option<RightBorder>,
    ///Paragraph Border Between Identical Paragraphs
    #[xml(child = "w:between")]
    pub between_border: Option<BetweenBorder>,
    ///Paragraph Border Between Facing Pages
    #[xml(child = "w:bar")]
    pub bar_border: Option<BarBorder>,
}
/// Defines the Shading Class.
/// When the object is serialized out as xml, it's qualified name is w:shd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:shd")]
pub struct Shading {
    /// Shading Pattern
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: ShadingPatternValues,
    /// Shading Pattern Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Shading Pattern Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Shading Pattern Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Shading Pattern Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Shading Background Color
    /// Represents the following attribute in the schema: w:fill
    #[xml(attr = "w:fill")]
    pub fill: Option<String>,
    /// Shading Background Theme Color
    /// Represents the following attribute in the schema: w:themeFill
    #[xml(attr = "w:themeFill")]
    pub theme_fill: Option<ThemeColorValues>,
    /// Shading Background Theme Color Tint
    /// Represents the following attribute in the schema: w:themeFillTint
    #[xml(attr = "w:themeFillTint")]
    pub theme_fill_tint: Option<String>,
    /// Shading Background Theme Color Shade
    /// Represents the following attribute in the schema: w:themeFillShade
    #[xml(attr = "w:themeFillShade")]
    pub theme_fill_shade: Option<String>,
}
/// Defines the Tabs Class.
/// When the object is serialized out as xml, it's qualified name is w:tabs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tabs")]
pub struct Tabs {
    /// _
    #[xml(child = "w:tab")]
    pub w_tab: Vec<TabStop>,
}
/// Defines the SpacingBetweenLines Class.
/// When the object is serialized out as xml, it's qualified name is w:spacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:spacing")]
pub struct SpacingBetweenLines {
    /// Spacing Above Paragraph
    /// Represents the following attribute in the schema: w:before
    #[xml(attr = "w:before")]
    pub before: Option<String>,
    /// Spacing Above Paragraph IN Line Units
    /// Represents the following attribute in the schema: w:beforeLines
    #[xml(attr = "w:beforeLines")]
    pub before_lines: Option<i32>,
    /// Automatically Determine Spacing Above Paragraph
    /// Represents the following attribute in the schema: w:beforeAutospacing
    #[xml(attr = "w:beforeAutospacing")]
    pub before_auto_spacing: Option<bool>,
    /// Spacing Below Paragraph
    /// Represents the following attribute in the schema: w:after
    #[xml(attr = "w:after")]
    pub after: Option<String>,
    /// Spacing Below Paragraph in Line Units
    /// Represents the following attribute in the schema: w:afterLines
    #[xml(attr = "w:afterLines")]
    pub after_lines: Option<i32>,
    /// Automatically Determine Spacing Below Paragraph
    /// Represents the following attribute in the schema: w:afterAutospacing
    #[xml(attr = "w:afterAutospacing")]
    pub after_auto_spacing: Option<bool>,
    /// Spacing Between Lines in Paragraph
    /// Represents the following attribute in the schema: w:line
    #[xml(attr = "w:line")]
    pub line: Option<String>,
    /// Type of Spacing Between Lines
    /// Represents the following attribute in the schema: w:lineRule
    #[xml(attr = "w:lineRule")]
    pub line_rule: Option<LineSpacingRuleValues>,
}
/// Defines the Indentation Class.
/// When the object is serialized out as xml, it's qualified name is w:ind.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ind")]
pub struct Indentation {
    /// Left Indentation
    /// Represents the following attribute in the schema: w:left
    #[xml(attr = "w:left")]
    pub left: Option<String>,
    /// start
    /// Represents the following attribute in the schema: w:start
    #[xml(attr = "w:start")]
    pub start: Option<String>,
    /// Left Indentation in Character Units
    /// Represents the following attribute in the schema: w:leftChars
    #[xml(attr = "w:leftChars")]
    pub left_chars: Option<i32>,
    /// startChars
    /// Represents the following attribute in the schema: w:startChars
    #[xml(attr = "w:startChars")]
    pub start_characters: Option<i32>,
    /// Right Indentation
    /// Represents the following attribute in the schema: w:right
    #[xml(attr = "w:right")]
    pub right: Option<String>,
    /// end
    /// Represents the following attribute in the schema: w:end
    #[xml(attr = "w:end")]
    pub end: Option<String>,
    /// Right Indentation in Character Units
    /// Represents the following attribute in the schema: w:rightChars
    #[xml(attr = "w:rightChars")]
    pub right_chars: Option<i32>,
    /// endChars
    /// Represents the following attribute in the schema: w:endChars
    #[xml(attr = "w:endChars")]
    pub end_characters: Option<i32>,
    /// Indentation Removed from First Line
    /// Represents the following attribute in the schema: w:hanging
    #[xml(attr = "w:hanging")]
    pub hanging: Option<String>,
    /// Indentation Removed From First Line in Character Units
    /// Represents the following attribute in the schema: w:hangingChars
    #[xml(attr = "w:hangingChars")]
    pub hanging_chars: Option<i32>,
    /// Additional First Line Indentation
    /// Represents the following attribute in the schema: w:firstLine
    #[xml(attr = "w:firstLine")]
    pub first_line: Option<String>,
    /// Additional First Line Indentation in Character Units
    /// Represents the following attribute in the schema: w:firstLineChars
    #[xml(attr = "w:firstLineChars")]
    pub first_line_chars: Option<i32>,
}
/// Defines the Justification Class.
/// When the object is serialized out as xml, it's qualified name is w:jc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:jc")]
pub struct Justification {
    /// Alignment Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: JustificationValues,
}
/// Defines the TextDirection Class.
/// When the object is serialized out as xml, it's qualified name is w:textDirection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:textDirection")]
pub struct TextDirection {
    /// Direction of Text Flow
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TextDirectionValues,
}
/// Defines the TextAlignment Class.
/// When the object is serialized out as xml, it's qualified name is w:textAlignment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:textAlignment")]
pub struct TextAlignment {
    /// Vertical Character Alignment Position
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: VerticalTextAlignmentValues,
}
/// Defines the TextBoxTightWrap Class.
/// When the object is serialized out as xml, it's qualified name is w:textboxTightWrap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:textboxTightWrap")]
pub struct TextBoxTightWrap {
    /// Lines to Tight Wrap to Paragraph Extents
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TextBoxTightWrapValues,
}
/// Defines the OutlineLevel Class.
/// When the object is serialized out as xml, it's qualified name is w:outlineLvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:outlineLvl")]
pub struct OutlineLevel {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the GridSpan Class.
/// When the object is serialized out as xml, it's qualified name is w:gridSpan.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:gridSpan")]
pub struct GridSpan {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the GridBefore Class.
/// When the object is serialized out as xml, it's qualified name is w:gridBefore.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:gridBefore")]
pub struct GridBefore {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the GridAfter Class.
/// When the object is serialized out as xml, it's qualified name is w:gridAfter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:gridAfter")]
pub struct GridAfter {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Drop-Down List Selection.
/// When the object is serialized out as xml, it's qualified name is w:result.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:result")]
pub struct DropDownListSelection {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Record Currently Displayed In Merged Document.
/// When the object is serialized out as xml, it's qualified name is w:activeRecord.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:activeRecord")]
pub struct ActiveRecord {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Mail Merge Error Reporting Setting.
/// When the object is serialized out as xml, it's qualified name is w:checkErrors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:checkErrors")]
pub struct CheckErrors {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Restart Numbering Level Symbol.
/// When the object is serialized out as xml, it's qualified name is w:lvlRestart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lvlRestart")]
pub struct LevelRestart {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Picture Numbering Symbol Definition Reference.
/// When the object is serialized out as xml, it's qualified name is w:lvlPicBulletId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lvlPicBulletId")]
pub struct LevelPictureBulletId {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Numbering Level Starting Value Override.
/// When the object is serialized out as xml, it's qualified name is w:startOverride.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:startOverride")]
pub struct StartOverrideNumberingValue {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Last Reviewed Abstract Numbering Definition.
/// When the object is serialized out as xml, it's qualified name is w:numIdMacAtCleanup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numIdMacAtCleanup")]
pub struct NumberingIdMacAtCleanup {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the SdtId Class.
/// When the object is serialized out as xml, it's qualified name is w:id.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:id")]
pub struct SdtId {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the PixelsPerInch Class.
/// When the object is serialized out as xml, it's qualified name is w:pixelsPerInch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pixelsPerInch")]
pub struct PixelsPerInch {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the DecimalNumberType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct DecimalNumberType {
    /// Decimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the ParagraphPropertiesChange Class.
/// When the object is serialized out as xml, it's qualified name is w:pPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPrChange")]
pub struct ParagraphPropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:pPr")]
    pub children: Vec<ParagraphPropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphPropertiesChangeChildChoice {
    #[xml(tag = "w:pPr")]
    WPPr(ParagraphPropertiesExtended),
}
/// Header Reference.
/// When the object is serialized out as xml, it's qualified name is w:headerReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:headerReference")]
pub struct HeaderReference {
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: HeaderFooterValues,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Footer Reference.
/// When the object is serialized out as xml, it's qualified name is w:footerReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footerReference")]
pub struct FooterReference {
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: HeaderFooterValues,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the HeaderFooterReferenceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct HeaderFooterReferenceType {
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: HeaderFooterValues,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Break.
/// When the object is serialized out as xml, it's qualified name is w:br.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:br")]
pub struct Break {
    /// Break Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<BreakValues>,
    /// Restart Location For Text Wrapping Break
    /// Represents the following attribute in the schema: w:clear
    #[xml(attr = "w:clear")]
    pub clear: Option<BreakTextRestartLocationValues>,
}
/// Text.
/// When the object is serialized out as xml, it's qualified name is w:t.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:t")]
pub struct Text {
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Deleted Text.
/// When the object is serialized out as xml, it's qualified name is w:delText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:delText")]
pub struct DeletedText {
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Field Code.
/// When the object is serialized out as xml, it's qualified name is w:instrText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:instrText")]
pub struct FieldCode {
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Deleted Field Code.
/// When the object is serialized out as xml, it's qualified name is w:delInstrText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:delInstrText")]
pub struct DeletedFieldCode {
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Defines the TextType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextType {
    /// space
    /// Represents the following attribute in the schema: xml:space
    #[xml(attr = "xml:space")]
    pub space: Option<
        crate::schemas::www_w3_org_xml_1998_namespace::SpaceProcessingModeValues,
    >,
    #[xml(text)]
    pub child: String,
}
/// Non Breaking Hyphen Character.
/// When the object is serialized out as xml, it's qualified name is w:noBreakHyphen.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noBreakHyphen")]
pub struct NoBreakHyphen {}
/// Optional Hyphen Character.
/// When the object is serialized out as xml, it's qualified name is w:softHyphen.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:softHyphen")]
pub struct SoftHyphen {}
/// Date Block - Short Day Format.
/// When the object is serialized out as xml, it's qualified name is w:dayShort.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dayShort")]
pub struct DayShort {}
/// Date Block - Short Month Format.
/// When the object is serialized out as xml, it's qualified name is w:monthShort.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:monthShort")]
pub struct MonthShort {}
/// Date Block - Short Year Format.
/// When the object is serialized out as xml, it's qualified name is w:yearShort.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:yearShort")]
pub struct YearShort {}
/// Date Block - Long Day Format.
/// When the object is serialized out as xml, it's qualified name is w:dayLong.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dayLong")]
pub struct DayLong {}
/// Date Block - Long Month Format.
/// When the object is serialized out as xml, it's qualified name is w:monthLong.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:monthLong")]
pub struct MonthLong {}
/// Date Block - Long Year Format.
/// When the object is serialized out as xml, it's qualified name is w:yearLong.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:yearLong")]
pub struct YearLong {}
/// Comment Information Block.
/// When the object is serialized out as xml, it's qualified name is w:annotationRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:annotationRef")]
pub struct AnnotationReferenceMark {}
/// Footnote Reference Mark.
/// When the object is serialized out as xml, it's qualified name is w:footnoteRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnoteRef")]
pub struct FootnoteReferenceMark {}
/// Endnote Reference Mark.
/// When the object is serialized out as xml, it's qualified name is w:endnoteRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnoteRef")]
pub struct EndnoteReferenceMark {}
/// Footnote/Endnote Separator Mark.
/// When the object is serialized out as xml, it's qualified name is w:separator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:separator")]
pub struct SeparatorMark {}
/// Continuation Separator Mark.
/// When the object is serialized out as xml, it's qualified name is w:continuationSeparator.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:continuationSeparator")]
pub struct ContinuationSeparatorMark {}
/// Page Number Block.
/// When the object is serialized out as xml, it's qualified name is w:pgNum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pgNum")]
pub struct PageNumber {}
/// Carriage Return.
/// When the object is serialized out as xml, it's qualified name is w:cr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cr")]
pub struct CarriageReturn {}
/// Tab Character.
/// When the object is serialized out as xml, it's qualified name is w:tab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tab")]
pub struct TabChar {}
/// Position of Last Calculated Page Break.
/// When the object is serialized out as xml, it's qualified name is w:lastRenderedPageBreak.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lastRenderedPageBreak")]
pub struct LastRenderedPageBreak {}
/// Defines the SdtContentEquation Class.
/// When the object is serialized out as xml, it's qualified name is w:equation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:equation")]
pub struct SdtContentEquation {}
/// Defines the SdtContentPicture Class.
/// When the object is serialized out as xml, it's qualified name is w:picture.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:picture")]
pub struct SdtContentPicture {}
/// Defines the SdtContentRichText Class.
/// When the object is serialized out as xml, it's qualified name is w:richText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:richText")]
pub struct SdtContentRichText {}
/// Defines the SdtContentCitation Class.
/// When the object is serialized out as xml, it's qualified name is w:citation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:citation")]
pub struct SdtContentCitation {}
/// Defines the SdtContentGroup Class.
/// When the object is serialized out as xml, it's qualified name is w:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:group")]
pub struct SdtContentGroup {}
/// Defines the SdtContentBibliography Class.
/// When the object is serialized out as xml, it's qualified name is w:bibliography.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bibliography")]
pub struct SdtContentBibliography {}
/// Upgrade Document on Open.
/// When the object is serialized out as xml, it's qualified name is w:forceUpgrade.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:forceUpgrade")]
pub struct ForceUpgrade {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Symbol Character.
/// When the object is serialized out as xml, it's qualified name is w:sym.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sym")]
pub struct SymbolChar {
    /// Symbol Character Font
    /// Represents the following attribute in the schema: w:font
    #[xml(attr = "w:font")]
    pub font: Option<String>,
    /// Symbol Character Code
    /// Represents the following attribute in the schema: w:char
    #[xml(attr = "w:char")]
    pub char: Option<String>,
}
/// Inline Embedded Object.
/// When the object is serialized out as xml, it's qualified name is w:object.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:object")]
pub struct EmbeddedObject {
    /// dxaOrig
    /// Represents the following attribute in the schema: w:dxaOrig
    #[xml(attr = "w:dxaOrig")]
    pub dxa_original: Option<String>,
    /// dyaOrig
    /// Represents the following attribute in the schema: w:dyaOrig
    #[xml(attr = "w:dyaOrig")]
    pub dya_original: Option<String>,
    /// anchorId
    /// Represents the following attribute in the schema: w14:anchorId
    #[xml(attr = "w14:anchorId")]
    pub w14_anchor_id: Option<String>,
    #[xml(
        child = "v:group",
        child = "v:image",
        child = "v:line",
        child = "v:oval",
        child = "v:polyline",
        child = "v:rect",
        child = "v:roundrect",
        child = "v:shape",
        child = "v:shapetype",
        child = "v:arc",
        child = "v:curve",
        child = "o:OLEObject",
        child = "w:drawing",
        child = "w:control",
        child = "w:objectEmbed",
        child = "w:objectLink",
    )]
    pub children: Vec<EmbeddedObjectChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EmbeddedObjectChildChoice {
    #[xml(tag = "v:group")]
    VGroup(crate::schemas::schemas_microsoft_com_vml::Group),
    #[xml(tag = "v:image")]
    VImage(crate::schemas::schemas_microsoft_com_vml::ImageFile),
    #[xml(tag = "v:line")]
    VLine(crate::schemas::schemas_microsoft_com_vml::Line),
    #[xml(tag = "v:oval")]
    VOval(crate::schemas::schemas_microsoft_com_vml::Oval),
    #[xml(tag = "v:polyline")]
    VPolyline(crate::schemas::schemas_microsoft_com_vml::PolyLine),
    #[xml(tag = "v:rect")]
    VRect(crate::schemas::schemas_microsoft_com_vml::Rectangle),
    #[xml(tag = "v:roundrect")]
    VRoundrect(crate::schemas::schemas_microsoft_com_vml::RoundRectangle),
    #[xml(tag = "v:shape")]
    VShape(crate::schemas::schemas_microsoft_com_vml::Shape),
    #[xml(tag = "v:shapetype")]
    VShapetype(crate::schemas::schemas_microsoft_com_vml::Shapetype),
    #[xml(tag = "v:arc")]
    VArc(crate::schemas::schemas_microsoft_com_vml::Arc),
    #[xml(tag = "v:curve")]
    VCurve(crate::schemas::schemas_microsoft_com_vml::Curve),
    #[xml(tag = "o:OLEObject")]
    OOleObject(crate::schemas::schemas_microsoft_com_office_office::OleObject),
    #[xml(tag = "w:drawing")]
    WDrawing(Drawing),
    #[xml(tag = "w:control")]
    WControl(Control),
    #[xml(tag = "w:objectEmbed")]
    WObjectEmbed(ObjectEmbed),
    #[xml(tag = "w:objectLink")]
    WObjectLink(ObjectLink),
}
/// VML Object.
/// When the object is serialized out as xml, it's qualified name is w:pict.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pict")]
pub struct Picture {
    /// anchorId
    /// Represents the following attribute in the schema: w14:anchorId
    #[xml(attr = "w14:anchorId")]
    pub w14_anchor_id: Option<String>,
    #[xml(
        child = "v:group",
        child = "v:image",
        child = "v:line",
        child = "v:oval",
        child = "v:polyline",
        child = "v:rect",
        child = "v:roundrect",
        child = "v:shape",
        child = "v:shapetype",
        child = "v:arc",
        child = "v:curve",
        child = "o:OLEObject",
        child = "w:movie",
        child = "w:control",
    )]
    pub children: Vec<PictureChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PictureChildChoice {
    #[xml(tag = "v:group")]
    VGroup(crate::schemas::schemas_microsoft_com_vml::Group),
    #[xml(tag = "v:image")]
    VImage(crate::schemas::schemas_microsoft_com_vml::ImageFile),
    #[xml(tag = "v:line")]
    VLine(crate::schemas::schemas_microsoft_com_vml::Line),
    #[xml(tag = "v:oval")]
    VOval(crate::schemas::schemas_microsoft_com_vml::Oval),
    #[xml(tag = "v:polyline")]
    VPolyline(crate::schemas::schemas_microsoft_com_vml::PolyLine),
    #[xml(tag = "v:rect")]
    VRect(crate::schemas::schemas_microsoft_com_vml::Rectangle),
    #[xml(tag = "v:roundrect")]
    VRoundrect(crate::schemas::schemas_microsoft_com_vml::RoundRectangle),
    #[xml(tag = "v:shape")]
    VShape(crate::schemas::schemas_microsoft_com_vml::Shape),
    #[xml(tag = "v:shapetype")]
    VShapetype(crate::schemas::schemas_microsoft_com_vml::Shapetype),
    #[xml(tag = "v:arc")]
    VArc(crate::schemas::schemas_microsoft_com_vml::Arc),
    #[xml(tag = "v:curve")]
    VCurve(crate::schemas::schemas_microsoft_com_vml::Curve),
    #[xml(tag = "o:OLEObject")]
    OOleObject(crate::schemas::schemas_microsoft_com_office_office::OleObject),
    #[xml(tag = "w:movie")]
    WMovie(MovieReference),
    #[xml(tag = "w:control")]
    WControl(Control),
}
/// Complex Field Character.
/// When the object is serialized out as xml, it's qualified name is w:fldChar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fldChar")]
pub struct FieldChar {
    /// Field Character Type
    /// Represents the following attribute in the schema: w:fldCharType
    #[xml(attr = "w:fldCharType")]
    pub field_char_type: FieldCharValues,
    /// Field Should Not Be Recalculated
    /// Represents the following attribute in the schema: w:fldLock
    #[xml(attr = "w:fldLock")]
    pub field_lock: Option<bool>,
    /// Field Result Invalidated
    /// Represents the following attribute in the schema: w:dirty
    #[xml(attr = "w:dirty")]
    pub dirty: Option<bool>,
    #[xml(child = "w:fldData", child = "w:ffData", child = "w:numberingChange")]
    pub children: Vec<FieldCharChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FieldCharChildChoice {
    #[xml(tag = "w:fldData")]
    WFldData(FieldData),
    #[xml(tag = "w:ffData")]
    WFfData(FormFieldData),
    #[xml(tag = "w:numberingChange")]
    WNumberingChange(NumberingChange),
}
/// Phonetic Guide.
/// When the object is serialized out as xml, it's qualified name is w:ruby.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ruby")]
pub struct Ruby {
    ///Phonetic Guide Properties
    #[xml(child = "w:rubyPr")]
    pub ruby_properties: RubyProperties,
    ///Phonetic Guide Text
    #[xml(child = "w:rt")]
    pub ruby_content: RubyContent,
    ///Phonetic Guide Base Text
    #[xml(child = "w:rubyBase")]
    pub ruby_base: RubyBase,
}
/// Footnote Reference.
/// When the object is serialized out as xml, it's qualified name is w:footnoteReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnoteReference")]
pub struct FootnoteReference {
    /// Suppress Footnote/Endnote Reference Mark
    /// Represents the following attribute in the schema: w:customMarkFollows
    #[xml(attr = "w:customMarkFollows")]
    pub custom_mark_follows: Option<bool>,
    /// Footnote/Endnote ID Reference
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
}
/// Endnote Reference.
/// When the object is serialized out as xml, it's qualified name is w:endnoteReference.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnoteReference")]
pub struct EndnoteReference {
    /// Suppress Footnote/Endnote Reference Mark
    /// Represents the following attribute in the schema: w:customMarkFollows
    #[xml(attr = "w:customMarkFollows")]
    pub custom_mark_follows: Option<bool>,
    /// Footnote/Endnote ID Reference
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
}
/// Defines the FootnoteEndnoteReferenceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct FootnoteEndnoteReferenceType {
    /// Suppress Footnote/Endnote Reference Mark
    /// Represents the following attribute in the schema: w:customMarkFollows
    #[xml(attr = "w:customMarkFollows")]
    pub custom_mark_follows: Option<bool>,
    /// Footnote/Endnote ID Reference
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
}
/// DrawingML Object.
/// When the object is serialized out as xml, it's qualified name is w:drawing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:drawing")]
pub struct Drawing {
    #[xml(child = "wp:anchor", child = "wp:inline")]
    pub children: Vec<DrawingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DrawingChildChoice {
    #[xml(tag = "wp:anchor")]
    WpAnchor(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing::Anchor,
    ),
    #[xml(tag = "wp:inline")]
    WpInline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing::Inline,
    ),
}
/// Absolute Position Tab Character.
/// When the object is serialized out as xml, it's qualified name is w:ptab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ptab")]
pub struct PositionalTab {
    /// Positional Tab Stop Alignment
    /// Represents the following attribute in the schema: w:alignment
    #[xml(attr = "w:alignment")]
    pub alignment: AbsolutePositionTabAlignmentValues,
    /// Positional Tab Base
    /// Represents the following attribute in the schema: w:relativeTo
    #[xml(attr = "w:relativeTo")]
    pub relative_to: AbsolutePositionTabPositioningBaseValues,
    /// Tab Leader Character
    /// Represents the following attribute in the schema: w:leader
    #[xml(attr = "w:leader")]
    pub leader: AbsolutePositionTabLeaderCharValues,
}
/// Defines the RunStyle Class.
/// When the object is serialized out as xml, it's qualified name is w:rStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rStyle")]
pub struct RunStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the TableStyle Class.
/// When the object is serialized out as xml, it's qualified name is w:tblStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblStyle")]
pub struct TableStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Paragraph Style's Associated Numbering Level.
/// When the object is serialized out as xml, it's qualified name is w:pStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pStyle")]
pub struct ParagraphStyleIdInLevel {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Abstract Numbering Definition Name.
/// When the object is serialized out as xml, it's qualified name is w:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:name")]
pub struct AbstractNumDefinitionName {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Numbering Style Definition.
/// When the object is serialized out as xml, it's qualified name is w:styleLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:styleLink")]
pub struct StyleLink {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Numbering Style Reference.
/// When the object is serialized out as xml, it's qualified name is w:numStyleLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numStyleLink")]
pub struct NumberingStyleLink {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Alternate Style Names.
/// When the object is serialized out as xml, it's qualified name is w:aliases.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:aliases")]
pub struct Aliases {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Parent Style ID.
/// When the object is serialized out as xml, it's qualified name is w:basedOn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:basedOn")]
pub struct BasedOn {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Style For Next Paragraph.
/// When the object is serialized out as xml, it's qualified name is w:next.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:next")]
pub struct NextParagraphStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Linked Style Reference.
/// When the object is serialized out as xml, it's qualified name is w:link.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:link")]
pub struct LinkedStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Paragraph Style Applied to Automatically Generated Paragraphs.
/// When the object is serialized out as xml, it's qualified name is w:clickAndTypeStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:clickAndTypeStyle")]
pub struct ClickAndTypeStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Default Table Style for Newly Inserted Tables.
/// When the object is serialized out as xml, it's qualified name is w:defaultTableStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:defaultTableStyle")]
pub struct DefaultTableStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the String253Type Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct String253Type {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the RunFonts Class.
/// When the object is serialized out as xml, it's qualified name is w:rFonts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rFonts")]
pub struct RunFonts {
    /// Font Content Type
    /// Represents the following attribute in the schema: w:hint
    #[xml(attr = "w:hint")]
    pub hint: Option<FontTypeHintValues>,
    /// ASCII Font
    /// Represents the following attribute in the schema: w:ascii
    #[xml(attr = "w:ascii")]
    pub ascii: Option<String>,
    /// High ANSI Font
    /// Represents the following attribute in the schema: w:hAnsi
    #[xml(attr = "w:hAnsi")]
    pub high_ansi: Option<String>,
    /// East Asian Font
    /// Represents the following attribute in the schema: w:eastAsia
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<String>,
    /// Complex Script Font
    /// Represents the following attribute in the schema: w:cs
    #[xml(attr = "w:cs")]
    pub complex_script: Option<String>,
    /// ASCII Theme Font
    /// Represents the following attribute in the schema: w:asciiTheme
    #[xml(attr = "w:asciiTheme")]
    pub ascii_theme: Option<ThemeFontValues>,
    /// High ANSI Theme Font
    /// Represents the following attribute in the schema: w:hAnsiTheme
    #[xml(attr = "w:hAnsiTheme")]
    pub high_ansi_theme: Option<ThemeFontValues>,
    /// East Asian Theme Font
    /// Represents the following attribute in the schema: w:eastAsiaTheme
    #[xml(attr = "w:eastAsiaTheme")]
    pub east_asia_theme: Option<ThemeFontValues>,
    /// Complex Script Theme Font
    /// Represents the following attribute in the schema: w:cstheme
    #[xml(attr = "w:cstheme")]
    pub complex_script_theme: Option<ThemeFontValues>,
}
/// Defines the Color Class.
/// When the object is serialized out as xml, it's qualified name is w:color.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:color")]
pub struct Color {
    /// Run Content Color
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
    /// Run Content Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Run Content Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Run Content Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
}
/// Defines the Spacing Class.
/// When the object is serialized out as xml, it's qualified name is w:spacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:spacing")]
pub struct Spacing {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the CharacterScale Class.
/// When the object is serialized out as xml, it's qualified name is w:w.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:w")]
pub struct CharacterScale {
    /// Text Expansion/Compression Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<i32>,
}
/// Defines the Kern Class.
/// When the object is serialized out as xml, it's qualified name is w:kern.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:kern")]
pub struct Kern {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the Position Class.
/// When the object is serialized out as xml, it's qualified name is w:position.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:position")]
pub struct Position {
    /// Signed Half-Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the FontSize Class.
/// When the object is serialized out as xml, it's qualified name is w:sz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sz")]
pub struct FontSize {
    /// Half Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the FontSizeComplexScript Class.
/// When the object is serialized out as xml, it's qualified name is w:szCs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:szCs")]
pub struct FontSizeComplexScript {
    /// Half Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Checkbox Form Field Size.
/// When the object is serialized out as xml, it's qualified name is w:size.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:size")]
pub struct FormFieldSize {
    /// Half Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Phonetic Guide Text Font Size.
/// When the object is serialized out as xml, it's qualified name is w:hps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hps")]
pub struct PhoneticGuideTextFontSize {
    /// Half Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Phonetic Guide Base Text Font Size.
/// When the object is serialized out as xml, it's qualified name is w:hpsBaseText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hpsBaseText")]
pub struct PhoneticGuideBaseTextSize {
    /// Half Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the HpsMeasureType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct HpsMeasureType {
    /// Half Point Measurement
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the Highlight Class.
/// When the object is serialized out as xml, it's qualified name is w:highlight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:highlight")]
pub struct Highlight {
    /// Highlighting Color
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: HighlightColorValues,
}
/// Defines the Underline Class.
/// When the object is serialized out as xml, it's qualified name is w:u.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:u")]
pub struct Underline {
    /// Underline Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<UnderlineValues>,
    /// Underline Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Underline Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Underline Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Underline Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
}
/// Defines the TextEffect Class.
/// When the object is serialized out as xml, it's qualified name is w:effect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:effect")]
pub struct TextEffect {
    /// Animated Text Effect Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TextEffectValues,
}
/// Defines the Border Class.
/// When the object is serialized out as xml, it's qualified name is w:bdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bdr")]
pub struct Border {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Paragraph Border Above Identical Paragraphs.
/// When the object is serialized out as xml, it's qualified name is w:top.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:top")]
pub struct TopBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Left Paragraph Border.
/// When the object is serialized out as xml, it's qualified name is w:left.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:left")]
pub struct LeftBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Paragraph Border Between Identical Paragraphs.
/// When the object is serialized out as xml, it's qualified name is w:bottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bottom")]
pub struct BottomBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Right Paragraph Border.
/// When the object is serialized out as xml, it's qualified name is w:right.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:right")]
pub struct RightBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Paragraph Border Between Identical Paragraphs.
/// When the object is serialized out as xml, it's qualified name is w:between.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:between")]
pub struct BetweenBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Paragraph Border Between Facing Pages.
/// When the object is serialized out as xml, it's qualified name is w:bar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bar")]
pub struct BarBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Defines the StartBorder Class.
/// When the object is serialized out as xml, it's qualified name is w:start.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:start")]
pub struct StartBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Defines the EndBorder Class.
/// When the object is serialized out as xml, it's qualified name is w:end.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:end")]
pub struct EndBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Table Inside Horizontal Edges Border.
/// When the object is serialized out as xml, it's qualified name is w:insideH.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:insideH")]
pub struct InsideHorizontalBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Table Inside Vertical Edges Border.
/// When the object is serialized out as xml, it's qualified name is w:insideV.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:insideV")]
pub struct InsideVerticalBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Table Cell Top Left to Bottom Right Diagonal Border.
/// When the object is serialized out as xml, it's qualified name is w:tl2br.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tl2br")]
pub struct TopLeftToBottomRightCellBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Table Cell Top Right to Bottom Left Diagonal Border.
/// When the object is serialized out as xml, it's qualified name is w:tr2bl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tr2bl")]
pub struct TopRightToBottomLeftCellBorder {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Defines the BorderType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BorderType {
    /// Border Style
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: BorderValues,
    /// Border Color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// Border Theme Color
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// Border Theme Color Tint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// Border Theme Color Shade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    /// Border Width
    /// Represents the following attribute in the schema: w:sz
    #[xml(attr = "w:sz")]
    pub size: Option<i32>,
    /// Border Spacing Measurement
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<i32>,
    /// Border Shadow
    /// Represents the following attribute in the schema: w:shadow
    #[xml(attr = "w:shadow")]
    pub shadow: Option<bool>,
    /// Create Frame Effect
    /// Represents the following attribute in the schema: w:frame
    #[xml(attr = "w:frame")]
    pub frame: Option<bool>,
}
/// Defines the FitText Class.
/// When the object is serialized out as xml, it's qualified name is w:fitText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fitText")]
pub struct FitText {
    /// Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
    /// Fit Text Run ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: Option<i32>,
}
/// Defines the VerticalTextAlignment Class.
/// When the object is serialized out as xml, it's qualified name is w:vertAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:vertAlign")]
pub struct VerticalTextAlignment {
    /// Subscript/Superscript Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: VerticalPositionValues,
}
/// Defines the Emphasis Class.
/// When the object is serialized out as xml, it's qualified name is w:em.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:em")]
pub struct Emphasis {
    /// Emphasis Mark Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: EmphasisMarkValues,
}
/// Defines the Languages Class.
/// When the object is serialized out as xml, it's qualified name is w:lang.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lang")]
pub struct Languages {
    /// Latin Language
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
    /// East Asian Language
    /// Represents the following attribute in the schema: w:eastAsia
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<String>,
    /// Complex Script Language
    /// Represents the following attribute in the schema: w:bidi
    #[xml(attr = "w:bidi")]
    pub bidi: Option<String>,
}
/// Theme Font Languages.
/// When the object is serialized out as xml, it's qualified name is w:themeFontLang.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:themeFontLang")]
pub struct ThemeFontLanguages {
    /// Latin Language
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
    /// East Asian Language
    /// Represents the following attribute in the schema: w:eastAsia
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<String>,
    /// Complex Script Language
    /// Represents the following attribute in the schema: w:bidi
    #[xml(attr = "w:bidi")]
    pub bidi: Option<String>,
}
/// Defines the LanguageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LanguageType {
    /// Latin Language
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
    /// East Asian Language
    /// Represents the following attribute in the schema: w:eastAsia
    #[xml(attr = "w:eastAsia")]
    pub east_asia: Option<String>,
    /// Complex Script Language
    /// Represents the following attribute in the schema: w:bidi
    #[xml(attr = "w:bidi")]
    pub bidi: Option<String>,
}
/// Defines the EastAsianLayout Class.
/// When the object is serialized out as xml, it's qualified name is w:eastAsianLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:eastAsianLayout")]
pub struct EastAsianLayout {
    /// East Asian Typography Run ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: Option<i32>,
    /// Two Lines in One
    /// Represents the following attribute in the schema: w:combine
    #[xml(attr = "w:combine")]
    pub combine: Option<bool>,
    /// Display Brackets Around Two Lines in One
    /// Represents the following attribute in the schema: w:combineBrackets
    #[xml(attr = "w:combineBrackets")]
    pub combine_brackets: Option<CombineBracketValues>,
    /// Horizontal in Vertical (Rotate Text)
    /// Represents the following attribute in the schema: w:vert
    #[xml(attr = "w:vert")]
    pub vertical: Option<bool>,
    /// Compress Rotated Text to Line Height
    /// Represents the following attribute in the schema: w:vertCompress
    #[xml(attr = "w:vertCompress")]
    pub vertical_compress: Option<bool>,
}
/// Defines the RunPropertiesChange Class.
/// When the object is serialized out as xml, it's qualified name is w:rPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPrChange")]
pub struct RunPropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr")]
    pub children: Vec<RunPropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunPropertiesChangeChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(PreviousRunProperties),
}
/// Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct RunProperties {
    #[xml(
        child = "w:rStyle",
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:highlight",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:rtl",
        child = "w:cs",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
        child = "w14:glow",
        child = "w14:shadow",
        child = "w14:reflection",
        child = "w14:textOutline",
        child = "w14:textFill",
        child = "w14:scene3d",
        child = "w14:props3d",
        child = "w14:ligatures",
        child = "w14:numForm",
        child = "w14:numSpacing",
        child = "w14:stylisticSets",
        child = "w14:cntxtAlts",
        child = "w:rPrChange",
    )]
    pub children: Vec<RunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunPropertiesChildChoice {
    #[xml(tag = "w:rStyle")]
    WRStyle(RunStyle),
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:highlight")]
    WHighlight(Highlight),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:rtl")]
    WRtl(RightToLeftText),
    #[xml(tag = "w:cs")]
    WCs(ComplexScript),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
    #[xml(tag = "w14:glow")]
    W14Glow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow),
    #[xml(tag = "w14:shadow")]
    W14Shadow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow),
    #[xml(tag = "w14:reflection")]
    W14Reflection(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection,
    ),
    #[xml(tag = "w14:textOutline")]
    W14TextOutline(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect,
    ),
    #[xml(tag = "w14:textFill")]
    W14TextFill(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect,
    ),
    #[xml(tag = "w14:scene3d")]
    W14Scene3d(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D),
    #[xml(tag = "w14:props3d")]
    W14Props3d(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D,
    ),
    #[xml(tag = "w14:ligatures")]
    W14Ligatures(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures,
    ),
    #[xml(tag = "w14:numForm")]
    W14NumForm(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat,
    ),
    #[xml(tag = "w14:numSpacing")]
    W14NumSpacing(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing,
    ),
    #[xml(tag = "w14:stylisticSets")]
    W14StylisticSets(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets,
    ),
    #[xml(tag = "w14:cntxtAlts")]
    W14CntxtAlts(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives,
    ),
    #[xml(tag = "w:rPrChange")]
    WRPrChange(RunPropertiesChange),
}
/// Defines the InsertedMathControl Class.
/// When the object is serialized out as xml, it's qualified name is w:ins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ins")]
pub struct InsertedMathControl {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr", child = "w:del")]
    pub children: Vec<InsertedMathControlChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InsertedMathControlChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
    #[xml(tag = "w:del")]
    WDel(DeletedMathControl),
}
/// Defines the DeletedMathControl Class.
/// When the object is serialized out as xml, it's qualified name is w:del.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:del")]
pub struct DeletedMathControl {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr")]
    pub children: Vec<DeletedMathControlChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DeletedMathControlChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
}
/// Defines the MoveFromMathControl Class.
/// When the object is serialized out as xml, it's qualified name is w:moveFrom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveFrom")]
pub struct MoveFromMathControl {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr", child = "w:ins", child = "w:del")]
    pub children: Vec<MoveFromMathControlChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MoveFromMathControlChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
    #[xml(tag = "w:ins")]
    WIns(InsertedMathControl),
    #[xml(tag = "w:del")]
    WDel(DeletedMathControl),
}
/// Defines the MoveToMathControl Class.
/// When the object is serialized out as xml, it's qualified name is w:moveTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveTo")]
pub struct MoveToMathControl {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr", child = "w:ins", child = "w:del")]
    pub children: Vec<MoveToMathControlChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MoveToMathControlChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
    #[xml(tag = "w:ins")]
    WIns(InsertedMathControl),
    #[xml(tag = "w:del")]
    WDel(DeletedMathControl),
}
/// Defines the MathControlMoveType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MathControlMoveType {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr", child = "w:ins", child = "w:del")]
    pub children: Vec<MathControlMoveTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MathControlMoveTypeChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
    #[xml(tag = "w:ins")]
    WIns(InsertedMathControl),
    #[xml(tag = "w:del")]
    WDel(DeletedMathControl),
}
/// Defines the CustomXmlRuby Class.
/// When the object is serialized out as xml, it's qualified name is w:customXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXml")]
pub struct CustomXmlRuby {
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<CustomXmlRubyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomXmlRubyChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Defines the SimpleFieldRuby Class.
/// When the object is serialized out as xml, it's qualified name is w:fldSimple.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fldSimple")]
pub struct SimpleFieldRuby {
    /// instr
    /// Represents the following attribute in the schema: w:instr
    #[xml(attr = "w:instr")]
    pub instruction: String,
    /// fldLock
    /// Represents the following attribute in the schema: w:fldLock
    #[xml(attr = "w:fldLock")]
    pub field_lock: Option<bool>,
    /// dirty
    /// Represents the following attribute in the schema: w:dirty
    #[xml(attr = "w:dirty")]
    pub dirty: Option<bool>,
    #[xml(
        child = "w:fldData",
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<SimpleFieldRubyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SimpleFieldRubyChildChoice {
    #[xml(tag = "w:fldData")]
    WFldData(FieldData),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Defines the HyperlinkRuby Class.
/// When the object is serialized out as xml, it's qualified name is w:hyperlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hyperlink")]
pub struct HyperlinkRuby {
    /// tgtFrame
    /// Represents the following attribute in the schema: w:tgtFrame
    #[xml(attr = "w:tgtFrame")]
    pub target_frame: Option<String>,
    /// tooltip
    /// Represents the following attribute in the schema: w:tooltip
    #[xml(attr = "w:tooltip")]
    pub tooltip: Option<String>,
    /// docLocation
    /// Represents the following attribute in the schema: w:docLocation
    #[xml(attr = "w:docLocation")]
    pub doc_location: Option<String>,
    /// history
    /// Represents the following attribute in the schema: w:history
    #[xml(attr = "w:history")]
    pub history: Option<bool>,
    /// anchor
    /// Represents the following attribute in the schema: w:anchor
    #[xml(attr = "w:anchor")]
    pub anchor: Option<String>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<HyperlinkRubyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HyperlinkRubyChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Phonetic Guide Text Run.
/// When the object is serialized out as xml, it's qualified name is w:r.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:r")]
pub struct Run {
    /// Revision Identifier for Run Properties
    /// Represents the following attribute in the schema: w:rsidRPr
    #[xml(attr = "w:rsidRPr")]
    pub rsid_run_properties: Option<String>,
    /// Revision Identifier for Run Deletion
    /// Represents the following attribute in the schema: w:rsidDel
    #[xml(attr = "w:rsidDel")]
    pub rsid_run_deletion: Option<String>,
    /// Revision Identifier for Run
    /// Represents the following attribute in the schema: w:rsidR
    #[xml(attr = "w:rsidR")]
    pub rsid_run_addition: Option<String>,
    #[xml(
        child = "w:rPr",
        child = "w:br",
        child = "w:t",
        child = "w:delText",
        child = "w:instrText",
        child = "w:delInstrText",
        child = "w:noBreakHyphen",
        child = "w:softHyphen",
        child = "w:dayShort",
        child = "w:monthShort",
        child = "w:yearShort",
        child = "w:dayLong",
        child = "w:monthLong",
        child = "w:yearLong",
        child = "w:annotationRef",
        child = "w:footnoteRef",
        child = "w:endnoteRef",
        child = "w:separator",
        child = "w:continuationSeparator",
        child = "w:sym",
        child = "w:pgNum",
        child = "w:cr",
        child = "w:tab",
        child = "w:object",
        child = "w:pict",
        child = "w:fldChar",
        child = "w:ruby",
        child = "w:footnoteReference",
        child = "w:endnoteReference",
        child = "w:commentReference",
        child = "w:drawing",
        child = "w:ptab",
        child = "w:lastRenderedPageBreak",
    )]
    pub children: Vec<RunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
    #[xml(tag = "w:br")]
    WBr(Break),
    #[xml(tag = "w:t")]
    WT(Text),
    #[xml(tag = "w:delText")]
    WDelText(DeletedText),
    #[xml(tag = "w:instrText")]
    WInstrText(FieldCode),
    #[xml(tag = "w:delInstrText")]
    WDelInstrText(DeletedFieldCode),
    #[xml(tag = "w:noBreakHyphen")]
    WNoBreakHyphen(NoBreakHyphen),
    #[xml(tag = "w:softHyphen")]
    WSoftHyphen(SoftHyphen),
    #[xml(tag = "w:dayShort")]
    WDayShort(DayShort),
    #[xml(tag = "w:monthShort")]
    WMonthShort(MonthShort),
    #[xml(tag = "w:yearShort")]
    WYearShort(YearShort),
    #[xml(tag = "w:dayLong")]
    WDayLong(DayLong),
    #[xml(tag = "w:monthLong")]
    WMonthLong(MonthLong),
    #[xml(tag = "w:yearLong")]
    WYearLong(YearLong),
    #[xml(tag = "w:annotationRef")]
    WAnnotationRef(AnnotationReferenceMark),
    #[xml(tag = "w:footnoteRef")]
    WFootnoteRef(FootnoteReferenceMark),
    #[xml(tag = "w:endnoteRef")]
    WEndnoteRef(EndnoteReferenceMark),
    #[xml(tag = "w:separator")]
    WSeparator(SeparatorMark),
    #[xml(tag = "w:continuationSeparator")]
    WContinuationSeparator(ContinuationSeparatorMark),
    #[xml(tag = "w:sym")]
    WSym(SymbolChar),
    #[xml(tag = "w:pgNum")]
    WPgNum(PageNumber),
    #[xml(tag = "w:cr")]
    WCr(CarriageReturn),
    #[xml(tag = "w:tab")]
    WTab(TabChar),
    #[xml(tag = "w:object")]
    WObject(EmbeddedObject),
    #[xml(tag = "w:pict")]
    WPict(Picture),
    #[xml(tag = "w:fldChar")]
    WFldChar(FieldChar),
    #[xml(tag = "w:ruby")]
    WRuby(Ruby),
    #[xml(tag = "w:footnoteReference")]
    WFootnoteReference(FootnoteReference),
    #[xml(tag = "w:endnoteReference")]
    WEndnoteReference(EndnoteReference),
    #[xml(tag = "w:commentReference")]
    WCommentReference(CommentReference),
    #[xml(tag = "w:drawing")]
    WDrawing(Drawing),
    #[xml(tag = "w:ptab")]
    WPtab(PositionalTab),
    #[xml(tag = "w:lastRenderedPageBreak")]
    WLastRenderedPageBreak(LastRenderedPageBreak),
}
/// Defines the SdtRunRuby Class.
/// When the object is serialized out as xml, it's qualified name is w:sdt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdt")]
pub struct SdtRunRuby {
    #[xml(
        child = "w:sdtContent",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
    )]
    pub children: Vec<SdtRunRubyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtRunRubyChildChoice {
    #[xml(tag = "w:sdtContent")]
    WSdtContent(SdtContentRunRuby),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
}
/// Defines the ProofError Class.
/// When the object is serialized out as xml, it's qualified name is w:proofErr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:proofErr")]
pub struct ProofError {
    /// Proofing Error Anchor Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: ProofingErrorValues,
}
/// Defines the PermStart Class.
/// When the object is serialized out as xml, it's qualified name is w:permStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:permStart")]
pub struct PermStart {
    /// edGrp
    /// Represents the following attribute in the schema: w:edGrp
    #[xml(attr = "w:edGrp")]
    pub editor_group: Option<RangePermissionEditingGroupValues>,
    /// ed
    /// Represents the following attribute in the schema: w:ed
    #[xml(attr = "w:ed")]
    pub ed: Option<String>,
    /// colFirst
    /// Represents the following attribute in the schema: w:colFirst
    #[xml(attr = "w:colFirst")]
    pub column_first: Option<i32>,
    /// colLast
    /// Represents the following attribute in the schema: w:colLast
    #[xml(attr = "w:colLast")]
    pub column_last: Option<i32>,
    /// Annotation ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
    /// Annotation Displaced By Custom XML Markup
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
}
/// Defines the PermEnd Class.
/// When the object is serialized out as xml, it's qualified name is w:permEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:permEnd")]
pub struct PermEnd {
    /// Annotation ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
    /// Annotation Displaced By Custom XML Markup
    /// Represents the following attribute in the schema: w:displacedByCustomXml
    #[xml(attr = "w:displacedByCustomXml")]
    pub displaced_by_custom_xml: Option<DisplacedByCustomXmlValues>,
}
/// Inserted Run Content.
/// When the object is serialized out as xml, it's qualified name is w:ins.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ins")]
pub struct InsertedRun {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
    )]
    pub children: Vec<InsertedRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InsertedRunChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
}
/// Deleted Run Content.
/// When the object is serialized out as xml, it's qualified name is w:del.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:del")]
pub struct DeletedRun {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
    )]
    pub children: Vec<DeletedRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DeletedRunChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
}
/// Move Source Run Content.
/// When the object is serialized out as xml, it's qualified name is w:moveFrom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveFrom")]
pub struct MoveFromRun {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
    )]
    pub children: Vec<MoveFromRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MoveFromRunChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
}
/// Move Destination Run Content.
/// When the object is serialized out as xml, it's qualified name is w:moveTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:moveTo")]
pub struct MoveToRun {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
    )]
    pub children: Vec<MoveToRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MoveToRunChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
}
/// Defines the RunTrackChangeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RunTrackChangeType {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
    )]
    pub children: Vec<RunTrackChangeTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunTrackChangeTypeChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
}
/// Defines the ContentPart Class.
/// When the object is serialized out as xml, it's qualified name is w:contentPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:contentPart")]
pub struct ContentPart {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the SdtRun Class.
/// When the object is serialized out as xml, it's qualified name is w:sdt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdt")]
pub struct SdtRun {
    #[xml(
        child = "w:sdtContent",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
    )]
    pub children: Vec<SdtRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtRunChildChoice {
    #[xml(tag = "w:sdtContent")]
    WSdtContent(SdtContentRun),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
}
/// Defines the CustomXmlBlock Class.
/// When the object is serialized out as xml, it's qualified name is w:customXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXml")]
pub struct CustomXmlBlock {
    #[xml(
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<CustomXmlBlockChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomXmlBlockChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Defines the SdtBlock Class.
/// When the object is serialized out as xml, it's qualified name is w:sdt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdt")]
pub struct SdtBlock {
    #[xml(
        child = "w:sdtContent",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
    )]
    pub children: Vec<SdtBlockChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtBlockChildChoice {
    #[xml(tag = "w:sdtContent")]
    WSdtContent(SdtContentBlock),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
}
/// Defines the Paragraph Class.
/// When the object is serialized out as xml, it's qualified name is w:p.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:p")]
pub struct Paragraph {
    /// Revision Identifier for Paragraph Glyph Formatting
    /// Represents the following attribute in the schema: w:rsidRPr
    #[xml(attr = "w:rsidRPr")]
    pub rsid_paragraph_mark_revision: Option<String>,
    /// Revision Identifier for Paragraph
    /// Represents the following attribute in the schema: w:rsidR
    #[xml(attr = "w:rsidR")]
    pub rsid_paragraph_addition: Option<String>,
    /// Revision Identifier for Paragraph Deletion
    /// Represents the following attribute in the schema: w:rsidDel
    #[xml(attr = "w:rsidDel")]
    pub rsid_paragraph_deletion: Option<String>,
    /// Revision Identifier for Paragraph Properties
    /// Represents the following attribute in the schema: w:rsidP
    #[xml(attr = "w:rsidP")]
    pub rsid_paragraph_properties: Option<String>,
    /// Default Revision Identifier for Runs
    /// Represents the following attribute in the schema: w:rsidRDefault
    #[xml(attr = "w:rsidRDefault")]
    pub rsid_run_addition_default: Option<String>,
    /// paraId
    /// Represents the following attribute in the schema: w14:paraId
    #[xml(attr = "w14:paraId")]
    pub paragraph_id: Option<String>,
    /// textId
    /// Represents the following attribute in the schema: w14:textId
    #[xml(attr = "w14:textId")]
    pub text_id: Option<String>,
    /// noSpellErr
    /// Represents the following attribute in the schema: w14:noSpellErr
    #[xml(attr = "w14:noSpellErr")]
    pub no_spell_error: Option<bool>,
    #[xml(
        child = "w:pPr",
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<ParagraphChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphChildChoice {
    #[xml(tag = "w:pPr")]
    WPPr(ParagraphProperties),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Defines the Table Class.
/// When the object is serialized out as xml, it's qualified name is w:tbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tbl")]
pub struct Table {
    #[xml(
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:tblPr",
        child = "w:tblGrid",
        child = "w:tr",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<TableChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableChildChoice {
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:tblPr")]
    WTblPr(TableProperties),
    #[xml(tag = "w:tblGrid")]
    WTblGrid(TableGrid),
    #[xml(tag = "w:tr")]
    WTr(TableRow),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRow),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRow),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Table Row.
/// When the object is serialized out as xml, it's qualified name is w:tr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tr")]
pub struct TableRow {
    /// Revision Identifier for Table Row Glyph Formatting
    /// Represents the following attribute in the schema: w:rsidRPr
    #[xml(attr = "w:rsidRPr")]
    pub rsid_table_row_mark_revision: Option<String>,
    /// Revision Identifier for Table Row
    /// Represents the following attribute in the schema: w:rsidR
    #[xml(attr = "w:rsidR")]
    pub rsid_table_row_addition: Option<String>,
    /// Revision Identifier for Table Row Deletion
    /// Represents the following attribute in the schema: w:rsidDel
    #[xml(attr = "w:rsidDel")]
    pub rsid_table_row_deletion: Option<String>,
    /// Revision Identifier for Table Row Properties
    /// Represents the following attribute in the schema: w:rsidTr
    #[xml(attr = "w:rsidTr")]
    pub rsid_table_row_properties: Option<String>,
    /// paraId
    /// Represents the following attribute in the schema: w14:paraId
    #[xml(attr = "w14:paraId")]
    pub paragraph_id: Option<String>,
    /// textId
    /// Represents the following attribute in the schema: w14:textId
    #[xml(attr = "w14:textId")]
    pub text_id: Option<String>,
    #[xml(
        child = "w:tblPrEx",
        child = "w:trPr",
        child = "w:tc",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<TableRowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableRowChildChoice {
    #[xml(tag = "w:tblPrEx")]
    WTblPrEx(TablePropertyExceptions),
    #[xml(tag = "w:trPr")]
    WTrPr(TableRowProperties),
    #[xml(tag = "w:tc")]
    WTc(TableCell),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlCell),
    #[xml(tag = "w:sdt")]
    WSdt(SdtCell),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Row-Level Custom XML Element.
/// When the object is serialized out as xml, it's qualified name is w:customXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXml")]
pub struct CustomXmlRow {
    #[xml(
        child = "w:tr",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<CustomXmlRowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomXmlRowChildChoice {
    #[xml(tag = "w:tr")]
    WTr(TableRow),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRow),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRow),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Row-Level Structured Document Tag.
/// When the object is serialized out as xml, it's qualified name is w:sdt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdt")]
pub struct SdtRow {
    #[xml(
        child = "w:sdtContent",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
    )]
    pub children: Vec<SdtRowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtRowChildChoice {
    #[xml(tag = "w:sdtContent")]
    WSdtContent(SdtContentRow),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
}
/// Table Cell.
/// When the object is serialized out as xml, it's qualified name is w:tc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tc")]
pub struct TableCell {
    #[xml(
        child = "w:tcPr",
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<TableCellChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableCellChildChoice {
    #[xml(tag = "w:tcPr")]
    WTcPr(TableCellProperties),
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Cell-Level Custom XML Element.
/// When the object is serialized out as xml, it's qualified name is w:customXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXml")]
pub struct CustomXmlCell {
    #[xml(
        child = "w:tc",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<CustomXmlCellChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomXmlCellChildChoice {
    #[xml(tag = "w:tc")]
    WTc(TableCell),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlCell),
    #[xml(tag = "w:sdt")]
    WSdt(SdtCell),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Cell-Level Structured Document Tag.
/// When the object is serialized out as xml, it's qualified name is w:sdt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdt")]
pub struct SdtCell {
    #[xml(
        child = "w:sdtContent",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
    )]
    pub children: Vec<SdtCellChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtCellChildChoice {
    #[xml(tag = "w:sdtContent")]
    WSdtContent(SdtContentCell),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
}
/// Defines the CustomXmlRun Class.
/// When the object is serialized out as xml, it's qualified name is w:customXml.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXml")]
pub struct CustomXmlRun {
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<CustomXmlRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomXmlRunChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Defines the SimpleField Class.
/// When the object is serialized out as xml, it's qualified name is w:fldSimple.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fldSimple")]
pub struct SimpleField {
    /// Field Codes
    /// Represents the following attribute in the schema: w:instr
    #[xml(attr = "w:instr")]
    pub instruction: String,
    /// Field Should Not Be Recalculated
    /// Represents the following attribute in the schema: w:fldLock
    #[xml(attr = "w:fldLock")]
    pub field_lock: Option<bool>,
    /// Field Result Invalidated
    /// Represents the following attribute in the schema: w:dirty
    #[xml(attr = "w:dirty")]
    pub dirty: Option<bool>,
    #[xml(
        child = "w:fldData",
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<SimpleFieldChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SimpleFieldChildChoice {
    #[xml(tag = "w:fldData")]
    WFldData(FieldData),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Defines the Hyperlink Class.
/// When the object is serialized out as xml, it's qualified name is w:hyperlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hyperlink")]
pub struct Hyperlink {
    /// Hyperlink Target Frame
    /// Represents the following attribute in the schema: w:tgtFrame
    #[xml(attr = "w:tgtFrame")]
    pub target_frame: Option<String>,
    /// Associated String
    /// Represents the following attribute in the schema: w:tooltip
    #[xml(attr = "w:tooltip")]
    pub tooltip: Option<String>,
    /// Location in Target Document
    /// Represents the following attribute in the schema: w:docLocation
    #[xml(attr = "w:docLocation")]
    pub doc_location: Option<String>,
    /// Add To Viewed Hyperlinks
    /// Represents the following attribute in the schema: w:history
    #[xml(attr = "w:history")]
    pub history: Option<bool>,
    /// Hyperlink Anchor
    /// Represents the following attribute in the schema: w:anchor
    #[xml(attr = "w:anchor")]
    pub anchor: Option<String>,
    /// Hyperlink Target
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<HyperlinkChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HyperlinkChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Defines the BidirectionalOverride Class.
/// When the object is serialized out as xml, it's qualified name is w:bdo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bdo")]
pub struct BidirectionalOverride {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: Option<DirectionValues>,
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<BidirectionalOverrideChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BidirectionalOverrideChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Defines the BidirectionalEmbedding Class.
/// When the object is serialized out as xml, it's qualified name is w:dir.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dir")]
pub struct BidirectionalEmbedding {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: Option<DirectionValues>,
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<BidirectionalEmbeddingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BidirectionalEmbeddingChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Anchor for Subdocument Location.
/// When the object is serialized out as xml, it's qualified name is w:subDoc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:subDoc")]
pub struct SubDocumentReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the PrinterSettingsReference Class.
/// When the object is serialized out as xml, it's qualified name is w:printerSettings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:printerSettings")]
pub struct PrinterSettingsReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// ODSO Data Source File Path.
/// When the object is serialized out as xml, it's qualified name is w:src.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:src")]
pub struct SourceReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Reference to Inclusion/Exclusion Data for Data Source.
/// When the object is serialized out as xml, it's qualified name is w:recipientData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:recipientData")]
pub struct RecipientDataReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Data Source File Path.
/// When the object is serialized out as xml, it's qualified name is w:dataSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dataSource")]
pub struct DataSourceReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Header Definition File Path.
/// When the object is serialized out as xml, it's qualified name is w:headerSource.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:headerSource")]
pub struct HeaderSource {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Source File for Frame.
/// When the object is serialized out as xml, it's qualified name is w:sourceFileName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sourceFileName")]
pub struct SourceFileReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the MovieReference Class.
/// When the object is serialized out as xml, it's qualified name is w:movie.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:movie")]
pub struct MovieReference {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Attached Document Template.
/// When the object is serialized out as xml, it's qualified name is w:attachedTemplate.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:attachedTemplate")]
pub struct AttachedTemplate {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the RelationshipType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RelationshipType {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the ConditionalFormatStyle Class.
/// When the object is serialized out as xml, it's qualified name is w:cnfStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cnfStyle")]
pub struct ConditionalFormatStyle {
    /// Conditional Formatting Bit Mask
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
    /// firstRow
    /// Represents the following attribute in the schema: w:firstRow
    #[xml(attr = "w:firstRow")]
    pub first_row: Option<bool>,
    /// lastRow
    /// Represents the following attribute in the schema: w:lastRow
    #[xml(attr = "w:lastRow")]
    pub last_row: Option<bool>,
    /// firstColumn
    /// Represents the following attribute in the schema: w:firstColumn
    #[xml(attr = "w:firstColumn")]
    pub first_column: Option<bool>,
    /// lastColumn
    /// Represents the following attribute in the schema: w:lastColumn
    #[xml(attr = "w:lastColumn")]
    pub last_column: Option<bool>,
    /// oddVBand
    /// Represents the following attribute in the schema: w:oddVBand
    #[xml(attr = "w:oddVBand")]
    pub odd_vertical_band: Option<bool>,
    /// evenVBand
    /// Represents the following attribute in the schema: w:evenVBand
    #[xml(attr = "w:evenVBand")]
    pub even_vertical_band: Option<bool>,
    /// oddHBand
    /// Represents the following attribute in the schema: w:oddHBand
    #[xml(attr = "w:oddHBand")]
    pub odd_horizontal_band: Option<bool>,
    /// evenHBand
    /// Represents the following attribute in the schema: w:evenHBand
    #[xml(attr = "w:evenHBand")]
    pub even_horizontal_band: Option<bool>,
    /// firstRowFirstColumn
    /// Represents the following attribute in the schema: w:firstRowFirstColumn
    #[xml(attr = "w:firstRowFirstColumn")]
    pub first_row_first_column: Option<bool>,
    /// firstRowLastColumn
    /// Represents the following attribute in the schema: w:firstRowLastColumn
    #[xml(attr = "w:firstRowLastColumn")]
    pub first_row_last_column: Option<bool>,
    /// lastRowFirstColumn
    /// Represents the following attribute in the schema: w:lastRowFirstColumn
    #[xml(attr = "w:lastRowFirstColumn")]
    pub last_row_first_column: Option<bool>,
    /// lastRowLastColumn
    /// Represents the following attribute in the schema: w:lastRowLastColumn
    #[xml(attr = "w:lastRowLastColumn")]
    pub last_row_last_column: Option<bool>,
}
/// Defines the TableCellWidth Class.
/// When the object is serialized out as xml, it's qualified name is w:tcW.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcW")]
pub struct TableCellWidth {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the WidthBeforeTableRow Class.
/// When the object is serialized out as xml, it's qualified name is w:wBefore.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:wBefore")]
pub struct WidthBeforeTableRow {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the WidthAfterTableRow Class.
/// When the object is serialized out as xml, it's qualified name is w:wAfter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:wAfter")]
pub struct WidthAfterTableRow {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableCellSpacing Class.
/// When the object is serialized out as xml, it's qualified name is w:tblCellSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblCellSpacing")]
pub struct TableCellSpacing {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableWidth Class.
/// When the object is serialized out as xml, it's qualified name is w:tblW.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblW")]
pub struct TableWidth {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Top Margin Default.
/// When the object is serialized out as xml, it's qualified name is w:top.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:top")]
pub struct TopMargin {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the StartMargin Class.
/// When the object is serialized out as xml, it's qualified name is w:start.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:start")]
pub struct StartMargin {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Bottom Margin Default.
/// When the object is serialized out as xml, it's qualified name is w:bottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bottom")]
pub struct BottomMargin {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the EndMargin Class.
/// When the object is serialized out as xml, it's qualified name is w:end.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:end")]
pub struct EndMargin {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Left Margin Exception.
/// When the object is serialized out as xml, it's qualified name is w:left.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:left")]
pub struct LeftMargin {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Table Cell Right Margin Exception.
/// When the object is serialized out as xml, it's qualified name is w:right.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:right")]
pub struct RightMargin {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableWidthType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TableWidthType {
    /// Table Width Value
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Table Width Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the HorizontalMerge Class.
/// When the object is serialized out as xml, it's qualified name is w:hMerge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hMerge")]
pub struct HorizontalMerge {
    /// Horizontal Merge Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<MergedCellValues>,
}
/// Defines the VerticalMerge Class.
/// When the object is serialized out as xml, it's qualified name is w:vMerge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:vMerge")]
pub struct VerticalMerge {
    /// Vertical Merge Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<MergedCellValues>,
}
/// Defines the TableCellBorders Class.
/// When the object is serialized out as xml, it's qualified name is w:tcBorders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcBorders")]
pub struct TableCellBorders {
    ///Table Cell Top Border
    #[xml(child = "w:top")]
    pub top_border: Option<TopBorder>,
    ///Table Cell Left Border
    #[xml(child = "w:left")]
    pub left_border: Option<LeftBorder>,
    /// _
    #[xml(child = "w:start")]
    pub start_border: Option<StartBorder>,
    ///Table Cell Bottom Border
    #[xml(child = "w:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Table Cell Right Border
    #[xml(child = "w:right")]
    pub right_border: Option<RightBorder>,
    /// _
    #[xml(child = "w:end")]
    pub end_border: Option<EndBorder>,
    ///Table Cell Inside Horizontal Edges Border
    #[xml(child = "w:insideH")]
    pub inside_horizontal_border: Option<InsideHorizontalBorder>,
    ///Table Cell Inside Vertical Edges Border
    #[xml(child = "w:insideV")]
    pub inside_vertical_border: Option<InsideVerticalBorder>,
    ///Table Cell Top Left to Bottom Right Diagonal Border
    #[xml(child = "w:tl2br")]
    pub top_left_to_bottom_right_cell_border: Option<TopLeftToBottomRightCellBorder>,
    ///Table Cell Top Right to Bottom Left Diagonal Border
    #[xml(child = "w:tr2bl")]
    pub top_right_to_bottom_left_cell_border: Option<TopRightToBottomLeftCellBorder>,
}
/// Defines the NoWrap Class.
/// When the object is serialized out as xml, it's qualified name is w:noWrap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noWrap")]
pub struct NoWrap {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the TableCellFitText Class.
/// When the object is serialized out as xml, it's qualified name is w:tcFitText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcFitText")]
pub struct TableCellFitText {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the HideMark Class.
/// When the object is serialized out as xml, it's qualified name is w:hideMark.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hideMark")]
pub struct HideMark {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the CantSplit Class.
/// When the object is serialized out as xml, it's qualified name is w:cantSplit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cantSplit")]
pub struct CantSplit {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the TableHeader Class.
/// When the object is serialized out as xml, it's qualified name is w:tblHeader.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblHeader")]
pub struct TableHeader {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the BiDiVisual Class.
/// When the object is serialized out as xml, it's qualified name is w:bidiVisual.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bidiVisual")]
pub struct BiDiVisual {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Frame Cannot Be Resized.
/// When the object is serialized out as xml, it's qualified name is w:noResizeAllowed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noResizeAllowed")]
pub struct NoResizeAllowed {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Maintain Link to Existing File.
/// When the object is serialized out as xml, it's qualified name is w:linkedToFile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:linkedToFile")]
pub struct LinkedToFile {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Do Not Display Frameset Splitters.
/// When the object is serialized out as xml, it's qualified name is w:noBorder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noBorder")]
pub struct NoBorder {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Frameset Splitter Border Style.
/// When the object is serialized out as xml, it's qualified name is w:flatBorders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:flatBorders")]
pub struct FlatBorders {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Automatically Merge User Formatting Into Style Definition.
/// When the object is serialized out as xml, it's qualified name is w:autoRedefine.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoRedefine")]
pub struct AutoRedefine {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Hide Style From User Interface.
/// When the object is serialized out as xml, it's qualified name is w:hidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hidden")]
pub struct StyleHidden {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Hide Style From Main User Interface.
/// When the object is serialized out as xml, it's qualified name is w:semiHidden.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:semiHidden")]
pub struct SemiHidden {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Remove Semi-Hidden Property When Style Is Used.
/// When the object is serialized out as xml, it's qualified name is w:unhideWhenUsed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:unhideWhenUsed")]
pub struct UnhideWhenUsed {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Primary Style.
/// When the object is serialized out as xml, it's qualified name is w:qFormat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:qFormat")]
pub struct PrimaryStyle {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Style Cannot Be Applied.
/// When the object is serialized out as xml, it's qualified name is w:locked.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:locked")]
pub struct Locked {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// E-Mail Message Text Style.
/// When the object is serialized out as xml, it's qualified name is w:personal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:personal")]
pub struct Personal {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// E-Mail Message Composition Style.
/// When the object is serialized out as xml, it's qualified name is w:personalCompose.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:personalCompose")]
pub struct PersonalCompose {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// E-Mail Message Reply Style.
/// When the object is serialized out as xml, it's qualified name is w:personalReply.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:personalReply")]
pub struct PersonalReply {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the OnOffOnlyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OnOffOnlyType {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<OnOffOnlyValues>,
}
/// Defines the TableCellMargin Class.
/// When the object is serialized out as xml, it's qualified name is w:tcMar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcMar")]
pub struct TableCellMargin {
    ///Table Cell Top Margin Exception
    #[xml(child = "w:top")]
    pub top_margin: Option<TopMargin>,
    ///Table Cell Left Margin Exception
    #[xml(child = "w:left")]
    pub left_margin: Option<LeftMargin>,
    /// _
    #[xml(child = "w:start")]
    pub start_margin: Option<StartMargin>,
    ///Table Cell Bottom Margin Exception
    #[xml(child = "w:bottom")]
    pub bottom_margin: Option<BottomMargin>,
    ///Table Cell Right Margin Exception
    #[xml(child = "w:right")]
    pub right_margin: Option<RightMargin>,
    /// _
    #[xml(child = "w:end")]
    pub end_margin: Option<EndMargin>,
}
/// Defines the TableCellVerticalAlignment Class.
/// When the object is serialized out as xml, it's qualified name is w:vAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:vAlign")]
pub struct TableCellVerticalAlignment {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TableVerticalAlignmentValues,
}
/// Defines the DivId Class.
/// When the object is serialized out as xml, it's qualified name is w:divId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:divId")]
pub struct DivId {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the TableRowHeight Class.
/// When the object is serialized out as xml, it's qualified name is w:trHeight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:trHeight")]
pub struct TableRowHeight {
    /// Table Row Height
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<i32>,
    /// Table Row Height Type
    /// Represents the following attribute in the schema: w:hRule
    #[xml(attr = "w:hRule")]
    pub height_type: Option<HeightRuleValues>,
}
/// Defines the TableJustification Class.
/// When the object is serialized out as xml, it's qualified name is w:jc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:jc")]
pub struct TableJustification {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TableRowAlignmentValues,
}
/// Defines the TablePositionProperties Class.
/// When the object is serialized out as xml, it's qualified name is w:tblpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblpPr")]
pub struct TablePositionProperties {
    /// Distance From Left of Table to Text
    /// Represents the following attribute in the schema: w:leftFromText
    #[xml(attr = "w:leftFromText")]
    pub left_from_text: Option<i16>,
    /// (Distance From Right of Table to Text
    /// Represents the following attribute in the schema: w:rightFromText
    #[xml(attr = "w:rightFromText")]
    pub right_from_text: Option<i16>,
    /// Distance From Top of Table to Text
    /// Represents the following attribute in the schema: w:topFromText
    #[xml(attr = "w:topFromText")]
    pub top_from_text: Option<i16>,
    /// Distance From Bottom of Table to Text
    /// Represents the following attribute in the schema: w:bottomFromText
    #[xml(attr = "w:bottomFromText")]
    pub bottom_from_text: Option<i16>,
    /// Table Vertical Anchor
    /// Represents the following attribute in the schema: w:vertAnchor
    #[xml(attr = "w:vertAnchor")]
    pub vertical_anchor: Option<VerticalAnchorValues>,
    /// Table Horizontal Anchor
    /// Represents the following attribute in the schema: w:horzAnchor
    #[xml(attr = "w:horzAnchor")]
    pub horizontal_anchor: Option<HorizontalAnchorValues>,
    /// Relative Horizontal Alignment From Anchor
    /// Represents the following attribute in the schema: w:tblpXSpec
    #[xml(attr = "w:tblpXSpec")]
    pub table_position_x_alignment: Option<HorizontalAlignmentValues>,
    /// Absolute Horizontal Distance From Anchor
    /// Represents the following attribute in the schema: w:tblpX
    #[xml(attr = "w:tblpX")]
    pub table_position_x: Option<i32>,
    /// Relative Vertical Alignment from Anchor
    /// Represents the following attribute in the schema: w:tblpYSpec
    #[xml(attr = "w:tblpYSpec")]
    pub table_position_y_alignment: Option<VerticalAlignmentValues>,
    /// Absolute Vertical Distance From Anchor
    /// Represents the following attribute in the schema: w:tblpY
    #[xml(attr = "w:tblpY")]
    pub table_position_y: Option<i32>,
}
/// Defines the TableOverlap Class.
/// When the object is serialized out as xml, it's qualified name is w:tblOverlap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblOverlap")]
pub struct TableOverlap {
    /// Floating Table Overlap Setting
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TableOverlapValues,
}
/// Defines the TableStyleRowBandSize Class.
/// When the object is serialized out as xml, it's qualified name is w:tblStyleRowBandSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblStyleRowBandSize")]
pub struct TableStyleRowBandSize {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the TableStyleColumnBandSize Class.
/// When the object is serialized out as xml, it's qualified name is w:tblStyleColBandSize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblStyleColBandSize")]
pub struct TableStyleColumnBandSize {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the UnsignedDecimalNumberMax3Type Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UnsignedDecimalNumberMax3Type {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the TableIndentation Class.
/// When the object is serialized out as xml, it's qualified name is w:tblInd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblInd")]
pub struct TableIndentation {
    /// w
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<i32>,
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableWidthUnitValues>,
}
/// Defines the TableBorders Class.
/// When the object is serialized out as xml, it's qualified name is w:tblBorders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblBorders")]
pub struct TableBorders {
    ///Table Top Border
    #[xml(child = "w:top")]
    pub top_border: Option<TopBorder>,
    ///Table Left Border
    #[xml(child = "w:left")]
    pub left_border: Option<LeftBorder>,
    /// _
    #[xml(child = "w:start")]
    pub start_border: Option<StartBorder>,
    ///Table Bottom Border
    #[xml(child = "w:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Table Right Border
    #[xml(child = "w:right")]
    pub right_border: Option<RightBorder>,
    /// _
    #[xml(child = "w:end")]
    pub end_border: Option<EndBorder>,
    ///Table Inside Horizontal Edges Border
    #[xml(child = "w:insideH")]
    pub inside_horizontal_border: Option<InsideHorizontalBorder>,
    ///Table Inside Vertical Edges Border
    #[xml(child = "w:insideV")]
    pub inside_vertical_border: Option<InsideVerticalBorder>,
}
/// Defines the TableLayout Class.
/// When the object is serialized out as xml, it's qualified name is w:tblLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblLayout")]
pub struct TableLayout {
    /// Table Layout Setting
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<TableLayoutValues>,
}
/// Defines the TableCellMarginDefault Class.
/// When the object is serialized out as xml, it's qualified name is w:tblCellMar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblCellMar")]
pub struct TableCellMarginDefault {
    ///Table Cell Top Margin Default
    #[xml(child = "w:top")]
    pub top_margin: Option<TopMargin>,
    ///Table Cell Left Margin Default
    #[xml(child = "w:left")]
    pub table_cell_left_margin: Option<TableCellLeftMargin>,
    /// _
    #[xml(child = "w:start")]
    pub start_margin: Option<StartMargin>,
    ///Table Cell Bottom Margin Default
    #[xml(child = "w:bottom")]
    pub bottom_margin: Option<BottomMargin>,
    ///Table Cell Right Margin Default
    #[xml(child = "w:right")]
    pub table_cell_right_margin: Option<TableCellRightMargin>,
    /// _
    #[xml(child = "w:end")]
    pub end_margin: Option<EndMargin>,
}
/// Footnote and Endnote Numbering Starting Value.
/// When the object is serialized out as xml, it's qualified name is w:numStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numStart")]
pub struct NumberingStart {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Footnote and Endnote Numbering Restart Location.
/// When the object is serialized out as xml, it's qualified name is w:numRestart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numRestart")]
pub struct NumberingRestart {
    /// Automatic Numbering Restart Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: RestartNumberValues,
}
/// Defines the AltChunk Class.
/// When the object is serialized out as xml, it's qualified name is w:altChunk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:altChunk")]
pub struct AltChunk {
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    ///External Content Import Properties
    #[xml(child = "w:altChunkPr")]
    pub alt_chunk_properties: Option<AltChunkProperties>,
}
/// Defines the TableLook Class.
/// When the object is serialized out as xml, it's qualified name is w:tblLook.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblLook")]
pub struct TableLook {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: Option<String>,
    /// firstRow
    /// Represents the following attribute in the schema: w:firstRow
    #[xml(attr = "w:firstRow")]
    pub first_row: Option<bool>,
    /// lastRow
    /// Represents the following attribute in the schema: w:lastRow
    #[xml(attr = "w:lastRow")]
    pub last_row: Option<bool>,
    /// firstColumn
    /// Represents the following attribute in the schema: w:firstColumn
    #[xml(attr = "w:firstColumn")]
    pub first_column: Option<bool>,
    /// lastColumn
    /// Represents the following attribute in the schema: w:lastColumn
    #[xml(attr = "w:lastColumn")]
    pub last_column: Option<bool>,
    /// noHBand
    /// Represents the following attribute in the schema: w:noHBand
    #[xml(attr = "w:noHBand")]
    pub no_horizontal_band: Option<bool>,
    /// noVBand
    /// Represents the following attribute in the schema: w:noVBand
    #[xml(attr = "w:noVBand")]
    pub no_vertical_band: Option<bool>,
}
/// Defines the FootnoteProperties Class.
/// When the object is serialized out as xml, it's qualified name is w:footnotePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnotePr")]
pub struct FootnoteProperties {
    #[xml(
        child = "w:pos",
        child = "w:numFmt",
        child = "w:numStart",
        child = "w:numRestart",
    )]
    pub children: Vec<FootnotePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FootnotePropertiesChildChoice {
    #[xml(tag = "w:pos")]
    WPos(FootnotePosition),
    #[xml(tag = "w:numFmt")]
    WNumFmt(NumberingFormat),
    #[xml(tag = "w:numStart")]
    WNumStart(NumberingStart),
    #[xml(tag = "w:numRestart")]
    WNumRestart(NumberingRestart),
}
/// Defines the EndnoteProperties Class.
/// When the object is serialized out as xml, it's qualified name is w:endnotePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnotePr")]
pub struct EndnoteProperties {
    #[xml(
        child = "w:pos",
        child = "w:numFmt",
        child = "w:numStart",
        child = "w:numRestart",
    )]
    pub children: Vec<EndnotePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndnotePropertiesChildChoice {
    #[xml(tag = "w:pos")]
    WPos(EndnotePosition),
    #[xml(tag = "w:numFmt")]
    WNumFmt(NumberingFormat),
    #[xml(tag = "w:numStart")]
    WNumStart(NumberingStart),
    #[xml(tag = "w:numRestart")]
    WNumRestart(NumberingRestart),
}
/// Defines the SectionType Class.
/// When the object is serialized out as xml, it's qualified name is w:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:type")]
pub struct SectionType {
    /// Section Type Setting
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: SectionMarkValues,
}
/// Defines the PageSize Class.
/// When the object is serialized out as xml, it's qualified name is w:pgSz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pgSz")]
pub struct PageSize {
    /// Page Width
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<i32>,
    /// Page Height
    /// Represents the following attribute in the schema: w:h
    #[xml(attr = "w:h")]
    pub height: Option<i32>,
    /// Page Orientation
    /// Represents the following attribute in the schema: w:orient
    #[xml(attr = "w:orient")]
    pub orient: Option<PageOrientationValues>,
    /// Printer Paper Code
    /// Represents the following attribute in the schema: w:code
    #[xml(attr = "w:code")]
    pub code: Option<i16>,
}
/// Defines the PageMargin Class.
/// When the object is serialized out as xml, it's qualified name is w:pgMar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pgMar")]
pub struct PageMargin {
    /// Top Margin Spacing
    /// Represents the following attribute in the schema: w:top
    #[xml(attr = "w:top")]
    pub top: Option<i32>,
    /// Right Margin Spacing
    /// Represents the following attribute in the schema: w:right
    #[xml(attr = "w:right")]
    pub right: Option<i32>,
    /// Page Bottom Spacing
    /// Represents the following attribute in the schema: w:bottom
    #[xml(attr = "w:bottom")]
    pub bottom: Option<i32>,
    /// Left Margin Spacing
    /// Represents the following attribute in the schema: w:left
    #[xml(attr = "w:left")]
    pub left: Option<i32>,
    /// Spacing to Top of Header
    /// Represents the following attribute in the schema: w:header
    #[xml(attr = "w:header")]
    pub header: Option<i32>,
    /// Spacing to Bottom of Footer
    /// Represents the following attribute in the schema: w:footer
    #[xml(attr = "w:footer")]
    pub footer: Option<i32>,
    /// Page Gutter Spacing
    /// Represents the following attribute in the schema: w:gutter
    #[xml(attr = "w:gutter")]
    pub gutter: Option<i32>,
}
/// Defines the PaperSource Class.
/// When the object is serialized out as xml, it's qualified name is w:paperSrc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:paperSrc")]
pub struct PaperSource {
    /// First Page Printer Tray Code
    /// Represents the following attribute in the schema: w:first
    #[xml(attr = "w:first")]
    pub first: Option<i16>,
    /// Non-First Page Printer Tray Code
    /// Represents the following attribute in the schema: w:other
    #[xml(attr = "w:other")]
    pub other: Option<i16>,
}
/// Defines the PageBorders Class.
/// When the object is serialized out as xml, it's qualified name is w:pgBorders.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pgBorders")]
pub struct PageBorders {
    /// Z-Ordering of Page Border
    /// Represents the following attribute in the schema: w:zOrder
    #[xml(attr = "w:zOrder")]
    pub z_order: Option<PageBorderZOrderValues>,
    /// Pages to Display Page Borders
    /// Represents the following attribute in the schema: w:display
    #[xml(attr = "w:display")]
    pub display: Option<PageBorderDisplayValues>,
    /// Page Border Positioning
    /// Represents the following attribute in the schema: w:offsetFrom
    #[xml(attr = "w:offsetFrom")]
    pub offset_from: Option<PageBorderOffsetValues>,
    ///Top Border
    #[xml(child = "w:top")]
    pub top_border: Option<TopBorder>,
    ///Left Border
    #[xml(child = "w:left")]
    pub left_border: Option<LeftBorder>,
    ///Bottom Border
    #[xml(child = "w:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Right Border
    #[xml(child = "w:right")]
    pub right_border: Option<RightBorder>,
}
/// Defines the LineNumberType Class.
/// When the object is serialized out as xml, it's qualified name is w:lnNumType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lnNumType")]
pub struct LineNumberType {
    /// Line Number Increments to Display
    /// Represents the following attribute in the schema: w:countBy
    #[xml(attr = "w:countBy")]
    pub count_by: Option<i16>,
    /// Line Numbering Starting Value
    /// Represents the following attribute in the schema: w:start
    #[xml(attr = "w:start")]
    pub start: Option<i16>,
    /// Distance Between Text and Line Numbering
    /// Represents the following attribute in the schema: w:distance
    #[xml(attr = "w:distance")]
    pub distance: Option<String>,
    /// Line Numbering Restart Setting
    /// Represents the following attribute in the schema: w:restart
    #[xml(attr = "w:restart")]
    pub restart: Option<LineNumberRestartValues>,
}
/// Defines the PageNumberType Class.
/// When the object is serialized out as xml, it's qualified name is w:pgNumType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pgNumType")]
pub struct PageNumberType {
    /// Page Number Format
    /// Represents the following attribute in the schema: w:fmt
    #[xml(attr = "w:fmt")]
    pub format: Option<NumberFormatValues>,
    /// Starting Page Number
    /// Represents the following attribute in the schema: w:start
    #[xml(attr = "w:start")]
    pub start: Option<i32>,
    /// Chapter Heading Style
    /// Represents the following attribute in the schema: w:chapStyle
    #[xml(attr = "w:chapStyle")]
    pub chapter_style: Option<u8>,
    /// Chapter Separator Character
    /// Represents the following attribute in the schema: w:chapSep
    #[xml(attr = "w:chapSep")]
    pub chapter_separator: Option<ChapterSeparatorValues>,
}
/// Defines the Columns Class.
/// When the object is serialized out as xml, it's qualified name is w:cols.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:cols")]
pub struct Columns {
    /// Equal Column Widths
    /// Represents the following attribute in the schema: w:equalWidth
    #[xml(attr = "w:equalWidth")]
    pub equal_width: Option<bool>,
    /// Spacing Between Equal Width Columns
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<String>,
    /// Number of Equal Width Columns
    /// Represents the following attribute in the schema: w:num
    #[xml(attr = "w:num")]
    pub column_count: Option<i16>,
    /// Draw Line Between Columns
    /// Represents the following attribute in the schema: w:sep
    #[xml(attr = "w:sep")]
    pub separator: Option<bool>,
    /// _
    #[xml(child = "w:col")]
    pub w_col: Vec<Column>,
}
/// Defines the VerticalTextAlignmentOnPage Class.
/// When the object is serialized out as xml, it's qualified name is w:vAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:vAlign")]
pub struct VerticalTextAlignmentOnPage {
    /// Vertical Alignment Setting
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: VerticalJustificationValues,
}
/// Defines the DocGrid Class.
/// When the object is serialized out as xml, it's qualified name is w:docGrid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docGrid")]
pub struct DocGrid {
    /// Document Grid Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<DocGridValues>,
    /// Document Grid Line Pitch
    /// Represents the following attribute in the schema: w:linePitch
    #[xml(attr = "w:linePitch")]
    pub line_pitch: Option<i32>,
    /// Document Grid Character Pitch
    /// Represents the following attribute in the schema: w:charSpace
    #[xml(attr = "w:charSpace")]
    pub character_space: Option<i32>,
}
/// Inclusion/Exclusion Data for Data Source.
/// When the object is serialized out as xml, it's qualified name is w:recipients.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:recipients")]
pub struct Recipients {
    #[xml(attr = "xmlns", with = "recipients_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:recipientData")]
    pub w_recipient_data: Vec<RecipientData>,
}
mod recipients_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Rich Text Box Content Container.
/// When the object is serialized out as xml, it's qualified name is w:txbxContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:txbxContent")]
pub struct TextBoxContent {
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<TextBoxContentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextBoxContentChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Comments Collection.
/// When the object is serialized out as xml, it's qualified name is w:comments.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:comments")]
pub struct Comments {
    #[xml(attr = "xmlns", with = "comments_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:comment")]
    pub w_comment: Vec<Comment>,
}
mod comments_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Document Footnotes.
/// When the object is serialized out as xml, it's qualified name is w:footnotes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnotes")]
pub struct Footnotes {
    #[xml(attr = "xmlns", with = "footnotes_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:footnote")]
    pub w_footnote: Option<Footnote>,
}
mod footnotes_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Document Endnotes.
/// When the object is serialized out as xml, it's qualified name is w:endnotes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnotes")]
pub struct Endnotes {
    #[xml(attr = "xmlns", with = "endnotes_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:endnote")]
    pub w_endnote: Option<Endnote>,
}
mod endnotes_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Header.
/// When the object is serialized out as xml, it's qualified name is w:hdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hdr")]
pub struct Header {
    #[xml(attr = "xmlns", with = "header_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<HeaderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HeaderChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
mod header_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Footer.
/// When the object is serialized out as xml, it's qualified name is w:ftr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ftr")]
pub struct Footer {
    #[xml(attr = "xmlns", with = "footer_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<FooterChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FooterChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
mod footer_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Defines the HeaderFooterType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct HeaderFooterType {
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<HeaderFooterTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HeaderFooterTypeChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Document Settings.
/// When the object is serialized out as xml, it's qualified name is w:settings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:settings")]
pub struct Settings {
    #[xml(attr = "xmlns", with = "settings_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Write Protection
    #[xml(child = "w:writeProtection")]
    pub write_protection: Option<WriteProtection>,
    ///Document View Setting
    #[xml(child = "w:view")]
    pub view: Option<View>,
    ///Magnification Setting
    #[xml(child = "w:zoom")]
    pub zoom: Option<Zoom>,
    ///Remove Personal Information from Document Properties
    #[xml(child = "w:removePersonalInformation")]
    pub remove_personal_information: Option<RemovePersonalInformation>,
    ///Remove Date and Time from Annotations
    #[xml(child = "w:removeDateAndTime")]
    pub remove_date_and_time: Option<RemoveDateAndTime>,
    ///Do Not Display Visual Boundary For Header/Footer or Between Pages
    #[xml(child = "w:doNotDisplayPageBoundaries")]
    pub do_not_display_page_boundaries: Option<DoNotDisplayPageBoundaries>,
    ///Display Background Objects When Displaying Document
    #[xml(child = "w:displayBackgroundShape")]
    pub display_background_shape: Option<DisplayBackgroundShape>,
    ///Print PostScript Codes With Document Text
    #[xml(child = "w:printPostScriptOverText")]
    pub print_post_script_over_text: Option<PrintPostScriptOverText>,
    ///Print Fractional Character Widths
    #[xml(child = "w:printFractionalCharacterWidth")]
    pub print_fractional_character_width: Option<PrintFractionalCharacterWidth>,
    ///Only Print Form Field Content
    #[xml(child = "w:printFormsData")]
    pub print_forms_data: Option<PrintFormsData>,
    ///Embed TrueType Fonts
    #[xml(child = "w:embedTrueTypeFonts")]
    pub embed_true_type_fonts: Option<EmbedTrueTypeFonts>,
    ///Embed Common System Fonts
    #[xml(child = "w:embedSystemFonts")]
    pub embed_system_fonts: Option<EmbedSystemFonts>,
    ///Subset Fonts When Embedding
    #[xml(child = "w:saveSubsetFonts")]
    pub save_subset_fonts: Option<SaveSubsetFonts>,
    ///Only Save Form Field Content
    #[xml(child = "w:saveFormsData")]
    pub save_forms_data: Option<SaveFormsData>,
    ///Mirror Page Margins
    #[xml(child = "w:mirrorMargins")]
    pub mirror_margins: Option<MirrorMargins>,
    ///Align Paragraph and Table Borders with Page Border
    #[xml(child = "w:alignBordersAndEdges")]
    pub align_border_and_edges: Option<AlignBorderAndEdges>,
    ///Page Border Excludes Header
    #[xml(child = "w:bordersDoNotSurroundHeader")]
    pub borders_do_not_surround_header: Option<BordersDoNotSurroundHeader>,
    ///Page Border Excludes Footer
    #[xml(child = "w:bordersDoNotSurroundFooter")]
    pub borders_do_not_surround_footer: Option<BordersDoNotSurroundFooter>,
    ///Position Gutter At Top of Page
    #[xml(child = "w:gutterAtTop")]
    pub gutter_at_top: Option<GutterAtTop>,
    ///Do Not Display Visual Indication of Spelling Errors
    #[xml(child = "w:hideSpellingErrors")]
    pub hide_spelling_errors: Option<HideSpellingErrors>,
    ///Do Not Display Visual Indication of Grammatical Errors
    #[xml(child = "w:hideGrammaticalErrors")]
    pub hide_grammatical_errors: Option<HideGrammaticalErrors>,
    /// _
    #[xml(child = "w:activeWritingStyle")]
    pub w_active_writing_style: Vec<ActiveWritingStyle>,
    /// _
    #[xml(child = "w:proofState")]
    pub w_proof_state: Option<ProofState>,
    /// _
    #[xml(child = "w:formsDesign")]
    pub w_forms_design: Option<FormsDesign>,
    /// _
    #[xml(child = "w:attachedTemplate")]
    pub w_attached_template: Option<AttachedTemplate>,
    /// _
    #[xml(child = "w:linkStyles")]
    pub w_link_styles: Option<LinkStyles>,
    /// _
    #[xml(child = "w:stylePaneFormatFilter")]
    pub w_style_pane_format_filter: Option<StylePaneFormatFilter>,
    /// _
    #[xml(child = "w:stylePaneSortMethod")]
    pub w_style_pane_sort_method: Option<StylePaneSortMethods>,
    /// _
    #[xml(child = "w:documentType")]
    pub w_document_type: Option<DocumentType>,
    /// _
    #[xml(child = "w:mailMerge")]
    pub w_mail_merge: Option<MailMerge>,
    /// _
    #[xml(child = "w:revisionView")]
    pub w_revision_view: Option<RevisionView>,
    /// _
    #[xml(child = "w:trackRevisions")]
    pub w_track_revisions: Option<TrackRevisions>,
    /// _
    #[xml(child = "w:doNotTrackMoves")]
    pub w_do_not_track_moves: Option<DoNotTrackMoves>,
    /// _
    #[xml(child = "w:doNotTrackFormatting")]
    pub w_do_not_track_formatting: Option<DoNotTrackFormatting>,
    /// _
    #[xml(child = "w:documentProtection")]
    pub w_document_protection: Option<DocumentProtection>,
    /// _
    #[xml(child = "w:autoFormatOverride")]
    pub w_auto_format_override: Option<AutoFormatOverride>,
    /// _
    #[xml(child = "w:styleLockTheme")]
    pub w_style_lock_theme: Option<StyleLockThemesPart>,
    /// _
    #[xml(child = "w:styleLockQFSet")]
    pub w_style_lock_qf_set: Option<StyleLockStylesPart>,
    /// _
    #[xml(child = "w:defaultTabStop")]
    pub w_default_tab_stop: Option<DefaultTabStop>,
    /// _
    #[xml(child = "w:autoHyphenation")]
    pub w_auto_hyphenation: Option<AutoHyphenation>,
    /// _
    #[xml(child = "w:consecutiveHyphenLimit")]
    pub w_consecutive_hyphen_limit: Option<ConsecutiveHyphenLimit>,
    /// _
    #[xml(child = "w:hyphenationZone")]
    pub w_hyphenation_zone: Option<HyphenationZone>,
    /// _
    #[xml(child = "w:doNotHyphenateCaps")]
    pub w_do_not_hyphenate_caps: Option<DoNotHyphenateCaps>,
    /// _
    #[xml(child = "w:showEnvelope")]
    pub w_show_envelope: Option<ShowEnvelope>,
    /// _
    #[xml(child = "w:summaryLength")]
    pub w_summary_length: Option<SummaryLength>,
    /// _
    #[xml(child = "w:clickAndTypeStyle")]
    pub w_click_and_type_style: Option<ClickAndTypeStyle>,
    /// _
    #[xml(child = "w:defaultTableStyle")]
    pub w_default_table_style: Option<DefaultTableStyle>,
    /// _
    #[xml(child = "w:evenAndOddHeaders")]
    pub w_even_and_odd_headers: Option<EvenAndOddHeaders>,
    /// _
    #[xml(child = "w:bookFoldRevPrinting")]
    pub w_book_fold_rev_printing: Option<BookFoldReversePrinting>,
    /// _
    #[xml(child = "w:bookFoldPrinting")]
    pub w_book_fold_printing: Option<BookFoldPrinting>,
    /// _
    #[xml(child = "w:bookFoldPrintingSheets")]
    pub w_book_fold_printing_sheets: Option<BookFoldPrintingSheets>,
    /// _
    #[xml(child = "w:drawingGridHorizontalSpacing")]
    pub w_drawing_grid_horizontal_spacing: Option<DrawingGridHorizontalSpacing>,
    /// _
    #[xml(child = "w:drawingGridVerticalSpacing")]
    pub w_drawing_grid_vertical_spacing: Option<DrawingGridVerticalSpacing>,
    /// _
    #[xml(child = "w:displayHorizontalDrawingGridEvery")]
    pub w_display_horizontal_drawing_grid_every: Option<DisplayHorizontalDrawingGrid>,
    /// _
    #[xml(child = "w:displayVerticalDrawingGridEvery")]
    pub w_display_vertical_drawing_grid_every: Option<DisplayVerticalDrawingGrid>,
    /// _
    #[xml(child = "w:doNotUseMarginsForDrawingGridOrigin")]
    pub w_do_not_use_margins_for_drawing_grid_origin: Option<
        DoNotUseMarginsForDrawingGridOrigin,
    >,
    /// _
    #[xml(child = "w:drawingGridHorizontalOrigin")]
    pub w_drawing_grid_horizontal_origin: Option<DrawingGridHorizontalOrigin>,
    /// _
    #[xml(child = "w:drawingGridVerticalOrigin")]
    pub w_drawing_grid_vertical_origin: Option<DrawingGridVerticalOrigin>,
    /// _
    #[xml(child = "w:doNotShadeFormData")]
    pub w_do_not_shade_form_data: Option<DoNotShadeFormData>,
    /// _
    #[xml(child = "w:noPunctuationKerning")]
    pub w_no_punctuation_kerning: Option<NoPunctuationKerning>,
    /// _
    #[xml(child = "w:characterSpacingControl")]
    pub w_character_spacing_control: Option<CharacterSpacingControl>,
    /// _
    #[xml(child = "w:printTwoOnOne")]
    pub w_print_two_on_one: Option<PrintTwoOnOne>,
    /// _
    #[xml(child = "w:strictFirstAndLastChars")]
    pub w_strict_first_and_last_chars: Option<StrictFirstAndLastChars>,
    /// _
    #[xml(child = "w:noLineBreaksAfter")]
    pub w_no_line_breaks_after: Option<NoLineBreaksAfterKinsoku>,
    /// _
    #[xml(child = "w:noLineBreaksBefore")]
    pub w_no_line_breaks_before: Option<NoLineBreaksBeforeKinsoku>,
    /// _
    #[xml(child = "w:savePreviewPicture")]
    pub w_save_preview_picture: Option<SavePreviewPicture>,
    /// _
    #[xml(child = "w:doNotValidateAgainstSchema")]
    pub w_do_not_validate_against_schema: Option<DoNotValidateAgainstSchema>,
    /// _
    #[xml(child = "w:saveInvalidXml")]
    pub w_save_invalid_xml: Option<SaveInvalidXml>,
    /// _
    #[xml(child = "w:ignoreMixedContent")]
    pub w_ignore_mixed_content: Option<IgnoreMixedContent>,
    /// _
    #[xml(child = "w:alwaysShowPlaceholderText")]
    pub w_always_show_placeholder_text: Option<AlwaysShowPlaceholderText>,
    /// _
    #[xml(child = "w:doNotDemarcateInvalidXml")]
    pub w_do_not_demarcate_invalid_xml: Option<DoNotDemarcateInvalidXml>,
    /// _
    #[xml(child = "w:saveXmlDataOnly")]
    pub w_save_xml_data_only: Option<SaveXmlDataOnly>,
    /// _
    #[xml(child = "w:useXSLTWhenSaving")]
    pub w_use_xslt_when_saving: Option<UseXsltWhenSaving>,
    /// _
    #[xml(child = "w:saveThroughXslt")]
    pub w_save_through_xslt: Option<SaveThroughXslt>,
    /// _
    #[xml(child = "w:showXMLTags")]
    pub w_show_xml_tags: Option<ShowXmlTags>,
    /// _
    #[xml(child = "w:alwaysMergeEmptyNamespace")]
    pub w_always_merge_empty_namespace: Option<AlwaysMergeEmptyNamespace>,
    /// _
    #[xml(child = "w:updateFields")]
    pub w_update_fields: Option<UpdateFieldsOnOpen>,
    /// _
    #[xml(child = "w:hdrShapeDefaults")]
    pub w_hdr_shape_defaults: Option<HeaderShapeDefaults>,
    /// _
    #[xml(child = "w:footnotePr")]
    pub w_footnote_pr: Option<FootnoteDocumentWideProperties>,
    /// _
    #[xml(child = "w:endnotePr")]
    pub w_endnote_pr: Option<EndnoteDocumentWideProperties>,
    /// _
    #[xml(child = "w:compat")]
    pub w_compat: Option<Compatibility>,
    /// _
    #[xml(child = "w:docVars")]
    pub w_doc_vars: Option<DocumentVariables>,
    /// _
    #[xml(child = "w:rsids")]
    pub w_rsids: Option<Rsids>,
    /// _
    #[xml(child = "m:mathPr")]
    pub m_math_pr: Option<
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathProperties,
    >,
    /// _
    #[xml(child = "w:uiCompat97To2003")]
    pub w_ui_compat97_to2003: Option<UiCompatibleWith97To2003>,
    /// _
    #[xml(child = "w:attachedSchema")]
    pub w_attached_schema: Vec<AttachedSchema>,
    /// _
    #[xml(child = "w:themeFontLang")]
    pub w_theme_font_lang: Option<ThemeFontLanguages>,
    /// _
    #[xml(child = "w:clrSchemeMapping")]
    pub w_clr_scheme_mapping: Option<ColorSchemeMapping>,
    /// _
    #[xml(child = "w:doNotIncludeSubdocsInStats")]
    pub w_do_not_include_subdocs_in_stats: Option<DoNotIncludeSubdocsInStats>,
    /// _
    #[xml(child = "w:doNotAutoCompressPictures")]
    pub w_do_not_auto_compress_pictures: Option<DoNotAutoCompressPictures>,
    /// _
    #[xml(child = "w:forceUpgrade")]
    pub w_force_upgrade: Option<ForceUpgrade>,
    /// _
    #[xml(child = "w:captions")]
    pub w_captions: Option<Captions>,
    /// _
    #[xml(child = "w:readModeInkLockDown")]
    pub w_read_mode_ink_lock_down: Option<ReadModeInkLockDown>,
    /// _
    #[xml(child = "sl:schemaLibrary")]
    pub sl_schema_library: Option<
        crate::schemas::schemas_openxmlformats_org_schema_library_2006_main::SchemaLibrary,
    >,
    /// _
    #[xml(child = "w:shapeDefaults")]
    pub w_shape_defaults: Option<ShapeDefaults>,
    /// _
    #[xml(child = "w:decimalSymbol")]
    pub w_decimal_symbol: Option<DecimalSymbol>,
    /// _
    #[xml(child = "w:listSeparator")]
    pub w_list_separator: Option<ListSeparator>,
    /// _
    #[xml(child = "w14:docId")]
    pub w14_doc_id: Option<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DocumentId,
    >,
    /// _
    #[xml(child = "w14:discardImageEditingData")]
    pub w14_discard_image_editing_data: Option<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DiscardImageEditingData,
    >,
    /// _
    #[xml(child = "w14:defaultImageDpi")]
    pub w14_default_image_dpi: Option<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DefaultImageDpi,
    >,
    /// _
    #[xml(child = "w14:conflictMode")]
    pub w14_conflict_mode: Option<
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictMode,
    >,
    /// _
    #[xml(child = "w15:chartTrackingRefBased")]
    pub w15_chart_tracking_ref_based: Option<
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::ChartTrackingRefBased,
    >,
    /// _
    #[xml(child = "w15:docId")]
    pub w15_doc_id: Option<
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::PersistentDocumentId,
    >,
}
mod settings_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Web Page Settings.
/// When the object is serialized out as xml, it's qualified name is w:webSettings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:webSettings")]
pub struct WebSettings {
    #[xml(attr = "xmlns", with = "web_settings_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:frameset")]
    pub frameset: Option<Frameset>,
    /// _
    #[xml(child = "w:divs")]
    pub divs: Option<Divs>,
    /// _
    #[xml(child = "w:encoding")]
    pub web_page_encoding: Option<WebPageEncoding>,
    /// _
    #[xml(child = "w:optimizeForBrowser")]
    pub optimize_for_browser: Option<OptimizeForBrowser>,
    /// _
    #[xml(child = "w:relyOnVML")]
    pub rely_on_vml: Option<RelyOnVml>,
    /// _
    #[xml(child = "w:allowPNG")]
    pub allow_png: Option<AllowPng>,
    /// _
    #[xml(child = "w:doNotRelyOnCSS")]
    pub do_not_rely_on_css: Option<DoNotRelyOnCss>,
    /// _
    #[xml(child = "w:doNotSaveAsSingleFile")]
    pub do_not_save_as_single_file: Option<DoNotSaveAsSingleFile>,
    /// _
    #[xml(child = "w:doNotOrganizeInFolder")]
    pub do_not_organize_in_folder: Option<DoNotOrganizeInFolder>,
    /// _
    #[xml(child = "w:doNotUseLongFileNames")]
    pub do_not_use_long_file_names: Option<DoNotUseLongFileNames>,
    /// _
    #[xml(child = "w:pixelsPerInch")]
    pub pixels_per_inch: Option<PixelsPerInch>,
    /// _
    #[xml(child = "w:targetScreenSz")]
    pub target_screen_size: Option<TargetScreenSize>,
}
mod web_settings_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Font Table Root Element.
/// When the object is serialized out as xml, it's qualified name is w:fonts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fonts")]
pub struct Fonts {
    #[xml(attr = "xmlns", with = "fonts_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:font")]
    pub w_font: Vec<Font>,
}
mod fonts_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Numbering Definitions.
/// When the object is serialized out as xml, it's qualified name is w:numbering.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numbering")]
pub struct Numbering {
    #[xml(attr = "xmlns", with = "numbering_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "w:numPicBullet")]
    pub w_num_pic_bullet: Vec<NumberingPictureBullet>,
    /// _
    #[xml(child = "w:abstractNum")]
    pub w_abstract_num: Vec<AbstractNum>,
    /// _
    #[xml(child = "w:num")]
    pub w_num: Vec<NumberingInstance>,
    /// _
    #[xml(child = "w:numIdMacAtCleanup")]
    pub w_num_id_mac_at_cleanup: Option<NumberingIdMacAtCleanup>,
}
mod numbering_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Style Definitions.
/// When the object is serialized out as xml, it's qualified name is w:styles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:styles")]
pub struct Styles {
    #[xml(attr = "xmlns", with = "styles_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Document Default Paragraph and Run Properties
    #[xml(child = "w:docDefaults")]
    pub doc_defaults: Option<DocDefaults>,
    ///Latent Style Information
    #[xml(child = "w:latentStyles")]
    pub latent_styles: Option<LatentStyles>,
    /// _
    #[xml(child = "w:style")]
    pub w_style: Vec<Style>,
}
mod styles_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Document.
/// When the object is serialized out as xml, it's qualified name is w:document.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:document")]
pub struct Document {
    #[xml(attr = "xmlns", with = "document_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// conformance
    /// Represents the following attribute in the schema: w:conformance
    #[xml(attr = "w:conformance")]
    pub w_conformance: Option<DocumentConformance>,
    #[xml(child = "w:background", child = "w:body")]
    pub children: Vec<DocumentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocumentChildChoice {
    #[xml(tag = "w:background")]
    WBackground(DocumentBackground),
    #[xml(tag = "w:body")]
    WBody(Body),
}
mod document_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Glossary Document Root Element.
/// When the object is serialized out as xml, it's qualified name is w:glossaryDocument.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:glossaryDocument")]
pub struct GlossaryDocument {
    #[xml(attr = "xmlns", with = "glossary_document_xmlns_derive")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "w:background", child = "w:docParts")]
    pub children: Vec<GlossaryDocumentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GlossaryDocumentChildChoice {
    #[xml(tag = "w:background")]
    WBackground(DocumentBackground),
    #[xml(tag = "w:docParts")]
    WDocParts(DocParts),
}
mod glossary_document_xmlns_derive {
    pub fn from_xml(mode: &str) -> hard_xml::XmlResult<String> {
        Ok(mode.to_string())
    }
    pub fn to_xml(_: &String) -> hard_xml::XmlResult<&'static str> {
        Ok("http://schemas.openxmlformats.org/wordprocessingml/2006/main")
    }
}
/// Previous Table-Level Property Exceptions.
/// When the object is serialized out as xml, it's qualified name is w:tblPrEx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPrEx")]
pub struct PreviousTablePropertyExceptions {
    ///Preferred Table Width Exception
    #[xml(child = "w:tblW")]
    pub table_width: Option<TableWidth>,
    ///Table Alignment Exception
    #[xml(child = "w:jc")]
    pub table_justification: Option<TableJustification>,
    ///Table Cell Spacing Exception
    #[xml(child = "w:tblCellSpacing")]
    pub table_cell_spacing: Option<TableCellSpacing>,
    ///Table Indent from Leading Margin Exception
    #[xml(child = "w:tblInd")]
    pub table_indentation: Option<TableIndentation>,
    ///Table Borders Exceptions
    #[xml(child = "w:tblBorders")]
    pub table_borders: Option<TableBorders>,
    ///Table Shading Exception
    #[xml(child = "w:shd")]
    pub shading: Option<Shading>,
    ///Table Layout Exception
    #[xml(child = "w:tblLayout")]
    pub table_layout: Option<TableLayout>,
    ///Table Cell Margin Exceptions
    #[xml(child = "w:tblCellMar")]
    pub table_cell_margin_default: Option<TableCellMarginDefault>,
    ///Table Style Conditional Formatting Settings Exception
    #[xml(child = "w:tblLook")]
    pub table_look: Option<TableLook>,
}
/// Previous Table Cell Properties.
/// When the object is serialized out as xml, it's qualified name is w:tcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcPr")]
pub struct PreviousTableCellProperties {
    #[xml(
        child = "w:cnfStyle",
        child = "w:tcW",
        child = "w:gridSpan",
        child = "w:hMerge",
        child = "w:vMerge",
        child = "w:tcBorders",
        child = "w:shd",
        child = "w:noWrap",
        child = "w:tcMar",
        child = "w:textDirection",
        child = "w:tcFitText",
        child = "w:vAlign",
        child = "w:hideMark",
        child = "w:cellIns",
        child = "w:cellDel",
        child = "w:cellMerge",
    )]
    pub children: Vec<PreviousTableCellPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousTableCellPropertiesChildChoice {
    #[xml(tag = "w:cnfStyle")]
    WCnfStyle(ConditionalFormatStyle),
    #[xml(tag = "w:tcW")]
    WTcW(TableCellWidth),
    #[xml(tag = "w:gridSpan")]
    WGridSpan(GridSpan),
    #[xml(tag = "w:hMerge")]
    WHMerge(HorizontalMerge),
    #[xml(tag = "w:vMerge")]
    WVMerge(VerticalMerge),
    #[xml(tag = "w:tcBorders")]
    WTcBorders(TableCellBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:noWrap")]
    WNoWrap(NoWrap),
    #[xml(tag = "w:tcMar")]
    WTcMar(TableCellMargin),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:tcFitText")]
    WTcFitText(TableCellFitText),
    #[xml(tag = "w:vAlign")]
    WVAlign(TableCellVerticalAlignment),
    #[xml(tag = "w:hideMark")]
    WHideMark(HideMark),
    #[xml(tag = "w:cellIns")]
    WCellIns(CellInsertion),
    #[xml(tag = "w:cellDel")]
    WCellDel(CellDeletion),
    #[xml(tag = "w:cellMerge")]
    WCellMerge(CellMerge),
}
/// Previous Table Row Properties.
/// When the object is serialized out as xml, it's qualified name is w:trPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:trPr")]
pub struct PreviousTableRowProperties {
    #[xml(
        child = "w:cnfStyle",
        child = "w:divId",
        child = "w:gridBefore",
        child = "w:gridAfter",
        child = "w:wBefore",
        child = "w:wAfter",
        child = "w:trHeight",
        child = "w:hidden",
        child = "w:cantSplit",
        child = "w:tblHeader",
        child = "w:tblCellSpacing",
        child = "w:jc",
    )]
    pub children: Vec<PreviousTableRowPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousTableRowPropertiesChildChoice {
    #[xml(tag = "w:cnfStyle")]
    WCnfStyle(ConditionalFormatStyle),
    #[xml(tag = "w:divId")]
    WDivId(DivId),
    #[xml(tag = "w:gridBefore")]
    WGridBefore(GridBefore),
    #[xml(tag = "w:gridAfter")]
    WGridAfter(GridAfter),
    #[xml(tag = "w:wBefore")]
    WWBefore(WidthBeforeTableRow),
    #[xml(tag = "w:wAfter")]
    WWAfter(WidthAfterTableRow),
    #[xml(tag = "w:trHeight")]
    WTrHeight(TableRowHeight),
    #[xml(tag = "w:hidden")]
    WHidden(Hidden),
    #[xml(tag = "w:cantSplit")]
    WCantSplit(CantSplit),
    #[xml(tag = "w:tblHeader")]
    WTblHeader(TableHeader),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
}
/// Previous Table Properties.
/// When the object is serialized out as xml, it's qualified name is w:tblPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPr")]
pub struct PreviousTableProperties {
    #[xml(
        child = "w:tblStyle",
        child = "w:tblpPr",
        child = "w:tblOverlap",
        child = "w:bidiVisual",
        child = "w:tblW",
        child = "w:jc",
        child = "w:tblCellSpacing",
        child = "w:tblInd",
        child = "w:tblBorders",
        child = "w:shd",
        child = "w:tblLayout",
        child = "w:tblCellMar",
        child = "w:tblLook",
        child = "w:tblCaption",
        child = "w:tblDescription",
    )]
    pub children: Vec<PreviousTablePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousTablePropertiesChildChoice {
    #[xml(tag = "w:tblStyle")]
    WTblStyle(TableStyle),
    #[xml(tag = "w:tblpPr")]
    WTblpPr(TablePositionProperties),
    #[xml(tag = "w:tblOverlap")]
    WTblOverlap(TableOverlap),
    #[xml(tag = "w:bidiVisual")]
    WBidiVisual(BiDiVisual),
    #[xml(tag = "w:tblW")]
    WTblW(TableWidth),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:tblInd")]
    WTblInd(TableIndentation),
    #[xml(tag = "w:tblBorders")]
    WTblBorders(TableBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tblLayout")]
    WTblLayout(TableLayout),
    #[xml(tag = "w:tblCellMar")]
    WTblCellMar(TableCellMarginDefault),
    #[xml(tag = "w:tblLook")]
    WTblLook(TableLook),
    #[xml(tag = "w:tblCaption")]
    WTblCaption(TableCaption),
    #[xml(tag = "w:tblDescription")]
    WTblDescription(TableDescription),
}
/// Previous Section Properties.
/// When the object is serialized out as xml, it's qualified name is w:sectPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sectPr")]
pub struct PreviousSectionProperties {
    /// Physical Section Mark Character Revision ID
    /// Represents the following attribute in the schema: w:rsidRPr
    #[xml(attr = "w:rsidRPr")]
    pub rsid_r_pr: Option<String>,
    /// Section Deletion Revision ID
    /// Represents the following attribute in the schema: w:rsidDel
    #[xml(attr = "w:rsidDel")]
    pub rsid_del: Option<String>,
    /// Section Addition Revision ID
    /// Represents the following attribute in the schema: w:rsidR
    #[xml(attr = "w:rsidR")]
    pub rsid_r: Option<String>,
    /// Section Properties Revision ID
    /// Represents the following attribute in the schema: w:rsidSect
    #[xml(attr = "w:rsidSect")]
    pub rsid_sect: Option<String>,
    #[xml(
        child = "w:footnotePr",
        child = "w:endnotePr",
        child = "w:type",
        child = "w:pgSz",
        child = "w:pgMar",
        child = "w:paperSrc",
        child = "w:pgBorders",
        child = "w:lnNumType",
        child = "w:pgNumType",
        child = "w:cols",
        child = "w:formProt",
        child = "w:vAlign",
        child = "w:noEndnote",
        child = "w:titlePg",
        child = "w:textDirection",
        child = "w:bidi",
        child = "w:rtlGutter",
        child = "w:docGrid",
        child = "w:printerSettings",
        child = "w15:footnoteColumns",
    )]
    pub children: Vec<PreviousSectionPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousSectionPropertiesChildChoice {
    #[xml(tag = "w:footnotePr")]
    WFootnotePr(FootnoteProperties),
    #[xml(tag = "w:endnotePr")]
    WEndnotePr(EndnoteProperties),
    #[xml(tag = "w:type")]
    WType(SectionType),
    #[xml(tag = "w:pgSz")]
    WPgSz(PageSize),
    #[xml(tag = "w:pgMar")]
    WPgMar(PageMargin),
    #[xml(tag = "w:paperSrc")]
    WPaperSrc(PaperSource),
    #[xml(tag = "w:pgBorders")]
    WPgBorders(PageBorders),
    #[xml(tag = "w:lnNumType")]
    WLnNumType(LineNumberType),
    #[xml(tag = "w:pgNumType")]
    WPgNumType(PageNumberType),
    #[xml(tag = "w:cols")]
    WCols(Columns),
    #[xml(tag = "w:formProt")]
    WFormProt(FormProtection),
    #[xml(tag = "w:vAlign")]
    WVAlign(VerticalTextAlignmentOnPage),
    #[xml(tag = "w:noEndnote")]
    WNoEndnote(NoEndnote),
    #[xml(tag = "w:titlePg")]
    WTitlePg(TitlePage),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:rtlGutter")]
    WRtlGutter(GutterOnRight),
    #[xml(tag = "w:docGrid")]
    WDocGrid(DocGrid),
    #[xml(tag = "w:printerSettings")]
    WPrinterSettings(PrinterSettingsReference),
    #[xml(tag = "w15:footnoteColumns")]
    W15FootnoteColumns(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::FootnoteColumns,
    ),
}
/// Previous Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is w:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPr")]
pub struct ParagraphPropertiesExtended {
    #[xml(
        child = "w:pStyle",
        child = "w:keepNext",
        child = "w:keepLines",
        child = "w:pageBreakBefore",
        child = "w:framePr",
        child = "w:widowControl",
        child = "w:numPr",
        child = "w:suppressLineNumbers",
        child = "w:pBdr",
        child = "w:shd",
        child = "w:tabs",
        child = "w:suppressAutoHyphens",
        child = "w:kinsoku",
        child = "w:wordWrap",
        child = "w:overflowPunct",
        child = "w:topLinePunct",
        child = "w:autoSpaceDE",
        child = "w:autoSpaceDN",
        child = "w:bidi",
        child = "w:adjustRightInd",
        child = "w:snapToGrid",
        child = "w:spacing",
        child = "w:ind",
        child = "w:contextualSpacing",
        child = "w:mirrorIndents",
        child = "w:suppressOverlap",
        child = "w:jc",
        child = "w:textDirection",
        child = "w:textAlignment",
        child = "w:textboxTightWrap",
        child = "w:outlineLvl",
        child = "w:divId",
        child = "w:cnfStyle",
    )]
    pub children: Vec<ParagraphPropertiesExtendedChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphPropertiesExtendedChildChoice {
    #[xml(tag = "w:pStyle")]
    WPStyle(ParagraphStyleId),
    #[xml(tag = "w:keepNext")]
    WKeepNext(KeepNext),
    #[xml(tag = "w:keepLines")]
    WKeepLines(KeepLines),
    #[xml(tag = "w:pageBreakBefore")]
    WPageBreakBefore(PageBreakBefore),
    #[xml(tag = "w:framePr")]
    WFramePr(FrameProperties),
    #[xml(tag = "w:widowControl")]
    WWidowControl(WidowControl),
    #[xml(tag = "w:numPr")]
    WNumPr(NumberingProperties),
    #[xml(tag = "w:suppressLineNumbers")]
    WSuppressLineNumbers(SuppressLineNumbers),
    #[xml(tag = "w:pBdr")]
    WPBdr(ParagraphBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tabs")]
    WTabs(Tabs),
    #[xml(tag = "w:suppressAutoHyphens")]
    WSuppressAutoHyphens(SuppressAutoHyphens),
    #[xml(tag = "w:kinsoku")]
    WKinsoku(Kinsoku),
    #[xml(tag = "w:wordWrap")]
    WWordWrap(WordWrap),
    #[xml(tag = "w:overflowPunct")]
    WOverflowPunct(OverflowPunctuation),
    #[xml(tag = "w:topLinePunct")]
    WTopLinePunct(TopLinePunctuation),
    #[xml(tag = "w:autoSpaceDE")]
    WAutoSpaceDe(AutoSpaceDe),
    #[xml(tag = "w:autoSpaceDN")]
    WAutoSpaceDn(AutoSpaceDn),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:adjustRightInd")]
    WAdjustRightInd(AdjustRightIndent),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:spacing")]
    WSpacing(SpacingBetweenLines),
    #[xml(tag = "w:ind")]
    WInd(Indentation),
    #[xml(tag = "w:contextualSpacing")]
    WContextualSpacing(ContextualSpacing),
    #[xml(tag = "w:mirrorIndents")]
    WMirrorIndents(MirrorIndents),
    #[xml(tag = "w:suppressOverlap")]
    WSuppressOverlap(SuppressOverlap),
    #[xml(tag = "w:jc")]
    WJc(Justification),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:textAlignment")]
    WTextAlignment(TextAlignment),
    #[xml(tag = "w:textboxTightWrap")]
    WTextboxTightWrap(TextBoxTightWrap),
    #[xml(tag = "w:outlineLvl")]
    WOutlineLvl(OutlineLevel),
    #[xml(tag = "w:divId")]
    WDivId(DivId),
    #[xml(tag = "w:cnfStyle")]
    WCnfStyle(ConditionalFormatStyle),
}
/// Previous Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct PreviousRunProperties {
    #[xml(
        child = "w:rStyle",
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:highlight",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:rtl",
        child = "w:cs",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
        child = "w14:glow",
        child = "w14:shadow",
        child = "w14:reflection",
        child = "w14:textOutline",
        child = "w14:textFill",
        child = "w14:scene3d",
        child = "w14:props3d",
        child = "w14:ligatures",
        child = "w14:numForm",
        child = "w14:numSpacing",
        child = "w14:stylisticSets",
        child = "w14:cntxtAlts",
    )]
    pub children: Vec<PreviousRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousRunPropertiesChildChoice {
    #[xml(tag = "w:rStyle")]
    WRStyle(RunStyle),
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:highlight")]
    WHighlight(Highlight),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:rtl")]
    WRtl(RightToLeftText),
    #[xml(tag = "w:cs")]
    WCs(ComplexScript),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
    #[xml(tag = "w14:glow")]
    W14Glow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow),
    #[xml(tag = "w14:shadow")]
    W14Shadow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow),
    #[xml(tag = "w14:reflection")]
    W14Reflection(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection,
    ),
    #[xml(tag = "w14:textOutline")]
    W14TextOutline(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect,
    ),
    #[xml(tag = "w14:textFill")]
    W14TextFill(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect,
    ),
    #[xml(tag = "w14:scene3d")]
    W14Scene3d(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D),
    #[xml(tag = "w14:props3d")]
    W14Props3d(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D,
    ),
    #[xml(tag = "w14:ligatures")]
    W14Ligatures(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures,
    ),
    #[xml(tag = "w14:numForm")]
    W14NumForm(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat,
    ),
    #[xml(tag = "w14:numSpacing")]
    W14NumSpacing(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing,
    ),
    #[xml(tag = "w14:stylisticSets")]
    W14StylisticSets(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets,
    ),
    #[xml(tag = "w14:cntxtAlts")]
    W14CntxtAlts(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives,
    ),
}
/// Previous Run Properties for the Paragraph Mark.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct PreviousParagraphMarkRunProperties {
    #[xml(
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "w:rStyle",
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:highlight",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:rtl",
        child = "w:cs",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
        child = "w14:glow",
        child = "w14:shadow",
        child = "w14:reflection",
        child = "w14:textOutline",
        child = "w14:textFill",
        child = "w14:scene3d",
        child = "w14:props3d",
        child = "w14:ligatures",
        child = "w14:numForm",
        child = "w14:numSpacing",
        child = "w14:stylisticSets",
        child = "w14:cntxtAlts",
        child = "w:oMath",
    )]
    pub children: Vec<PreviousParagraphMarkRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousParagraphMarkRunPropertiesChildChoice {
    #[xml(tag = "w:ins")]
    WIns(Inserted),
    #[xml(tag = "w:del")]
    WDel(Deleted),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFrom),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveTo),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictDeletion,
    ),
    #[xml(tag = "w:rStyle")]
    WRStyle(RunStyle),
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:highlight")]
    WHighlight(Highlight),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:rtl")]
    WRtl(RightToLeftText),
    #[xml(tag = "w:cs")]
    WCs(ComplexScript),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
    #[xml(tag = "w14:glow")]
    W14Glow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow),
    #[xml(tag = "w14:shadow")]
    W14Shadow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow),
    #[xml(tag = "w14:reflection")]
    W14Reflection(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection,
    ),
    #[xml(tag = "w14:textOutline")]
    W14TextOutline(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect,
    ),
    #[xml(tag = "w14:textFill")]
    W14TextFill(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect,
    ),
    #[xml(tag = "w14:scene3d")]
    W14Scene3d(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D),
    #[xml(tag = "w14:props3d")]
    W14Props3d(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D,
    ),
    #[xml(tag = "w14:ligatures")]
    W14Ligatures(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures,
    ),
    #[xml(tag = "w14:numForm")]
    W14NumForm(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat,
    ),
    #[xml(tag = "w14:numSpacing")]
    W14NumSpacing(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing,
    ),
    #[xml(tag = "w14:stylisticSets")]
    W14StylisticSets(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets,
    ),
    #[xml(tag = "w14:cntxtAlts")]
    W14CntxtAlts(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives,
    ),
    #[xml(tag = "w:oMath")]
    WOMath(OfficeMath),
}
/// Numbering Level Reference.
/// When the object is serialized out as xml, it's qualified name is w:ilvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ilvl")]
pub struct NumberingLevelReference {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Numbering Definition Instance Reference.
/// When the object is serialized out as xml, it's qualified name is w:numId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numId")]
pub struct NumberingId {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Starting Value.
/// When the object is serialized out as xml, it's qualified name is w:start.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:start")]
pub struct StartNumberingValue {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the AbstractNumId Class.
/// When the object is serialized out as xml, it's qualified name is w:abstractNumId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:abstractNumId")]
pub struct AbstractNumId {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the NonNegativeDecimalNumberType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NonNegativeDecimalNumberType {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Previous Paragraph Numbering Properties.
/// When the object is serialized out as xml, it's qualified name is w:numberingChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numberingChange")]
pub struct NumberingChange {
    /// original
    /// Represents the following attribute in the schema: w:original
    #[xml(attr = "w:original")]
    pub original: Option<String>,
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Custom Tab Stop.
/// When the object is serialized out as xml, it's qualified name is w:tab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tab")]
pub struct TabStop {
    /// Tab Stop Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TabStopValues,
    /// Tab Leader Character
    /// Represents the following attribute in the schema: w:leader
    #[xml(attr = "w:leader")]
    pub leader: Option<TabStopLeaderCharValues>,
    /// Tab Stop Position
    /// Represents the following attribute in the schema: w:pos
    #[xml(attr = "w:pos")]
    pub position: i32,
}
/// Run Properties for the Paragraph Mark.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct ParagraphMarkRunProperties {
    #[xml(
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "w:rStyle",
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:highlight",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:rtl",
        child = "w:cs",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
        child = "w14:glow",
        child = "w14:shadow",
        child = "w14:reflection",
        child = "w14:textOutline",
        child = "w14:textFill",
        child = "w14:scene3d",
        child = "w14:props3d",
        child = "w14:ligatures",
        child = "w14:numForm",
        child = "w14:numSpacing",
        child = "w14:stylisticSets",
        child = "w14:cntxtAlts",
        child = "w:oMath",
        child = "w:rPrChange",
    )]
    pub children: Vec<ParagraphMarkRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphMarkRunPropertiesChildChoice {
    #[xml(tag = "w:ins")]
    WIns(Inserted),
    #[xml(tag = "w:del")]
    WDel(Deleted),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFrom),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveTo),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictDeletion,
    ),
    #[xml(tag = "w:rStyle")]
    WRStyle(RunStyle),
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:highlight")]
    WHighlight(Highlight),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:rtl")]
    WRtl(RightToLeftText),
    #[xml(tag = "w:cs")]
    WCs(ComplexScript),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
    #[xml(tag = "w14:glow")]
    W14Glow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Glow),
    #[xml(tag = "w14:shadow")]
    W14Shadow(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Shadow),
    #[xml(tag = "w14:reflection")]
    W14Reflection(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Reflection,
    ),
    #[xml(tag = "w14:textOutline")]
    W14TextOutline(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::TextOutlineEffect,
    ),
    #[xml(tag = "w14:textFill")]
    W14TextFill(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::FillTextEffect,
    ),
    #[xml(tag = "w14:scene3d")]
    W14Scene3d(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Scene3D),
    #[xml(tag = "w14:props3d")]
    W14Props3d(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Properties3D,
    ),
    #[xml(tag = "w14:ligatures")]
    W14Ligatures(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::Ligatures,
    ),
    #[xml(tag = "w14:numForm")]
    W14NumForm(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberingFormat,
    ),
    #[xml(tag = "w14:numSpacing")]
    W14NumSpacing(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::NumberSpacing,
    ),
    #[xml(tag = "w14:stylisticSets")]
    W14StylisticSets(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::StylisticSets,
    ),
    #[xml(tag = "w14:cntxtAlts")]
    W14CntxtAlts(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContextualAlternatives,
    ),
    #[xml(tag = "w:oMath")]
    WOMath(OfficeMath),
    #[xml(tag = "w:rPrChange")]
    WRPrChange(ParagraphMarkRunPropertiesChange),
}
/// Section Properties.
/// When the object is serialized out as xml, it's qualified name is w:sectPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sectPr")]
pub struct SectionProperties {
    /// Physical Section Mark Character Revision ID
    /// Represents the following attribute in the schema: w:rsidRPr
    #[xml(attr = "w:rsidRPr")]
    pub rsid_r_pr: Option<String>,
    /// Section Deletion Revision ID
    /// Represents the following attribute in the schema: w:rsidDel
    #[xml(attr = "w:rsidDel")]
    pub rsid_del: Option<String>,
    /// Section Addition Revision ID
    /// Represents the following attribute in the schema: w:rsidR
    #[xml(attr = "w:rsidR")]
    pub rsid_r: Option<String>,
    /// Section Properties Revision ID
    /// Represents the following attribute in the schema: w:rsidSect
    #[xml(attr = "w:rsidSect")]
    pub rsid_sect: Option<String>,
    #[xml(
        child = "w:headerReference",
        child = "w:footerReference",
        child = "w:footnotePr",
        child = "w:endnotePr",
        child = "w:type",
        child = "w:pgSz",
        child = "w:pgMar",
        child = "w:paperSrc",
        child = "w:pgBorders",
        child = "w:lnNumType",
        child = "w:pgNumType",
        child = "w:cols",
        child = "w:formProt",
        child = "w:vAlign",
        child = "w:noEndnote",
        child = "w:titlePg",
        child = "w:textDirection",
        child = "w:bidi",
        child = "w:rtlGutter",
        child = "w:docGrid",
        child = "w:printerSettings",
        child = "w15:footnoteColumns",
        child = "w:sectPrChange",
    )]
    pub children: Vec<SectionPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SectionPropertiesChildChoice {
    #[xml(tag = "w:headerReference")]
    WHeaderReference(HeaderReference),
    #[xml(tag = "w:footerReference")]
    WFooterReference(FooterReference),
    #[xml(tag = "w:footnotePr")]
    WFootnotePr(FootnoteProperties),
    #[xml(tag = "w:endnotePr")]
    WEndnotePr(EndnoteProperties),
    #[xml(tag = "w:type")]
    WType(SectionType),
    #[xml(tag = "w:pgSz")]
    WPgSz(PageSize),
    #[xml(tag = "w:pgMar")]
    WPgMar(PageMargin),
    #[xml(tag = "w:paperSrc")]
    WPaperSrc(PaperSource),
    #[xml(tag = "w:pgBorders")]
    WPgBorders(PageBorders),
    #[xml(tag = "w:lnNumType")]
    WLnNumType(LineNumberType),
    #[xml(tag = "w:pgNumType")]
    WPgNumType(PageNumberType),
    #[xml(tag = "w:cols")]
    WCols(Columns),
    #[xml(tag = "w:formProt")]
    WFormProt(FormProtection),
    #[xml(tag = "w:vAlign")]
    WVAlign(VerticalTextAlignmentOnPage),
    #[xml(tag = "w:noEndnote")]
    WNoEndnote(NoEndnote),
    #[xml(tag = "w:titlePg")]
    WTitlePg(TitlePage),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:rtlGutter")]
    WRtlGutter(GutterOnRight),
    #[xml(tag = "w:docGrid")]
    WDocGrid(DocGrid),
    #[xml(tag = "w:printerSettings")]
    WPrinterSettings(PrinterSettingsReference),
    #[xml(tag = "w15:footnoteColumns")]
    W15FootnoteColumns(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::FootnoteColumns,
    ),
    #[xml(tag = "w:sectPrChange")]
    WSectPrChange(SectionPropertiesChange),
}
/// Custom Field Data.
/// When the object is serialized out as xml, it's qualified name is w:fldData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fldData")]
pub struct FieldData {
    #[xml(text)]
    pub child: String,
}
/// Form Field Properties.
/// When the object is serialized out as xml, it's qualified name is w:ffData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ffData")]
pub struct FormFieldData {
    #[xml(
        child = "w:name",
        child = "w:enabled",
        child = "w:calcOnExit",
        child = "w:entryMacro",
        child = "w:exitMacro",
        child = "w:helpText",
        child = "w:statusText",
        child = "w:checkBox",
        child = "w:ddList",
        child = "w:textInput",
    )]
    pub children: Vec<FormFieldDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FormFieldDataChildChoice {
    #[xml(tag = "w:name")]
    WName(FormFieldName),
    #[xml(tag = "w:enabled")]
    WEnabled(Enabled),
    #[xml(tag = "w:calcOnExit")]
    WCalcOnExit(CalculateOnExit),
    #[xml(tag = "w:entryMacro")]
    WEntryMacro(EntryMacro),
    #[xml(tag = "w:exitMacro")]
    WExitMacro(ExitMacro),
    #[xml(tag = "w:helpText")]
    WHelpText(HelpText),
    #[xml(tag = "w:statusText")]
    WStatusText(StatusText),
    #[xml(tag = "w:checkBox")]
    WCheckBox(CheckBox),
    #[xml(tag = "w:ddList")]
    WDdList(DropDownListFormField),
    #[xml(tag = "w:textInput")]
    WTextInput(TextInput),
}
/// Form Field Name.
/// When the object is serialized out as xml, it's qualified name is w:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:name")]
pub struct FormFieldName {
    /// Form Field Name Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
}
/// Script Function to Execute on Form Field Entry.
/// When the object is serialized out as xml, it's qualified name is w:entryMacro.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:entryMacro")]
pub struct EntryMacro {
    /// Name of Script Function
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Script Function to Execute on Form Field Exit.
/// When the object is serialized out as xml, it's qualified name is w:exitMacro.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:exitMacro")]
pub struct ExitMacro {
    /// Name of Script Function
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the MacroNameType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct MacroNameType {
    /// Name of Script Function
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Associated Help Text.
/// When the object is serialized out as xml, it's qualified name is w:helpText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:helpText")]
pub struct HelpText {
    /// Help Text Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<InfoTextValues>,
    /// Help Text Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
}
/// Associated Status Text.
/// When the object is serialized out as xml, it's qualified name is w:statusText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:statusText")]
pub struct StatusText {
    /// Status Text Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<InfoTextValues>,
    /// Status Text Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
}
/// Checkbox Form Field Properties.
/// When the object is serialized out as xml, it's qualified name is w:checkBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:checkBox")]
pub struct CheckBox {
    #[xml(
        child = "w:size",
        child = "w:sizeAuto",
        child = "w:default",
        child = "w:checked",
    )]
    pub children: Vec<CheckBoxChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CheckBoxChildChoice {
    #[xml(tag = "w:size")]
    WSize(FormFieldSize),
    #[xml(tag = "w:sizeAuto")]
    WSizeAuto(AutomaticallySizeFormField),
    #[xml(tag = "w:default")]
    WDefault(DefaultCheckBoxFormFieldState),
    #[xml(tag = "w:checked")]
    WChecked(Checked),
}
/// Drop-Down List Form Field Properties.
/// When the object is serialized out as xml, it's qualified name is w:ddList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:ddList")]
pub struct DropDownListFormField {
    ///Drop-Down List Selection
    #[xml(child = "w:result")]
    pub drop_down_list_selection: Option<DropDownListSelection>,
    ///Default Drop-Down List Item Index
    #[xml(child = "w:default")]
    pub default_drop_down_list_item_index: Option<DefaultDropDownListItemIndex>,
    /// _
    #[xml(child = "w:listEntry")]
    pub w_list_entry: Vec<ListEntryFormField>,
}
/// Text Box Form Field Properties.
/// When the object is serialized out as xml, it's qualified name is w:textInput.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:textInput")]
pub struct TextInput {
    ///Text Box Form Field Type
    #[xml(child = "w:type")]
    pub text_box_form_field_type: Option<TextBoxFormFieldType>,
    ///Default Text Box Form Field String
    #[xml(child = "w:default")]
    pub default_text_box_form_field_string: Option<DefaultTextBoxFormFieldString>,
    ///Text Box Form Field Maximum Length
    #[xml(child = "w:maxLength")]
    pub max_length: Option<MaxLength>,
    ///Text Box Form Field Formatting
    #[xml(child = "w:format")]
    pub format: Option<Format>,
}
/// Default Drop-Down List Item Index.
/// When the object is serialized out as xml, it's qualified name is w:default.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:default")]
pub struct DefaultDropDownListItemIndex {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Drop-Down List Entry.
/// When the object is serialized out as xml, it's qualified name is w:listEntry.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:listEntry")]
pub struct ListEntryFormField {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Default Text Box Form Field String.
/// When the object is serialized out as xml, it's qualified name is w:default.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:default")]
pub struct DefaultTextBoxFormFieldString {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Frame Name.
/// When the object is serialized out as xml, it's qualified name is w:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:name")]
pub struct FrameName {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the String255Type Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct String255Type {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Text Box Form Field Type.
/// When the object is serialized out as xml, it's qualified name is w:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:type")]
pub struct TextBoxFormFieldType {
    /// Text Box Form Field Type Values
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TextBoxFormFieldValues,
}
/// Text Box Form Field Maximum Length.
/// When the object is serialized out as xml, it's qualified name is w:maxLength.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:maxLength")]
pub struct MaxLength {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Text Box Form Field Formatting.
/// When the object is serialized out as xml, it's qualified name is w:format.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:format")]
pub struct Format {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Single Column Definition.
/// When the object is serialized out as xml, it's qualified name is w:col.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:col")]
pub struct Column {
    /// Column Width
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
    /// Space Before Following Column
    /// Represents the following attribute in the schema: w:space
    #[xml(attr = "w:space")]
    pub space: Option<String>,
}
/// Revision Information for Section Properties.
/// When the object is serialized out as xml, it's qualified name is w:sectPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sectPrChange")]
pub struct SectionPropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:sectPr")]
    pub children: Vec<SectionPropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SectionPropertiesChangeChildChoice {
    #[xml(tag = "w:sectPr")]
    WSectPr(PreviousSectionProperties),
}
/// Revision Information for Run Properties on the Paragraph Mark.
/// When the object is serialized out as xml, it's qualified name is w:rPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPrChange")]
pub struct ParagraphMarkRunPropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:rPr")]
    pub children: Vec<ParagraphMarkRunPropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphMarkRunPropertiesChangeChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(PreviousParagraphMarkRunProperties),
}
/// External Content Import Properties.
/// When the object is serialized out as xml, it's qualified name is w:altChunkPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:altChunkPr")]
pub struct AltChunkProperties {
    ///Keep Source Formatting on Import
    #[xml(child = "w:matchSrc")]
    pub match_source: Option<MatchSource>,
}
/// Phonetic Guide Text Alignment.
/// When the object is serialized out as xml, it's qualified name is w:rubyAlign.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rubyAlign")]
pub struct RubyAlign {
    /// Phonetic Guide Text Alignment Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: RubyAlignValues,
}
/// Distance Between Phonetic Guide Text and Phonetic Guide Base Text.
/// When the object is serialized out as xml, it's qualified name is w:hpsRaise.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hpsRaise")]
pub struct PhoneticGuideRaise {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Language ID for Phonetic Guide.
/// When the object is serialized out as xml, it's qualified name is w:lid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lid")]
pub struct LanguageId {
    /// Language Code
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Phonetic Guide Properties.
/// When the object is serialized out as xml, it's qualified name is w:rubyPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rubyPr")]
pub struct RubyProperties {
    ///Phonetic Guide Text Alignment
    #[xml(child = "w:rubyAlign")]
    pub ruby_align: RubyAlign,
    ///Phonetic Guide Text Font Size
    #[xml(child = "w:hps")]
    pub phonetic_guide_text_font_size: PhoneticGuideTextFontSize,
    ///Distance Between Phonetic Guide Text and Phonetic Guide Base Text
    #[xml(child = "w:hpsRaise")]
    pub phonetic_guide_raise: PhoneticGuideRaise,
    ///Phonetic Guide Base Text Font Size
    #[xml(child = "w:hpsBaseText")]
    pub phonetic_guide_base_text_size: PhoneticGuideBaseTextSize,
    ///Language ID for Phonetic Guide
    #[xml(child = "w:lid")]
    pub language_id: LanguageId,
    ///Invalidated Field Cache
    #[xml(child = "w:dirty")]
    pub dirty: Option<Dirty>,
}
/// Phonetic Guide Text.
/// When the object is serialized out as xml, it's qualified name is w:rt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rt")]
pub struct RubyContent {
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<RubyContentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RubyContentChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Phonetic Guide Base Text.
/// When the object is serialized out as xml, it's qualified name is w:rubyBase.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rubyBase")]
pub struct RubyBase {
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<RubyBaseChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RubyBaseChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Defines the RubyContentType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RubyContentType {
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<RubyContentTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RubyContentTypeChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Custom XML Data Date Storage Format.
/// When the object is serialized out as xml, it's qualified name is w:storeMappedDataAs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:storeMappedDataAs")]
pub struct SdtDateMappingType {
    /// Date Storage Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<DateFormatValues>,
}
/// Date Picker Calendar Type.
/// When the object is serialized out as xml, it's qualified name is w:calendar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:calendar")]
pub struct Calendar {
    /// Calendar Type Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<CalendarValues>,
}
/// Combo Box List Item.
/// When the object is serialized out as xml, it's qualified name is w:listItem.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:listItem")]
pub struct ListItem {
    /// List Entry Display Text
    /// Represents the following attribute in the schema: w:displayText
    #[xml(attr = "w:displayText")]
    pub display_text: Option<String>,
    /// List Entry Value
    /// Represents the following attribute in the schema: w:value
    #[xml(attr = "w:value")]
    pub value: Option<String>,
}
/// Structured Document Tag Properties.
/// When the object is serialized out as xml, it's qualified name is w:sdtPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtPr")]
pub struct SdtProperties {
    #[xml(
        child = "w:rPr",
        child = "w:alias",
        child = "w:lock",
        child = "w:placeholder",
        child = "w:showingPlcHdr",
        child = "w:dataBinding",
        child = "w15:dataBinding",
        child = "w:temporary",
        child = "w:id",
        child = "w:tag",
        child = "w15:color",
        child = "w15:appearance",
        child = "w15:webExtensionLinked",
        child = "w15:webExtensionCreated",
        child = "w:equation",
        child = "w:comboBox",
        child = "w:date",
        child = "w:docPartObj",
        child = "w:docPartList",
        child = "w:dropDownList",
        child = "w:picture",
        child = "w:richText",
        child = "w:text",
        child = "w:citation",
        child = "w:group",
        child = "w:bibliography",
        child = "w14:entityPicker",
        child = "w14:checkbox",
        child = "w15:repeatingSection",
        child = "w15:repeatingSectionItem",
    )]
    pub children: Vec<SdtPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtPropertiesChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
    #[xml(tag = "w:alias")]
    WAlias(SdtAlias),
    #[xml(tag = "w:lock")]
    WLock(Lock),
    #[xml(tag = "w:placeholder")]
    WPlaceholder(SdtPlaceholder),
    #[xml(tag = "w:showingPlcHdr")]
    WShowingPlcHdr(ShowingPlaceholder),
    #[xml(tag = "w:dataBinding")]
    WDataBinding(DataBinding),
    #[xml(tag = "w15:dataBinding")]
    W15DataBinding(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::DataBinding,
    ),
    #[xml(tag = "w:temporary")]
    WTemporary(TemporarySdt),
    #[xml(tag = "w:id")]
    WId(SdtId),
    #[xml(tag = "w:tag")]
    WTag(Tag),
    #[xml(tag = "w15:color")]
    W15Color(crate::schemas::schemas_microsoft_com_office_word_2012_wordml::Color),
    #[xml(tag = "w15:appearance")]
    W15Appearance(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::Appearance,
    ),
    #[xml(tag = "w15:webExtensionLinked")]
    W15WebExtensionLinked(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::WebExtensionLinked,
    ),
    #[xml(tag = "w15:webExtensionCreated")]
    W15WebExtensionCreated(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::WebExtensionCreated,
    ),
    #[xml(tag = "w:equation")]
    WEquation(SdtContentEquation),
    #[xml(tag = "w:comboBox")]
    WComboBox(SdtContentComboBox),
    #[xml(tag = "w:date")]
    WDate(SdtContentDate),
    #[xml(tag = "w:docPartObj")]
    WDocPartObj(SdtContentDocPartObject),
    #[xml(tag = "w:docPartList")]
    WDocPartList(SdtContentDocPartList),
    #[xml(tag = "w:dropDownList")]
    WDropDownList(SdtContentDropDownList),
    #[xml(tag = "w:picture")]
    WPicture(SdtContentPicture),
    #[xml(tag = "w:richText")]
    WRichText(SdtContentRichText),
    #[xml(tag = "w:text")]
    WText(SdtContentText),
    #[xml(tag = "w:citation")]
    WCitation(SdtContentCitation),
    #[xml(tag = "w:group")]
    WGroup(SdtContentGroup),
    #[xml(tag = "w:bibliography")]
    WBibliography(SdtContentBibliography),
    #[xml(tag = "w14:entityPicker")]
    W14EntityPicker(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::EntityPickerEmpty,
    ),
    #[xml(tag = "w14:checkbox")]
    W14Checkbox(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::SdtContentCheckBox,
    ),
    #[xml(tag = "w15:repeatingSection")]
    W15RepeatingSection(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::SdtRepeatedSection,
    ),
    #[xml(tag = "w15:repeatingSectionItem")]
    W15RepeatingSectionItem(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::SdtRepeatedSectionItem,
    ),
}
/// Structured Document Tag End Character Properties.
/// When the object is serialized out as xml, it's qualified name is w:sdtEndPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtEndPr")]
pub struct SdtEndCharProperties {
    #[xml(child = "w:rPr")]
    pub children: Vec<SdtEndCharPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtEndCharPropertiesChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(RunProperties),
}
/// Block-Level Structured Document Tag Content.
/// When the object is serialized out as xml, it's qualified name is w:sdtContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtContent")]
pub struct SdtContentBlock {
    #[xml(
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<SdtContentBlockChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtContentBlockChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Inline-Level Structured Document Tag Content.
/// When the object is serialized out as xml, it's qualified name is w:sdtContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtContent")]
pub struct SdtContentRun {
    #[xml(
        child = "m:r",
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "w:r",
        child = "w:bdo",
        child = "w:dir",
        child = "w:subDoc",
    )]
    pub children: Vec<SdtContentRunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtContentRunChildChoice {
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRun),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleField),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(Hyperlink),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:bdo")]
    WBdo(BidirectionalOverride),
    #[xml(tag = "w:dir")]
    WDir(BidirectionalEmbedding),
    #[xml(tag = "w:subDoc")]
    WSubDoc(SubDocumentReference),
}
/// Defines the SdtContentRunRuby Class.
/// When the object is serialized out as xml, it's qualified name is w:sdtContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtContent")]
pub struct SdtContentRunRuby {
    #[xml(
        child = "w:customXml",
        child = "w:fldSimple",
        child = "w:hyperlink",
        child = "w:r",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "m:acc",
        child = "m:bar",
        child = "m:box",
        child = "m:borderBox",
        child = "m:d",
        child = "m:eqArr",
        child = "m:f",
        child = "m:func",
        child = "m:groupChr",
        child = "m:limLow",
        child = "m:limUpp",
        child = "m:m",
        child = "m:nary",
        child = "m:phant",
        child = "m:rad",
        child = "m:sPre",
        child = "m:sSub",
        child = "m:sSubSup",
        child = "m:sSup",
        child = "m:r",
    )]
    pub children: Vec<SdtContentRunRubyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtContentRunRubyChildChoice {
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRuby),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(SimpleFieldRuby),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(HyperlinkRuby),
    #[xml(tag = "w:r")]
    WR(Run),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRunRuby),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "m:acc")]
    MAcc(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Accent),
    #[xml(tag = "m:bar")]
    MBar(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Bar),
    #[xml(tag = "m:box")]
    MBox(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::BorderBox,
    ),
    #[xml(tag = "m:d")]
    MD(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::EquationArray,
    ),
    #[xml(tag = "m:f")]
    MF(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Fraction),
    #[xml(tag = "m:func")]
    MFunc(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathFunction,
    ),
    #[xml(tag = "m:groupChr")]
    MGroupChr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::GroupChar,
    ),
    #[xml(tag = "m:limLow")]
    MLimLow(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitLower,
    ),
    #[xml(tag = "m:limUpp")]
    MLimUpp(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::LimitUpper,
    ),
    #[xml(tag = "m:m")]
    MM(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Matrix),
    #[xml(tag = "m:nary")]
    MNary(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Nary),
    #[xml(tag = "m:phant")]
    MPhant(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Phantom,
    ),
    #[xml(tag = "m:rad")]
    MRad(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Radical),
    #[xml(tag = "m:sPre")]
    MSPre(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::PreSubSuper,
    ),
    #[xml(tag = "m:sSub")]
    MSSub(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Subscript,
    ),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::SubSuperscript,
    ),
    #[xml(tag = "m:sSup")]
    MSSup(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Superscript,
    ),
    #[xml(tag = "m:r")]
    MR(crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Run),
}
/// Cell-Level Structured Document Tag Content.
/// When the object is serialized out as xml, it's qualified name is w:sdtContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtContent")]
pub struct SdtContentCell {
    #[xml(
        child = "w:tc",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<SdtContentCellChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtContentCellChildChoice {
    #[xml(tag = "w:tc")]
    WTc(TableCell),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlCell),
    #[xml(tag = "w:sdt")]
    WSdt(SdtCell),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Row-Level Structured Document Tag Content.
/// When the object is serialized out as xml, it's qualified name is w:sdtContent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sdtContent")]
pub struct SdtContentRow {
    #[xml(
        child = "w:tr",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<SdtContentRowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SdtContentRowChildChoice {
    #[xml(tag = "w:tr")]
    WTr(TableRow),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlRow),
    #[xml(tag = "w:sdt")]
    WSdt(SdtRow),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Custom XML Element Properties.
/// When the object is serialized out as xml, it's qualified name is w:customXmlPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:customXmlPr")]
pub struct CustomXmlProperties {
    ///Custom XML Element Placeholder Text
    #[xml(child = "w:placeholder")]
    pub custom_xml_placeholder: Option<CustomXmlPlaceholder>,
    /// _
    #[xml(child = "w:attr")]
    pub w_attr: Vec<CustomXmlAttribute>,
}
/// Custom XML Attribute.
/// When the object is serialized out as xml, it's qualified name is w:attr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:attr")]
pub struct CustomXmlAttribute {
    /// uri
    /// Represents the following attribute in the schema: w:uri
    #[xml(attr = "w:uri")]
    pub uri: Option<String>,
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Grid Column Definition.
/// When the object is serialized out as xml, it's qualified name is w:gridCol.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:gridCol")]
pub struct GridColumn {
    /// Grid Column Width
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: Option<String>,
}
/// Revision Information for Table Grid Column Definitions.
/// When the object is serialized out as xml, it's qualified name is w:tblGridChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblGridChange")]
pub struct TableGridChange {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:tblGrid")]
    pub children: Vec<TableGridChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableGridChangeChildChoice {
    #[xml(tag = "w:tblGrid")]
    WTblGrid(PreviousTableGrid),
}
/// Revision Information for Table Cell Properties.
/// When the object is serialized out as xml, it's qualified name is w:tcPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcPrChange")]
pub struct TableCellPropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:tcPr")]
    pub children: Vec<TableCellPropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableCellPropertiesChangeChildChoice {
    #[xml(tag = "w:tcPr")]
    WTcPr(PreviousTableCellProperties),
}
/// Table Cell Properties.
/// When the object is serialized out as xml, it's qualified name is w:tcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcPr")]
pub struct TableCellProperties {
    #[xml(
        child = "w:cnfStyle",
        child = "w:tcW",
        child = "w:gridSpan",
        child = "w:hMerge",
        child = "w:vMerge",
        child = "w:tcBorders",
        child = "w:shd",
        child = "w:noWrap",
        child = "w:tcMar",
        child = "w:textDirection",
        child = "w:tcFitText",
        child = "w:vAlign",
        child = "w:hideMark",
        child = "w:cellIns",
        child = "w:cellDel",
        child = "w:cellMerge",
        child = "w:tcPrChange",
    )]
    pub children: Vec<TableCellPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableCellPropertiesChildChoice {
    #[xml(tag = "w:cnfStyle")]
    WCnfStyle(ConditionalFormatStyle),
    #[xml(tag = "w:tcW")]
    WTcW(TableCellWidth),
    #[xml(tag = "w:gridSpan")]
    WGridSpan(GridSpan),
    #[xml(tag = "w:hMerge")]
    WHMerge(HorizontalMerge),
    #[xml(tag = "w:vMerge")]
    WVMerge(VerticalMerge),
    #[xml(tag = "w:tcBorders")]
    WTcBorders(TableCellBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:noWrap")]
    WNoWrap(NoWrap),
    #[xml(tag = "w:tcMar")]
    WTcMar(TableCellMargin),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:tcFitText")]
    WTcFitText(TableCellFitText),
    #[xml(tag = "w:vAlign")]
    WVAlign(TableCellVerticalAlignment),
    #[xml(tag = "w:hideMark")]
    WHideMark(HideMark),
    #[xml(tag = "w:cellIns")]
    WCellIns(CellInsertion),
    #[xml(tag = "w:cellDel")]
    WCellDel(CellDeletion),
    #[xml(tag = "w:cellMerge")]
    WCellMerge(CellMerge),
    #[xml(tag = "w:tcPrChange")]
    WTcPrChange(TableCellPropertiesChange),
}
/// Revision Information for Table Properties.
/// When the object is serialized out as xml, it's qualified name is w:tblPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPrChange")]
pub struct TablePropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:tblPr")]
    pub children: Vec<TablePropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TablePropertiesChangeChildChoice {
    #[xml(tag = "w:tblPr")]
    WTblPr(PreviousTableProperties),
}
/// Revision Information for Table-Level Property Exceptions.
/// When the object is serialized out as xml, it's qualified name is w:tblPrExChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPrExChange")]
pub struct TablePropertyExceptionsChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:tblPrEx")]
    pub children: Vec<TablePropertyExceptionsChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TablePropertyExceptionsChangeChildChoice {
    #[xml(tag = "w:tblPrEx")]
    WTblPrEx(PreviousTablePropertyExceptions),
}
/// Table Properties.
/// When the object is serialized out as xml, it's qualified name is w:tblPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPr")]
pub struct TableProperties {
    #[xml(
        child = "w:tblStyle",
        child = "w:tblpPr",
        child = "w:tblOverlap",
        child = "w:bidiVisual",
        child = "w:tblW",
        child = "w:jc",
        child = "w:tblCellSpacing",
        child = "w:tblInd",
        child = "w:tblBorders",
        child = "w:shd",
        child = "w:tblLayout",
        child = "w:tblCellMar",
        child = "w:tblLook",
        child = "w:tblCaption",
        child = "w:tblDescription",
        child = "w:tblPrChange",
    )]
    pub children: Vec<TablePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TablePropertiesChildChoice {
    #[xml(tag = "w:tblStyle")]
    WTblStyle(TableStyle),
    #[xml(tag = "w:tblpPr")]
    WTblpPr(TablePositionProperties),
    #[xml(tag = "w:tblOverlap")]
    WTblOverlap(TableOverlap),
    #[xml(tag = "w:bidiVisual")]
    WBidiVisual(BiDiVisual),
    #[xml(tag = "w:tblW")]
    WTblW(TableWidth),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:tblInd")]
    WTblInd(TableIndentation),
    #[xml(tag = "w:tblBorders")]
    WTblBorders(TableBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tblLayout")]
    WTblLayout(TableLayout),
    #[xml(tag = "w:tblCellMar")]
    WTblCellMar(TableCellMarginDefault),
    #[xml(tag = "w:tblLook")]
    WTblLook(TableLook),
    #[xml(tag = "w:tblCaption")]
    WTblCaption(TableCaption),
    #[xml(tag = "w:tblDescription")]
    WTblDescription(TableDescription),
    #[xml(tag = "w:tblPrChange")]
    WTblPrChange(TablePropertiesChange),
}
/// Table Grid.
/// When the object is serialized out as xml, it's qualified name is w:tblGrid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblGrid")]
pub struct TableGrid {
    #[xml(child = "w:gridCol", child = "w:tblGridChange")]
    pub children: Vec<TableGridChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableGridChildChoice {
    #[xml(tag = "w:gridCol")]
    WGridCol(GridColumn),
    #[xml(tag = "w:tblGridChange")]
    WTblGridChange(TableGridChange),
}
/// Footnote Placement.
/// When the object is serialized out as xml, it's qualified name is w:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pos")]
pub struct FootnotePosition {
    /// Footnote Position Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: FootnotePositionValues,
}
/// Footnote Numbering Format.
/// When the object is serialized out as xml, it's qualified name is w:numFmt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numFmt")]
pub struct NumberingFormat {
    /// Numbering Format Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: NumberFormatValues,
    /// format
    /// Represents the following attribute in the schema: w:format
    #[xml(attr = "w:format")]
    pub format: Option<String>,
}
/// Endnote Placement.
/// When the object is serialized out as xml, it's qualified name is w:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pos")]
pub struct EndnotePosition {
    /// Endnote Position Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: EndnotePositionValues,
}
/// Special Footnote List.
/// When the object is serialized out as xml, it's qualified name is w:footnote.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnote")]
pub struct FootnoteSpecialReference {
    /// Footnote/Endnote ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
}
/// Special Endnote List.
/// When the object is serialized out as xml, it's qualified name is w:endnote.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnote")]
pub struct EndnoteSpecialReference {
    /// Footnote/Endnote ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
}
/// Defines the FootnoteEndnoteSeparatorReferenceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct FootnoteEndnoteSeparatorReferenceType {
    /// Footnote/Endnote ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
}
/// Index of Column Containing Unique Values for Record.
/// When the object is serialized out as xml, it's qualified name is w:column.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:column")]
pub struct ColumnIndex {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Column Delimiter for Data Source.
/// When the object is serialized out as xml, it's qualified name is w:colDelim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:colDelim")]
pub struct ColumnDelimiter {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the UnsignedDecimalNumberType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UnsignedDecimalNumberType {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Unique Value for Record.
/// When the object is serialized out as xml, it's qualified name is w:uniqueTag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:uniqueTag")]
pub struct UniqueTag {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Data About Single Data Source Record.
/// When the object is serialized out as xml, it's qualified name is w:recipientData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:recipientData")]
pub struct RecipientData {
    ///Record Is Included in Mail Merge
    #[xml(child = "w:active")]
    pub active: Option<Active>,
    ///Index of Column Containing Unique Values for Record
    #[xml(child = "w:column")]
    pub column_index: ColumnIndex,
    ///Unique Value for Record
    #[xml(child = "w:uniqueTag")]
    pub unique_tag: UniqueTag,
}
/// Merge Field Mapping.
/// When the object is serialized out as xml, it's qualified name is w:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:type")]
pub struct MailMergeFieldType {
    /// Merge Field Mapping Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: MailMergeOdsoFieldValues,
}
/// ODSO Data Source Type.
/// When the object is serialized out as xml, it's qualified name is w:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:type")]
pub struct MailMergeSource {
    /// Data Source Type Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: MailMergeSourceValues,
}
/// External Data Source to Merge Field Mapping.
/// When the object is serialized out as xml, it's qualified name is w:fieldMapData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:fieldMapData")]
pub struct FieldMapData {
    #[xml(
        child = "w:type",
        child = "w:name",
        child = "w:mappedName",
        child = "w:column",
        child = "w:lid",
        child = "w:dynamicAddress",
    )]
    pub children: Vec<FieldMapDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FieldMapDataChildChoice {
    #[xml(tag = "w:type")]
    WType(MailMergeFieldType),
    #[xml(tag = "w:name")]
    WName(Name),
    #[xml(tag = "w:mappedName")]
    WMappedName(MappedName),
    #[xml(tag = "w:column")]
    WColumn(ColumnIndex),
    #[xml(tag = "w:lid")]
    WLid(LanguageId),
    #[xml(tag = "w:dynamicAddress")]
    WDynamicAddress(DynamicAddress),
}
/// Source Document Type.
/// When the object is serialized out as xml, it's qualified name is w:mainDocumentType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mainDocumentType")]
pub struct MainDocumentType {
    /// Mail Merge Source Document Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: MailMergeDocumentValues,
}
/// Data Source Type.
/// When the object is serialized out as xml, it's qualified name is w:dataType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dataType")]
pub struct DataType {
    /// Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: MailMergeDataValues,
}
/// Merged Document Destination.
/// When the object is serialized out as xml, it's qualified name is w:destination.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:destination")]
pub struct Destination {
    /// Mail Merge Merged Document Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: MailMergeDestinationValues,
}
/// Office Data Source Object Settings.
/// When the object is serialized out as xml, it's qualified name is w:odso.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:odso")]
pub struct DataSourceObject {
    ///UDL Connection String
    #[xml(child = "w:udl")]
    pub udl_connection_string: Option<UdlConnectionString>,
    ///Data Source Table Name
    #[xml(child = "w:table")]
    pub data_source_table_name: Option<DataSourceTableName>,
    ///ODSO Data Source File Path
    #[xml(child = "w:src")]
    pub source_reference: Option<SourceReference>,
    ///Column Delimiter for Data Source
    #[xml(child = "w:colDelim")]
    pub column_delimiter: Option<ColumnDelimiter>,
    ///ODSO Data Source Type
    #[xml(child = "w:type")]
    pub mail_merge_source: Option<MailMergeSource>,
    ///First Row of Data Source Contains Column Names
    #[xml(child = "w:fHdr")]
    pub first_row_header: Option<FirstRowHeader>,
    /// _
    #[xml(child = "w:fieldMapData")]
    pub w_field_map_data: Vec<FieldMapData>,
    /// _
    #[xml(child = "w:recipientData")]
    pub w_recipient_data: Option<RecipientDataReference>,
}
/// Single Document Variable.
/// When the object is serialized out as xml, it's qualified name is w:docVar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docVar")]
pub struct DocumentVariable {
    /// Document Variable Name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// Document Variable Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Original Document Revision Save ID.
/// When the object is serialized out as xml, it's qualified name is w:rsidRoot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rsidRoot")]
pub struct RsidRoot {
    /// Long Hexadecimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Single Session Revision Save ID.
/// When the object is serialized out as xml, it's qualified name is w:rsid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rsid")]
pub struct Rsid {
    /// Long Hexadecimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Abstract Numbering Definition Identifier.
/// When the object is serialized out as xml, it's qualified name is w:nsid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:nsid")]
pub struct Nsid {
    /// Long Hexadecimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Numbering Template Code.
/// When the object is serialized out as xml, it's qualified name is w:tmpl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tmpl")]
pub struct TemplateCode {
    /// Long Hexadecimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the LongHexNumberType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LongHexNumberType {
    /// Long Hexadecimal Number Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct RunPropertiesBaseStyle {
    #[xml(
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
    )]
    pub children: Vec<RunPropertiesBaseStyleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunPropertiesBaseStyleChildChoice {
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
}
/// Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is w:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPr")]
pub struct ParagraphPropertiesBaseStyle {
    #[xml(
        child = "w:keepNext",
        child = "w:keepLines",
        child = "w:pageBreakBefore",
        child = "w:framePr",
        child = "w:widowControl",
        child = "w:numPr",
        child = "w:suppressLineNumbers",
        child = "w:pBdr",
        child = "w:shd",
        child = "w:tabs",
        child = "w:suppressAutoHyphens",
        child = "w:kinsoku",
        child = "w:wordWrap",
        child = "w:overflowPunct",
        child = "w:topLinePunct",
        child = "w:autoSpaceDE",
        child = "w:autoSpaceDN",
        child = "w:bidi",
        child = "w:adjustRightInd",
        child = "w:snapToGrid",
        child = "w:spacing",
        child = "w:ind",
        child = "w:contextualSpacing",
        child = "w:mirrorIndents",
        child = "w:suppressOverlap",
        child = "w:jc",
        child = "w:textDirection",
        child = "w:textAlignment",
        child = "w:textboxTightWrap",
        child = "w:outlineLvl",
    )]
    pub children: Vec<ParagraphPropertiesBaseStyleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphPropertiesBaseStyleChildChoice {
    #[xml(tag = "w:keepNext")]
    WKeepNext(KeepNext),
    #[xml(tag = "w:keepLines")]
    WKeepLines(KeepLines),
    #[xml(tag = "w:pageBreakBefore")]
    WPageBreakBefore(PageBreakBefore),
    #[xml(tag = "w:framePr")]
    WFramePr(FrameProperties),
    #[xml(tag = "w:widowControl")]
    WWidowControl(WidowControl),
    #[xml(tag = "w:numPr")]
    WNumPr(NumberingProperties),
    #[xml(tag = "w:suppressLineNumbers")]
    WSuppressLineNumbers(SuppressLineNumbers),
    #[xml(tag = "w:pBdr")]
    WPBdr(ParagraphBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tabs")]
    WTabs(Tabs),
    #[xml(tag = "w:suppressAutoHyphens")]
    WSuppressAutoHyphens(SuppressAutoHyphens),
    #[xml(tag = "w:kinsoku")]
    WKinsoku(Kinsoku),
    #[xml(tag = "w:wordWrap")]
    WWordWrap(WordWrap),
    #[xml(tag = "w:overflowPunct")]
    WOverflowPunct(OverflowPunctuation),
    #[xml(tag = "w:topLinePunct")]
    WTopLinePunct(TopLinePunctuation),
    #[xml(tag = "w:autoSpaceDE")]
    WAutoSpaceDe(AutoSpaceDe),
    #[xml(tag = "w:autoSpaceDN")]
    WAutoSpaceDn(AutoSpaceDn),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:adjustRightInd")]
    WAdjustRightInd(AdjustRightIndent),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:spacing")]
    WSpacing(SpacingBetweenLines),
    #[xml(tag = "w:ind")]
    WInd(Indentation),
    #[xml(tag = "w:contextualSpacing")]
    WContextualSpacing(ContextualSpacing),
    #[xml(tag = "w:mirrorIndents")]
    WMirrorIndents(MirrorIndents),
    #[xml(tag = "w:suppressOverlap")]
    WSuppressOverlap(SuppressOverlap),
    #[xml(tag = "w:jc")]
    WJc(Justification),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:textAlignment")]
    WTextAlignment(TextAlignment),
    #[xml(tag = "w:textboxTightWrap")]
    WTextboxTightWrap(TextBoxTightWrap),
    #[xml(tag = "w:outlineLvl")]
    WOutlineLvl(OutlineLevel),
}
/// Default Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:rPrDefault.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPrDefault")]
pub struct RunPropertiesDefault {
    ///Run Properties
    #[xml(child = "w:rPr")]
    pub run_properties_base_style: Option<RunPropertiesBaseStyle>,
}
/// Default Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is w:pPrDefault.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPrDefault")]
pub struct ParagraphPropertiesDefault {
    ///Paragraph Properties
    #[xml(child = "w:pPr")]
    pub paragraph_properties_base_style: Option<ParagraphPropertiesBaseStyle>,
}
/// Left and Right Margin for Frame.
/// When the object is serialized out as xml, it's qualified name is w:marW.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:marW")]
pub struct MarginWidth {
    /// Measurement in Pixels
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Top and Bottom Margin for Frame.
/// When the object is serialized out as xml, it's qualified name is w:marH.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:marH")]
pub struct MarginHeight {
    /// Measurement in Pixels
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the PixelsMeasureType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PixelsMeasureType {
    /// Measurement in Pixels
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Scrollbar Display Option.
/// When the object is serialized out as xml, it's qualified name is w:scrollbar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:scrollbar")]
pub struct ScrollbarVisibility {
    /// Scrollbar Display Option Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: FrameScrollbarVisibilityValues,
}
/// Frameset Splitter Width.
/// When the object is serialized out as xml, it's qualified name is w:w.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:w")]
pub struct Width {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Hyphenation Zone.
/// When the object is serialized out as xml, it's qualified name is w:hyphenationZone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hyphenationZone")]
pub struct HyphenationZone {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Drawing Grid Horizontal Grid Unit Size.
/// When the object is serialized out as xml, it's qualified name is w:drawingGridHorizontalSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:drawingGridHorizontalSpacing")]
pub struct DrawingGridHorizontalSpacing {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Drawing Grid Vertical Grid Unit Size.
/// When the object is serialized out as xml, it's qualified name is w:drawingGridVerticalSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:drawingGridVerticalSpacing")]
pub struct DrawingGridVerticalSpacing {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Drawing Grid Horizontal Origin Point.
/// When the object is serialized out as xml, it's qualified name is w:drawingGridHorizontalOrigin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:drawingGridHorizontalOrigin")]
pub struct DrawingGridHorizontalOrigin {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Drawing Grid Vertical Origin Point.
/// When the object is serialized out as xml, it's qualified name is w:drawingGridVerticalOrigin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:drawingGridVerticalOrigin")]
pub struct DrawingGridVerticalOrigin {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the TwipsMeasureType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TwipsMeasureType {
    /// Measurement in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Frameset Splitter Properties.
/// When the object is serialized out as xml, it's qualified name is w:framesetSplitbar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:framesetSplitbar")]
pub struct FramesetSplitbar {
    ///Frameset Splitter Width
    #[xml(child = "w:w")]
    pub width: Option<Width>,
    ///Frameset Splitter Color
    #[xml(child = "w:color")]
    pub color: Option<Color>,
    ///Do Not Display Frameset Splitters
    #[xml(child = "w:noBorder")]
    pub no_border: Option<NoBorder>,
    ///Frameset Splitter Border Style
    #[xml(child = "w:flatBorders")]
    pub flat_borders: Option<FlatBorders>,
}
/// Frameset Layout.
/// When the object is serialized out as xml, it's qualified name is w:frameLayout.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:frameLayout")]
pub struct FrameLayout {
    /// Frameset Layout Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: FrameLayoutValues,
}
/// Nested Frameset Definition.
/// When the object is serialized out as xml, it's qualified name is w:frameset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:frameset")]
pub struct Frameset {
    #[xml(
        child = "w:sz",
        child = "w:framesetSplitbar",
        child = "w:frameLayout",
        child = "w:frameset",
        child = "w:frame",
    )]
    pub children: Vec<FramesetChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FramesetChildChoice {
    #[xml(tag = "w:sz")]
    WSz(FrameSize),
    #[xml(tag = "w:framesetSplitbar")]
    WFramesetSplitbar(FramesetSplitbar),
    #[xml(tag = "w:frameLayout")]
    WFrameLayout(FrameLayout),
    #[xml(tag = "w:frameset")]
    WFrameset(Frameset),
    #[xml(tag = "w:frame")]
    WFrame(Frame),
}
/// Single Frame Properties.
/// When the object is serialized out as xml, it's qualified name is w:frame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:frame")]
pub struct Frame {
    ///Frame Size
    #[xml(child = "w:sz")]
    pub frame_size: Option<FrameSize>,
    ///Frame Name
    #[xml(child = "w:name")]
    pub frame_name: Option<FrameName>,
    ///Source File for Frame
    #[xml(child = "w:sourceFileName")]
    pub source_file_reference: Option<SourceFileReference>,
    ///Left and Right Margin for Frame
    #[xml(child = "w:marW")]
    pub margin_width: Option<MarginWidth>,
    ///Top and Bottom Margin for Frame
    #[xml(child = "w:marH")]
    pub margin_height: Option<MarginHeight>,
    ///Scrollbar Display Option
    #[xml(child = "w:scrollbar")]
    pub scrollbar_visibility: Option<ScrollbarVisibility>,
    ///Frame Cannot Be Resized
    #[xml(child = "w:noResizeAllowed")]
    pub no_resize_allowed: Option<NoResizeAllowed>,
    ///Maintain Link to Existing File
    #[xml(child = "w:linkedToFile")]
    pub linked_to_file: Option<LinkedToFile>,
}
/// Content Between Numbering Symbol and Paragraph Text.
/// When the object is serialized out as xml, it's qualified name is w:suff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:suff")]
pub struct LevelSuffix {
    /// Character Type Between Numbering and Text
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: LevelSuffixValues,
}
/// Numbering Level Text.
/// When the object is serialized out as xml, it's qualified name is w:lvlText.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lvlText")]
pub struct LevelText {
    /// Level Text
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
    /// Level Text Is Null Character
    /// Represents the following attribute in the schema: w:null
    #[xml(attr = "w:null")]
    pub null: Option<bool>,
}
/// Legacy Numbering Level Properties.
/// When the object is serialized out as xml, it's qualified name is w:legacy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:legacy")]
pub struct LegacyNumbering {
    /// Use Legacy Numbering Properties
    /// Represents the following attribute in the schema: w:legacy
    #[xml(attr = "w:legacy")]
    pub legacy: Option<bool>,
    /// Legacy Spacing
    /// Represents the following attribute in the schema: w:legacySpace
    #[xml(attr = "w:legacySpace")]
    pub legacy_space: Option<String>,
    /// Legacy Indent
    /// Represents the following attribute in the schema: w:legacyIndent
    #[xml(attr = "w:legacyIndent")]
    pub legacy_indent: Option<String>,
}
/// Justification.
/// When the object is serialized out as xml, it's qualified name is w:lvlJc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lvlJc")]
pub struct LevelJustification {
    /// Alignment Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: LevelJustificationValues,
}
/// Numbering Level Associated Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is w:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPr")]
pub struct PreviousParagraphProperties {
    #[xml(
        child = "w:pStyle",
        child = "w:keepNext",
        child = "w:keepLines",
        child = "w:pageBreakBefore",
        child = "w:framePr",
        child = "w:widowControl",
        child = "w:numPr",
        child = "w:suppressLineNumbers",
        child = "w:pBdr",
        child = "w:shd",
        child = "w:tabs",
        child = "w:suppressAutoHyphens",
        child = "w:kinsoku",
        child = "w:wordWrap",
        child = "w:overflowPunct",
        child = "w:topLinePunct",
        child = "w:autoSpaceDE",
        child = "w:autoSpaceDN",
        child = "w:bidi",
        child = "w:adjustRightInd",
        child = "w:snapToGrid",
        child = "w:spacing",
        child = "w:ind",
        child = "w:contextualSpacing",
        child = "w:mirrorIndents",
        child = "w:suppressOverlap",
        child = "w:jc",
        child = "w:textDirection",
        child = "w:textAlignment",
        child = "w:textboxTightWrap",
        child = "w:outlineLvl",
    )]
    pub children: Vec<PreviousParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PreviousParagraphPropertiesChildChoice {
    #[xml(tag = "w:pStyle")]
    WPStyle(ParagraphStyleId),
    #[xml(tag = "w:keepNext")]
    WKeepNext(KeepNext),
    #[xml(tag = "w:keepLines")]
    WKeepLines(KeepLines),
    #[xml(tag = "w:pageBreakBefore")]
    WPageBreakBefore(PageBreakBefore),
    #[xml(tag = "w:framePr")]
    WFramePr(FrameProperties),
    #[xml(tag = "w:widowControl")]
    WWidowControl(WidowControl),
    #[xml(tag = "w:numPr")]
    WNumPr(NumberingProperties),
    #[xml(tag = "w:suppressLineNumbers")]
    WSuppressLineNumbers(SuppressLineNumbers),
    #[xml(tag = "w:pBdr")]
    WPBdr(ParagraphBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tabs")]
    WTabs(Tabs),
    #[xml(tag = "w:suppressAutoHyphens")]
    WSuppressAutoHyphens(SuppressAutoHyphens),
    #[xml(tag = "w:kinsoku")]
    WKinsoku(Kinsoku),
    #[xml(tag = "w:wordWrap")]
    WWordWrap(WordWrap),
    #[xml(tag = "w:overflowPunct")]
    WOverflowPunct(OverflowPunctuation),
    #[xml(tag = "w:topLinePunct")]
    WTopLinePunct(TopLinePunctuation),
    #[xml(tag = "w:autoSpaceDE")]
    WAutoSpaceDe(AutoSpaceDe),
    #[xml(tag = "w:autoSpaceDN")]
    WAutoSpaceDn(AutoSpaceDn),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:adjustRightInd")]
    WAdjustRightInd(AdjustRightIndent),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:spacing")]
    WSpacing(SpacingBetweenLines),
    #[xml(tag = "w:ind")]
    WInd(Indentation),
    #[xml(tag = "w:contextualSpacing")]
    WContextualSpacing(ContextualSpacing),
    #[xml(tag = "w:mirrorIndents")]
    WMirrorIndents(MirrorIndents),
    #[xml(tag = "w:suppressOverlap")]
    WSuppressOverlap(SuppressOverlap),
    #[xml(tag = "w:jc")]
    WJc(Justification),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:textAlignment")]
    WTextAlignment(TextAlignment),
    #[xml(tag = "w:textboxTightWrap")]
    WTextboxTightWrap(TextBoxTightWrap),
    #[xml(tag = "w:outlineLvl")]
    WOutlineLvl(OutlineLevel),
}
/// Numbering Symbol Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct NumberingSymbolRunProperties {
    #[xml(
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:rtl",
        child = "w:cs",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
    )]
    pub children: Vec<NumberingSymbolRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NumberingSymbolRunPropertiesChildChoice {
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:rtl")]
    WRtl(RightToLeftText),
    #[xml(tag = "w:cs")]
    WCs(ComplexScript),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
}
/// Abstract Numbering Definition Type.
/// When the object is serialized out as xml, it's qualified name is w:multiLevelType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:multiLevelType")]
pub struct MultiLevelType {
    /// Abstract Numbering Definition Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: MultiLevelValues,
}
/// Numbering Level Definition.
/// When the object is serialized out as xml, it's qualified name is w:lvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lvl")]
pub struct Level {
    /// Numbering Level
    /// Represents the following attribute in the schema: w:ilvl
    #[xml(attr = "w:ilvl")]
    pub level_index: i32,
    /// Template Code
    /// Represents the following attribute in the schema: w:tplc
    #[xml(attr = "w:tplc")]
    pub template_code: Option<String>,
    /// Tentative Numbering
    /// Represents the following attribute in the schema: w:tentative
    #[xml(attr = "w:tentative")]
    pub tentative: Option<bool>,
    ///Starting Value
    #[xml(child = "w:start")]
    pub start_numbering_value: Option<StartNumberingValue>,
    ///Numbering Format
    #[xml(child = "w:numFmt")]
    pub numbering_format: Option<NumberingFormat>,
    ///Restart Numbering Level Symbol
    #[xml(child = "w:lvlRestart")]
    pub level_restart: Option<LevelRestart>,
    ///Paragraph Style's Associated Numbering Level
    #[xml(child = "w:pStyle")]
    pub paragraph_style_id_in_level: Option<ParagraphStyleIdInLevel>,
    ///Display All Levels Using Arabic Numerals
    #[xml(child = "w:isLgl")]
    pub is_legal_numbering_style: Option<IsLegalNumberingStyle>,
    ///Content Between Numbering Symbol and Paragraph Text
    #[xml(child = "w:suff")]
    pub level_suffix: Option<LevelSuffix>,
    ///Numbering Level Text
    #[xml(child = "w:lvlText")]
    pub level_text: Option<LevelText>,
    ///Picture Numbering Symbol Definition Reference
    #[xml(child = "w:lvlPicBulletId")]
    pub level_picture_bullet_id: Option<LevelPictureBulletId>,
    ///Legacy Numbering Level Properties
    #[xml(child = "w:legacy")]
    pub legacy_numbering: Option<LegacyNumbering>,
    ///Justification
    #[xml(child = "w:lvlJc")]
    pub level_justification: Option<LevelJustification>,
    ///Numbering Level Associated Paragraph Properties
    #[xml(child = "w:pPr")]
    pub previous_paragraph_properties: Option<PreviousParagraphProperties>,
    ///Numbering Symbol Run Properties
    #[xml(child = "w:rPr")]
    pub numbering_symbol_run_properties: Option<NumberingSymbolRunProperties>,
}
/// Picture Numbering Symbol Definition.
/// When the object is serialized out as xml, it's qualified name is w:numPicBullet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:numPicBullet")]
pub struct NumberingPictureBullet {
    /// numPicBulletId
    /// Represents the following attribute in the schema: w:numPicBulletId
    #[xml(attr = "w:numPicBulletId")]
    pub numbering_picture_bullet_id: i32,
    #[xml(child = "w:pict", child = "w:drawing")]
    pub children: Vec<NumberingPictureBulletChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NumberingPictureBulletChildChoice {
    #[xml(tag = "w:pict")]
    WPict(PictureBulletBase),
    #[xml(tag = "w:drawing")]
    WDrawing(Drawing),
}
/// Abstract Numbering Definition.
/// When the object is serialized out as xml, it's qualified name is w:abstractNum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:abstractNum")]
pub struct AbstractNum {
    /// Abstract Numbering Definition ID
    /// Represents the following attribute in the schema: w:abstractNumId
    #[xml(attr = "w:abstractNumId")]
    pub abstract_number_id: i32,
    ///Abstract Numbering Definition Identifier
    #[xml(child = "w:nsid")]
    pub nsid: Option<Nsid>,
    ///Abstract Numbering Definition Type
    #[xml(child = "w:multiLevelType")]
    pub multi_level_type: Option<MultiLevelType>,
    ///Numbering Template Code
    #[xml(child = "w:tmpl")]
    pub template_code: Option<TemplateCode>,
    ///Abstract Numbering Definition Name
    #[xml(child = "w:name")]
    pub abstract_num_definition_name: Option<AbstractNumDefinitionName>,
    ///Numbering Style Definition
    #[xml(child = "w:styleLink")]
    pub style_link: Option<StyleLink>,
    ///Numbering Style Reference
    #[xml(child = "w:numStyleLink")]
    pub numbering_style_link: Option<NumberingStyleLink>,
    /// _
    #[xml(child = "w:lvl")]
    pub w_lvl: Vec<Level>,
}
/// Numbering Definition Instance.
/// When the object is serialized out as xml, it's qualified name is w:num.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:num")]
pub struct NumberingInstance {
    /// numId
    /// Represents the following attribute in the schema: w:numId
    #[xml(attr = "w:numId")]
    pub number_id: i32,
    /// durableId
    /// Represents the following attribute in the schema: w:durableId
    #[xml(attr = "w:durableId")]
    pub w_durable_id: Option<i32>,
    /// _
    #[xml(child = "w:abstractNumId")]
    pub abstract_num_id: AbstractNumId,
    /// _
    #[xml(child = "w:lvlOverride")]
    pub w_lvl_override: Vec<LevelOverride>,
}
/// Table Style Conditional Formatting Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is w:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPr")]
pub struct StyleParagraphProperties {
    #[xml(
        child = "w:keepNext",
        child = "w:keepLines",
        child = "w:pageBreakBefore",
        child = "w:framePr",
        child = "w:widowControl",
        child = "w:numPr",
        child = "w:suppressLineNumbers",
        child = "w:pBdr",
        child = "w:shd",
        child = "w:tabs",
        child = "w:suppressAutoHyphens",
        child = "w:kinsoku",
        child = "w:wordWrap",
        child = "w:overflowPunct",
        child = "w:topLinePunct",
        child = "w:autoSpaceDE",
        child = "w:autoSpaceDN",
        child = "w:bidi",
        child = "w:adjustRightInd",
        child = "w:snapToGrid",
        child = "w:spacing",
        child = "w:ind",
        child = "w:contextualSpacing",
        child = "w:mirrorIndents",
        child = "w:suppressOverlap",
        child = "w:jc",
        child = "w:textDirection",
        child = "w:textAlignment",
        child = "w:textboxTightWrap",
        child = "w:outlineLvl",
        child = "w:pPrChange",
    )]
    pub children: Vec<StyleParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleParagraphPropertiesChildChoice {
    #[xml(tag = "w:keepNext")]
    WKeepNext(KeepNext),
    #[xml(tag = "w:keepLines")]
    WKeepLines(KeepLines),
    #[xml(tag = "w:pageBreakBefore")]
    WPageBreakBefore(PageBreakBefore),
    #[xml(tag = "w:framePr")]
    WFramePr(FrameProperties),
    #[xml(tag = "w:widowControl")]
    WWidowControl(WidowControl),
    #[xml(tag = "w:numPr")]
    WNumPr(NumberingProperties),
    #[xml(tag = "w:suppressLineNumbers")]
    WSuppressLineNumbers(SuppressLineNumbers),
    #[xml(tag = "w:pBdr")]
    WPBdr(ParagraphBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tabs")]
    WTabs(Tabs),
    #[xml(tag = "w:suppressAutoHyphens")]
    WSuppressAutoHyphens(SuppressAutoHyphens),
    #[xml(tag = "w:kinsoku")]
    WKinsoku(Kinsoku),
    #[xml(tag = "w:wordWrap")]
    WWordWrap(WordWrap),
    #[xml(tag = "w:overflowPunct")]
    WOverflowPunct(OverflowPunctuation),
    #[xml(tag = "w:topLinePunct")]
    WTopLinePunct(TopLinePunctuation),
    #[xml(tag = "w:autoSpaceDE")]
    WAutoSpaceDe(AutoSpaceDe),
    #[xml(tag = "w:autoSpaceDN")]
    WAutoSpaceDn(AutoSpaceDn),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:adjustRightInd")]
    WAdjustRightInd(AdjustRightIndent),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:spacing")]
    WSpacing(SpacingBetweenLines),
    #[xml(tag = "w:ind")]
    WInd(Indentation),
    #[xml(tag = "w:contextualSpacing")]
    WContextualSpacing(ContextualSpacing),
    #[xml(tag = "w:mirrorIndents")]
    WMirrorIndents(MirrorIndents),
    #[xml(tag = "w:suppressOverlap")]
    WSuppressOverlap(SuppressOverlap),
    #[xml(tag = "w:jc")]
    WJc(Justification),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:textAlignment")]
    WTextAlignment(TextAlignment),
    #[xml(tag = "w:textboxTightWrap")]
    WTextboxTightWrap(TextBoxTightWrap),
    #[xml(tag = "w:outlineLvl")]
    WOutlineLvl(OutlineLevel),
    #[xml(tag = "w:pPrChange")]
    WPPrChange(ParagraphPropertiesChange),
}
/// Table Style Conditional Formatting Table Properties.
/// When the object is serialized out as xml, it's qualified name is w:tblPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPr")]
pub struct TableStyleConditionalFormattingTableProperties {
    #[xml(
        child = "w:jc",
        child = "w:tblCellSpacing",
        child = "w:tblInd",
        child = "w:tblBorders",
        child = "w:shd",
        child = "w:tblCellMar",
    )]
    pub children: Vec<TableStyleConditionalFormattingTablePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableStyleConditionalFormattingTablePropertiesChildChoice {
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:tblInd")]
    WTblInd(TableIndentation),
    #[xml(tag = "w:tblBorders")]
    WTblBorders(TableBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tblCellMar")]
    WTblCellMar(TableCellMarginDefault),
}
/// Table Style Conditional Formatting Table Row Properties.
/// When the object is serialized out as xml, it's qualified name is w:trPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:trPr")]
pub struct TableStyleConditionalFormattingTableRowProperties {
    #[xml(
        child = "w:hidden",
        child = "w:cantSplit",
        child = "w:tblHeader",
        child = "w:tblCellSpacing",
        child = "w:jc",
    )]
    pub children: Vec<TableStyleConditionalFormattingTableRowPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableStyleConditionalFormattingTableRowPropertiesChildChoice {
    #[xml(tag = "w:hidden")]
    WHidden(Hidden),
    #[xml(tag = "w:cantSplit")]
    WCantSplit(CantSplit),
    #[xml(tag = "w:tblHeader")]
    WTblHeader(TableHeader),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
}
/// Table Style Conditional Formatting Table Cell Properties.
/// When the object is serialized out as xml, it's qualified name is w:tcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcPr")]
pub struct TableStyleConditionalFormattingTableCellProperties {
    #[xml(
        child = "w:tcBorders",
        child = "w:shd",
        child = "w:noWrap",
        child = "w:tcMar",
        child = "w:vAlign",
    )]
    pub children: Vec<TableStyleConditionalFormattingTableCellPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableStyleConditionalFormattingTableCellPropertiesChildChoice {
    #[xml(tag = "w:tcBorders")]
    WTcBorders(TableCellBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:noWrap")]
    WNoWrap(NoWrap),
    #[xml(tag = "w:tcMar")]
    WTcMar(TableCellMargin),
    #[xml(tag = "w:vAlign")]
    WVAlign(TableCellVerticalAlignment),
}
/// Primary Style Name.
/// When the object is serialized out as xml, it's qualified name is w:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:name")]
pub struct StyleName {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Optional User Interface Sorting Order.
/// When the object is serialized out as xml, it's qualified name is w:uiPriority.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:uiPriority")]
pub struct UiPriority {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rPr")]
pub struct StyleRunProperties {
    #[xml(
        child = "w:rFonts",
        child = "w:b",
        child = "w:bCs",
        child = "w:i",
        child = "w:iCs",
        child = "w:caps",
        child = "w:smallCaps",
        child = "w:strike",
        child = "w:dstrike",
        child = "w:outline",
        child = "w:shadow",
        child = "w:emboss",
        child = "w:imprint",
        child = "w:noProof",
        child = "w:snapToGrid",
        child = "w:vanish",
        child = "w:webHidden",
        child = "w:color",
        child = "w:spacing",
        child = "w:w",
        child = "w:kern",
        child = "w:position",
        child = "w:sz",
        child = "w:szCs",
        child = "w:u",
        child = "w:effect",
        child = "w:bdr",
        child = "w:shd",
        child = "w:fitText",
        child = "w:vertAlign",
        child = "w:em",
        child = "w:lang",
        child = "w:eastAsianLayout",
        child = "w:specVanish",
        child = "w:rPrChange",
    )]
    pub children: Vec<StyleRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleRunPropertiesChildChoice {
    #[xml(tag = "w:rFonts")]
    WRFonts(RunFonts),
    #[xml(tag = "w:b")]
    WB(Bold),
    #[xml(tag = "w:bCs")]
    WBCs(BoldComplexScript),
    #[xml(tag = "w:i")]
    WI(Italic),
    #[xml(tag = "w:iCs")]
    WICs(ItalicComplexScript),
    #[xml(tag = "w:caps")]
    WCaps(Caps),
    #[xml(tag = "w:smallCaps")]
    WSmallCaps(SmallCaps),
    #[xml(tag = "w:strike")]
    WStrike(Strike),
    #[xml(tag = "w:dstrike")]
    WDstrike(DoubleStrike),
    #[xml(tag = "w:outline")]
    WOutline(Outline),
    #[xml(tag = "w:shadow")]
    WShadow(Shadow),
    #[xml(tag = "w:emboss")]
    WEmboss(Emboss),
    #[xml(tag = "w:imprint")]
    WImprint(Imprint),
    #[xml(tag = "w:noProof")]
    WNoProof(NoProof),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:vanish")]
    WVanish(Vanish),
    #[xml(tag = "w:webHidden")]
    WWebHidden(WebHidden),
    #[xml(tag = "w:color")]
    WColor(Color),
    #[xml(tag = "w:spacing")]
    WSpacing(Spacing),
    #[xml(tag = "w:w")]
    WW(CharacterScale),
    #[xml(tag = "w:kern")]
    WKern(Kern),
    #[xml(tag = "w:position")]
    WPosition(Position),
    #[xml(tag = "w:sz")]
    WSz(FontSize),
    #[xml(tag = "w:szCs")]
    WSzCs(FontSizeComplexScript),
    #[xml(tag = "w:u")]
    WU(Underline),
    #[xml(tag = "w:effect")]
    WEffect(TextEffect),
    #[xml(tag = "w:bdr")]
    WBdr(Border),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:fitText")]
    WFitText(FitText),
    #[xml(tag = "w:vertAlign")]
    WVertAlign(VerticalTextAlignment),
    #[xml(tag = "w:em")]
    WEm(Emphasis),
    #[xml(tag = "w:lang")]
    WLang(Languages),
    #[xml(tag = "w:eastAsianLayout")]
    WEastAsianLayout(EastAsianLayout),
    #[xml(tag = "w:specVanish")]
    WSpecVanish(SpecVanish),
    #[xml(tag = "w:rPrChange")]
    WRPrChange(RunPropertiesChange),
}
/// Style Table Properties.
/// When the object is serialized out as xml, it's qualified name is w:tblPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPr")]
pub struct StyleTableProperties {
    #[xml(
        child = "w:tblStyleRowBandSize",
        child = "w:tblStyleColBandSize",
        child = "w:jc",
        child = "w:tblCellSpacing",
        child = "w:tblInd",
        child = "w:tblBorders",
        child = "w:shd",
        child = "w:tblCellMar",
    )]
    pub children: Vec<StyleTablePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleTablePropertiesChildChoice {
    #[xml(tag = "w:tblStyleRowBandSize")]
    WTblStyleRowBandSize(TableStyleRowBandSize),
    #[xml(tag = "w:tblStyleColBandSize")]
    WTblStyleColBandSize(TableStyleColumnBandSize),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:tblInd")]
    WTblInd(TableIndentation),
    #[xml(tag = "w:tblBorders")]
    WTblBorders(TableBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tblCellMar")]
    WTblCellMar(TableCellMarginDefault),
}
/// Style Table Cell Properties.
/// When the object is serialized out as xml, it's qualified name is w:tcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tcPr")]
pub struct StyleTableCellProperties {
    #[xml(child = "w:shd", child = "w:noWrap", child = "w:tcMar", child = "w:vAlign")]
    pub children: Vec<StyleTableCellPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleTableCellPropertiesChildChoice {
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:noWrap")]
    WNoWrap(NoWrap),
    #[xml(tag = "w:tcMar")]
    WTcMar(TableCellMargin),
    #[xml(tag = "w:vAlign")]
    WVAlign(TableCellVerticalAlignment),
}
/// Style Conditional Table Formatting Properties.
/// When the object is serialized out as xml, it's qualified name is w:tblStylePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblStylePr")]
pub struct TableStyleProperties {
    /// Table Style Conditional Formatting Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: TableStyleOverrideValues,
    ///Table Style Conditional Formatting Paragraph Properties
    #[xml(child = "w:pPr")]
    pub style_paragraph_properties: Option<StyleParagraphProperties>,
    ///Table Style Conditional Formatting Run Properties
    #[xml(child = "w:rPr")]
    pub run_properties_base_style: Option<RunPropertiesBaseStyle>,
    ///Table Style Conditional Formatting Table Properties
    #[xml(child = "w:tblPr")]
    pub table_style_conditional_formatting_table_properties: Option<
        TableStyleConditionalFormattingTableProperties,
    >,
    ///Table Style Conditional Formatting Table Row Properties
    #[xml(child = "w:trPr")]
    pub table_style_conditional_formatting_table_row_properties: Option<
        TableStyleConditionalFormattingTableRowProperties,
    >,
    ///Table Style Conditional Formatting Table Cell Properties
    #[xml(child = "w:tcPr")]
    pub table_style_conditional_formatting_table_cell_properties: Option<
        TableStyleConditionalFormattingTableCellProperties,
    >,
}
/// Latent Style Exception.
/// When the object is serialized out as xml, it's qualified name is w:lsdException.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lsdException")]
pub struct LatentStyleExceptionInfo {
    /// Primary Style Name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// Latent Style Locking Setting
    /// Represents the following attribute in the schema: w:locked
    #[xml(attr = "w:locked")]
    pub locked: Option<bool>,
    /// Override default sorting order
    /// Represents the following attribute in the schema: w:uiPriority
    #[xml(attr = "w:uiPriority")]
    pub ui_priority: Option<i32>,
    /// Semi hidden text override
    /// Represents the following attribute in the schema: w:semiHidden
    #[xml(attr = "w:semiHidden")]
    pub semi_hidden: Option<bool>,
    /// Unhide when used
    /// Represents the following attribute in the schema: w:unhideWhenUsed
    #[xml(attr = "w:unhideWhenUsed")]
    pub unhide_when_used: Option<bool>,
    /// Latent Style Primary Style Setting
    /// Represents the following attribute in the schema: w:qFormat
    #[xml(attr = "w:qFormat")]
    pub primary_style: Option<bool>,
}
/// Document Default Paragraph and Run Properties.
/// When the object is serialized out as xml, it's qualified name is w:docDefaults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docDefaults")]
pub struct DocDefaults {
    ///Default Run Properties
    #[xml(child = "w:rPrDefault")]
    pub run_properties_default: Option<RunPropertiesDefault>,
    ///Default Paragraph Properties
    #[xml(child = "w:pPrDefault")]
    pub paragraph_properties_default: Option<ParagraphPropertiesDefault>,
}
/// Latent Style Information.
/// When the object is serialized out as xml, it's qualified name is w:latentStyles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:latentStyles")]
pub struct LatentStyles {
    /// Default Style Locking Setting
    /// Represents the following attribute in the schema: w:defLockedState
    #[xml(attr = "w:defLockedState")]
    pub default_locked_state: Option<bool>,
    /// Default User Interface Priority Setting
    /// Represents the following attribute in the schema: w:defUIPriority
    #[xml(attr = "w:defUIPriority")]
    pub default_ui_priority: Option<i32>,
    /// Default Semi-Hidden Setting
    /// Represents the following attribute in the schema: w:defSemiHidden
    #[xml(attr = "w:defSemiHidden")]
    pub default_semi_hidden: Option<bool>,
    /// Default Hidden Until Used Setting
    /// Represents the following attribute in the schema: w:defUnhideWhenUsed
    #[xml(attr = "w:defUnhideWhenUsed")]
    pub default_unhide_when_used: Option<bool>,
    /// Default Primary Style Setting
    /// Represents the following attribute in the schema: w:defQFormat
    #[xml(attr = "w:defQFormat")]
    pub default_primary_style: Option<bool>,
    /// Latent Style Count
    /// Represents the following attribute in the schema: w:count
    #[xml(attr = "w:count")]
    pub count: Option<i32>,
    /// _
    #[xml(child = "w:lsdException")]
    pub w_lsd_exception: Vec<LatentStyleExceptionInfo>,
}
/// Style Definition.
/// When the object is serialized out as xml, it's qualified name is w:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:style")]
pub struct Style {
    /// Style Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<StyleValues>,
    /// Style ID
    /// Represents the following attribute in the schema: w:styleId
    #[xml(attr = "w:styleId")]
    pub style_id: Option<String>,
    /// Default Style
    /// Represents the following attribute in the schema: w:default
    #[xml(attr = "w:default")]
    pub default: Option<bool>,
    /// User-Defined Style
    /// Represents the following attribute in the schema: w:customStyle
    #[xml(attr = "w:customStyle")]
    pub custom_style: Option<bool>,
    ///Primary Style Name
    #[xml(child = "w:name")]
    pub style_name: Option<StyleName>,
    ///Alternate Style Names
    #[xml(child = "w:aliases")]
    pub aliases: Option<Aliases>,
    ///Parent Style ID
    #[xml(child = "w:basedOn")]
    pub based_on: Option<BasedOn>,
    ///Style For Next Paragraph
    #[xml(child = "w:next")]
    pub next_paragraph_style: Option<NextParagraphStyle>,
    ///Linked Style Reference
    #[xml(child = "w:link")]
    pub linked_style: Option<LinkedStyle>,
    ///Automatically Merge User Formatting Into Style Definition
    #[xml(child = "w:autoRedefine")]
    pub auto_redefine: Option<AutoRedefine>,
    ///Hide Style From User Interface
    #[xml(child = "w:hidden")]
    pub style_hidden: Option<StyleHidden>,
    ///Optional User Interface Sorting Order
    #[xml(child = "w:uiPriority")]
    pub ui_priority: Option<UiPriority>,
    ///Hide Style From Main User Interface
    #[xml(child = "w:semiHidden")]
    pub semi_hidden: Option<SemiHidden>,
    ///Remove Semi-Hidden Property When Style Is Used
    #[xml(child = "w:unhideWhenUsed")]
    pub unhide_when_used: Option<UnhideWhenUsed>,
    ///Primary Style
    #[xml(child = "w:qFormat")]
    pub primary_style: Option<PrimaryStyle>,
    ///Style Cannot Be Applied
    #[xml(child = "w:locked")]
    pub locked: Option<Locked>,
    ///E-Mail Message Text Style
    #[xml(child = "w:personal")]
    pub personal: Option<Personal>,
    ///E-Mail Message Composition Style
    #[xml(child = "w:personalCompose")]
    pub personal_compose: Option<PersonalCompose>,
    ///E-Mail Message Reply Style
    #[xml(child = "w:personalReply")]
    pub personal_reply: Option<PersonalReply>,
    ///Revision Identifier for Style Definition
    #[xml(child = "w:rsid")]
    pub rsid: Option<Rsid>,
    ///Style Paragraph Properties
    #[xml(child = "w:pPr")]
    pub style_paragraph_properties: Option<StyleParagraphProperties>,
    ///Run Properties
    #[xml(child = "w:rPr")]
    pub style_run_properties: Option<StyleRunProperties>,
    ///Style Table Properties
    #[xml(child = "w:tblPr")]
    pub style_table_properties: Option<StyleTableProperties>,
    ///Style Table Row Properties
    #[xml(child = "w:trPr")]
    pub table_style_conditional_formatting_table_row_properties: Option<
        TableStyleConditionalFormattingTableRowProperties,
    >,
    ///Style Table Cell Properties
    #[xml(child = "w:tcPr")]
    pub style_table_cell_properties: Option<StyleTableCellProperties>,
    /// _
    #[xml(child = "w:tblStylePr")]
    pub w_tbl_style_pr: Vec<TableStyleProperties>,
}
/// Properties for a Single Font.
/// When the object is serialized out as xml, it's qualified name is w:font.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:font")]
pub struct Font {
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// _
    #[xml(child = "w:altName")]
    pub alt_name: Option<AltName>,
    /// _
    #[xml(child = "w:panose1")]
    pub panose1_number: Option<Panose1Number>,
    /// _
    #[xml(child = "w:charset")]
    pub font_char_set: Option<FontCharSet>,
    /// _
    #[xml(child = "w:family")]
    pub font_family: Option<FontFamily>,
    /// _
    #[xml(child = "w:notTrueType")]
    pub not_true_type: Option<NotTrueType>,
    /// _
    #[xml(child = "w:pitch")]
    pub pitch: Option<Pitch>,
    /// _
    #[xml(child = "w:sig")]
    pub font_signature: Option<FontSignature>,
    /// _
    #[xml(child = "w:embedRegular")]
    pub embed_regular_font: Option<EmbedRegularFont>,
    /// _
    #[xml(child = "w:embedBold")]
    pub embed_bold_font: Option<EmbedBoldFont>,
    /// _
    #[xml(child = "w:embedItalic")]
    pub embed_italic_font: Option<EmbedItalicFont>,
    /// _
    #[xml(child = "w:embedBoldItalic")]
    pub embed_bold_italic_font: Option<EmbedBoldItalicFont>,
}
/// Left Margin for HTML div.
/// When the object is serialized out as xml, it's qualified name is w:marLeft.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:marLeft")]
pub struct LeftMarginDiv {
    /// Positive or Negative Value in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Right Margin for HTML div.
/// When the object is serialized out as xml, it's qualified name is w:marRight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:marRight")]
pub struct RightMarginDiv {
    /// Positive or Negative Value in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Top Margin for HTML div.
/// When the object is serialized out as xml, it's qualified name is w:marTop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:marTop")]
pub struct TopMarginDiv {
    /// Positive or Negative Value in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Bottom Margin for HTML div.
/// When the object is serialized out as xml, it's qualified name is w:marBottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:marBottom")]
pub struct BottomMarginDiv {
    /// Positive or Negative Value in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the SignedTwipsMeasureType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SignedTwipsMeasureType {
    /// Positive or Negative Value in Twentieths of a Point
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Set of Borders for HTML div.
/// When the object is serialized out as xml, it's qualified name is w:divBdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:divBdr")]
pub struct DivBorder {
    ///Top Border for HTML div
    #[xml(child = "w:top")]
    pub top_border: Option<TopBorder>,
    ///Left Border for HTML div
    #[xml(child = "w:left")]
    pub left_border: Option<LeftBorder>,
    ///Bottom Border for HTML div
    #[xml(child = "w:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Right Border for HTML div
    #[xml(child = "w:right")]
    pub right_border: Option<RightBorder>,
}
/// Child div Elements Contained within Current div.
/// When the object is serialized out as xml, it's qualified name is w:divsChild.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:divsChild")]
pub struct DivsChild {
    #[xml(child = "w:div")]
    pub children: Vec<DivsChildChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DivsChildChildChoice {
    #[xml(tag = "w:div")]
    WDiv(Div),
}
/// Defines the Divs Class.
/// When the object is serialized out as xml, it's qualified name is w:divs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:divs")]
pub struct Divs {
    #[xml(child = "w:div")]
    pub children: Vec<DivsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DivsChildChoice {
    #[xml(tag = "w:div")]
    WDiv(Div),
}
/// Defines the DivsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct DivsType {
    #[xml(child = "w:div")]
    pub children: Vec<DivsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DivsTypeChildChoice {
    #[xml(tag = "w:div")]
    WDiv(Div),
}
/// Information About Single HTML div Element.
/// When the object is serialized out as xml, it's qualified name is w:div.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:div")]
pub struct Div {
    /// div Data ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    ///Data for HTML blockquote Element
    #[xml(child = "w:blockQuote")]
    pub block_quote: Option<BlockQuote>,
    ///Data for HTML body Element
    #[xml(child = "w:bodyDiv")]
    pub body_div: Option<BodyDiv>,
    ///Left Margin for HTML div
    #[xml(child = "w:marLeft")]
    pub left_margin_div: LeftMarginDiv,
    ///Right Margin for HTML div
    #[xml(child = "w:marRight")]
    pub right_margin_div: RightMarginDiv,
    ///Top Margin for HTML div
    #[xml(child = "w:marTop")]
    pub top_margin_div: TopMarginDiv,
    ///Bottom Margin for HTML div
    #[xml(child = "w:marBottom")]
    pub bottom_margin_div: BottomMarginDiv,
    ///Set of Borders for HTML div
    #[xml(child = "w:divBdr")]
    pub div_border: Option<DivBorder>,
    /// _
    #[xml(child = "w:divsChild")]
    pub w_divs_child: Vec<DivsChild>,
}
/// Comment Content.
/// When the object is serialized out as xml, it's qualified name is w:comment.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:comment")]
pub struct Comment {
    /// initials
    /// Represents the following attribute in the schema: w:initials
    #[xml(attr = "w:initials")]
    pub initials: Option<String>,
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
    )]
    pub children: Vec<CommentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CommentChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
}
/// Footnote Content.
/// When the object is serialized out as xml, it's qualified name is w:footnote.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnote")]
pub struct Footnote {
    /// Footnote/Endnote Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<FootnoteEndnoteValues>,
    /// Footnote/Endnote ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<FootnoteChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FootnoteChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Endnote Content.
/// When the object is serialized out as xml, it's qualified name is w:endnote.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnote")]
pub struct Endnote {
    /// Footnote/Endnote Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<FootnoteEndnoteValues>,
    /// Footnote/Endnote ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<EndnoteChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndnoteChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Defines the FootnoteEndnoteType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct FootnoteEndnoteType {
    /// Footnote/Endnote Type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: Option<FootnoteEndnoteValues>,
    /// Footnote/Endnote ID
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: i32,
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<FootnoteEndnoteTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FootnoteEndnoteTypeChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
}
/// Entry Insertion Behavior.
/// When the object is serialized out as xml, it's qualified name is w:behavior.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:behavior")]
pub struct Behavior {
    /// Insertion Behavior Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: DocPartBehaviorValues,
}
/// Entry Type.
/// When the object is serialized out as xml, it's qualified name is w:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:type")]
pub struct DocPartType {
    /// Type Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: DocPartValues,
}
/// Gallery Associated With Entry.
/// When the object is serialized out as xml, it's qualified name is w:gallery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:gallery")]
pub struct Gallery {
    /// Gallery Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: DocPartGalleryValues,
}
/// Single Automatic Captioning Setting.
/// When the object is serialized out as xml, it's qualified name is w:autoCaption.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoCaption")]
pub struct AutoCaption {
    /// Identifier of Object to be Automatically Captioned
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// Caption Used for Automatic Captioning
    /// Represents the following attribute in the schema: w:caption
    #[xml(attr = "w:caption")]
    pub caption: String,
}
/// Single Caption Type Definition.
/// When the object is serialized out as xml, it's qualified name is w:caption.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:caption")]
pub struct Caption {
    /// Caption Type Name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: String,
    /// Automatic Caption Placement
    /// Represents the following attribute in the schema: w:pos
    #[xml(attr = "w:pos")]
    pub position: Option<CaptionPositionValues>,
    /// Include Chapter Number in Field for Caption
    /// Represents the following attribute in the schema: w:chapNum
    #[xml(attr = "w:chapNum")]
    pub chapter_number: Option<bool>,
    /// Style for Chapter Headings
    /// Represents the following attribute in the schema: w:heading
    #[xml(attr = "w:heading")]
    pub heading: Option<i32>,
    /// Do Not Include Name In Caption
    /// Represents the following attribute in the schema: w:noLabel
    #[xml(attr = "w:noLabel")]
    pub no_label: Option<bool>,
    /// Caption Numbering Format
    /// Represents the following attribute in the schema: w:numFmt
    #[xml(attr = "w:numFmt")]
    pub number_format: Option<NumberFormatValues>,
    /// Chapter Number/Item Index Separator
    /// Represents the following attribute in the schema: w:sep
    #[xml(attr = "w:sep")]
    pub separator: Option<ChapterSeparatorValues>,
}
/// Automatic Captioning Settings.
/// When the object is serialized out as xml, it's qualified name is w:autoCaptions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:autoCaptions")]
pub struct AutoCaptions {
    /// _
    #[xml(child = "w:autoCaption")]
    pub w_auto_caption: Vec<AutoCaption>,
}
/// Document Background.
/// When the object is serialized out as xml, it's qualified name is w:background.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:background")]
pub struct DocumentBackground {
    /// color
    /// Represents the following attribute in the schema: w:color
    #[xml(attr = "w:color")]
    pub color: Option<String>,
    /// themeColor
    /// Represents the following attribute in the schema: w:themeColor
    #[xml(attr = "w:themeColor")]
    pub theme_color: Option<ThemeColorValues>,
    /// themeTint
    /// Represents the following attribute in the schema: w:themeTint
    #[xml(attr = "w:themeTint")]
    pub theme_tint: Option<String>,
    /// themeShade
    /// Represents the following attribute in the schema: w:themeShade
    #[xml(attr = "w:themeShade")]
    pub theme_shade: Option<String>,
    #[xml(child = "v:background")]
    pub children: Vec<DocumentBackgroundChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocumentBackgroundChildChoice {
    #[xml(tag = "v:background")]
    VBackground(crate::schemas::schemas_microsoft_com_vml::Background),
}
/// List of Glossary Document Entries.
/// When the object is serialized out as xml, it's qualified name is w:docParts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docParts")]
pub struct DocParts {
    #[xml(child = "w:docPart")]
    pub children: Vec<DocPartsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocPartsChildChoice {
    #[xml(tag = "w:docPart")]
    WDocPart(DocPart),
}
/// Entry Name.
/// When the object is serialized out as xml, it's qualified name is w:name.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:name")]
pub struct DocPartName {
    /// Name Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
    /// Built-In Entry
    /// Represents the following attribute in the schema: w:decorated
    #[xml(attr = "w:decorated")]
    pub decorated: Option<bool>,
}
/// Entry Categorization.
/// When the object is serialized out as xml, it's qualified name is w:category.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:category")]
pub struct Category {
    ///Category Associated With Entry
    #[xml(child = "w:name")]
    pub name: Name,
    ///Gallery Associated With Entry
    #[xml(child = "w:gallery")]
    pub gallery: Gallery,
}
/// Entry Types.
/// When the object is serialized out as xml, it's qualified name is w:types.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:types")]
pub struct DocPartTypes {
    /// Entry Is Of All Types
    /// Represents the following attribute in the schema: w:all
    #[xml(attr = "w:all")]
    pub all: Option<bool>,
    #[xml(child = "w:type")]
    pub children: Vec<DocPartTypesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocPartTypesChildChoice {
    #[xml(tag = "w:type")]
    WType(DocPartType),
}
/// Entry Insertion Behaviors.
/// When the object is serialized out as xml, it's qualified name is w:behaviors.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:behaviors")]
pub struct Behaviors {
    #[xml(child = "w:behavior")]
    pub children: Vec<BehaviorsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BehaviorsChildChoice {
    #[xml(tag = "w:behavior")]
    WBehavior(Behavior),
}
/// Entry ID.
/// When the object is serialized out as xml, it's qualified name is w:guid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:guid")]
pub struct DocPartId {
    /// GUID Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
}
/// Glossary Document Entry Properties.
/// When the object is serialized out as xml, it's qualified name is w:docPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartPr")]
pub struct DocPartProperties {
    ///Entry Name
    #[xml(child = "w:name")]
    pub doc_part_name: Option<DocPartName>,
    ///Associated Paragraph Style Name
    #[xml(child = "w:style")]
    pub style_id: Option<StyleId>,
    ///Entry Categorization
    #[xml(child = "w:category")]
    pub category: Option<Category>,
    ///Entry Types
    #[xml(child = "w:types")]
    pub doc_part_types: Option<DocPartTypes>,
    ///Entry Insertion Behaviors
    #[xml(child = "w:behaviors")]
    pub behaviors: Option<Behaviors>,
    ///Description for Entry
    #[xml(child = "w:description")]
    pub description: Option<Description>,
    ///Entry ID
    #[xml(child = "w:guid")]
    pub doc_part_id: Option<DocPartId>,
}
/// Contents of Glossary Document Entry.
/// When the object is serialized out as xml, it's qualified name is w:docPartBody.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartBody")]
pub struct DocPartBody {
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "w:sectPr",
    )]
    pub children: Vec<DocPartBodyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DocPartBodyChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "w:sectPr")]
    WSectPr(SectionProperties),
}
/// Defines the Body Class.
/// When the object is serialized out as xml, it's qualified name is w:body.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:body")]
pub struct Body {
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "w:sectPr",
    )]
    pub children: Vec<BodyChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BodyChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "w:sectPr")]
    WSectPr(SectionProperties),
}
/// Defines the BodyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BodyType {
    #[xml(
        child = "w:altChunk",
        child = "w:customXml",
        child = "w:sdt",
        child = "w:p",
        child = "w:tbl",
        child = "w:proofErr",
        child = "w:permStart",
        child = "w:permEnd",
        child = "w:bookmarkStart",
        child = "w:bookmarkEnd",
        child = "w:commentRangeStart",
        child = "w:commentRangeEnd",
        child = "w:moveFromRangeStart",
        child = "w:moveFromRangeEnd",
        child = "w:moveToRangeStart",
        child = "w:moveToRangeEnd",
        child = "w:customXmlInsRangeStart",
        child = "w:customXmlInsRangeEnd",
        child = "w:customXmlDelRangeStart",
        child = "w:customXmlDelRangeEnd",
        child = "w:customXmlMoveFromRangeStart",
        child = "w:customXmlMoveFromRangeEnd",
        child = "w:customXmlMoveToRangeStart",
        child = "w:customXmlMoveToRangeEnd",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
        child = "w:contentPart",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
        child = "w:sectPr",
    )]
    pub children: Vec<BodyTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BodyTypeChildChoice {
    #[xml(tag = "w:altChunk")]
    WAltChunk(AltChunk),
    #[xml(tag = "w:customXml")]
    WCustomXml(CustomXmlBlock),
    #[xml(tag = "w:sdt")]
    WSdt(SdtBlock),
    #[xml(tag = "w:p")]
    WP(Paragraph),
    #[xml(tag = "w:tbl")]
    WTbl(Table),
    #[xml(tag = "w:proofErr")]
    WProofErr(ProofError),
    #[xml(tag = "w:permStart")]
    WPermStart(PermStart),
    #[xml(tag = "w:permEnd")]
    WPermEnd(PermEnd),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(BookmarkStart),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(BookmarkEnd),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(CommentRangeStart),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(CommentRangeEnd),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(MoveFromRangeStart),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(MoveFromRangeEnd),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(MoveToRangeStart),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(MoveToRangeEnd),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(CustomXmlInsRangeStart),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(CustomXmlInsRangeEnd),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(CustomXmlDelRangeStart),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(CustomXmlDelRangeEnd),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(CustomXmlMoveFromRangeStart),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(CustomXmlMoveFromRangeEnd),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(CustomXmlMoveToRangeStart),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(CustomXmlMoveToRangeEnd),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictInsertionRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeStart,
    ),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::CustomXmlConflictDeletionRangeEnd,
    ),
    #[xml(tag = "w:ins")]
    WIns(InsertedRun),
    #[xml(tag = "w:del")]
    WDel(DeletedRun),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(MoveFromRun),
    #[xml(tag = "w:moveTo")]
    WMoveTo(MoveToRun),
    #[xml(tag = "w:contentPart")]
    WContentPart(ContentPart),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "w:sectPr")]
    WSectPr(SectionProperties),
}
/// Glossary Document Entry.
/// When the object is serialized out as xml, it's qualified name is w:docPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPart")]
pub struct DocPart {
    ///Glossary Document Entry Properties
    #[xml(child = "w:docPartPr")]
    pub doc_part_properties: Option<DocPartProperties>,
    ///Contents of Glossary Document Entry
    #[xml(child = "w:docPartBody")]
    pub doc_part_body: Option<DocPartBody>,
}
/// Defines the CompatibilitySetting Class.
/// When the object is serialized out as xml, it's qualified name is w:compatSetting.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:compatSetting")]
pub struct CompatibilitySetting {
    /// name
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub w_name: CompatSettingNameValues,
    /// uri
    /// Represents the following attribute in the schema: w:uri
    #[xml(attr = "w:uri")]
    pub w_uri: String,
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: String,
}
/// Table Cell Left Margin Default.
/// When the object is serialized out as xml, it's qualified name is w:left.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:left")]
pub struct TableCellLeftMargin {
    /// w
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: i16,
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: TableWidthValues,
}
/// Table Cell Right Margin Default.
/// When the object is serialized out as xml, it's qualified name is w:right.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:right")]
pub struct TableCellRightMargin {
    /// w
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: i16,
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: TableWidthValues,
}
/// Defines the TableWidthDxaNilType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TableWidthDxaNilType {
    /// w
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: i16,
    /// type
    /// Represents the following attribute in the schema: w:type
    #[xml(attr = "w:type")]
    pub r#type: TableWidthValues,
}
/// Table-Level Property Exceptions.
/// When the object is serialized out as xml, it's qualified name is w:tblPrEx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblPrEx")]
pub struct TablePropertyExceptions {
    #[xml(
        child = "w:tblW",
        child = "w:jc",
        child = "w:tblCellSpacing",
        child = "w:tblInd",
        child = "w:tblBorders",
        child = "w:shd",
        child = "w:tblLayout",
        child = "w:tblCellMar",
        child = "w:tblLook",
        child = "w:tblPrExChange",
    )]
    pub children: Vec<TablePropertyExceptionsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TablePropertyExceptionsChildChoice {
    #[xml(tag = "w:tblW")]
    WTblW(TableWidth),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:tblInd")]
    WTblInd(TableIndentation),
    #[xml(tag = "w:tblBorders")]
    WTblBorders(TableBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tblLayout")]
    WTblLayout(TableLayout),
    #[xml(tag = "w:tblCellMar")]
    WTblCellMar(TableCellMarginDefault),
    #[xml(tag = "w:tblLook")]
    WTblLook(TableLook),
    #[xml(tag = "w:tblPrExChange")]
    WTblPrExChange(TablePropertyExceptionsChange),
}
/// Table Row Properties.
/// When the object is serialized out as xml, it's qualified name is w:trPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:trPr")]
pub struct TableRowProperties {
    #[xml(
        child = "w:cnfStyle",
        child = "w:divId",
        child = "w:gridBefore",
        child = "w:gridAfter",
        child = "w:wBefore",
        child = "w:wAfter",
        child = "w:trHeight",
        child = "w:hidden",
        child = "w:cantSplit",
        child = "w:tblHeader",
        child = "w:tblCellSpacing",
        child = "w:jc",
        child = "w:ins",
        child = "w:del",
        child = "w:trPrChange",
        child = "w14:conflictIns",
        child = "w14:conflictDel",
    )]
    pub children: Vec<TableRowPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableRowPropertiesChildChoice {
    #[xml(tag = "w:cnfStyle")]
    WCnfStyle(ConditionalFormatStyle),
    #[xml(tag = "w:divId")]
    WDivId(DivId),
    #[xml(tag = "w:gridBefore")]
    WGridBefore(GridBefore),
    #[xml(tag = "w:gridAfter")]
    WGridAfter(GridAfter),
    #[xml(tag = "w:wBefore")]
    WWBefore(WidthBeforeTableRow),
    #[xml(tag = "w:wAfter")]
    WWAfter(WidthAfterTableRow),
    #[xml(tag = "w:trHeight")]
    WTrHeight(TableRowHeight),
    #[xml(tag = "w:hidden")]
    WHidden(Hidden),
    #[xml(tag = "w:cantSplit")]
    WCantSplit(CantSplit),
    #[xml(tag = "w:tblHeader")]
    WTblHeader(TableHeader),
    #[xml(tag = "w:tblCellSpacing")]
    WTblCellSpacing(TableCellSpacing),
    #[xml(tag = "w:jc")]
    WJc(TableJustification),
    #[xml(tag = "w:ins")]
    WIns(Inserted),
    #[xml(tag = "w:del")]
    WDel(Deleted),
    #[xml(tag = "w:trPrChange")]
    WTrPrChange(TableRowPropertiesChange),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictDeletion,
    ),
}
/// Revision Information for Table Row Properties.
/// When the object is serialized out as xml, it's qualified name is w:trPrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:trPrChange")]
pub struct TableRowPropertiesChange {
    /// author
    /// Represents the following attribute in the schema: w:author
    #[xml(attr = "w:author")]
    pub author: String,
    /// date
    /// Represents the following attribute in the schema: w:date
    #[xml(attr = "w:date")]
    pub date: Option<String>,
    /// dateUtc
    /// Represents the following attribute in the schema: w16du:dateUtc
    #[xml(attr = "w16du:dateUtc")]
    pub date_utc: Option<String>,
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
    #[xml(child = "w:trPr")]
    pub children: Vec<TableRowPropertiesChangeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableRowPropertiesChangeChildChoice {
    #[xml(tag = "w:trPr")]
    WTrPr(PreviousTableRowProperties),
}
/// Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is w:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pPr")]
pub struct ParagraphProperties {
    #[xml(
        child = "w:pStyle",
        child = "w:keepNext",
        child = "w:keepLines",
        child = "w:pageBreakBefore",
        child = "w:framePr",
        child = "w:widowControl",
        child = "w:numPr",
        child = "w:suppressLineNumbers",
        child = "w:pBdr",
        child = "w:shd",
        child = "w:tabs",
        child = "w:suppressAutoHyphens",
        child = "w:kinsoku",
        child = "w:wordWrap",
        child = "w:overflowPunct",
        child = "w:topLinePunct",
        child = "w:autoSpaceDE",
        child = "w:autoSpaceDN",
        child = "w:bidi",
        child = "w:adjustRightInd",
        child = "w:snapToGrid",
        child = "w:spacing",
        child = "w:ind",
        child = "w:contextualSpacing",
        child = "w:mirrorIndents",
        child = "w:suppressOverlap",
        child = "w:jc",
        child = "w:textDirection",
        child = "w:textAlignment",
        child = "w:textboxTightWrap",
        child = "w:outlineLvl",
        child = "w:divId",
        child = "w:cnfStyle",
        child = "w:rPr",
        child = "w:sectPr",
        child = "w:pPrChange",
    )]
    pub children: Vec<ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphPropertiesChildChoice {
    #[xml(tag = "w:pStyle")]
    WPStyle(ParagraphStyleId),
    #[xml(tag = "w:keepNext")]
    WKeepNext(KeepNext),
    #[xml(tag = "w:keepLines")]
    WKeepLines(KeepLines),
    #[xml(tag = "w:pageBreakBefore")]
    WPageBreakBefore(PageBreakBefore),
    #[xml(tag = "w:framePr")]
    WFramePr(FrameProperties),
    #[xml(tag = "w:widowControl")]
    WWidowControl(WidowControl),
    #[xml(tag = "w:numPr")]
    WNumPr(NumberingProperties),
    #[xml(tag = "w:suppressLineNumbers")]
    WSuppressLineNumbers(SuppressLineNumbers),
    #[xml(tag = "w:pBdr")]
    WPBdr(ParagraphBorders),
    #[xml(tag = "w:shd")]
    WShd(Shading),
    #[xml(tag = "w:tabs")]
    WTabs(Tabs),
    #[xml(tag = "w:suppressAutoHyphens")]
    WSuppressAutoHyphens(SuppressAutoHyphens),
    #[xml(tag = "w:kinsoku")]
    WKinsoku(Kinsoku),
    #[xml(tag = "w:wordWrap")]
    WWordWrap(WordWrap),
    #[xml(tag = "w:overflowPunct")]
    WOverflowPunct(OverflowPunctuation),
    #[xml(tag = "w:topLinePunct")]
    WTopLinePunct(TopLinePunctuation),
    #[xml(tag = "w:autoSpaceDE")]
    WAutoSpaceDe(AutoSpaceDe),
    #[xml(tag = "w:autoSpaceDN")]
    WAutoSpaceDn(AutoSpaceDn),
    #[xml(tag = "w:bidi")]
    WBidi(BiDi),
    #[xml(tag = "w:adjustRightInd")]
    WAdjustRightInd(AdjustRightIndent),
    #[xml(tag = "w:snapToGrid")]
    WSnapToGrid(SnapToGrid),
    #[xml(tag = "w:spacing")]
    WSpacing(SpacingBetweenLines),
    #[xml(tag = "w:ind")]
    WInd(Indentation),
    #[xml(tag = "w:contextualSpacing")]
    WContextualSpacing(ContextualSpacing),
    #[xml(tag = "w:mirrorIndents")]
    WMirrorIndents(MirrorIndents),
    #[xml(tag = "w:suppressOverlap")]
    WSuppressOverlap(SuppressOverlap),
    #[xml(tag = "w:jc")]
    WJc(Justification),
    #[xml(tag = "w:textDirection")]
    WTextDirection(TextDirection),
    #[xml(tag = "w:textAlignment")]
    WTextAlignment(TextAlignment),
    #[xml(tag = "w:textboxTightWrap")]
    WTextboxTightWrap(TextBoxTightWrap),
    #[xml(tag = "w:outlineLvl")]
    WOutlineLvl(OutlineLevel),
    #[xml(tag = "w:divId")]
    WDivId(DivId),
    #[xml(tag = "w:cnfStyle")]
    WCnfStyle(ConditionalFormatStyle),
    #[xml(tag = "w:rPr")]
    WRPr(ParagraphMarkRunProperties),
    #[xml(tag = "w:sectPr")]
    WSectPr(SectionProperties),
    #[xml(tag = "w:pPrChange")]
    WPPrChange(ParagraphPropertiesChange),
}
/// Defines the Control Class.
/// When the object is serialized out as xml, it's qualified name is w:control.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:control")]
pub struct Control {
    /// Unique Name for Embedded Control
    /// Represents the following attribute in the schema: w:name
    #[xml(attr = "w:name")]
    pub name: Option<String>,
    /// Associated VML Data Reference
    /// Represents the following attribute in the schema: w:shapeid
    #[xml(attr = "w:shapeid")]
    pub shape_id: Option<String>,
    /// Embedded Control Properties Relationship Reference
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
}
/// Previous Table Grid.
/// When the object is serialized out as xml, it's qualified name is w:tblGrid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:tblGrid")]
pub struct PreviousTableGrid {
    /// _
    #[xml(child = "w:gridCol")]
    pub w_grid_col: Vec<GridColumn>,
}
/// Defines the ObjectEmbed Class.
/// When the object is serialized out as xml, it's qualified name is w:objectEmbed.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:objectEmbed")]
pub struct ObjectEmbed {
    /// drawAspect
    /// Represents the following attribute in the schema: w:drawAspect
    #[xml(attr = "w:drawAspect")]
    pub draw_aspect: Option<ObjectDrawAspect>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// progId
    /// Represents the following attribute in the schema: w:progId
    #[xml(attr = "w:progId")]
    pub prog_id: Option<String>,
    /// shapeId
    /// Represents the following attribute in the schema: w:shapeId
    #[xml(attr = "w:shapeId")]
    pub shape_id: Option<String>,
    /// fieldCodes
    /// Represents the following attribute in the schema: w:fieldCodes
    #[xml(attr = "w:fieldCodes")]
    pub field_codes: Option<String>,
}
/// Defines the ObjectLink Class.
/// When the object is serialized out as xml, it's qualified name is w:objectLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:objectLink")]
pub struct ObjectLink {
    /// updateMode
    /// Represents the following attribute in the schema: w:updateMode
    #[xml(attr = "w:updateMode")]
    pub update_mode: ObjectUpdateMode,
    /// lockedField
    /// Represents the following attribute in the schema: w:lockedField
    #[xml(attr = "w:lockedField")]
    pub locked_field: Option<bool>,
    /// drawAspect
    /// Represents the following attribute in the schema: w:drawAspect
    #[xml(attr = "w:drawAspect")]
    pub draw_aspect: Option<ObjectDrawAspect>,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// progId
    /// Represents the following attribute in the schema: w:progId
    #[xml(attr = "w:progId")]
    pub prog_id: Option<String>,
    /// shapeId
    /// Represents the following attribute in the schema: w:shapeId
    #[xml(attr = "w:shapeId")]
    pub shape_id: Option<String>,
    /// fieldCodes
    /// Represents the following attribute in the schema: w:fieldCodes
    #[xml(attr = "w:fieldCodes")]
    pub field_codes: Option<String>,
}
/// Defines the Lock Class.
/// When the object is serialized out as xml, it's qualified name is w:lock.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lock")]
pub struct Lock {
    /// Locking Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<LockingValues>,
}
/// Defines the SdtPlaceholder Class.
/// When the object is serialized out as xml, it's qualified name is w:placeholder.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:placeholder")]
pub struct SdtPlaceholder {
    ///Document Part Reference
    #[xml(child = "w:docPart")]
    pub doc_part_reference: DocPartReference,
}
/// Defines the DataBinding Class.
/// When the object is serialized out as xml, it's qualified name is w:dataBinding.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dataBinding")]
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
/// Defines the SdtContentComboBox Class.
/// When the object is serialized out as xml, it's qualified name is w:comboBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:comboBox")]
pub struct SdtContentComboBox {
    /// Combo Box Last Saved Value
    /// Represents the following attribute in the schema: w:lastValue
    #[xml(attr = "w:lastValue")]
    pub last_value: Option<String>,
    /// _
    #[xml(child = "w:listItem")]
    pub w_list_item: Vec<ListItem>,
}
/// Defines the SdtContentDate Class.
/// When the object is serialized out as xml, it's qualified name is w:date.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:date")]
pub struct SdtContentDate {
    /// Last Known Date in XML Schema DateTime Format
    /// Represents the following attribute in the schema: w:fullDate
    #[xml(attr = "w:fullDate")]
    pub full_date: Option<String>,
    ///Date Display Mask
    #[xml(child = "w:dateFormat")]
    pub date_format: Option<DateFormat>,
    ///Date Picker Language ID
    #[xml(child = "w:lid")]
    pub language_id: Option<LanguageId>,
    ///Custom XML Data Date Storage Format
    #[xml(child = "w:storeMappedDataAs")]
    pub sdt_date_mapping_type: Option<SdtDateMappingType>,
    ///Date Picker Calendar Type
    #[xml(child = "w:calendar")]
    pub calendar: Option<Calendar>,
}
/// Defines the SdtContentDocPartObject Class.
/// When the object is serialized out as xml, it's qualified name is w:docPartObj.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartObj")]
pub struct SdtContentDocPartObject {
    ///Document Part Gallery Filter
    #[xml(child = "w:docPartGallery")]
    pub doc_part_gallery: Option<DocPartGallery>,
    ///Document Part Category Filter
    #[xml(child = "w:docPartCategory")]
    pub doc_part_category: Option<DocPartCategory>,
    ///Built-In Document Part
    #[xml(child = "w:docPartUnique")]
    pub doc_part_unique: Option<DocPartUnique>,
}
/// Defines the SdtContentDocPartList Class.
/// When the object is serialized out as xml, it's qualified name is w:docPartList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docPartList")]
pub struct SdtContentDocPartList {
    ///Document Part Gallery Filter
    #[xml(child = "w:docPartGallery")]
    pub doc_part_gallery: Option<DocPartGallery>,
    ///Document Part Category Filter
    #[xml(child = "w:docPartCategory")]
    pub doc_part_category: Option<DocPartCategory>,
    ///Built-In Document Part
    #[xml(child = "w:docPartUnique")]
    pub doc_part_unique: Option<DocPartUnique>,
}
/// Defines the SdtDocPartType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SdtDocPartType {}
/// Defines the SdtContentDropDownList Class.
/// When the object is serialized out as xml, it's qualified name is w:dropDownList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:dropDownList")]
pub struct SdtContentDropDownList {
    /// Drop-down List Last Saved Value
    /// Represents the following attribute in the schema: w:lastValue
    #[xml(attr = "w:lastValue")]
    pub last_value: Option<String>,
    /// _
    #[xml(child = "w:listItem")]
    pub w_list_item: Vec<ListItem>,
}
/// Defines the SdtContentText Class.
/// When the object is serialized out as xml, it's qualified name is w:text.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:text")]
pub struct SdtContentText {
    /// Allow Soft Line Breaks
    /// Represents the following attribute in the schema: w:multiLine
    #[xml(attr = "w:multiLine")]
    pub multi_line: Option<bool>,
}
/// Write Protection.
/// When the object is serialized out as xml, it's qualified name is w:writeProtection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:writeProtection")]
pub struct WriteProtection {
    /// Recommend Write Protection in User Interface
    /// Represents the following attribute in the schema: w:recommended
    #[xml(attr = "w:recommended")]
    pub recommended: Option<bool>,
    /// Cryptographic Provider Type
    /// Represents the following attribute in the schema: w:cryptProviderType
    #[xml(attr = "w:cryptProviderType")]
    pub cryptographic_provider_type: Option<CryptProviderValues>,
    /// Cryptographic Algorithm Class
    /// Represents the following attribute in the schema: w:cryptAlgorithmClass
    #[xml(attr = "w:cryptAlgorithmClass")]
    pub cryptographic_algorithm_class: Option<CryptAlgorithmClassValues>,
    /// Cryptographic Algorithm Type
    /// Represents the following attribute in the schema: w:cryptAlgorithmType
    #[xml(attr = "w:cryptAlgorithmType")]
    pub cryptographic_algorithm_type: Option<CryptAlgorithmValues>,
    /// Cryptographic Hashing Algorithm
    /// Represents the following attribute in the schema: w:cryptAlgorithmSid
    #[xml(attr = "w:cryptAlgorithmSid")]
    pub cryptographic_algorithm_sid: Option<i32>,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: w:cryptSpinCount
    #[xml(attr = "w:cryptSpinCount")]
    pub cryptographic_spin_count: Option<i32>,
    /// Cryptographic Provider
    /// Represents the following attribute in the schema: w:cryptProvider
    #[xml(attr = "w:cryptProvider")]
    pub cryptographic_provider: Option<String>,
    /// Cryptographic Algorithm Extensibility
    /// Represents the following attribute in the schema: w:algIdExt
    #[xml(attr = "w:algIdExt")]
    pub algorithm_id_extensibility: Option<String>,
    /// Algorithm Extensibility Source
    /// Represents the following attribute in the schema: w:algIdExtSource
    #[xml(attr = "w:algIdExtSource")]
    pub algorithm_id_extensibility_source: Option<String>,
    /// Cryptographic Provider Type Extensibility
    /// Represents the following attribute in the schema: w:cryptProviderTypeExt
    #[xml(attr = "w:cryptProviderTypeExt")]
    pub cryptographic_provider_type_extensibility: Option<String>,
    /// Provider Type Extensibility Source
    /// Represents the following attribute in the schema: w:cryptProviderTypeExtSource
    #[xml(attr = "w:cryptProviderTypeExtSource")]
    pub cryptographic_provider_type_ext_source: Option<String>,
    /// Password Hash
    /// Represents the following attribute in the schema: w:hash
    #[xml(attr = "w:hash")]
    pub hash: Option<String>,
    /// Salt for Password Verifier
    /// Represents the following attribute in the schema: w:salt
    #[xml(attr = "w:salt")]
    pub salt: Option<String>,
    /// algorithmName
    /// Represents the following attribute in the schema: w:algorithmName
    #[xml(attr = "w:algorithmName")]
    pub algorithm_name: Option<String>,
    /// hashValue
    /// Represents the following attribute in the schema: w:hashValue
    #[xml(attr = "w:hashValue")]
    pub hash_value: Option<String>,
    /// saltValue
    /// Represents the following attribute in the schema: w:saltValue
    #[xml(attr = "w:saltValue")]
    pub salt_value: Option<String>,
    /// spinCount
    /// Represents the following attribute in the schema: w:spinCount
    #[xml(attr = "w:spinCount")]
    pub spin_count: Option<i32>,
}
/// Document View Setting.
/// When the object is serialized out as xml, it's qualified name is w:view.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:view")]
pub struct View {
    /// Document View Setting  Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: ViewValues,
}
/// Magnification Setting.
/// When the object is serialized out as xml, it's qualified name is w:zoom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:zoom")]
pub struct Zoom {
    /// Zoom Type
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<PresetZoomValues>,
    /// Zoom Percentage
    /// Represents the following attribute in the schema: w:percent
    #[xml(attr = "w:percent")]
    pub percent: Option<String>,
}
/// Grammar Checking Settings.
/// When the object is serialized out as xml, it's qualified name is w:activeWritingStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:activeWritingStyle")]
pub struct ActiveWritingStyle {
    /// Writing Style Language
    /// Represents the following attribute in the schema: w:lang
    #[xml(attr = "w:lang")]
    pub language: String,
    /// Grammatical Engine ID
    /// Represents the following attribute in the schema: w:vendorID
    #[xml(attr = "w:vendorID")]
    pub vendor_id: i16,
    /// Grammatical Check Engine Version
    /// Represents the following attribute in the schema: w:dllVersion
    #[xml(attr = "w:dllVersion")]
    pub dll_version: i32,
    /// Natural Language Grammar Check
    /// Represents the following attribute in the schema: w:nlCheck
    #[xml(attr = "w:nlCheck")]
    pub natural_language_grammar_check: Option<bool>,
    /// Check Stylistic Rules With Grammar
    /// Represents the following attribute in the schema: w:checkStyle
    #[xml(attr = "w:checkStyle")]
    pub check_style: bool,
    /// Application Name
    /// Represents the following attribute in the schema: w:appName
    #[xml(attr = "w:appName")]
    pub application_name: String,
}
/// Spelling and Grammatical Checking State.
/// When the object is serialized out as xml, it's qualified name is w:proofState.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:proofState")]
pub struct ProofState {
    /// Spell Checking State
    /// Represents the following attribute in the schema: w:spelling
    #[xml(attr = "w:spelling")]
    pub spelling: Option<ProofingStateValues>,
    /// Grammatical Checking State
    /// Represents the following attribute in the schema: w:grammar
    #[xml(attr = "w:grammar")]
    pub grammar: Option<ProofingStateValues>,
}
/// Suggested Filtering for List of Document Styles.
/// When the object is serialized out as xml, it's qualified name is w:stylePaneFormatFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:stylePaneFormatFilter")]
pub struct StylePaneFormatFilter {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: Option<String>,
    /// allStyles
    /// Represents the following attribute in the schema: w:allStyles
    #[xml(attr = "w:allStyles")]
    pub all_styles: Option<bool>,
    /// customStyles
    /// Represents the following attribute in the schema: w:customStyles
    #[xml(attr = "w:customStyles")]
    pub custom_styles: Option<bool>,
    /// latentStyles
    /// Represents the following attribute in the schema: w:latentStyles
    #[xml(attr = "w:latentStyles")]
    pub latent_styles: Option<bool>,
    /// stylesInUse
    /// Represents the following attribute in the schema: w:stylesInUse
    #[xml(attr = "w:stylesInUse")]
    pub styles_in_use: Option<bool>,
    /// headingStyles
    /// Represents the following attribute in the schema: w:headingStyles
    #[xml(attr = "w:headingStyles")]
    pub heading_styles: Option<bool>,
    /// numberingStyles
    /// Represents the following attribute in the schema: w:numberingStyles
    #[xml(attr = "w:numberingStyles")]
    pub numbering_styles: Option<bool>,
    /// tableStyles
    /// Represents the following attribute in the schema: w:tableStyles
    #[xml(attr = "w:tableStyles")]
    pub table_styles: Option<bool>,
    /// directFormattingOnRuns
    /// Represents the following attribute in the schema: w:directFormattingOnRuns
    #[xml(attr = "w:directFormattingOnRuns")]
    pub direct_formatting_on_runs: Option<bool>,
    /// directFormattingOnParagraphs
    /// Represents the following attribute in the schema: w:directFormattingOnParagraphs
    #[xml(attr = "w:directFormattingOnParagraphs")]
    pub direct_formatting_on_paragraphs: Option<bool>,
    /// directFormattingOnNumbering
    /// Represents the following attribute in the schema: w:directFormattingOnNumbering
    #[xml(attr = "w:directFormattingOnNumbering")]
    pub direct_formatting_on_numbering: Option<bool>,
    /// directFormattingOnTables
    /// Represents the following attribute in the schema: w:directFormattingOnTables
    #[xml(attr = "w:directFormattingOnTables")]
    pub direct_formatting_on_tables: Option<bool>,
    /// clearFormatting
    /// Represents the following attribute in the schema: w:clearFormatting
    #[xml(attr = "w:clearFormatting")]
    pub clear_formatting: Option<bool>,
    /// top3HeadingStyles
    /// Represents the following attribute in the schema: w:top3HeadingStyles
    #[xml(attr = "w:top3HeadingStyles")]
    pub top3_heading_styles: Option<bool>,
    /// visibleStyles
    /// Represents the following attribute in the schema: w:visibleStyles
    #[xml(attr = "w:visibleStyles")]
    pub visible_styles: Option<bool>,
    /// alternateStyleNames
    /// Represents the following attribute in the schema: w:alternateStyleNames
    #[xml(attr = "w:alternateStyleNames")]
    pub alternate_style_names: Option<bool>,
}
/// Suggested Sorting for List of Document Styles.
/// When the object is serialized out as xml, it's qualified name is w:stylePaneSortMethod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:stylePaneSortMethod")]
pub struct StylePaneSortMethods {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub w_val: String,
}
/// Document Classification.
/// When the object is serialized out as xml, it's qualified name is w:documentType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:documentType")]
pub struct DocumentType {
    /// Document Classification Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: DocumentTypeValues,
}
/// Mail Merge Settings.
/// When the object is serialized out as xml, it's qualified name is w:mailMerge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:mailMerge")]
pub struct MailMerge {
    ///Source Document Type
    #[xml(child = "w:mainDocumentType")]
    pub main_document_type: Option<MainDocumentType>,
    ///Query Contains Link to External Query File
    #[xml(child = "w:linkToQuery")]
    pub link_to_query: Option<LinkToQuery>,
    ///Data Source Type
    #[xml(child = "w:dataType")]
    pub data_type: DataType,
    ///Data Source Connection String
    #[xml(child = "w:connectString")]
    pub connect_string: Option<ConnectString>,
    ///Query For Data Source Records To Merge
    #[xml(child = "w:query")]
    pub query: Option<Query>,
    ///Data Source File Path
    #[xml(child = "w:dataSource")]
    pub data_source_reference: Option<DataSourceReference>,
    ///Header Definition File Path
    #[xml(child = "w:headerSource")]
    pub header_source: Option<HeaderSource>,
    ///Remove Blank Lines from Merged Documents
    #[xml(child = "w:doNotSuppressBlankLines")]
    pub do_not_suppress_blank_lines: Option<DoNotSuppressBlankLines>,
    ///Merged Document Destination
    #[xml(child = "w:destination")]
    pub destination: Option<Destination>,
    ///Column Containing E-mail Address
    #[xml(child = "w:addressFieldName")]
    pub address_field_name: Option<AddressFieldName>,
    ///Merged E-mail or Fax Subject Line
    #[xml(child = "w:mailSubject")]
    pub mail_subject: Option<MailSubject>,
    ///Merged Document To E-Mail Attachment
    #[xml(child = "w:mailAsAttachment")]
    pub mail_as_attachment: Option<MailAsAttachment>,
    ///View Merged Data Within Document
    #[xml(child = "w:viewMergedData")]
    pub view_merged_data: Option<ViewMergedData>,
    ///Record Currently Displayed In Merged Document
    #[xml(child = "w:activeRecord")]
    pub active_record: Option<ActiveRecord>,
    ///Mail Merge Error Reporting Setting
    #[xml(child = "w:checkErrors")]
    pub check_errors: Option<CheckErrors>,
    ///Office Data Source Object Settings
    #[xml(child = "w:odso")]
    pub data_source_object: Option<DataSourceObject>,
}
/// Visibility of Annotation Types.
/// When the object is serialized out as xml, it's qualified name is w:revisionView.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:revisionView")]
pub struct RevisionView {
    /// Display Visual Indicator Of Markup Area
    /// Represents the following attribute in the schema: w:markup
    #[xml(attr = "w:markup")]
    pub markup: Option<bool>,
    /// Display Comments
    /// Represents the following attribute in the schema: w:comments
    #[xml(attr = "w:comments")]
    pub comments: Option<bool>,
    /// Display Content Revisions
    /// Represents the following attribute in the schema: w:insDel
    #[xml(attr = "w:insDel")]
    pub display_revision: Option<bool>,
    /// Display Formatting Revisions
    /// Represents the following attribute in the schema: w:formatting
    #[xml(attr = "w:formatting")]
    pub formatting: Option<bool>,
    /// Display Ink Annotations
    /// Represents the following attribute in the schema: w:inkAnnotations
    #[xml(attr = "w:inkAnnotations")]
    pub ink_annotations: Option<bool>,
}
/// Document Editing Restrictions.
/// When the object is serialized out as xml, it's qualified name is w:documentProtection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:documentProtection")]
pub struct DocumentProtection {
    /// Document Editing Restrictions
    /// Represents the following attribute in the schema: w:edit
    #[xml(attr = "w:edit")]
    pub edit: Option<DocumentProtectionValues>,
    /// Only Allow Formatting With Unlocked Styles
    /// Represents the following attribute in the schema: w:formatting
    #[xml(attr = "w:formatting")]
    pub formatting: Option<bool>,
    /// Enforce Document Protection Settings
    /// Represents the following attribute in the schema: w:enforcement
    #[xml(attr = "w:enforcement")]
    pub enforcement: Option<bool>,
    /// Cryptographic Provider Type
    /// Represents the following attribute in the schema: w:cryptProviderType
    #[xml(attr = "w:cryptProviderType")]
    pub cryptographic_provider_type: Option<CryptProviderValues>,
    /// Cryptographic Algorithm Class
    /// Represents the following attribute in the schema: w:cryptAlgorithmClass
    #[xml(attr = "w:cryptAlgorithmClass")]
    pub cryptographic_algorithm_class: Option<CryptAlgorithmClassValues>,
    /// Cryptographic Algorithm Type
    /// Represents the following attribute in the schema: w:cryptAlgorithmType
    #[xml(attr = "w:cryptAlgorithmType")]
    pub cryptographic_algorithm_type: Option<CryptAlgorithmValues>,
    /// Cryptographic Hashing Algorithm
    /// Represents the following attribute in the schema: w:cryptAlgorithmSid
    #[xml(attr = "w:cryptAlgorithmSid")]
    pub cryptographic_algorithm_sid: Option<i32>,
    /// Iterations to Run Hashing Algorithm
    /// Represents the following attribute in the schema: w:cryptSpinCount
    #[xml(attr = "w:cryptSpinCount")]
    pub cryptographic_spin_count: Option<i32>,
    /// Cryptographic Provider
    /// Represents the following attribute in the schema: w:cryptProvider
    #[xml(attr = "w:cryptProvider")]
    pub cryptographic_provider: Option<String>,
    /// Cryptographic Algorithm Extensibility
    /// Represents the following attribute in the schema: w:algIdExt
    #[xml(attr = "w:algIdExt")]
    pub algorithm_id_extensibility: Option<String>,
    /// Algorithm Extensibility Source
    /// Represents the following attribute in the schema: w:algIdExtSource
    #[xml(attr = "w:algIdExtSource")]
    pub algorithm_id_extensibility_source: Option<String>,
    /// Cryptographic Provider Type Extensibility
    /// Represents the following attribute in the schema: w:cryptProviderTypeExt
    #[xml(attr = "w:cryptProviderTypeExt")]
    pub cryptographic_provider_type_extensibility: Option<String>,
    /// Provider Type Extensibility Source
    /// Represents the following attribute in the schema: w:cryptProviderTypeExtSource
    #[xml(attr = "w:cryptProviderTypeExtSource")]
    pub cryptographic_provider_type_ext_source: Option<String>,
    /// Password Hash
    /// Represents the following attribute in the schema: w:hash
    #[xml(attr = "w:hash")]
    pub hash: Option<String>,
    /// Salt for Password Verifier
    /// Represents the following attribute in the schema: w:salt
    #[xml(attr = "w:salt")]
    pub salt: Option<String>,
    /// algorithmName
    /// Represents the following attribute in the schema: w:algorithmName
    #[xml(attr = "w:algorithmName")]
    pub algorithm_name: Option<String>,
    /// hashValue
    /// Represents the following attribute in the schema: w:hashValue
    #[xml(attr = "w:hashValue")]
    pub hash_value: Option<String>,
    /// saltValue
    /// Represents the following attribute in the schema: w:saltValue
    #[xml(attr = "w:saltValue")]
    pub salt_value: Option<String>,
    /// spinCount
    /// Represents the following attribute in the schema: w:spinCount
    #[xml(attr = "w:spinCount")]
    pub spin_count: Option<i32>,
}
/// Distance Between Automatic Tab Stops.
/// When the object is serialized out as xml, it's qualified name is w:defaultTabStop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:defaultTabStop")]
pub struct DefaultTabStop {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Number of Pages Per Booklet.
/// When the object is serialized out as xml, it's qualified name is w:bookFoldPrintingSheets.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:bookFoldPrintingSheets")]
pub struct BookFoldPrintingSheets {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Defines the NonNegativeShortType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct NonNegativeShortType {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Maximum Number of Consecutively Hyphenated Lines.
/// When the object is serialized out as xml, it's qualified name is w:consecutiveHyphenLimit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:consecutiveHyphenLimit")]
pub struct ConsecutiveHyphenLimit {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i16,
}
/// Percentage of Document to Use When Generating Summary.
/// When the object is serialized out as xml, it's qualified name is w:summaryLength.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:summaryLength")]
pub struct SummaryLength {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Distance between Horizontal Gridlines.
/// When the object is serialized out as xml, it's qualified name is w:displayHorizontalDrawingGridEvery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:displayHorizontalDrawingGridEvery")]
pub struct DisplayHorizontalDrawingGrid {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Distance between Vertical Gridlines.
/// When the object is serialized out as xml, it's qualified name is w:displayVerticalDrawingGridEvery.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:displayVerticalDrawingGridEvery")]
pub struct DisplayVerticalDrawingGrid {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Defines the UnsignedInt7Type Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UnsignedInt7Type {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: i32,
}
/// Character-Level Whitespace Compression.
/// When the object is serialized out as xml, it's qualified name is w:characterSpacingControl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:characterSpacingControl")]
pub struct CharacterSpacingControl {
    /// Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: CharacterSpacingValues,
}
/// Custom Set of Characters Which Cannot End a Line.
/// When the object is serialized out as xml, it's qualified name is w:noLineBreaksAfter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noLineBreaksAfter")]
pub struct NoLineBreaksAfterKinsoku {
    /// lang
    /// Represents the following attribute in the schema: w:lang
    #[xml(attr = "w:lang")]
    pub language: String,
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Custom Set Of Characters Which Cannot Begin A Line.
/// When the object is serialized out as xml, it's qualified name is w:noLineBreaksBefore.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:noLineBreaksBefore")]
pub struct NoLineBreaksBeforeKinsoku {
    /// lang
    /// Represents the following attribute in the schema: w:lang
    #[xml(attr = "w:lang")]
    pub language: String,
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Custom XSL Transform To Use When Saving As XML File.
/// When the object is serialized out as xml, it's qualified name is w:saveThroughXslt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:saveThroughXslt")]
pub struct SaveThroughXslt {
    /// XSL Transformation Location
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// Local Identifier for XSL Transform
    /// Represents the following attribute in the schema: w:solutionID
    #[xml(attr = "w:solutionID")]
    pub solution_id: Option<String>,
}
/// Default Properties for VML Objects in Header and Footer.
/// When the object is serialized out as xml, it's qualified name is w:hdrShapeDefaults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:hdrShapeDefaults")]
pub struct HeaderShapeDefaults {
    #[xml(child = "o:shapedefaults", child = "o:shapelayout")]
    pub children: Vec<HeaderShapeDefaultsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HeaderShapeDefaultsChildChoice {
    #[xml(tag = "o:shapedefaults")]
    OShapedefaults(crate::schemas::schemas_microsoft_com_office_office::ShapeDefaults),
    #[xml(tag = "o:shapelayout")]
    OShapelayout(crate::schemas::schemas_microsoft_com_office_office::ShapeLayout),
}
/// Default Properties for VML Objects in Main Document.
/// When the object is serialized out as xml, it's qualified name is w:shapeDefaults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:shapeDefaults")]
pub struct ShapeDefaults {
    #[xml(child = "o:shapedefaults", child = "o:shapelayout")]
    pub children: Vec<ShapeDefaultsChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeDefaultsChildChoice {
    #[xml(tag = "o:shapedefaults")]
    OShapedefaults(crate::schemas::schemas_microsoft_com_office_office::ShapeDefaults),
    #[xml(tag = "o:shapelayout")]
    OShapelayout(crate::schemas::schemas_microsoft_com_office_office::ShapeLayout),
}
/// Defines the ShapeDefaultsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ShapeDefaultsType {
    #[xml(child = "o:shapedefaults", child = "o:shapelayout")]
    pub children: Vec<ShapeDefaultsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeDefaultsTypeChildChoice {
    #[xml(tag = "o:shapedefaults")]
    OShapedefaults(crate::schemas::schemas_microsoft_com_office_office::ShapeDefaults),
    #[xml(tag = "o:shapelayout")]
    OShapelayout(crate::schemas::schemas_microsoft_com_office_office::ShapeLayout),
}
/// Document-Wide Footnote Properties.
/// When the object is serialized out as xml, it's qualified name is w:footnotePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:footnotePr")]
pub struct FootnoteDocumentWideProperties {
    #[xml(
        child = "w:pos",
        child = "w:numFmt",
        child = "w:numStart",
        child = "w:numRestart",
        child = "w:footnote",
    )]
    pub children: Vec<FootnoteDocumentWidePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FootnoteDocumentWidePropertiesChildChoice {
    #[xml(tag = "w:pos")]
    WPos(FootnotePosition),
    #[xml(tag = "w:numFmt")]
    WNumFmt(NumberingFormat),
    #[xml(tag = "w:numStart")]
    WNumStart(NumberingStart),
    #[xml(tag = "w:numRestart")]
    WNumRestart(NumberingRestart),
    #[xml(tag = "w:footnote")]
    WFootnote(FootnoteSpecialReference),
}
/// Document-Wide Endnote Properties.
/// When the object is serialized out as xml, it's qualified name is w:endnotePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:endnotePr")]
pub struct EndnoteDocumentWideProperties {
    #[xml(
        child = "w:pos",
        child = "w:numFmt",
        child = "w:numStart",
        child = "w:numRestart",
        child = "w:endnote",
    )]
    pub children: Vec<EndnoteDocumentWidePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndnoteDocumentWidePropertiesChildChoice {
    #[xml(tag = "w:pos")]
    WPos(EndnotePosition),
    #[xml(tag = "w:numFmt")]
    WNumFmt(NumberingFormat),
    #[xml(tag = "w:numStart")]
    WNumStart(NumberingStart),
    #[xml(tag = "w:numRestart")]
    WNumRestart(NumberingRestart),
    #[xml(tag = "w:endnote")]
    WEndnote(EndnoteSpecialReference),
}
/// Compatibility Settings.
/// When the object is serialized out as xml, it's qualified name is w:compat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:compat")]
pub struct Compatibility {
    ///Use Simplified Rules For Table Border Conflicts
    #[xml(child = "w:useSingleBorderforContiguousCells")]
    pub use_single_border_for_contiguous_cells: Option<
        UseSingleBorderForContiguousCells,
    >,
    ///Emulate WordPerfect 6.x Paragraph Justification
    #[xml(child = "w:wpJustification")]
    pub word_perfect_justification: Option<WordPerfectJustification>,
    ///Do Not Create Custom Tab Stop for Hanging Indent
    #[xml(child = "w:noTabHangInd")]
    pub no_tab_hang_indent: Option<NoTabHangIndent>,
    ///Do Not Add Leading Between Lines of Text
    #[xml(child = "w:noLeading")]
    pub no_leading: Option<NoLeading>,
    ///Add Additional Space Below Baseline For Underlined East Asian Text
    #[xml(child = "w:spaceForUL")]
    pub space_for_underline: Option<SpaceForUnderline>,
    ///Do Not Balance Text Columns within a Section
    #[xml(child = "w:noColumnBalance")]
    pub no_column_balance: Option<NoColumnBalance>,
    ///Balance Single Byte and Double Byte Characters
    #[xml(child = "w:balanceSingleByteDoubleByteWidth")]
    pub balance_single_byte_double_byte_width: Option<BalanceSingleByteDoubleByteWidth>,
    ///Do Not Center Content on Lines With Exact Line Height
    #[xml(child = "w:noExtraLineSpacing")]
    pub no_extra_line_spacing: Option<NoExtraLineSpacing>,
    ///Convert Backslash To Yen Sign When Entered
    #[xml(child = "w:doNotLeaveBackslashAlone")]
    pub do_not_leave_backslash_alone: Option<DoNotLeaveBackslashAlone>,
    ///Underline All Trailing Spaces
    #[xml(child = "w:ulTrailSpace")]
    pub underline_trailing_spaces: Option<UnderlineTrailingSpaces>,
    ///Don't Justify Lines Ending in Soft Line Break
    #[xml(child = "w:doNotExpandShiftReturn")]
    pub do_not_expand_shift_return: Option<DoNotExpandShiftReturn>,
    ///Only Expand/Condense Text By Whole Points
    #[xml(child = "w:spacingInWholePoints")]
    pub spacing_in_whole_points: Option<SpacingInWholePoints>,
    ///Emulate Word 6.0 Line Wrapping for East Asian Text
    #[xml(child = "w:lineWrapLikeWord6")]
    pub line_wrap_like_word6: Option<LineWrapLikeWord6>,
    ///Print Body Text before Header/Footer Contents
    #[xml(child = "w:printBodyTextBeforeHeader")]
    pub print_body_text_before_header: Option<PrintBodyTextBeforeHeader>,
    ///Print Colors as Black And White without Dithering
    #[xml(child = "w:printColBlack")]
    pub print_color_black_white: Option<PrintColorBlackWhite>,
    ///Space width
    #[xml(child = "w:wpSpaceWidth")]
    pub word_perfect_space_width: Option<WordPerfectSpaceWidth>,
    ///Display Page/Column Breaks Present in Frames
    #[xml(child = "w:showBreaksInFrames")]
    pub show_breaks_in_frames: Option<ShowBreaksInFrames>,
    ///Increase Priority Of Font Size During Font Substitution
    #[xml(child = "w:subFontBySize")]
    pub sub_font_by_size: Option<SubFontBySize>,
    ///Ignore Exact Line Height for Last Line on Page
    #[xml(child = "w:suppressBottomSpacing")]
    pub suppress_bottom_spacing: Option<SuppressBottomSpacing>,
    ///Ignore Minimum and Exact Line Height for First Line on Page
    #[xml(child = "w:suppressTopSpacing")]
    pub suppress_top_spacing: Option<SuppressTopSpacing>,
    ///Ignore Minimum Line Height for First Line on Page
    #[xml(child = "w:suppressSpacingAtTopOfPage")]
    pub suppress_spacing_at_top_of_page: Option<SuppressSpacingAtTopOfPage>,
    ///Emulate WordPerfect 5.x Line Spacing
    #[xml(child = "w:suppressTopSpacingWP")]
    pub suppress_top_spacing_word_perfect: Option<SuppressTopSpacingWordPerfect>,
    ///Do Not Use Space Before On First Line After a Page Break
    #[xml(child = "w:suppressSpBfAfterPgBrk")]
    pub suppress_spacing_before_after_page_break: Option<
        SuppressSpacingBeforeAfterPageBreak,
    >,
    ///Swap Paragraph Borders on Odd Numbered Pages
    #[xml(child = "w:swapBordersFacingPages")]
    pub swap_borders_facing_pages: Option<SwapBordersFacingPages>,
    ///Treat Backslash Quotation Delimiter as Two Quotation Marks
    #[xml(child = "w:convMailMergeEsc")]
    pub convert_mail_merge_escape: Option<ConvertMailMergeEscape>,
    ///Emulate WordPerfect 6.x Font Height Calculation
    #[xml(child = "w:truncateFontHeightsLikeWP6")]
    pub truncate_font_heights_like_word_perfect: Option<
        TruncateFontHeightsLikeWordPerfect,
    >,
    ///Emulate Word 5.x for the Macintosh Small Caps Formatting
    #[xml(child = "w:mwSmallCaps")]
    pub mac_word_small_caps: Option<MacWordSmallCaps>,
    ///Use Printer Metrics To Display Documents
    #[xml(child = "w:usePrinterMetrics")]
    pub use_printer_metrics: Option<UsePrinterMetrics>,
    ///Do Not Suppress Paragraph Borders Next To Frames
    #[xml(child = "w:doNotSuppressParagraphBorders")]
    pub do_not_suppress_paragraph_borders: Option<DoNotSuppressParagraphBorders>,
    ///Line Wrap Trailing Spaces
    #[xml(child = "w:wrapTrailSpaces")]
    pub wrap_trail_spaces: Option<WrapTrailSpaces>,
    ///Emulate Word 6.x/95/97 Footnote Placement
    #[xml(child = "w:footnoteLayoutLikeWW8")]
    pub footnote_layout_like_word8: Option<FootnoteLayoutLikeWord8>,
    ///Emulate Word 97 Text Wrapping Around Floating Objects
    #[xml(child = "w:shapeLayoutLikeWW8")]
    pub shape_layout_like_word8: Option<ShapeLayoutLikeWord8>,
    ///Align Table Rows Independently
    #[xml(child = "w:alignTablesRowByRow")]
    pub align_tables_row_by_row: Option<AlignTablesRowByRow>,
    ///Ignore Width of Last Tab Stop When Aligning Paragraph If It Is Not Left Aligned
    #[xml(child = "w:forgetLastTabAlignment")]
    pub forget_last_tab_alignment: Option<ForgetLastTabAlignment>,
    ///Add Document Grid Line Pitch To Lines in Table Cells
    #[xml(child = "w:adjustLineHeightInTable")]
    pub adjust_line_height_in_table: Option<AdjustLineHeightInTable>,
    ///Emulate Word 95 Full-Width Character Spacing
    #[xml(child = "w:autoSpaceLikeWord95")]
    pub auto_space_like_word95: Option<AutoSpaceLikeWord95>,
    ///Do Not Increase Line Height for Raised/Lowered Text
    #[xml(child = "w:noSpaceRaiseLower")]
    pub no_space_raise_lower: Option<NoSpaceRaiseLower>,
    ///Use Fixed Paragraph Spacing for HTML Auto Setting
    #[xml(child = "w:doNotUseHTMLParagraphAutoSpacing")]
    pub do_not_use_html_paragraph_auto_spacing: Option<DoNotUseHtmlParagraphAutoSpacing>,
    ///Ignore Space Before Table When Deciding If Table Should Wrap Floating Object
    #[xml(child = "w:layoutRawTableWidth")]
    pub layout_raw_table_width: Option<LayoutRawTableWidth>,
    ///Allow Table Rows to Wrap Inline Objects Independently
    #[xml(child = "w:layoutTableRowsApart")]
    pub layout_table_rows_apart: Option<LayoutTableRowsApart>,
    ///Emulate Word 97 East Asian Line Breaking
    #[xml(child = "w:useWord97LineBreakRules")]
    pub use_word97_line_break_rules: Option<UseWord97LineBreakRules>,
    ///Do Not Allow Floating Tables To Break Across Pages
    #[xml(child = "w:doNotBreakWrappedTables")]
    pub do_not_break_wrapped_tables: Option<DoNotBreakWrappedTables>,
    ///Do Not Snap to Document Grid in Table Cells with Objects
    #[xml(child = "w:doNotSnapToGridInCell")]
    pub do_not_snap_to_grid_in_cell: Option<DoNotSnapToGridInCell>,
    ///Select Field When First or Last Character Is Selected
    #[xml(child = "w:selectFldWithFirstOrLastChar")]
    pub select_field_with_first_or_last_char: Option<SelectFieldWithFirstOrLastChar>,
    ///Use Legacy Ethiopic and Amharic Line Breaking Rules
    #[xml(child = "w:applyBreakingRules")]
    pub apply_breaking_rules: Option<ApplyBreakingRules>,
    ///Do Not Allow Hanging Punctuation With Character Grid
    #[xml(child = "w:doNotWrapTextWithPunct")]
    pub do_not_wrap_text_with_punctuation: Option<DoNotWrapTextWithPunctuation>,
    ///Do Not Compress Compressible Characters When Using Document Grid
    #[xml(child = "w:doNotUseEastAsianBreakRules")]
    pub do_not_use_east_asian_break_rules: Option<DoNotUseEastAsianBreakRules>,
    ///Emulate Word 2002 Table Style Rules
    #[xml(child = "w:useWord2002TableStyleRules")]
    pub use_word2002_table_style_rules: Option<UseWord2002TableStyleRules>,
    ///Allow Tables to AutoFit Into Page Margins
    #[xml(child = "w:growAutofit")]
    pub grow_autofit: Option<GrowAutofit>,
    ///Do Not Bypass East Asian/Complex Script Layout Code
    #[xml(child = "w:useFELayout")]
    pub use_far_east_layout: Option<UseFarEastLayout>,
    ///Do Not Automatically Apply List Paragraph Style To Bulleted/Numbered Text
    #[xml(child = "w:useNormalStyleForList")]
    pub use_normal_style_for_list: Option<UseNormalStyleForList>,
    ///Ignore Hanging Indent When Creating Tab Stop After Numbering
    #[xml(child = "w:doNotUseIndentAsNumberingTabStop")]
    pub do_not_use_indent_as_numbering_tab_stop: Option<
        DoNotUseIndentAsNumberingTabStop,
    >,
    ///Use Alternate Set of East Asian Line Breaking Rules
    #[xml(child = "w:useAltKinsokuLineBreakRules")]
    pub use_alt_kinsoku_line_break_rules: Option<UseAltKinsokuLineBreakRules>,
    ///Allow Contextual Spacing of Paragraphs in Tables
    #[xml(child = "w:allowSpaceOfSameStyleInTable")]
    pub allow_space_of_same_style_in_table: Option<AllowSpaceOfSameStyleInTable>,
    ///Do Not Ignore Floating Objects When Calculating Paragraph Indentation
    #[xml(child = "w:doNotSuppressIndentation")]
    pub do_not_suppress_indentation: Option<DoNotSuppressIndentation>,
    ///Do Not AutoFit Tables To Fit Next To Wrapped Objects
    #[xml(child = "w:doNotAutofitConstrainedTables")]
    pub do_not_autofit_constrained_tables: Option<DoNotAutofitConstrainedTables>,
    ///Allow Table Columns To Exceed Preferred Widths of Constituent Cells
    #[xml(child = "w:autofitToFirstFixedWidthCell")]
    pub autofit_to_first_fixed_width_cell: Option<AutofitToFirstFixedWidthCell>,
    ///Underline Following Character Following Numbering
    #[xml(child = "w:underlineTabInNumList")]
    pub underline_tab_in_numbering_list: Option<UnderlineTabInNumberingList>,
    ///Always Use Fixed Width for Hangul Characters
    #[xml(child = "w:displayHangulFixedWidth")]
    pub display_hangul_fixed_width: Option<DisplayHangulFixedWidth>,
    ///Always Move Paragraph Mark to Page after a Page Break
    #[xml(child = "w:splitPgBreakAndParaMark")]
    pub split_page_break_and_paragraph_mark: Option<SplitPageBreakAndParagraphMark>,
    ///Don't Vertically Align Cells Containing Floating Objects
    #[xml(child = "w:doNotVertAlignCellWithSp")]
    pub do_not_vertically_align_cell_with_shape: Option<
        DoNotVerticallyAlignCellWithShape,
    >,
    ///Don't Break Table Rows Around Floating Tables
    #[xml(child = "w:doNotBreakConstrainedForcedTable")]
    pub do_not_break_constrained_forced_table: Option<DoNotBreakConstrainedForcedTable>,
    ///Ignore Vertical Alignment in Textboxes
    #[xml(child = "w:doNotVertAlignInTxbx")]
    pub do_not_vertically_align_in_text_box: Option<DoNotVerticallyAlignInTextBox>,
    ///Use ANSI Kerning Pairs from Fonts
    #[xml(child = "w:useAnsiKerningPairs")]
    pub use_ansi_kerning_pairs: Option<UseAnsiKerningPairs>,
    ///Use Cached Paragraph Information for Column Balancing
    #[xml(child = "w:cachedColBalance")]
    pub cached_column_balance: Option<CachedColumnBalance>,
    /// _
    #[xml(child = "w:compatSetting")]
    pub w_compat_setting: Vec<CompatibilitySetting>,
}
/// Document Variables.
/// When the object is serialized out as xml, it's qualified name is w:docVars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:docVars")]
pub struct DocumentVariables {
    /// _
    #[xml(child = "w:docVar")]
    pub w_doc_var: Vec<DocumentVariable>,
}
/// Listing of All Revision Save ID Values.
/// When the object is serialized out as xml, it's qualified name is w:rsids.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:rsids")]
pub struct Rsids {
    ///Original Document Revision Save ID
    #[xml(child = "w:rsidRoot")]
    pub rsid_root: Option<RsidRoot>,
    /// _
    #[xml(child = "w:rsid")]
    pub w_rsid: Vec<Rsid>,
}
/// Theme Color Mappings.
/// When the object is serialized out as xml, it's qualified name is w:clrSchemeMapping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:clrSchemeMapping")]
pub struct ColorSchemeMapping {
    /// Background 1 Theme Color Mapping
    /// Represents the following attribute in the schema: w:bg1
    #[xml(attr = "w:bg1")]
    pub background1: Option<ColorSchemeIndexValues>,
    /// Text 1 Theme Color Mapping
    /// Represents the following attribute in the schema: w:t1
    #[xml(attr = "w:t1")]
    pub text1: Option<ColorSchemeIndexValues>,
    /// Background 2 Theme Color Mapping
    /// Represents the following attribute in the schema: w:bg2
    #[xml(attr = "w:bg2")]
    pub background2: Option<ColorSchemeIndexValues>,
    /// Text 2 Theme Color Mapping
    /// Represents the following attribute in the schema: w:t2
    #[xml(attr = "w:t2")]
    pub text2: Option<ColorSchemeIndexValues>,
    /// Accent 1 Theme Color Mapping
    /// Represents the following attribute in the schema: w:accent1
    #[xml(attr = "w:accent1")]
    pub accent1: Option<ColorSchemeIndexValues>,
    /// Accent 2 Theme Color Mapping
    /// Represents the following attribute in the schema: w:accent2
    #[xml(attr = "w:accent2")]
    pub accent2: Option<ColorSchemeIndexValues>,
    /// Accent3 Theme Color Mapping
    /// Represents the following attribute in the schema: w:accent3
    #[xml(attr = "w:accent3")]
    pub accent3: Option<ColorSchemeIndexValues>,
    /// Accent4 Theme Color Mapping
    /// Represents the following attribute in the schema: w:accent4
    #[xml(attr = "w:accent4")]
    pub accent4: Option<ColorSchemeIndexValues>,
    /// Accent5 Theme Color Mapping
    /// Represents the following attribute in the schema: w:accent5
    #[xml(attr = "w:accent5")]
    pub accent5: Option<ColorSchemeIndexValues>,
    /// Accent6 Theme Color Mapping
    /// Represents the following attribute in the schema: w:accent6
    #[xml(attr = "w:accent6")]
    pub accent6: Option<ColorSchemeIndexValues>,
    /// Hyperlink Theme Color Mapping
    /// Represents the following attribute in the schema: w:hyperlink
    #[xml(attr = "w:hyperlink")]
    pub hyperlink: Option<ColorSchemeIndexValues>,
    /// Followed Hyperlink Theme Color Mapping
    /// Represents the following attribute in the schema: w:followedHyperlink
    #[xml(attr = "w:followedHyperlink")]
    pub followed_hyperlink: Option<ColorSchemeIndexValues>,
}
/// Caption Settings.
/// When the object is serialized out as xml, it's qualified name is w:captions.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:captions")]
pub struct Captions {
    /// _
    #[xml(child = "w:caption")]
    pub w_caption: Vec<Caption>,
    /// _
    #[xml(child = "w:autoCaptions")]
    pub w_auto_captions: Option<AutoCaptions>,
}
/// Freeze Document Layout.
/// When the object is serialized out as xml, it's qualified name is w:readModeInkLockDown.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:readModeInkLockDown")]
pub struct ReadModeInkLockDown {
    /// Use Actual Pages, Not Virtual Pages
    /// Represents the following attribute in the schema: w:actualPg
    #[xml(attr = "w:actualPg")]
    pub use_actual_pages: Option<bool>,
    /// Virtual Page Width
    /// Represents the following attribute in the schema: w:w
    #[xml(attr = "w:w")]
    pub width: i32,
    /// Virtual Page Height
    /// Represents the following attribute in the schema: w:h
    #[xml(attr = "w:h")]
    pub height: i32,
    /// Font Size Scaling
    /// Represents the following attribute in the schema: w:fontSz
    #[xml(attr = "w:fontSz")]
    pub font_size: String,
}
/// Defines the TargetScreenSize Class.
/// When the object is serialized out as xml, it's qualified name is w:targetScreenSz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:targetScreenSz")]
pub struct TargetScreenSize {
    /// Target Screen Size Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: TargetScreenSizeValues,
}
/// Defines the PictureBulletBase Class.
/// When the object is serialized out as xml, it's qualified name is w:pict.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pict")]
pub struct PictureBulletBase {
    #[xml(
        child = "v:group",
        child = "v:image",
        child = "v:line",
        child = "v:oval",
        child = "v:polyline",
        child = "v:rect",
        child = "v:roundrect",
        child = "v:shape",
        child = "v:shapetype",
    )]
    pub children: Vec<PictureBulletBaseChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PictureBulletBaseChildChoice {
    #[xml(tag = "v:group")]
    VGroup(crate::schemas::schemas_microsoft_com_vml::Group),
    #[xml(tag = "v:image")]
    VImage(crate::schemas::schemas_microsoft_com_vml::ImageFile),
    #[xml(tag = "v:line")]
    VLine(crate::schemas::schemas_microsoft_com_vml::Line),
    #[xml(tag = "v:oval")]
    VOval(crate::schemas::schemas_microsoft_com_vml::Oval),
    #[xml(tag = "v:polyline")]
    VPolyline(crate::schemas::schemas_microsoft_com_vml::PolyLine),
    #[xml(tag = "v:rect")]
    VRect(crate::schemas::schemas_microsoft_com_vml::Rectangle),
    #[xml(tag = "v:roundrect")]
    VRoundrect(crate::schemas::schemas_microsoft_com_vml::RoundRectangle),
    #[xml(tag = "v:shape")]
    VShape(crate::schemas::schemas_microsoft_com_vml::Shape),
    #[xml(tag = "v:shapetype")]
    VShapetype(crate::schemas::schemas_microsoft_com_vml::Shapetype),
}
/// Defines the Panose1Number Class.
/// When the object is serialized out as xml, it's qualified name is w:panose1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:panose1")]
pub struct Panose1Number {
    /// Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: String,
}
/// Defines the FontCharSet Class.
/// When the object is serialized out as xml, it's qualified name is w:charset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:charset")]
pub struct FontCharSet {
    /// val
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: Option<String>,
    /// characterSet
    /// Represents the following attribute in the schema: w:characterSet
    #[xml(attr = "w:characterSet")]
    pub strict_character_set: Option<StrictCharacterSet>,
}
/// Defines the FontFamily Class.
/// When the object is serialized out as xml, it's qualified name is w:family.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:family")]
pub struct FontFamily {
    /// Font Family Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: FontFamilyValues,
}
/// Defines the Pitch Class.
/// When the object is serialized out as xml, it's qualified name is w:pitch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:pitch")]
pub struct Pitch {
    /// Value
    /// Represents the following attribute in the schema: w:val
    #[xml(attr = "w:val")]
    pub val: FontPitchValues,
}
/// Defines the FontSignature Class.
/// When the object is serialized out as xml, it's qualified name is w:sig.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:sig")]
pub struct FontSignature {
    /// First 32 Bits of Unicode Subset Bitfield
    /// Represents the following attribute in the schema: w:usb0
    #[xml(attr = "w:usb0")]
    pub unicode_signature0: String,
    /// Second 32 Bits of Unicode Subset Bitfield
    /// Represents the following attribute in the schema: w:usb1
    #[xml(attr = "w:usb1")]
    pub unicode_signature1: String,
    /// Third 32 Bits of Unicode Subset Bitfield
    /// Represents the following attribute in the schema: w:usb2
    #[xml(attr = "w:usb2")]
    pub unicode_signature2: String,
    /// Fourth 32 Bits of Unicode Subset Bitfield
    /// Represents the following attribute in the schema: w:usb3
    #[xml(attr = "w:usb3")]
    pub unicode_signature3: String,
    /// Lower 32 Bits of Code Page Bit Field
    /// Represents the following attribute in the schema: w:csb0
    #[xml(attr = "w:csb0")]
    pub code_page_signature0: String,
    /// Upper 32 Bits of Code Page Bit Field
    /// Represents the following attribute in the schema: w:csb1
    #[xml(attr = "w:csb1")]
    pub code_page_signature1: String,
}
/// Defines the EmbedRegularFont Class.
/// When the object is serialized out as xml, it's qualified name is w:embedRegular.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:embedRegular")]
pub struct EmbedRegularFont {
    /// fontKey
    /// Represents the following attribute in the schema: w:fontKey
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<String>,
    /// subsetted
    /// Represents the following attribute in the schema: w:subsetted
    #[xml(attr = "w:subsetted")]
    pub subsetted: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the EmbedBoldFont Class.
/// When the object is serialized out as xml, it's qualified name is w:embedBold.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:embedBold")]
pub struct EmbedBoldFont {
    /// fontKey
    /// Represents the following attribute in the schema: w:fontKey
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<String>,
    /// subsetted
    /// Represents the following attribute in the schema: w:subsetted
    #[xml(attr = "w:subsetted")]
    pub subsetted: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the EmbedItalicFont Class.
/// When the object is serialized out as xml, it's qualified name is w:embedItalic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:embedItalic")]
pub struct EmbedItalicFont {
    /// fontKey
    /// Represents the following attribute in the schema: w:fontKey
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<String>,
    /// subsetted
    /// Represents the following attribute in the schema: w:subsetted
    #[xml(attr = "w:subsetted")]
    pub subsetted: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the EmbedBoldItalicFont Class.
/// When the object is serialized out as xml, it's qualified name is w:embedBoldItalic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:embedBoldItalic")]
pub struct EmbedBoldItalicFont {
    /// fontKey
    /// Represents the following attribute in the schema: w:fontKey
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<String>,
    /// subsetted
    /// Represents the following attribute in the schema: w:subsetted
    #[xml(attr = "w:subsetted")]
    pub subsetted: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the FontRelationshipType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct FontRelationshipType {
    /// fontKey
    /// Represents the following attribute in the schema: w:fontKey
    #[xml(attr = "w:fontKey")]
    pub font_key: Option<String>,
    /// subsetted
    /// Represents the following attribute in the schema: w:subsetted
    #[xml(attr = "w:subsetted")]
    pub subsetted: Option<bool>,
    /// Relationship to Part
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: String,
}
/// Defines the LevelOverride Class.
/// When the object is serialized out as xml, it's qualified name is w:lvlOverride.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w:lvlOverride")]
pub struct LevelOverride {
    /// Numbering Level ID
    /// Represents the following attribute in the schema: w:ilvl
    #[xml(attr = "w:ilvl")]
    pub level_index: i32,
    ///Numbering Level Starting Value Override
    #[xml(child = "w:startOverride")]
    pub start_override_numbering_value: Option<StartOverrideNumberingValue>,
    ///Numbering Level Override Definition
    #[xml(child = "w:lvl")]
    pub level: Option<Level>,
}
