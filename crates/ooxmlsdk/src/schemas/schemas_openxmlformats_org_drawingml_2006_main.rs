#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FontCollectionIndexValues {
    #[default]
    Major,
    Minor,
    None,
}
crate::__string_enum! {
    FontCollectionIndexValues { Major = "major", Minor = "minor", None = "none", }
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
    ColorSchemeIndexValues { Dark1 = "dk1", Light1 = "lt1", Dark2 = "dk2", Light2 =
    "lt2", Accent1 = "accent1", Accent2 = "accent2", Accent3 = "accent3", Accent4 =
    "accent4", Accent5 = "accent5", Accent6 = "accent6", Hyperlink = "hlink",
    FollowedHyperlink = "folHlink", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SystemColorValues {
    #[default]
    ScrollBar,
    Background,
    ActiveCaption,
    InactiveCaption,
    Menu,
    Window,
    WindowFrame,
    MenuText,
    WindowText,
    CaptionText,
    ActiveBorder,
    InactiveBorder,
    ApplicationWorkspace,
    Highlight,
    HighlightText,
    ButtonFace,
    ButtonShadow,
    GrayText,
    ButtonText,
    InactiveCaptionText,
    ButtonHighlight,
    ThreeDDarkShadow,
    ThreeDLight,
    InfoText,
    InfoBack,
    HotLight,
    GradientActiveCaption,
    GradientInactiveCaption,
    MenuHighlight,
    MenuBar,
}
crate::__string_enum! {
    SystemColorValues { ScrollBar = "scrollBar", Background = "background", ActiveCaption
    = "activeCaption", InactiveCaption = "inactiveCaption", Menu = "menu", Window =
    "window", WindowFrame = "windowFrame", MenuText = "menuText", WindowText =
    "windowText", CaptionText = "captionText", ActiveBorder = "activeBorder",
    InactiveBorder = "inactiveBorder", ApplicationWorkspace = "appWorkspace", Highlight =
    "highlight", HighlightText = "highlightText", ButtonFace = "btnFace", ButtonShadow =
    "btnShadow", GrayText = "grayText", ButtonText = "btnText", InactiveCaptionText =
    "inactiveCaptionText", ButtonHighlight = "btnHighlight", ThreeDDarkShadow =
    "3dDkShadow", ThreeDLight = "3dLight", InfoText = "infoText", InfoBack = "infoBk",
    HotLight = "hotLight", GradientActiveCaption = "gradientActiveCaption",
    GradientInactiveCaption = "gradientInactiveCaption", MenuHighlight = "menuHighlight",
    MenuBar = "menuBar", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SchemeColorValues {
    #[default]
    Background1,
    Text1,
    Background2,
    Text2,
    Accent1,
    Accent2,
    Accent3,
    Accent4,
    Accent5,
    Accent6,
    Hyperlink,
    FollowedHyperlink,
    PhColor,
    Dark1,
    Light1,
    Dark2,
    Light2,
}
crate::__string_enum! {
    SchemeColorValues { Background1 = "bg1", Text1 = "tx1", Background2 = "bg2", Text2 =
    "tx2", Accent1 = "accent1", Accent2 = "accent2", Accent3 = "accent3", Accent4 =
    "accent4", Accent5 = "accent5", Accent6 = "accent6", Hyperlink = "hlink",
    FollowedHyperlink = "folHlink", PhColor = "phClr", Dark1 = "dk1", Light1 = "lt1",
    Dark2 = "dk2", Light2 = "lt2", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RectangleAlignmentValues {
    #[default]
    TopLeft,
    Top,
    TopRight,
    Left,
    Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}
crate::__string_enum! {
    RectangleAlignmentValues { TopLeft = "tl", Top = "t", TopRight = "tr", Left = "l",
    Center = "ctr", Right = "r", BottomLeft = "bl", Bottom = "b", BottomRight = "br", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BlackWhiteModeValues {
    #[default]
    Color,
    Auto,
    Gray,
    LightGray,
    InvGray,
    GrayWhite,
    BlackGray,
    BlackWhite,
    Black,
    White,
    Hidden,
}
crate::__string_enum! {
    BlackWhiteModeValues { Color = "clr", Auto = "auto", Gray = "gray", LightGray =
    "ltGray", InvGray = "invGray", GrayWhite = "grayWhite", BlackGray = "blackGray",
    BlackWhite = "blackWhite", Black = "black", White = "white", Hidden = "hidden", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChartBuildStepValues {
    #[default]
    Category,
    CategoryPoints,
    Series,
    SeriesPoints,
    AllPoints,
    GridLegend,
}
crate::__string_enum! {
    ChartBuildStepValues { Category = "category", CategoryPoints = "ptInCategory", Series
    = "series", SeriesPoints = "ptInSeries", AllPoints = "allPts", GridLegend =
    "gridLegend", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DiagramBuildStepValues {
    #[default]
    Shape,
    Background,
}
crate::__string_enum! {
    DiagramBuildStepValues { Shape = "sp", Background = "bg", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimationBuildValues {
    #[default]
    AllAtOnce,
}
crate::__string_enum! {
    AnimationBuildValues { AllAtOnce = "allAtOnce", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimationDiagramOnlyBuildValues {
    #[default]
    One,
    LevelOne,
    LevelAtOnce,
}
crate::__string_enum! {
    AnimationDiagramOnlyBuildValues { One = "one", LevelOne = "lvlOne", LevelAtOnce =
    "lvlAtOnce", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimationChartOnlyBuildValues {
    #[default]
    Series,
    Category,
    SeriesElement,
    CategoryElement,
}
crate::__string_enum! {
    AnimationChartOnlyBuildValues { Series = "series", Category = "category",
    SeriesElement = "seriesEl", CategoryElement = "categoryEl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetCameraValues {
    #[default]
    LegacyObliqueTopLeft,
    LegacyObliqueTop,
    LegacyObliqueTopRight,
    LegacyObliqueLeft,
    LegacyObliqueFront,
    LegacyObliqueRight,
    LegacyObliqueBottomLeft,
    LegacyObliqueBottom,
    LegacyObliqueBottomRight,
    LegacyPerspectiveTopLeft,
    LegacyPerspectiveTop,
    LegacyPerspectiveTopRight,
    LegacyPerspectiveLeft,
    LegacyPerspectiveFront,
    LegacyPerspectiveRight,
    LegacyPerspectiveBottomLeft,
    LegacyPerspectiveBottom,
    LegacyPerspectiveBottomRight,
    OrthographicFront,
    IsometricTopUp,
    IsometricTopDown,
    IsometricBottomUp,
    IsometricBottomDown,
    IsometricLeftUp,
    IsometricLeftDown,
    IsometricRightUp,
    IsometricRightDown,
    IsometricOffAxis1Left,
    IsometricOffAxis1Right,
    IsometricOffAxis1Top,
    IsometricOffAxis2Left,
    IsometricOffAxis2Right,
    IsometricOffAxis2Top,
    IsometricOffAxis3Left,
    IsometricOffAxis3Right,
    IsometricOffAxis3Bottom,
    IsometricOffAxis4Left,
    IsometricOffAxis4Right,
    IsometricOffAxis4Bottom,
    ObliqueTopLeft,
    ObliqueTop,
    ObliqueTopRight,
    ObliqueLeft,
    ObliqueRight,
    ObliqueBottomLeft,
    ObliqueBottom,
    ObliqueBottomRight,
    PerspectiveFront,
    PerspectiveLeft,
    PerspectiveRight,
    PerspectiveAbove,
    PerspectiveBelow,
    PerspectiveAboveLeftFacing,
    PerspectiveAboveRightFacing,
    PerspectiveContrastingLeftFacing,
    PerspectiveContrastingRightFacing,
    PerspectiveHeroicLeftFacing,
    PerspectiveHeroicRightFacing,
    PerspectiveHeroicExtremeLeftFacing,
    PerspectiveHeroicExtremeRightFacing,
    PerspectiveRelaxed,
    PerspectiveRelaxedModerately,
}
crate::__string_enum! {
    PresetCameraValues { LegacyObliqueTopLeft = "legacyObliqueTopLeft", LegacyObliqueTop
    = "legacyObliqueTop", LegacyObliqueTopRight = "legacyObliqueTopRight",
    LegacyObliqueLeft = "legacyObliqueLeft", LegacyObliqueFront = "legacyObliqueFront",
    LegacyObliqueRight = "legacyObliqueRight", LegacyObliqueBottomLeft =
    "legacyObliqueBottomLeft", LegacyObliqueBottom = "legacyObliqueBottom",
    LegacyObliqueBottomRight = "legacyObliqueBottomRight", LegacyPerspectiveTopLeft =
    "legacyPerspectiveTopLeft", LegacyPerspectiveTop = "legacyPerspectiveTop",
    LegacyPerspectiveTopRight = "legacyPerspectiveTopRight", LegacyPerspectiveLeft =
    "legacyPerspectiveLeft", LegacyPerspectiveFront = "legacyPerspectiveFront",
    LegacyPerspectiveRight = "legacyPerspectiveRight", LegacyPerspectiveBottomLeft =
    "legacyPerspectiveBottomLeft", LegacyPerspectiveBottom = "legacyPerspectiveBottom",
    LegacyPerspectiveBottomRight = "legacyPerspectiveBottomRight", OrthographicFront =
    "orthographicFront", IsometricTopUp = "isometricTopUp", IsometricTopDown =
    "isometricTopDown", IsometricBottomUp = "isometricBottomUp", IsometricBottomDown =
    "isometricBottomDown", IsometricLeftUp = "isometricLeftUp", IsometricLeftDown =
    "isometricLeftDown", IsometricRightUp = "isometricRightUp", IsometricRightDown =
    "isometricRightDown", IsometricOffAxis1Left = "isometricOffAxis1Left",
    IsometricOffAxis1Right = "isometricOffAxis1Right", IsometricOffAxis1Top =
    "isometricOffAxis1Top", IsometricOffAxis2Left = "isometricOffAxis2Left",
    IsometricOffAxis2Right = "isometricOffAxis2Right", IsometricOffAxis2Top =
    "isometricOffAxis2Top", IsometricOffAxis3Left = "isometricOffAxis3Left",
    IsometricOffAxis3Right = "isometricOffAxis3Right", IsometricOffAxis3Bottom =
    "isometricOffAxis3Bottom", IsometricOffAxis4Left = "isometricOffAxis4Left",
    IsometricOffAxis4Right = "isometricOffAxis4Right", IsometricOffAxis4Bottom =
    "isometricOffAxis4Bottom", ObliqueTopLeft = "obliqueTopLeft", ObliqueTop =
    "obliqueTop", ObliqueTopRight = "obliqueTopRight", ObliqueLeft = "obliqueLeft",
    ObliqueRight = "obliqueRight", ObliqueBottomLeft = "obliqueBottomLeft", ObliqueBottom
    = "obliqueBottom", ObliqueBottomRight = "obliqueBottomRight", PerspectiveFront =
    "perspectiveFront", PerspectiveLeft = "perspectiveLeft", PerspectiveRight =
    "perspectiveRight", PerspectiveAbove = "perspectiveAbove", PerspectiveBelow =
    "perspectiveBelow", PerspectiveAboveLeftFacing = "perspectiveAboveLeftFacing",
    PerspectiveAboveRightFacing = "perspectiveAboveRightFacing",
    PerspectiveContrastingLeftFacing = "perspectiveContrastingLeftFacing",
    PerspectiveContrastingRightFacing = "perspectiveContrastingRightFacing",
    PerspectiveHeroicLeftFacing = "perspectiveHeroicLeftFacing",
    PerspectiveHeroicRightFacing = "perspectiveHeroicRightFacing",
    PerspectiveHeroicExtremeLeftFacing = "perspectiveHeroicExtremeLeftFacing",
    PerspectiveHeroicExtremeRightFacing = "perspectiveHeroicExtremeRightFacing",
    PerspectiveRelaxed = "perspectiveRelaxed", PerspectiveRelaxedModerately =
    "perspectiveRelaxedModerately", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LightRigDirectionValues {
    #[default]
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}
crate::__string_enum! {
    LightRigDirectionValues { TopLeft = "tl", Top = "t", TopRight = "tr", Left = "l",
    Right = "r", BottomLeft = "bl", Bottom = "b", BottomRight = "br", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LightRigValues {
    #[default]
    LegacyFlat1,
    LegacyFlat2,
    LegacyFlat3,
    LegacyFlat4,
    LegacyNormal1,
    LegacyNormal2,
    LegacyNormal3,
    LegacyNormal4,
    LegacyHarsh1,
    LegacyHarsh2,
    LegacyHarsh3,
    LegacyHarsh4,
    ThreePoints,
    Balanced,
    Soft,
    Harsh,
    Flood,
    Contrasting,
    Morning,
    Sunrise,
    Sunset,
    Chilly,
    Freezing,
    Flat,
    TwoPoints,
    Glow,
    BrightRoom,
}
crate::__string_enum! {
    LightRigValues { LegacyFlat1 = "legacyFlat1", LegacyFlat2 = "legacyFlat2",
    LegacyFlat3 = "legacyFlat3", LegacyFlat4 = "legacyFlat4", LegacyNormal1 =
    "legacyNormal1", LegacyNormal2 = "legacyNormal2", LegacyNormal3 = "legacyNormal3",
    LegacyNormal4 = "legacyNormal4", LegacyHarsh1 = "legacyHarsh1", LegacyHarsh2 =
    "legacyHarsh2", LegacyHarsh3 = "legacyHarsh3", LegacyHarsh4 = "legacyHarsh4",
    ThreePoints = "threePt", Balanced = "balanced", Soft = "soft", Harsh = "harsh", Flood
    = "flood", Contrasting = "contrasting", Morning = "morning", Sunrise = "sunrise",
    Sunset = "sunset", Chilly = "chilly", Freezing = "freezing", Flat = "flat", TwoPoints
    = "twoPt", Glow = "glow", BrightRoom = "brightRoom", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BevelPresetValues {
    #[default]
    RelaxedInset,
    Circle,
    Slope,
    Cross,
    Angle,
    SoftRound,
    Convex,
    CoolSlant,
    Divot,
    Riblet,
    HardEdge,
    ArtDeco,
}
crate::__string_enum! {
    BevelPresetValues { RelaxedInset = "relaxedInset", Circle = "circle", Slope =
    "slope", Cross = "cross", Angle = "angle", SoftRound = "softRound", Convex =
    "convex", CoolSlant = "coolSlant", Divot = "divot", Riblet = "riblet", HardEdge =
    "hardEdge", ArtDeco = "artDeco", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetMaterialTypeValues {
    #[default]
    LegacyMatte,
    LegacyPlastic,
    LegacyMetal,
    LegacyWireframe,
    Matte,
    Plastic,
    Metal,
    WarmMatte,
    TranslucentPowder,
    Powder,
    DarkEdge,
    SoftEdge,
    Clear,
    Flat,
    SoftMetal,
}
crate::__string_enum! {
    PresetMaterialTypeValues { LegacyMatte = "legacyMatte", LegacyPlastic =
    "legacyPlastic", LegacyMetal = "legacyMetal", LegacyWireframe = "legacyWireframe",
    Matte = "matte", Plastic = "plastic", Metal = "metal", WarmMatte = "warmMatte",
    TranslucentPowder = "translucentPowder", Powder = "powder", DarkEdge = "dkEdge",
    SoftEdge = "softEdge", Clear = "clear", Flat = "flat", SoftMetal = "softmetal", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetShadowValues {
    #[default]
    TopLeftDropShadow,
    TopRightDropShadow,
    BackLeftPerspectiveShadow,
    BackRightPerspectiveShadow,
    BottomLeftDropShadow,
    BottomRightDropShadow,
    FrontLeftPerspectiveShadow,
    FrontRightPerspectiveShadow,
    TopLeftSmallDropShadow,
    TopLeftLargeDropShadow,
    BackLeftLongPerspectiveShadow,
    BackRightLongPerspectiveShadow,
    TopLeftDoubleDropShadow,
    BottomRightSmallDropShadow,
    FrontLeftLongPerspectiveShadow,
    FrontRightLongPerspectiveShadow,
    ThreeDimensionalOuterBoxShadow,
    ThreeDimensionalInnerBoxShadow,
    BackCenterPerspectiveShadow,
    FrontBottomShadow,
}
crate::__string_enum! {
    PresetShadowValues { TopLeftDropShadow = "shdw1", TopRightDropShadow = "shdw2",
    BackLeftPerspectiveShadow = "shdw3", BackRightPerspectiveShadow = "shdw4",
    BottomLeftDropShadow = "shdw5", BottomRightDropShadow = "shdw6",
    FrontLeftPerspectiveShadow = "shdw7", FrontRightPerspectiveShadow = "shdw8",
    TopLeftSmallDropShadow = "shdw9", TopLeftLargeDropShadow = "shdw10",
    BackLeftLongPerspectiveShadow = "shdw11", BackRightLongPerspectiveShadow = "shdw12",
    TopLeftDoubleDropShadow = "shdw13", BottomRightSmallDropShadow = "shdw14",
    FrontLeftLongPerspectiveShadow = "shdw15", FrontRightLongPerspectiveShadow =
    "shdw16", ThreeDimensionalOuterBoxShadow = "shdw17", ThreeDimensionalInnerBoxShadow =
    "shdw18", BackCenterPerspectiveShadow = "shdw19", FrontBottomShadow = "shdw20", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PathShadeValues {
    #[default]
    Shape,
    Circle,
    Rectangle,
}
crate::__string_enum! {
    PathShadeValues { Shape = "shape", Circle = "circle", Rectangle = "rect", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TileFlipValues {
    #[default]
    None,
    Horizontal,
    Vertical,
    HorizontalAndVertical,
}
crate::__string_enum! {
    TileFlipValues { None = "none", Horizontal = "x", Vertical = "y",
    HorizontalAndVertical = "xy", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BlipCompressionValues {
    #[default]
    Email,
    Screen,
    Print,
    HighQualityPrint,
    None,
}
crate::__string_enum! {
    BlipCompressionValues { Email = "email", Screen = "screen", Print = "print",
    HighQualityPrint = "hqprint", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetPatternValues {
    #[default]
    Percent5,
    Percent10,
    Percent20,
    Percent25,
    Percent30,
    Percent40,
    Percent50,
    Percent60,
    Percent70,
    Percent75,
    Percent80,
    Percent90,
    Horizontal,
    Vertical,
    LightHorizontal,
    LightVertical,
    DarkHorizontal,
    DarkVertical,
    NarrowHorizontal,
    NarrowVertical,
    DashedHorizontal,
    DashedVertical,
    Cross,
    DownwardDiagonal,
    UpwardDiagonal,
    LightDownwardDiagonal,
    LightUpwardDiagonal,
    DarkDownwardDiagonal,
    DarkUpwardDiagonal,
    WideDownwardDiagonal,
    WideUpwardDiagonal,
    DashedDownwardDiagonal,
    DashedUpwardDiagonal,
    DiagonalCross,
    SmallCheck,
    LargeCheck,
    SmallGrid,
    LargeGrid,
    DotGrid,
    SmallConfetti,
    LargeConfetti,
    HorizontalBrick,
    DiagonalBrick,
    SolidDiamond,
    OpenDiamond,
    DottedDiamond,
    Plaid,
    Sphere,
    Weave,
    Divot,
    Shingle,
    Wave,
    Trellis,
    ZigZag,
}
crate::__string_enum! {
    PresetPatternValues { Percent5 = "pct5", Percent10 = "pct10", Percent20 = "pct20",
    Percent25 = "pct25", Percent30 = "pct30", Percent40 = "pct40", Percent50 = "pct50",
    Percent60 = "pct60", Percent70 = "pct70", Percent75 = "pct75", Percent80 = "pct80",
    Percent90 = "pct90", Horizontal = "horz", Vertical = "vert", LightHorizontal =
    "ltHorz", LightVertical = "ltVert", DarkHorizontal = "dkHorz", DarkVertical =
    "dkVert", NarrowHorizontal = "narHorz", NarrowVertical = "narVert", DashedHorizontal
    = "dashHorz", DashedVertical = "dashVert", Cross = "cross", DownwardDiagonal =
    "dnDiag", UpwardDiagonal = "upDiag", LightDownwardDiagonal = "ltDnDiag",
    LightUpwardDiagonal = "ltUpDiag", DarkDownwardDiagonal = "dkDnDiag",
    DarkUpwardDiagonal = "dkUpDiag", WideDownwardDiagonal = "wdDnDiag",
    WideUpwardDiagonal = "wdUpDiag", DashedDownwardDiagonal = "dashDnDiag",
    DashedUpwardDiagonal = "dashUpDiag", DiagonalCross = "diagCross", SmallCheck =
    "smCheck", LargeCheck = "lgCheck", SmallGrid = "smGrid", LargeGrid = "lgGrid",
    DotGrid = "dotGrid", SmallConfetti = "smConfetti", LargeConfetti = "lgConfetti",
    HorizontalBrick = "horzBrick", DiagonalBrick = "diagBrick", SolidDiamond =
    "solidDmnd", OpenDiamond = "openDmnd", DottedDiamond = "dotDmnd", Plaid = "plaid",
    Sphere = "sphere", Weave = "weave", Divot = "divot", Shingle = "shingle", Wave =
    "wave", Trellis = "trellis", ZigZag = "zigZag", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BlendModeValues {
    #[default]
    Overlay,
    Multiply,
    Screen,
    Darken,
    Lighten,
}
crate::__string_enum! {
    BlendModeValues { Overlay = "over", Multiply = "mult", Screen = "screen", Darken =
    "darken", Lighten = "lighten", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EffectContainerValues {
    #[default]
    Sibling,
    Tree,
}
crate::__string_enum! {
    EffectContainerValues { Sibling = "sib", Tree = "tree", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ShapeTypeValues {
    #[default]
    Line,
    LineInverse,
    Triangle,
    RightTriangle,
    Rectangle,
    Diamond,
    Parallelogram,
    Trapezoid,
    NonIsoscelesTrapezoid,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Decagon,
    Dodecagon,
    Star4,
    Star5,
    Star6,
    Star7,
    Star8,
    Star10,
    Star12,
    Star16,
    Star24,
    Star32,
    RoundRectangle,
    Round1Rectangle,
    Round2SameRectangle,
    Round2DiagonalRectangle,
    SnipRoundRectangle,
    Snip1Rectangle,
    Snip2SameRectangle,
    Snip2DiagonalRectangle,
    Plaque,
    Ellipse,
    Teardrop,
    HomePlate,
    Chevron,
    PieWedge,
    Pie,
    BlockArc,
    Donut,
    NoSmoking,
    RightArrow,
    LeftArrow,
    UpArrow,
    DownArrow,
    StripedRightArrow,
    NotchedRightArrow,
    BentUpArrow,
    LeftRightArrow,
    UpDownArrow,
    LeftUpArrow,
    LeftRightUpArrow,
    QuadArrow,
    LeftArrowCallout,
    RightArrowCallout,
    UpArrowCallout,
    DownArrowCallout,
    LeftRightArrowCallout,
    UpDownArrowCallout,
    QuadArrowCallout,
    BentArrow,
    UTurnArrow,
    CircularArrow,
    LeftCircularArrow,
    LeftRightCircularArrow,
    CurvedRightArrow,
    CurvedLeftArrow,
    CurvedUpArrow,
    CurvedDownArrow,
    SwooshArrow,
    Cube,
    Can,
    LightningBolt,
    Heart,
    Sun,
    Moon,
    SmileyFace,
    IrregularSeal1,
    IrregularSeal2,
    FoldedCorner,
    Bevel,
    Frame,
    HalfFrame,
    Corner,
    DiagonalStripe,
    Chord,
    Arc,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    BracketPair,
    BracePair,
    StraightConnector1,
    BentConnector2,
    BentConnector3,
    BentConnector4,
    BentConnector5,
    CurvedConnector2,
    CurvedConnector3,
    CurvedConnector4,
    CurvedConnector5,
    Callout1,
    Callout2,
    Callout3,
    AccentCallout1,
    AccentCallout2,
    AccentCallout3,
    BorderCallout1,
    BorderCallout2,
    BorderCallout3,
    AccentBorderCallout1,
    AccentBorderCallout2,
    AccentBorderCallout3,
    WedgeRectangleCallout,
    WedgeRoundRectangleCallout,
    WedgeEllipseCallout,
    CloudCallout,
    Cloud,
    Ribbon,
    Ribbon2,
    EllipseRibbon,
    EllipseRibbon2,
    LeftRightRibbon,
    VerticalScroll,
    HorizontalScroll,
    Wave,
    DoubleWave,
    Plus,
    FlowChartProcess,
    FlowChartDecision,
    FlowChartInputOutput,
    FlowChartPredefinedProcess,
    FlowChartInternalStorage,
    FlowChartDocument,
    FlowChartMultidocument,
    FlowChartTerminator,
    FlowChartPreparation,
    FlowChartManualInput,
    FlowChartManualOperation,
    FlowChartConnector,
    FlowChartPunchedCard,
    FlowChartPunchedTape,
    FlowChartSummingJunction,
    FlowChartOr,
    FlowChartCollate,
    FlowChartSort,
    FlowChartExtract,
    FlowChartMerge,
    FlowChartOfflineStorage,
    FlowChartOnlineStorage,
    FlowChartMagneticTape,
    FlowChartMagneticDisk,
    FlowChartMagneticDrum,
    FlowChartDisplay,
    FlowChartDelay,
    FlowChartAlternateProcess,
    FlowChartOffpageConnector,
    ActionButtonBlank,
    ActionButtonHome,
    ActionButtonHelp,
    ActionButtonInformation,
    ActionButtonForwardNext,
    ActionButtonBackPrevious,
    ActionButtonEnd,
    ActionButtonBeginning,
    ActionButtonReturn,
    ActionButtonDocument,
    ActionButtonSound,
    ActionButtonMovie,
    Gear6,
    Gear9,
    Funnel,
    MathPlus,
    MathMinus,
    MathMultiply,
    MathDivide,
    MathEqual,
    MathNotEqual,
    CornerTabs,
    SquareTabs,
    PlaqueTabs,
    ChartX,
    ChartStar,
    ChartPlus,
}
crate::__string_enum! {
    ShapeTypeValues { Line = "line", LineInverse = "lineInv", Triangle = "triangle",
    RightTriangle = "rtTriangle", Rectangle = "rect", Diamond = "diamond", Parallelogram
    = "parallelogram", Trapezoid = "trapezoid", NonIsoscelesTrapezoid =
    "nonIsoscelesTrapezoid", Pentagon = "pentagon", Hexagon = "hexagon", Heptagon =
    "heptagon", Octagon = "octagon", Decagon = "decagon", Dodecagon = "dodecagon", Star4
    = "star4", Star5 = "star5", Star6 = "star6", Star7 = "star7", Star8 = "star8", Star10
    = "star10", Star12 = "star12", Star16 = "star16", Star24 = "star24", Star32 =
    "star32", RoundRectangle = "roundRect", Round1Rectangle = "round1Rect",
    Round2SameRectangle = "round2SameRect", Round2DiagonalRectangle = "round2DiagRect",
    SnipRoundRectangle = "snipRoundRect", Snip1Rectangle = "snip1Rect",
    Snip2SameRectangle = "snip2SameRect", Snip2DiagonalRectangle = "snip2DiagRect",
    Plaque = "plaque", Ellipse = "ellipse", Teardrop = "teardrop", HomePlate =
    "homePlate", Chevron = "chevron", PieWedge = "pieWedge", Pie = "pie", BlockArc =
    "blockArc", Donut = "donut", NoSmoking = "noSmoking", RightArrow = "rightArrow",
    LeftArrow = "leftArrow", UpArrow = "upArrow", DownArrow = "downArrow",
    StripedRightArrow = "stripedRightArrow", NotchedRightArrow = "notchedRightArrow",
    BentUpArrow = "bentUpArrow", LeftRightArrow = "leftRightArrow", UpDownArrow =
    "upDownArrow", LeftUpArrow = "leftUpArrow", LeftRightUpArrow = "leftRightUpArrow",
    QuadArrow = "quadArrow", LeftArrowCallout = "leftArrowCallout", RightArrowCallout =
    "rightArrowCallout", UpArrowCallout = "upArrowCallout", DownArrowCallout =
    "downArrowCallout", LeftRightArrowCallout = "leftRightArrowCallout",
    UpDownArrowCallout = "upDownArrowCallout", QuadArrowCallout = "quadArrowCallout",
    BentArrow = "bentArrow", UTurnArrow = "uturnArrow", CircularArrow = "circularArrow",
    LeftCircularArrow = "leftCircularArrow", LeftRightCircularArrow =
    "leftRightCircularArrow", CurvedRightArrow = "curvedRightArrow", CurvedLeftArrow =
    "curvedLeftArrow", CurvedUpArrow = "curvedUpArrow", CurvedDownArrow =
    "curvedDownArrow", SwooshArrow = "swooshArrow", Cube = "cube", Can = "can",
    LightningBolt = "lightningBolt", Heart = "heart", Sun = "sun", Moon = "moon",
    SmileyFace = "smileyFace", IrregularSeal1 = "irregularSeal1", IrregularSeal2 =
    "irregularSeal2", FoldedCorner = "foldedCorner", Bevel = "bevel", Frame = "frame",
    HalfFrame = "halfFrame", Corner = "corner", DiagonalStripe = "diagStripe", Chord =
    "chord", Arc = "arc", LeftBracket = "leftBracket", RightBracket = "rightBracket",
    LeftBrace = "leftBrace", RightBrace = "rightBrace", BracketPair = "bracketPair",
    BracePair = "bracePair", StraightConnector1 = "straightConnector1", BentConnector2 =
    "bentConnector2", BentConnector3 = "bentConnector3", BentConnector4 =
    "bentConnector4", BentConnector5 = "bentConnector5", CurvedConnector2 =
    "curvedConnector2", CurvedConnector3 = "curvedConnector3", CurvedConnector4 =
    "curvedConnector4", CurvedConnector5 = "curvedConnector5", Callout1 = "callout1",
    Callout2 = "callout2", Callout3 = "callout3", AccentCallout1 = "accentCallout1",
    AccentCallout2 = "accentCallout2", AccentCallout3 = "accentCallout3", BorderCallout1
    = "borderCallout1", BorderCallout2 = "borderCallout2", BorderCallout3 =
    "borderCallout3", AccentBorderCallout1 = "accentBorderCallout1", AccentBorderCallout2
    = "accentBorderCallout2", AccentBorderCallout3 = "accentBorderCallout3",
    WedgeRectangleCallout = "wedgeRectCallout", WedgeRoundRectangleCallout =
    "wedgeRoundRectCallout", WedgeEllipseCallout = "wedgeEllipseCallout", CloudCallout =
    "cloudCallout", Cloud = "cloud", Ribbon = "ribbon", Ribbon2 = "ribbon2",
    EllipseRibbon = "ellipseRibbon", EllipseRibbon2 = "ellipseRibbon2", LeftRightRibbon =
    "leftRightRibbon", VerticalScroll = "verticalScroll", HorizontalScroll =
    "horizontalScroll", Wave = "wave", DoubleWave = "doubleWave", Plus = "plus",
    FlowChartProcess = "flowChartProcess", FlowChartDecision = "flowChartDecision",
    FlowChartInputOutput = "flowChartInputOutput", FlowChartPredefinedProcess =
    "flowChartPredefinedProcess", FlowChartInternalStorage = "flowChartInternalStorage",
    FlowChartDocument = "flowChartDocument", FlowChartMultidocument =
    "flowChartMultidocument", FlowChartTerminator = "flowChartTerminator",
    FlowChartPreparation = "flowChartPreparation", FlowChartManualInput =
    "flowChartManualInput", FlowChartManualOperation = "flowChartManualOperation",
    FlowChartConnector = "flowChartConnector", FlowChartPunchedCard =
    "flowChartPunchedCard", FlowChartPunchedTape = "flowChartPunchedTape",
    FlowChartSummingJunction = "flowChartSummingJunction", FlowChartOr = "flowChartOr",
    FlowChartCollate = "flowChartCollate", FlowChartSort = "flowChartSort",
    FlowChartExtract = "flowChartExtract", FlowChartMerge = "flowChartMerge",
    FlowChartOfflineStorage = "flowChartOfflineStorage", FlowChartOnlineStorage =
    "flowChartOnlineStorage", FlowChartMagneticTape = "flowChartMagneticTape",
    FlowChartMagneticDisk = "flowChartMagneticDisk", FlowChartMagneticDrum =
    "flowChartMagneticDrum", FlowChartDisplay = "flowChartDisplay", FlowChartDelay =
    "flowChartDelay", FlowChartAlternateProcess = "flowChartAlternateProcess",
    FlowChartOffpageConnector = "flowChartOffpageConnector", ActionButtonBlank =
    "actionButtonBlank", ActionButtonHome = "actionButtonHome", ActionButtonHelp =
    "actionButtonHelp", ActionButtonInformation = "actionButtonInformation",
    ActionButtonForwardNext = "actionButtonForwardNext", ActionButtonBackPrevious =
    "actionButtonBackPrevious", ActionButtonEnd = "actionButtonEnd",
    ActionButtonBeginning = "actionButtonBeginning", ActionButtonReturn =
    "actionButtonReturn", ActionButtonDocument = "actionButtonDocument",
    ActionButtonSound = "actionButtonSound", ActionButtonMovie = "actionButtonMovie",
    Gear6 = "gear6", Gear9 = "gear9", Funnel = "funnel", MathPlus = "mathPlus", MathMinus
    = "mathMinus", MathMultiply = "mathMultiply", MathDivide = "mathDivide", MathEqual =
    "mathEqual", MathNotEqual = "mathNotEqual", CornerTabs = "cornerTabs", SquareTabs =
    "squareTabs", PlaqueTabs = "plaqueTabs", ChartX = "chartX", ChartStar = "chartStar",
    ChartPlus = "chartPlus", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextShapeValues {
    #[default]
    TextNoShape,
    TextPlain,
    TextStop,
    TextTriangle,
    TextTriangleInverted,
    TextChevron,
    TextChevronInverted,
    TextRingInside,
    TextRingOutside,
    TextArchUp,
    TextArchDown,
    TextCircle,
    TextButton,
    TextArchUpPour,
    TextArchDownPour,
    TextCirclePour,
    TextButtonPour,
    TextCurveUp,
    TextCurveDown,
    TextCanUp,
    TextCanDown,
    TextWave1,
    TextWave2,
    TextDoubleWave1,
    TextWave4,
    TextInflate,
    TextDeflate,
    TextInflateBottom,
    TextDeflateBottom,
    TextInflateTop,
    TextDeflateTop,
    TextDeflateInflate,
    TextDeflateInflateDeflate,
    TextFadeRight,
    TextFadeLeft,
    TextFadeUp,
    TextFadeDown,
    TextSlantUp,
    TextSlantDown,
    TextCascadeUp,
    TextCascadeDown,
}
crate::__string_enum! {
    TextShapeValues { TextNoShape = "textNoShape", TextPlain = "textPlain", TextStop =
    "textStop", TextTriangle = "textTriangle", TextTriangleInverted =
    "textTriangleInverted", TextChevron = "textChevron", TextChevronInverted =
    "textChevronInverted", TextRingInside = "textRingInside", TextRingOutside =
    "textRingOutside", TextArchUp = "textArchUp", TextArchDown = "textArchDown",
    TextCircle = "textCircle", TextButton = "textButton", TextArchUpPour =
    "textArchUpPour", TextArchDownPour = "textArchDownPour", TextCirclePour =
    "textCirclePour", TextButtonPour = "textButtonPour", TextCurveUp = "textCurveUp",
    TextCurveDown = "textCurveDown", TextCanUp = "textCanUp", TextCanDown =
    "textCanDown", TextWave1 = "textWave1", TextWave2 = "textWave2", TextDoubleWave1 =
    "textDoubleWave1", TextWave4 = "textWave4", TextInflate = "textInflate", TextDeflate
    = "textDeflate", TextInflateBottom = "textInflateBottom", TextDeflateBottom =
    "textDeflateBottom", TextInflateTop = "textInflateTop", TextDeflateTop =
    "textDeflateTop", TextDeflateInflate = "textDeflateInflate",
    TextDeflateInflateDeflate = "textDeflateInflateDeflate", TextFadeRight =
    "textFadeRight", TextFadeLeft = "textFadeLeft", TextFadeUp = "textFadeUp",
    TextFadeDown = "textFadeDown", TextSlantUp = "textSlantUp", TextSlantDown =
    "textSlantDown", TextCascadeUp = "textCascadeUp", TextCascadeDown =
    "textCascadeDown", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PathFillModeValues {
    #[default]
    None,
    Norm,
    Lighten,
    LightenLess,
    Darken,
    DarkenLess,
}
crate::__string_enum! {
    PathFillModeValues { None = "none", Norm = "norm", Lighten = "lighten", LightenLess =
    "lightenLess", Darken = "darken", DarkenLess = "darkenLess", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LineEndValues {
    #[default]
    None,
    Triangle,
    Stealth,
    Diamond,
    Oval,
    Arrow,
}
crate::__string_enum! {
    LineEndValues { None = "none", Triangle = "triangle", Stealth = "stealth", Diamond =
    "diamond", Oval = "oval", Arrow = "arrow", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LineEndWidthValues {
    #[default]
    Small,
    Medium,
    Large,
}
crate::__string_enum! {
    LineEndWidthValues { Small = "sm", Medium = "med", Large = "lg", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LineEndLengthValues {
    #[default]
    Small,
    Medium,
    Large,
}
crate::__string_enum! {
    LineEndLengthValues { Small = "sm", Medium = "med", Large = "lg", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetLineDashValues {
    #[default]
    Solid,
    Dot,
    Dash,
    LargeDash,
    DashDot,
    LargeDashDot,
    LargeDashDotDot,
    SystemDash,
    SystemDot,
    SystemDashDot,
    SystemDashDotDot,
}
crate::__string_enum! {
    PresetLineDashValues { Solid = "solid", Dot = "dot", Dash = "dash", LargeDash =
    "lgDash", DashDot = "dashDot", LargeDashDot = "lgDashDot", LargeDashDotDot =
    "lgDashDotDot", SystemDash = "sysDash", SystemDot = "sysDot", SystemDashDot =
    "sysDashDot", SystemDashDotDot = "sysDashDotDot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LineCapValues {
    #[default]
    Round,
    Square,
    Flat,
}
crate::__string_enum! {
    LineCapValues { Round = "rnd", Square = "sq", Flat = "flat", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PenAlignmentValues {
    #[default]
    Center,
    Insert,
}
crate::__string_enum! {
    PenAlignmentValues { Center = "ctr", Insert = "in", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CompoundLineValues {
    #[default]
    Single,
    Double,
    ThickThin,
    ThinThick,
    Triple,
}
crate::__string_enum! {
    CompoundLineValues { Single = "sng", Double = "dbl", ThickThin = "thickThin",
    ThinThick = "thinThick", Triple = "tri", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BooleanStyleValues {
    #[default]
    On,
    Off,
    Default,
}
crate::__string_enum! {
    BooleanStyleValues { On = "on", Off = "off", Default = "def", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextVerticalOverflowValues {
    #[default]
    Overflow,
    Ellipsis,
    Clip,
}
crate::__string_enum! {
    TextVerticalOverflowValues { Overflow = "overflow", Ellipsis = "ellipsis", Clip =
    "clip", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextHorizontalOverflowValues {
    #[default]
    Overflow,
    Clip,
}
crate::__string_enum! {
    TextHorizontalOverflowValues { Overflow = "overflow", Clip = "clip", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextVerticalValues {
    #[default]
    Horizontal,
    Vertical,
    Vertical270,
    WordArtVertical,
    EastAsianVetical,
    MongolianVertical,
    WordArtLeftToRight,
}
crate::__string_enum! {
    TextVerticalValues { Horizontal = "horz", Vertical = "vert", Vertical270 = "vert270",
    WordArtVertical = "wordArtVert", EastAsianVetical = "eaVert", MongolianVertical =
    "mongolianVert", WordArtLeftToRight = "wordArtVertRtl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextWrappingValues {
    #[default]
    None,
    Square,
}
crate::__string_enum! {
    TextWrappingValues { None = "none", Square = "square", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextAnchoringTypeValues {
    #[default]
    Top,
    Center,
    Bottom,
}
crate::__string_enum! {
    TextAnchoringTypeValues { Top = "t", Center = "ctr", Bottom = "b", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextAutoNumberSchemeValues {
    #[default]
    AlphaLowerCharacterParenBoth,
    AlphaUpperCharacterParenBoth,
    AlphaLowerCharacterParenR,
    AlphaUpperCharacterParenR,
    AlphaLowerCharacterPeriod,
    AlphaUpperCharacterPeriod,
    ArabicParenBoth,
    ArabicParenR,
    ArabicPeriod,
    ArabicPlain,
    RomanLowerCharacterParenBoth,
    RomanUpperCharacterParenBoth,
    RomanLowerCharacterParenR,
    RomanUpperCharacterParenR,
    RomanLowerCharacterPeriod,
    RomanUpperCharacterPeriod,
    CircleNumberDoubleBytePlain,
    CircleNumberWingdingsBlackPlain,
    CircleNumberWingdingsWhitePlain,
    ArabicDoubleBytePeriod,
    ArabicDoubleBytePlain,
    EastAsianSimplifiedChinesePeriod,
    EastAsianSimplifiedChinesePlain,
    EastAsianTraditionalChinesePeriod,
    EastAsianTraditionalChinesePlain,
    EastAsianJapaneseDoubleBytePeriod,
    EastAsianJapaneseKoreanPlain,
    EastAsianJapaneseKoreanPeriod,
    Arabic1Minus,
    Arabic2Minus,
    Hebrew2Minus,
    ThaiAlphaPeriod,
    ThaiAlphaParenthesisRight,
    ThaiAlphaParenthesisBoth,
    ThaiNumberPeriod,
    ThaiNumberParenthesisRight,
    ThaiNumberParenthesisBoth,
    HindiAlphaPeriod,
    HindiNumPeriod,
    HindiNumberParenthesisRight,
    HindiAlpha1Period,
}
crate::__string_enum! {
    TextAutoNumberSchemeValues { AlphaLowerCharacterParenBoth = "alphaLcParenBoth",
    AlphaUpperCharacterParenBoth = "alphaUcParenBoth", AlphaLowerCharacterParenR =
    "alphaLcParenR", AlphaUpperCharacterParenR = "alphaUcParenR",
    AlphaLowerCharacterPeriod = "alphaLcPeriod", AlphaUpperCharacterPeriod =
    "alphaUcPeriod", ArabicParenBoth = "arabicParenBoth", ArabicParenR = "arabicParenR",
    ArabicPeriod = "arabicPeriod", ArabicPlain = "arabicPlain",
    RomanLowerCharacterParenBoth = "romanLcParenBoth", RomanUpperCharacterParenBoth =
    "romanUcParenBoth", RomanLowerCharacterParenR = "romanLcParenR",
    RomanUpperCharacterParenR = "romanUcParenR", RomanLowerCharacterPeriod =
    "romanLcPeriod", RomanUpperCharacterPeriod = "romanUcPeriod",
    CircleNumberDoubleBytePlain = "circleNumDbPlain", CircleNumberWingdingsBlackPlain =
    "circleNumWdBlackPlain", CircleNumberWingdingsWhitePlain = "circleNumWdWhitePlain",
    ArabicDoubleBytePeriod = "arabicDbPeriod", ArabicDoubleBytePlain = "arabicDbPlain",
    EastAsianSimplifiedChinesePeriod = "ea1ChsPeriod", EastAsianSimplifiedChinesePlain =
    "ea1ChsPlain", EastAsianTraditionalChinesePeriod = "ea1ChtPeriod",
    EastAsianTraditionalChinesePlain = "ea1ChtPlain", EastAsianJapaneseDoubleBytePeriod =
    "ea1JpnChsDbPeriod", EastAsianJapaneseKoreanPlain = "ea1JpnKorPlain",
    EastAsianJapaneseKoreanPeriod = "ea1JpnKorPeriod", Arabic1Minus = "arabic1Minus",
    Arabic2Minus = "arabic2Minus", Hebrew2Minus = "hebrew2Minus", ThaiAlphaPeriod =
    "thaiAlphaPeriod", ThaiAlphaParenthesisRight = "thaiAlphaParenR",
    ThaiAlphaParenthesisBoth = "thaiAlphaParenBoth", ThaiNumberPeriod = "thaiNumPeriod",
    ThaiNumberParenthesisRight = "thaiNumParenR", ThaiNumberParenthesisBoth =
    "thaiNumParenBoth", HindiAlphaPeriod = "hindiAlphaPeriod", HindiNumPeriod =
    "hindiNumPeriod", HindiNumberParenthesisRight = "hindiNumParenR", HindiAlpha1Period =
    "hindiAlpha1Period", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextUnderlineValues {
    #[default]
    None,
    Words,
    Single,
    Double,
    Heavy,
    Dotted,
    HeavyDotted,
    Dash,
    DashHeavy,
    DashLong,
    DashLongHeavy,
    DotDash,
    DotDashHeavy,
    DotDotDash,
    DotDotDashHeavy,
    Wavy,
    WavyHeavy,
    WavyDouble,
}
crate::__string_enum! {
    TextUnderlineValues { None = "none", Words = "words", Single = "sng", Double = "dbl",
    Heavy = "heavy", Dotted = "dotted", HeavyDotted = "dottedHeavy", Dash = "dash",
    DashHeavy = "dashHeavy", DashLong = "dashLong", DashLongHeavy = "dashLongHeavy",
    DotDash = "dotDash", DotDashHeavy = "dotDashHeavy", DotDotDash = "dotDotDash",
    DotDotDashHeavy = "dotDotDashHeavy", Wavy = "wavy", WavyHeavy = "wavyHeavy",
    WavyDouble = "wavyDbl", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextStrikeValues {
    #[default]
    NoStrike,
    SingleStrike,
    DoubleStrike,
}
crate::__string_enum! {
    TextStrikeValues { NoStrike = "noStrike", SingleStrike = "sngStrike", DoubleStrike =
    "dblStrike", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextCapsValues {
    #[default]
    None,
    Small,
    All,
}
crate::__string_enum! {
    TextCapsValues { None = "none", Small = "small", All = "all", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextTabAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
    Decimal,
}
crate::__string_enum! {
    TextTabAlignmentValues { Left = "l", Center = "ctr", Right = "r", Decimal = "dec", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextAlignmentTypeValues {
    #[default]
    Left,
    Center,
    Right,
    Justified,
    JustifiedLow,
    Distributed,
    ThaiDistributed,
}
crate::__string_enum! {
    TextAlignmentTypeValues { Left = "l", Center = "ctr", Right = "r", Justified =
    "just", JustifiedLow = "justLow", Distributed = "dist", ThaiDistributed = "thaiDist",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextFontAlignmentValues {
    #[default]
    Automatic,
    Top,
    Center,
    Baseline,
    Bottom,
}
crate::__string_enum! {
    TextFontAlignmentValues { Automatic = "auto", Top = "t", Center = "ctr", Baseline =
    "base", Bottom = "b", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetColorValues {
    #[default]
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGray,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DodgerBlue,
    Firebrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    Goldenrod,
    Gray,
    Green,
    GreenYellow,
    Honeydew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenrodYellow,
    LightGray,
    LightGreen,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MedAquamarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenrod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
    DarkBlue2010,
    DarkCyan2010,
    DarkGoldenrod2010,
    DarkGray2010,
    DarkGrey2010,
    DarkGreen2010,
    DarkKhaki2010,
    DarkMagenta2010,
    DarkOliveGreen2010,
    DarkOrange2010,
    DarkOrchid2010,
    DarkRed2010,
    DarkSalmon2010,
    DarkSeaGreen2010,
    DarkSlateBlue2010,
    DarkSlateGray2010,
    DarkSlateGrey2010,
    DarkTurquoise2010,
    DarkViolet2010,
    LightBlue2010,
    LightCoral2010,
    LightCyan2010,
    LightGoldenrodYellow2010,
    LightGray2010,
    LightGrey2010,
    LightGreen2010,
    LightPink2010,
    LightSalmon2010,
    LightSeaGreen2010,
    LightSkyBlue2010,
    LightSlateGray2010,
    LightSlateGrey2010,
    LightSteelBlue2010,
    LightYellow2010,
    MediumAquamarine2010,
    MediumBlue2010,
    MediumOrchid2010,
    MediumPurple2010,
    MediumSeaGreen2010,
    MediumSlateBlue2010,
    MediumSpringGreen2010,
    MediumTurquoise2010,
    MediumVioletRed2010,
    DarkGrey,
    DimGrey,
    DarkSlateGrey,
    Grey,
    LightGrey,
    LightSlateGrey,
    SlateGrey,
}
crate::__string_enum! {
    PresetColorValues { AliceBlue = "aliceBlue", AntiqueWhite = "antiqueWhite", Aqua =
    "aqua", Aquamarine = "aquamarine", Azure = "azure", Beige = "beige", Bisque =
    "bisque", Black = "black", BlanchedAlmond = "blanchedAlmond", Blue = "blue",
    BlueViolet = "blueViolet", Brown = "brown", BurlyWood = "burlyWood", CadetBlue =
    "cadetBlue", Chartreuse = "chartreuse", Chocolate = "chocolate", Coral = "coral",
    CornflowerBlue = "cornflowerBlue", Cornsilk = "cornsilk", Crimson = "crimson", Cyan =
    "cyan", DarkBlue = "dkBlue", DarkCyan = "dkCyan", DarkGoldenrod = "dkGoldenrod",
    DarkGray = "dkGray", DarkGreen = "dkGreen", DarkKhaki = "dkKhaki", DarkMagenta =
    "dkMagenta", DarkOliveGreen = "dkOliveGreen", DarkOrange = "dkOrange", DarkOrchid =
    "dkOrchid", DarkRed = "dkRed", DarkSalmon = "dkSalmon", DarkSeaGreen = "dkSeaGreen",
    DarkSlateBlue = "dkSlateBlue", DarkSlateGray = "dkSlateGray", DarkTurquoise =
    "dkTurquoise", DarkViolet = "dkViolet", DeepPink = "deepPink", DeepSkyBlue =
    "deepSkyBlue", DimGray = "dimGray", DodgerBlue = "dodgerBlue", Firebrick =
    "firebrick", FloralWhite = "floralWhite", ForestGreen = "forestGreen", Fuchsia =
    "fuchsia", Gainsboro = "gainsboro", GhostWhite = "ghostWhite", Gold = "gold",
    Goldenrod = "goldenrod", Gray = "gray", Green = "green", GreenYellow = "greenYellow",
    Honeydew = "honeydew", HotPink = "hotPink", IndianRed = "indianRed", Indigo =
    "indigo", Ivory = "ivory", Khaki = "khaki", Lavender = "lavender", LavenderBlush =
    "lavenderBlush", LawnGreen = "lawnGreen", LemonChiffon = "lemonChiffon", LightBlue =
    "ltBlue", LightCoral = "ltCoral", LightCyan = "ltCyan", LightGoldenrodYellow =
    "ltGoldenrodYellow", LightGray = "ltGray", LightGreen = "ltGreen", LightPink =
    "ltPink", LightSalmon = "ltSalmon", LightSeaGreen = "ltSeaGreen", LightSkyBlue =
    "ltSkyBlue", LightSlateGray = "ltSlateGray", LightSteelBlue = "ltSteelBlue",
    LightYellow = "ltYellow", Lime = "lime", LimeGreen = "limeGreen", Linen = "linen",
    Magenta = "magenta", Maroon = "maroon", MedAquamarine = "medAquamarine", MediumBlue =
    "medBlue", MediumOrchid = "medOrchid", MediumPurple = "medPurple", MediumSeaGreen =
    "medSeaGreen", MediumSlateBlue = "medSlateBlue", MediumSpringGreen =
    "medSpringGreen", MediumTurquoise = "medTurquoise", MediumVioletRed = "medVioletRed",
    MidnightBlue = "midnightBlue", MintCream = "mintCream", MistyRose = "mistyRose",
    Moccasin = "moccasin", NavajoWhite = "navajoWhite", Navy = "navy", OldLace =
    "oldLace", Olive = "olive", OliveDrab = "oliveDrab", Orange = "orange", OrangeRed =
    "orangeRed", Orchid = "orchid", PaleGoldenrod = "paleGoldenrod", PaleGreen =
    "paleGreen", PaleTurquoise = "paleTurquoise", PaleVioletRed = "paleVioletRed",
    PapayaWhip = "papayaWhip", PeachPuff = "peachPuff", Peru = "peru", Pink = "pink",
    Plum = "plum", PowderBlue = "powderBlue", Purple = "purple", Red = "red", RosyBrown =
    "rosyBrown", RoyalBlue = "royalBlue", SaddleBrown = "saddleBrown", Salmon = "salmon",
    SandyBrown = "sandyBrown", SeaGreen = "seaGreen", SeaShell = "seaShell", Sienna =
    "sienna", Silver = "silver", SkyBlue = "skyBlue", SlateBlue = "slateBlue", SlateGray
    = "slateGray", Snow = "snow", SpringGreen = "springGreen", SteelBlue = "steelBlue",
    Tan = "tan", Teal = "teal", Thistle = "thistle", Tomato = "tomato", Turquoise =
    "turquoise", Violet = "violet", Wheat = "wheat", White = "white", WhiteSmoke =
    "whiteSmoke", Yellow = "yellow", YellowGreen = "yellowGreen", DarkBlue2010 =
    "darkBlue", DarkCyan2010 = "darkCyan", DarkGoldenrod2010 = "darkGoldenrod",
    DarkGray2010 = "darkGray", DarkGrey2010 = "darkGrey", DarkGreen2010 = "darkGreen",
    DarkKhaki2010 = "darkKhaki", DarkMagenta2010 = "darkMagenta", DarkOliveGreen2010 =
    "darkOliveGreen", DarkOrange2010 = "darkOrange", DarkOrchid2010 = "darkOrchid",
    DarkRed2010 = "darkRed", DarkSalmon2010 = "darkSalmon", DarkSeaGreen2010 =
    "darkSeaGreen", DarkSlateBlue2010 = "darkSlateBlue", DarkSlateGray2010 =
    "darkSlateGray", DarkSlateGrey2010 = "darkSlateGrey", DarkTurquoise2010 =
    "darkTurquoise", DarkViolet2010 = "darkViolet", LightBlue2010 = "lightBlue",
    LightCoral2010 = "lightCoral", LightCyan2010 = "lightCyan", LightGoldenrodYellow2010
    = "lightGoldenrodYellow", LightGray2010 = "lightGray", LightGrey2010 = "lightGrey",
    LightGreen2010 = "lightGreen", LightPink2010 = "lightPink", LightSalmon2010 =
    "lightSalmon", LightSeaGreen2010 = "lightSeaGreen", LightSkyBlue2010 =
    "lightSkyBlue", LightSlateGray2010 = "lightSlateGray", LightSlateGrey2010 =
    "lightSlateGrey", LightSteelBlue2010 = "lightSteelBlue", LightYellow2010 =
    "lightYellow", MediumAquamarine2010 = "mediumAquamarine", MediumBlue2010 =
    "mediumBlue", MediumOrchid2010 = "mediumOrchid", MediumPurple2010 = "mediumPurple",
    MediumSeaGreen2010 = "mediumSeaGreen", MediumSlateBlue2010 = "mediumSlateBlue",
    MediumSpringGreen2010 = "mediumSpringGreen", MediumTurquoise2010 = "mediumTurquoise",
    MediumVioletRed2010 = "mediumVioletRed", DarkGrey = "dkGrey", DimGrey = "dimGrey",
    DarkSlateGrey = "dkSlateGrey", Grey = "grey", LightGrey = "ltGrey", LightSlateGrey =
    "ltSlateGrey", SlateGrey = "slateGrey", }
}
/// Audio from CD.
/// When the object is serialized out as xml, it's qualified name is a:audioCd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:audioCd")]
pub struct AudioFromCd {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Audio Start Time
    #[xml(child = "a:st")]
    pub start_time: StartTime,
    ///Audio End Time
    #[xml(child = "a:end")]
    pub end_time: EndTime,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Audio from WAV File.
/// When the object is serialized out as xml, it's qualified name is a:wavAudioFile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:wavAudioFile")]
pub struct WaveAudioFile {
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
/// Sound to play..
/// When the object is serialized out as xml, it's qualified name is a:snd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:snd")]
pub struct HyperlinkSound {
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
/// Audio from File.
/// When the object is serialized out as xml, it's qualified name is a:audioFile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:audioFile")]
pub struct AudioFromFile {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Linked Relationship ID
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: String,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Video from File.
/// When the object is serialized out as xml, it's qualified name is a:videoFile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:videoFile")]
pub struct VideoFromFile {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Linked Relationship ID
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: String,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// QuickTime from File.
/// When the object is serialized out as xml, it's qualified name is a:quickTimeFile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:quickTimeFile")]
pub struct QuickTimeFromFile {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Linked Relationship ID
    /// Represents the following attribute in the schema: r:link
    #[xml(attr = "r:link")]
    pub link: String,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Tint.
/// When the object is serialized out as xml, it's qualified name is a:tint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tint")]
pub struct Tint {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Shade.
/// When the object is serialized out as xml, it's qualified name is a:shade.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:shade")]
pub struct Shade {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Alpha.
/// When the object is serialized out as xml, it's qualified name is a:alpha.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alpha")]
pub struct Alpha {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the PositiveFixedPercentageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PositiveFixedPercentageType {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Complement.
/// When the object is serialized out as xml, it's qualified name is a:comp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:comp")]
pub struct Complement {}
/// Inverse.
/// When the object is serialized out as xml, it's qualified name is a:inv.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:inv")]
pub struct Inverse {}
/// Gray.
/// When the object is serialized out as xml, it's qualified name is a:gray.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gray")]
pub struct Gray {}
/// Alpha Offset.
/// When the object is serialized out as xml, it's qualified name is a:alphaOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaOff")]
pub struct AlphaOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Alpha Modulation.
/// When the object is serialized out as xml, it's qualified name is a:alphaMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaMod")]
pub struct AlphaModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Hue Modulate.
/// When the object is serialized out as xml, it's qualified name is a:hueMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hueMod")]
pub struct HueModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the PositivePercentageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PositivePercentageType {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Hue.
/// When the object is serialized out as xml, it's qualified name is a:hue.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hue")]
pub struct Hue {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Hue Offset.
/// When the object is serialized out as xml, it's qualified name is a:hueOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hueOff")]
pub struct HueOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Saturation.
/// When the object is serialized out as xml, it's qualified name is a:sat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sat")]
pub struct Saturation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Saturation Offset.
/// When the object is serialized out as xml, it's qualified name is a:satOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:satOff")]
pub struct SaturationOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Saturation Modulation.
/// When the object is serialized out as xml, it's qualified name is a:satMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:satMod")]
pub struct SaturationModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Luminance.
/// When the object is serialized out as xml, it's qualified name is a:lum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lum")]
pub struct Luminance {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Luminance Offset.
/// When the object is serialized out as xml, it's qualified name is a:lumOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lumOff")]
pub struct LuminanceOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Luminance Modulation.
/// When the object is serialized out as xml, it's qualified name is a:lumMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lumMod")]
pub struct LuminanceModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Red.
/// When the object is serialized out as xml, it's qualified name is a:red.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:red")]
pub struct Red {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Red Offset.
/// When the object is serialized out as xml, it's qualified name is a:redOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:redOff")]
pub struct RedOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Red Modulation.
/// When the object is serialized out as xml, it's qualified name is a:redMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:redMod")]
pub struct RedModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Green.
/// When the object is serialized out as xml, it's qualified name is a:green.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:green")]
pub struct Green {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Green Offset.
/// When the object is serialized out as xml, it's qualified name is a:greenOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:greenOff")]
pub struct GreenOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Green Modification.
/// When the object is serialized out as xml, it's qualified name is a:greenMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:greenMod")]
pub struct GreenModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Blue.
/// When the object is serialized out as xml, it's qualified name is a:blue.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blue")]
pub struct Blue {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Blue Offset.
/// When the object is serialized out as xml, it's qualified name is a:blueOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blueOff")]
pub struct BlueOffset {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Blue Modification.
/// When the object is serialized out as xml, it's qualified name is a:blueMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blueMod")]
pub struct BlueModulation {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Defines the PercentageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PercentageType {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Gamma.
/// When the object is serialized out as xml, it's qualified name is a:gamma.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gamma")]
pub struct Gamma {}
/// Inverse Gamma.
/// When the object is serialized out as xml, it's qualified name is a:invGamma.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:invGamma")]
pub struct InverseGamma {}
/// Extension.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct Extension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: Option<String>,
}
/// RGB Color Model - Percentage Variant.
/// When the object is serialized out as xml, it's qualified name is a:scrgbClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:scrgbClr")]
pub struct RgbColorModelPercentage {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Red
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub red_portion: i32,
    /// Green
    /// Represents the following attribute in the schema: :g
    #[xml(attr = "g")]
    pub green_portion: i32,
    /// Blue
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub blue_portion: i32,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<RgbColorModelPercentageChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RgbColorModelPercentageChildChoice {
    #[xml(tag = "a:tint")]
    ATint(Tint),
    #[xml(tag = "a:shade")]
    AShade(Shade),
    #[xml(tag = "a:comp")]
    AComp(Complement),
    #[xml(tag = "a:inv")]
    AInv(Inverse),
    #[xml(tag = "a:gray")]
    AGray(Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(AlphaOffset),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulation),
    #[xml(tag = "a:hue")]
    AHue(Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(HueModulation),
    #[xml(tag = "a:sat")]
    ASat(Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(SaturationOffset),
    #[xml(tag = "a:satMod")]
    ASatMod(SaturationModulation),
    #[xml(tag = "a:lum")]
    ALum(Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(LuminanceOffset),
    #[xml(tag = "a:lumMod")]
    ALumMod(LuminanceModulation),
    #[xml(tag = "a:red")]
    ARed(Red),
    #[xml(tag = "a:redOff")]
    ARedOff(RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(RedModulation),
    #[xml(tag = "a:green")]
    AGreen(Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(GreenOffset),
    #[xml(tag = "a:greenMod")]
    AGreenMod(GreenModulation),
    #[xml(tag = "a:blue")]
    ABlue(Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(BlueModulation),
    #[xml(tag = "a:gamma")]
    AGamma(Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(InverseGamma),
}
/// RGB Color Model - Hex Variant.
/// When the object is serialized out as xml, it's qualified name is a:srgbClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:srgbClr")]
pub struct RgbColorModelHex {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
    /// legacySpreadsheetColorIndex
    /// Represents the following attribute in the schema: a14:legacySpreadsheetColorIndex
    #[xml(attr = "a14:legacySpreadsheetColorIndex")]
    pub legacy_spreadsheet_color_index: Option<i32>,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<RgbColorModelHexChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RgbColorModelHexChildChoice {
    #[xml(tag = "a:tint")]
    ATint(Tint),
    #[xml(tag = "a:shade")]
    AShade(Shade),
    #[xml(tag = "a:comp")]
    AComp(Complement),
    #[xml(tag = "a:inv")]
    AInv(Inverse),
    #[xml(tag = "a:gray")]
    AGray(Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(AlphaOffset),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulation),
    #[xml(tag = "a:hue")]
    AHue(Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(HueModulation),
    #[xml(tag = "a:sat")]
    ASat(Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(SaturationOffset),
    #[xml(tag = "a:satMod")]
    ASatMod(SaturationModulation),
    #[xml(tag = "a:lum")]
    ALum(Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(LuminanceOffset),
    #[xml(tag = "a:lumMod")]
    ALumMod(LuminanceModulation),
    #[xml(tag = "a:red")]
    ARed(Red),
    #[xml(tag = "a:redOff")]
    ARedOff(RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(RedModulation),
    #[xml(tag = "a:green")]
    AGreen(Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(GreenOffset),
    #[xml(tag = "a:greenMod")]
    AGreenMod(GreenModulation),
    #[xml(tag = "a:blue")]
    ABlue(Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(BlueModulation),
    #[xml(tag = "a:gamma")]
    AGamma(Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(InverseGamma),
}
/// Hue, Saturation, Luminance Color Model.
/// When the object is serialized out as xml, it's qualified name is a:hslClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hslClr")]
pub struct HslColor {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Hue
    /// Represents the following attribute in the schema: :hue
    #[xml(attr = "hue")]
    pub hue_value: i32,
    /// Saturation
    /// Represents the following attribute in the schema: :sat
    #[xml(attr = "sat")]
    pub sat_value: i32,
    /// Luminance
    /// Represents the following attribute in the schema: :lum
    #[xml(attr = "lum")]
    pub lum_value: i32,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<HslColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HslColorChildChoice {
    #[xml(tag = "a:tint")]
    ATint(Tint),
    #[xml(tag = "a:shade")]
    AShade(Shade),
    #[xml(tag = "a:comp")]
    AComp(Complement),
    #[xml(tag = "a:inv")]
    AInv(Inverse),
    #[xml(tag = "a:gray")]
    AGray(Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(AlphaOffset),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulation),
    #[xml(tag = "a:hue")]
    AHue(Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(HueModulation),
    #[xml(tag = "a:sat")]
    ASat(Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(SaturationOffset),
    #[xml(tag = "a:satMod")]
    ASatMod(SaturationModulation),
    #[xml(tag = "a:lum")]
    ALum(Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(LuminanceOffset),
    #[xml(tag = "a:lumMod")]
    ALumMod(LuminanceModulation),
    #[xml(tag = "a:red")]
    ARed(Red),
    #[xml(tag = "a:redOff")]
    ARedOff(RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(RedModulation),
    #[xml(tag = "a:green")]
    AGreen(Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(GreenOffset),
    #[xml(tag = "a:greenMod")]
    AGreenMod(GreenModulation),
    #[xml(tag = "a:blue")]
    ABlue(Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(BlueModulation),
    #[xml(tag = "a:gamma")]
    AGamma(Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(InverseGamma),
}
/// System Color.
/// When the object is serialized out as xml, it's qualified name is a:sysClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sysClr")]
pub struct SystemColor {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: SystemColorValues,
    /// Last Color
    /// Represents the following attribute in the schema: :lastClr
    #[xml(attr = "lastClr")]
    pub last_color: Option<String>,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<SystemColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SystemColorChildChoice {
    #[xml(tag = "a:tint")]
    ATint(Tint),
    #[xml(tag = "a:shade")]
    AShade(Shade),
    #[xml(tag = "a:comp")]
    AComp(Complement),
    #[xml(tag = "a:inv")]
    AInv(Inverse),
    #[xml(tag = "a:gray")]
    AGray(Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(AlphaOffset),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulation),
    #[xml(tag = "a:hue")]
    AHue(Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(HueModulation),
    #[xml(tag = "a:sat")]
    ASat(Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(SaturationOffset),
    #[xml(tag = "a:satMod")]
    ASatMod(SaturationModulation),
    #[xml(tag = "a:lum")]
    ALum(Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(LuminanceOffset),
    #[xml(tag = "a:lumMod")]
    ALumMod(LuminanceModulation),
    #[xml(tag = "a:red")]
    ARed(Red),
    #[xml(tag = "a:redOff")]
    ARedOff(RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(RedModulation),
    #[xml(tag = "a:green")]
    AGreen(Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(GreenOffset),
    #[xml(tag = "a:greenMod")]
    AGreenMod(GreenModulation),
    #[xml(tag = "a:blue")]
    ABlue(Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(BlueModulation),
    #[xml(tag = "a:gamma")]
    AGamma(Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(InverseGamma),
}
/// Scheme Color.
/// When the object is serialized out as xml, it's qualified name is a:schemeClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:schemeClr")]
pub struct SchemeColor {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: SchemeColorValues,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<SchemeColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SchemeColorChildChoice {
    #[xml(tag = "a:tint")]
    ATint(Tint),
    #[xml(tag = "a:shade")]
    AShade(Shade),
    #[xml(tag = "a:comp")]
    AComp(Complement),
    #[xml(tag = "a:inv")]
    AInv(Inverse),
    #[xml(tag = "a:gray")]
    AGray(Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(AlphaOffset),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulation),
    #[xml(tag = "a:hue")]
    AHue(Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(HueModulation),
    #[xml(tag = "a:sat")]
    ASat(Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(SaturationOffset),
    #[xml(tag = "a:satMod")]
    ASatMod(SaturationModulation),
    #[xml(tag = "a:lum")]
    ALum(Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(LuminanceOffset),
    #[xml(tag = "a:lumMod")]
    ALumMod(LuminanceModulation),
    #[xml(tag = "a:red")]
    ARed(Red),
    #[xml(tag = "a:redOff")]
    ARedOff(RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(RedModulation),
    #[xml(tag = "a:green")]
    AGreen(Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(GreenOffset),
    #[xml(tag = "a:greenMod")]
    AGreenMod(GreenModulation),
    #[xml(tag = "a:blue")]
    ABlue(Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(BlueModulation),
    #[xml(tag = "a:gamma")]
    AGamma(Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(InverseGamma),
}
/// Preset Color.
/// When the object is serialized out as xml, it's qualified name is a:prstClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:prstClr")]
pub struct PresetColor {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: PresetColorValues,
    #[xml(
        child = "a:tint",
        child = "a:shade",
        child = "a:comp",
        child = "a:inv",
        child = "a:gray",
        child = "a:alpha",
        child = "a:alphaOff",
        child = "a:alphaMod",
        child = "a:hue",
        child = "a:hueOff",
        child = "a:hueMod",
        child = "a:sat",
        child = "a:satOff",
        child = "a:satMod",
        child = "a:lum",
        child = "a:lumOff",
        child = "a:lumMod",
        child = "a:red",
        child = "a:redOff",
        child = "a:redMod",
        child = "a:green",
        child = "a:greenOff",
        child = "a:greenMod",
        child = "a:blue",
        child = "a:blueOff",
        child = "a:blueMod",
        child = "a:gamma",
        child = "a:invGamma",
    )]
    pub children: Vec<PresetColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PresetColorChildChoice {
    #[xml(tag = "a:tint")]
    ATint(Tint),
    #[xml(tag = "a:shade")]
    AShade(Shade),
    #[xml(tag = "a:comp")]
    AComp(Complement),
    #[xml(tag = "a:inv")]
    AInv(Inverse),
    #[xml(tag = "a:gray")]
    AGray(Gray),
    #[xml(tag = "a:alpha")]
    AAlpha(Alpha),
    #[xml(tag = "a:alphaOff")]
    AAlphaOff(AlphaOffset),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulation),
    #[xml(tag = "a:hue")]
    AHue(Hue),
    #[xml(tag = "a:hueOff")]
    AHueOff(HueOffset),
    #[xml(tag = "a:hueMod")]
    AHueMod(HueModulation),
    #[xml(tag = "a:sat")]
    ASat(Saturation),
    #[xml(tag = "a:satOff")]
    ASatOff(SaturationOffset),
    #[xml(tag = "a:satMod")]
    ASatMod(SaturationModulation),
    #[xml(tag = "a:lum")]
    ALum(Luminance),
    #[xml(tag = "a:lumOff")]
    ALumOff(LuminanceOffset),
    #[xml(tag = "a:lumMod")]
    ALumMod(LuminanceModulation),
    #[xml(tag = "a:red")]
    ARed(Red),
    #[xml(tag = "a:redOff")]
    ARedOff(RedOffset),
    #[xml(tag = "a:redMod")]
    ARedMod(RedModulation),
    #[xml(tag = "a:green")]
    AGreen(Green),
    #[xml(tag = "a:greenOff")]
    AGreenOff(GreenOffset),
    #[xml(tag = "a:greenMod")]
    AGreenMod(GreenModulation),
    #[xml(tag = "a:blue")]
    ABlue(Blue),
    #[xml(tag = "a:blueOff")]
    ABlueOff(BlueOffset),
    #[xml(tag = "a:blueMod")]
    ABlueMod(BlueModulation),
    #[xml(tag = "a:gamma")]
    AGamma(Gamma),
    #[xml(tag = "a:invGamma")]
    AInvGamma(InverseGamma),
}
/// Apply 3D shape properties.
/// When the object is serialized out as xml, it's qualified name is a:sp3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sp3d")]
pub struct Shape3DType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Shape Depth
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: Option<i64>,
    /// Extrusion Height
    /// Represents the following attribute in the schema: :extrusionH
    #[xml(attr = "extrusionH")]
    pub extrusion_height: Option<i64>,
    /// Contour Width
    /// Represents the following attribute in the schema: :contourW
    #[xml(attr = "contourW")]
    pub contour_width: Option<i64>,
    /// Preset Material Type
    /// Represents the following attribute in the schema: :prstMaterial
    #[xml(attr = "prstMaterial")]
    pub preset_material: Option<PresetMaterialTypeValues>,
    ///Top Bevel
    #[xml(child = "a:bevelT")]
    pub bevel_top: Option<BevelTop>,
    ///Bottom Bevel
    #[xml(child = "a:bevelB")]
    pub bevel_bottom: Option<BevelBottom>,
    ///Extrusion Color
    #[xml(child = "a:extrusionClr")]
    pub extrusion_color: Option<ExtrusionColor>,
    ///Contour Color
    #[xml(child = "a:contourClr")]
    pub contour_color: Option<ContourColor>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// No text in 3D scene.
/// When the object is serialized out as xml, it's qualified name is a:flatTx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:flatTx")]
pub struct FlatText {
    /// Z Coordinate
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: Option<i64>,
}
/// Linear Gradient Fill.
/// When the object is serialized out as xml, it's qualified name is a:lin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lin")]
pub struct LinearGradientFill {
    /// Angle
    /// Represents the following attribute in the schema: :ang
    #[xml(attr = "ang")]
    pub angle: Option<i32>,
    /// Scaled
    /// Represents the following attribute in the schema: :scaled
    #[xml(attr = "scaled")]
    pub scaled: Option<bool>,
}
/// Path Gradient.
/// When the object is serialized out as xml, it's qualified name is a:path.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:path")]
pub struct PathGradientFill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Gradient Fill Path
    /// Represents the following attribute in the schema: :path
    #[xml(attr = "path")]
    pub path: Option<PathShadeValues>,
    ///Fill To Rectangle
    #[xml(child = "a:fillToRect")]
    pub fill_to_rectangle: Option<FillToRectangle>,
}
/// Tile.
/// When the object is serialized out as xml, it's qualified name is a:tile.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tile")]
pub struct Tile {
    /// Horizontal Offset
    /// Represents the following attribute in the schema: :tx
    #[xml(attr = "tx")]
    pub horizontal_offset: Option<i64>,
    /// Vertical Offset
    /// Represents the following attribute in the schema: :ty
    #[xml(attr = "ty")]
    pub vertical_offset: Option<i64>,
    /// Horizontal Ratio
    /// Represents the following attribute in the schema: :sx
    #[xml(attr = "sx")]
    pub horizontal_ratio: Option<i32>,
    /// Vertical Ratio
    /// Represents the following attribute in the schema: :sy
    #[xml(attr = "sy")]
    pub vertical_ratio: Option<i32>,
    /// Tile Flipping
    /// Represents the following attribute in the schema: :flip
    #[xml(attr = "flip")]
    pub flip: Option<TileFlipValues>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<RectangleAlignmentValues>,
}
/// Stretch.
/// When the object is serialized out as xml, it's qualified name is a:stretch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:stretch")]
pub struct Stretch {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Fill Rectangle
    #[xml(child = "a:fillRect")]
    pub fill_rectangle: Option<FillRectangle>,
}
/// Defines the NoFill Class.
/// When the object is serialized out as xml, it's qualified name is a:noFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:noFill")]
pub struct NoFill {}
/// Defines the SolidFill Class.
/// When the object is serialized out as xml, it's qualified name is a:solidFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:solidFill")]
pub struct SolidFill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<SolidFillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SolidFillChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Defines the GradientFill Class.
/// When the object is serialized out as xml, it's qualified name is a:gradFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gradFill")]
pub struct GradientFill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Tile Flip
    /// Represents the following attribute in the schema: :flip
    #[xml(attr = "flip")]
    pub flip: Option<TileFlipValues>,
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
    #[xml(child = "a:gsLst", child = "a:lin", child = "a:path", child = "a:tileRect")]
    pub children: Vec<GradientFillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GradientFillChildChoice {
    #[xml(tag = "a:gsLst")]
    AGsLst(GradientStopList),
    #[xml(tag = "a:lin")]
    ALin(LinearGradientFill),
    #[xml(tag = "a:path")]
    APath(PathGradientFill),
    #[xml(tag = "a:tileRect")]
    ATileRect(TileRectangle),
}
/// Defines the BlipFill Class.
/// When the object is serialized out as xml, it's qualified name is a:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blipFill")]
pub struct BlipFill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// DPI Setting
    /// Represents the following attribute in the schema: :dpi
    #[xml(attr = "dpi")]
    pub dpi: Option<i32>,
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
    ABlip(Blip),
    #[xml(tag = "a:srcRect")]
    ASrcRect(SourceRectangle),
    #[xml(tag = "a:tile")]
    ATile(Tile),
    #[xml(tag = "a:stretch")]
    AStretch(Stretch),
}
/// Pattern Fill.
/// When the object is serialized out as xml, it's qualified name is a:pattFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:pattFill")]
pub struct PatternFill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Preset Pattern
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: Option<PresetPatternValues>,
    ///Foreground color
    #[xml(child = "a:fgClr")]
    pub foreground_color: Option<ForegroundColor>,
    ///Background color
    #[xml(child = "a:bgClr")]
    pub background_color: Option<BackgroundColor>,
}
/// Group Fill.
/// When the object is serialized out as xml, it's qualified name is a:grpFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:grpFill")]
pub struct GroupFill {}
/// Effect Container.
/// When the object is serialized out as xml, it's qualified name is a:cont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cont")]
pub struct EffectContainer {
    /// Effect Container Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<EffectContainerValues>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(
        child = "a:cont",
        child = "a:effect",
        child = "a:alphaBiLevel",
        child = "a:alphaCeiling",
        child = "a:alphaFloor",
        child = "a:alphaInv",
        child = "a:alphaMod",
        child = "a:alphaModFix",
        child = "a:alphaOutset",
        child = "a:alphaRepl",
        child = "a:biLevel",
        child = "a:blend",
        child = "a:blur",
        child = "a:clrChange",
        child = "a:clrRepl",
        child = "a:duotone",
        child = "a:fill",
        child = "a:fillOverlay",
        child = "a:glow",
        child = "a:grayscl",
        child = "a:hsl",
        child = "a:innerShdw",
        child = "a:lum",
        child = "a:outerShdw",
        child = "a:prstShdw",
        child = "a:reflection",
        child = "a:relOff",
        child = "a:softEdge",
        child = "a:tint",
        child = "a:xfrm",
    )]
    pub children: Vec<EffectContainerChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectContainerChildChoice {
    #[xml(tag = "a:cont")]
    ACont(EffectContainer),
    #[xml(tag = "a:effect")]
    AEffect(Effect),
    #[xml(tag = "a:alphaBiLevel")]
    AAlphaBiLevel(AlphaBiLevel),
    #[xml(tag = "a:alphaCeiling")]
    AAlphaCeiling(AlphaCeiling),
    #[xml(tag = "a:alphaFloor")]
    AAlphaFloor(AlphaFloor),
    #[xml(tag = "a:alphaInv")]
    AAlphaInv(AlphaInverse),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulationEffect),
    #[xml(tag = "a:alphaModFix")]
    AAlphaModFix(AlphaModulationFixed),
    #[xml(tag = "a:alphaOutset")]
    AAlphaOutset(AlphaOutset),
    #[xml(tag = "a:alphaRepl")]
    AAlphaRepl(AlphaReplace),
    #[xml(tag = "a:biLevel")]
    ABiLevel(BiLevel),
    #[xml(tag = "a:blend")]
    ABlend(Blend),
    #[xml(tag = "a:blur")]
    ABlur(Blur),
    #[xml(tag = "a:clrChange")]
    AClrChange(ColorChange),
    #[xml(tag = "a:clrRepl")]
    AClrRepl(ColorReplacement),
    #[xml(tag = "a:duotone")]
    ADuotone(Duotone),
    #[xml(tag = "a:fill")]
    AFill(Fill),
    #[xml(tag = "a:fillOverlay")]
    AFillOverlay(FillOverlay),
    #[xml(tag = "a:glow")]
    AGlow(Glow),
    #[xml(tag = "a:grayscl")]
    AGrayscl(Grayscale),
    #[xml(tag = "a:hsl")]
    AHsl(Hsl),
    #[xml(tag = "a:innerShdw")]
    AInnerShdw(InnerShadow),
    #[xml(tag = "a:lum")]
    ALum(LuminanceEffect),
    #[xml(tag = "a:outerShdw")]
    AOuterShdw(OuterShadow),
    #[xml(tag = "a:prstShdw")]
    APrstShdw(PresetShadow),
    #[xml(tag = "a:reflection")]
    AReflection(Reflection),
    #[xml(tag = "a:relOff")]
    ARelOff(RelativeOffset),
    #[xml(tag = "a:softEdge")]
    ASoftEdge(SoftEdge),
    #[xml(tag = "a:tint")]
    ATint(TintEffect),
    #[xml(tag = "a:xfrm")]
    AXfrm(TransformEffect),
}
/// Effect Container.
/// When the object is serialized out as xml, it's qualified name is a:effectDag.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effectDag")]
pub struct EffectDag {
    /// Effect Container Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<EffectContainerValues>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(
        child = "a:cont",
        child = "a:effect",
        child = "a:alphaBiLevel",
        child = "a:alphaCeiling",
        child = "a:alphaFloor",
        child = "a:alphaInv",
        child = "a:alphaMod",
        child = "a:alphaModFix",
        child = "a:alphaOutset",
        child = "a:alphaRepl",
        child = "a:biLevel",
        child = "a:blend",
        child = "a:blur",
        child = "a:clrChange",
        child = "a:clrRepl",
        child = "a:duotone",
        child = "a:fill",
        child = "a:fillOverlay",
        child = "a:glow",
        child = "a:grayscl",
        child = "a:hsl",
        child = "a:innerShdw",
        child = "a:lum",
        child = "a:outerShdw",
        child = "a:prstShdw",
        child = "a:reflection",
        child = "a:relOff",
        child = "a:softEdge",
        child = "a:tint",
        child = "a:xfrm",
    )]
    pub children: Vec<EffectDagChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectDagChildChoice {
    #[xml(tag = "a:cont")]
    ACont(EffectContainer),
    #[xml(tag = "a:effect")]
    AEffect(Effect),
    #[xml(tag = "a:alphaBiLevel")]
    AAlphaBiLevel(AlphaBiLevel),
    #[xml(tag = "a:alphaCeiling")]
    AAlphaCeiling(AlphaCeiling),
    #[xml(tag = "a:alphaFloor")]
    AAlphaFloor(AlphaFloor),
    #[xml(tag = "a:alphaInv")]
    AAlphaInv(AlphaInverse),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulationEffect),
    #[xml(tag = "a:alphaModFix")]
    AAlphaModFix(AlphaModulationFixed),
    #[xml(tag = "a:alphaOutset")]
    AAlphaOutset(AlphaOutset),
    #[xml(tag = "a:alphaRepl")]
    AAlphaRepl(AlphaReplace),
    #[xml(tag = "a:biLevel")]
    ABiLevel(BiLevel),
    #[xml(tag = "a:blend")]
    ABlend(Blend),
    #[xml(tag = "a:blur")]
    ABlur(Blur),
    #[xml(tag = "a:clrChange")]
    AClrChange(ColorChange),
    #[xml(tag = "a:clrRepl")]
    AClrRepl(ColorReplacement),
    #[xml(tag = "a:duotone")]
    ADuotone(Duotone),
    #[xml(tag = "a:fill")]
    AFill(Fill),
    #[xml(tag = "a:fillOverlay")]
    AFillOverlay(FillOverlay),
    #[xml(tag = "a:glow")]
    AGlow(Glow),
    #[xml(tag = "a:grayscl")]
    AGrayscl(Grayscale),
    #[xml(tag = "a:hsl")]
    AHsl(Hsl),
    #[xml(tag = "a:innerShdw")]
    AInnerShdw(InnerShadow),
    #[xml(tag = "a:lum")]
    ALum(LuminanceEffect),
    #[xml(tag = "a:outerShdw")]
    AOuterShdw(OuterShadow),
    #[xml(tag = "a:prstShdw")]
    APrstShdw(PresetShadow),
    #[xml(tag = "a:reflection")]
    AReflection(Reflection),
    #[xml(tag = "a:relOff")]
    ARelOff(RelativeOffset),
    #[xml(tag = "a:softEdge")]
    ASoftEdge(SoftEdge),
    #[xml(tag = "a:tint")]
    ATint(TintEffect),
    #[xml(tag = "a:xfrm")]
    AXfrm(TransformEffect),
}
/// Defines the EffectContainerType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EffectContainerType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Effect Container Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<EffectContainerValues>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(
        child = "a:cont",
        child = "a:effect",
        child = "a:alphaBiLevel",
        child = "a:alphaCeiling",
        child = "a:alphaFloor",
        child = "a:alphaInv",
        child = "a:alphaMod",
        child = "a:alphaModFix",
        child = "a:alphaOutset",
        child = "a:alphaRepl",
        child = "a:biLevel",
        child = "a:blend",
        child = "a:blur",
        child = "a:clrChange",
        child = "a:clrRepl",
        child = "a:duotone",
        child = "a:fill",
        child = "a:fillOverlay",
        child = "a:glow",
        child = "a:grayscl",
        child = "a:hsl",
        child = "a:innerShdw",
        child = "a:lum",
        child = "a:outerShdw",
        child = "a:prstShdw",
        child = "a:reflection",
        child = "a:relOff",
        child = "a:softEdge",
        child = "a:tint",
        child = "a:xfrm",
    )]
    pub children: Vec<EffectContainerTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectContainerTypeChildChoice {
    #[xml(tag = "a:cont")]
    ACont(EffectContainer),
    #[xml(tag = "a:effect")]
    AEffect(Effect),
    #[xml(tag = "a:alphaBiLevel")]
    AAlphaBiLevel(AlphaBiLevel),
    #[xml(tag = "a:alphaCeiling")]
    AAlphaCeiling(AlphaCeiling),
    #[xml(tag = "a:alphaFloor")]
    AAlphaFloor(AlphaFloor),
    #[xml(tag = "a:alphaInv")]
    AAlphaInv(AlphaInverse),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulationEffect),
    #[xml(tag = "a:alphaModFix")]
    AAlphaModFix(AlphaModulationFixed),
    #[xml(tag = "a:alphaOutset")]
    AAlphaOutset(AlphaOutset),
    #[xml(tag = "a:alphaRepl")]
    AAlphaRepl(AlphaReplace),
    #[xml(tag = "a:biLevel")]
    ABiLevel(BiLevel),
    #[xml(tag = "a:blend")]
    ABlend(Blend),
    #[xml(tag = "a:blur")]
    ABlur(Blur),
    #[xml(tag = "a:clrChange")]
    AClrChange(ColorChange),
    #[xml(tag = "a:clrRepl")]
    AClrRepl(ColorReplacement),
    #[xml(tag = "a:duotone")]
    ADuotone(Duotone),
    #[xml(tag = "a:fill")]
    AFill(Fill),
    #[xml(tag = "a:fillOverlay")]
    AFillOverlay(FillOverlay),
    #[xml(tag = "a:glow")]
    AGlow(Glow),
    #[xml(tag = "a:grayscl")]
    AGrayscl(Grayscale),
    #[xml(tag = "a:hsl")]
    AHsl(Hsl),
    #[xml(tag = "a:innerShdw")]
    AInnerShdw(InnerShadow),
    #[xml(tag = "a:lum")]
    ALum(LuminanceEffect),
    #[xml(tag = "a:outerShdw")]
    AOuterShdw(OuterShadow),
    #[xml(tag = "a:prstShdw")]
    APrstShdw(PresetShadow),
    #[xml(tag = "a:reflection")]
    AReflection(Reflection),
    #[xml(tag = "a:relOff")]
    ARelOff(RelativeOffset),
    #[xml(tag = "a:softEdge")]
    ASoftEdge(SoftEdge),
    #[xml(tag = "a:tint")]
    ATint(TintEffect),
    #[xml(tag = "a:xfrm")]
    AXfrm(TransformEffect),
}
/// Effect.
/// When the object is serialized out as xml, it's qualified name is a:effect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effect")]
pub struct Effect {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
/// Defines the AlphaBiLevel Class.
/// When the object is serialized out as xml, it's qualified name is a:alphaBiLevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaBiLevel")]
pub struct AlphaBiLevel {
    /// Threshold
    /// Represents the following attribute in the schema: :thresh
    #[xml(attr = "thresh")]
    pub threshold: i32,
}
/// Alpha Ceiling Effect.
/// When the object is serialized out as xml, it's qualified name is a:alphaCeiling.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaCeiling")]
pub struct AlphaCeiling {}
/// Alpha Floor Effect.
/// When the object is serialized out as xml, it's qualified name is a:alphaFloor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaFloor")]
pub struct AlphaFloor {}
/// Alpha Inverse Effect.
/// When the object is serialized out as xml, it's qualified name is a:alphaInv.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaInv")]
pub struct AlphaInverse {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<AlphaInverseChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AlphaInverseChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Alpha Modulate Effect.
/// When the object is serialized out as xml, it's qualified name is a:alphaMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaMod")]
pub struct AlphaModulationEffect {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:cont")]
    pub effect_container: EffectContainer,
}
/// Defines the AlphaModulationFixed Class.
/// When the object is serialized out as xml, it's qualified name is a:alphaModFix.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaModFix")]
pub struct AlphaModulationFixed {
    /// Amount
    /// Represents the following attribute in the schema: :amt
    #[xml(attr = "amt")]
    pub amount: Option<i32>,
}
/// Alpha Inset/Outset Effect.
/// When the object is serialized out as xml, it's qualified name is a:alphaOutset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaOutset")]
pub struct AlphaOutset {
    /// Radius
    /// Represents the following attribute in the schema: :rad
    #[xml(attr = "rad")]
    pub radius: Option<i64>,
}
/// Alpha Replace Effect.
/// When the object is serialized out as xml, it's qualified name is a:alphaRepl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:alphaRepl")]
pub struct AlphaReplace {
    /// Alpha
    /// Represents the following attribute in the schema: :a
    #[xml(attr = "a")]
    pub alpha: i32,
}
/// Defines the BiLevel Class.
/// When the object is serialized out as xml, it's qualified name is a:biLevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:biLevel")]
pub struct BiLevel {
    /// Threshold
    /// Represents the following attribute in the schema: :thresh
    #[xml(attr = "thresh")]
    pub threshold: i32,
}
/// Blend Effect.
/// When the object is serialized out as xml, it's qualified name is a:blend.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blend")]
pub struct Blend {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Blend Mode
    /// Represents the following attribute in the schema: :blend
    #[xml(attr = "blend")]
    pub blend_mode: BlendModeValues,
    ///Effect to blend
    #[xml(child = "a:cont")]
    pub effect_container: EffectContainer,
}
/// Defines the Blur Class.
/// When the object is serialized out as xml, it's qualified name is a:blur.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blur")]
pub struct Blur {
    /// Radius
    /// Represents the following attribute in the schema: :rad
    #[xml(attr = "rad")]
    pub radius: Option<i64>,
    /// Grow Bounds
    /// Represents the following attribute in the schema: :grow
    #[xml(attr = "grow")]
    pub grow: Option<bool>,
}
/// Color Change Effect.
/// When the object is serialized out as xml, it's qualified name is a:clrChange.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:clrChange")]
pub struct ColorChange {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Consider Alpha Values
    /// Represents the following attribute in the schema: :useA
    #[xml(attr = "useA")]
    pub use_alpha: Option<bool>,
    ///Change Color From
    #[xml(child = "a:clrFrom")]
    pub color_from: ColorFrom,
    ///Change Color To
    #[xml(child = "a:clrTo")]
    pub color_to: ColorTo,
}
/// Defines the ColorReplacement Class.
/// When the object is serialized out as xml, it's qualified name is a:clrRepl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:clrRepl")]
pub struct ColorReplacement {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorReplacementChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorReplacementChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Duotone Effect.
/// When the object is serialized out as xml, it's qualified name is a:duotone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:duotone")]
pub struct Duotone {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<DuotoneChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DuotoneChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Fill.
/// When the object is serialized out as xml, it's qualified name is a:fill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fill")]
pub struct Fill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<FillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
}
/// Fill Overlay Effect.
/// When the object is serialized out as xml, it's qualified name is a:fillOverlay.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fillOverlay")]
pub struct FillOverlay {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Blend
    /// Represents the following attribute in the schema: :blend
    #[xml(attr = "blend")]
    pub blend: BlendModeValues,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<FillOverlayChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillOverlayChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
}
/// Glow Effect.
/// When the object is serialized out as xml, it's qualified name is a:glow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:glow")]
pub struct Glow {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Radius
    /// Represents the following attribute in the schema: :rad
    #[xml(attr = "rad")]
    pub radius: Option<i64>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<GlowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GlowChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Gray Scale Effect.
/// When the object is serialized out as xml, it's qualified name is a:grayscl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:grayscl")]
pub struct Grayscale {}
/// Hue Saturation Luminance Effect.
/// When the object is serialized out as xml, it's qualified name is a:hsl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hsl")]
pub struct Hsl {
    /// Hue
    /// Represents the following attribute in the schema: :hue
    #[xml(attr = "hue")]
    pub hue: Option<i32>,
    /// Saturation
    /// Represents the following attribute in the schema: :sat
    #[xml(attr = "sat")]
    pub saturation: Option<i32>,
    /// Luminance
    /// Represents the following attribute in the schema: :lum
    #[xml(attr = "lum")]
    pub luminance: Option<i32>,
}
/// Inner Shadow Effect.
/// When the object is serialized out as xml, it's qualified name is a:innerShdw.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:innerShdw")]
pub struct InnerShadow {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Blur Radius
    /// Represents the following attribute in the schema: :blurRad
    #[xml(attr = "blurRad")]
    pub blur_radius: Option<i64>,
    /// Distance
    /// Represents the following attribute in the schema: :dist
    #[xml(attr = "dist")]
    pub distance: Option<i64>,
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<i32>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<InnerShadowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InnerShadowChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Luminance.
/// When the object is serialized out as xml, it's qualified name is a:lum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lum")]
pub struct LuminanceEffect {
    /// Brightness
    /// Represents the following attribute in the schema: :bright
    #[xml(attr = "bright")]
    pub brightness: Option<i32>,
    /// Contrast
    /// Represents the following attribute in the schema: :contrast
    #[xml(attr = "contrast")]
    pub contrast: Option<i32>,
}
/// Outer Shadow Effect.
/// When the object is serialized out as xml, it's qualified name is a:outerShdw.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:outerShdw")]
pub struct OuterShadow {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Blur Radius
    /// Represents the following attribute in the schema: :blurRad
    #[xml(attr = "blurRad")]
    pub blur_radius: Option<i64>,
    /// Shadow Offset Distance
    /// Represents the following attribute in the schema: :dist
    #[xml(attr = "dist")]
    pub distance: Option<i64>,
    /// Shadow Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<i32>,
    /// Horizontal Scaling Factor
    /// Represents the following attribute in the schema: :sx
    #[xml(attr = "sx")]
    pub horizontal_ratio: Option<i32>,
    /// Vertical Scaling Factor
    /// Represents the following attribute in the schema: :sy
    #[xml(attr = "sy")]
    pub vertical_ratio: Option<i32>,
    /// Horizontal Skew
    /// Represents the following attribute in the schema: :kx
    #[xml(attr = "kx")]
    pub horizontal_skew: Option<i32>,
    /// Vertical Skew
    /// Represents the following attribute in the schema: :ky
    #[xml(attr = "ky")]
    pub vertical_skew: Option<i32>,
    /// Shadow Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<RectangleAlignmentValues>,
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<OuterShadowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OuterShadowChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Preset Shadow.
/// When the object is serialized out as xml, it's qualified name is a:prstShdw.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:prstShdw")]
pub struct PresetShadow {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Preset Shadow
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: PresetShadowValues,
    /// Distance
    /// Represents the following attribute in the schema: :dist
    #[xml(attr = "dist")]
    pub distance: Option<i64>,
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<i32>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<PresetShadowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PresetShadowChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Reflection Effect.
/// When the object is serialized out as xml, it's qualified name is a:reflection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:reflection")]
pub struct Reflection {
    /// Blur Radius
    /// Represents the following attribute in the schema: :blurRad
    #[xml(attr = "blurRad")]
    pub blur_radius: Option<i64>,
    /// Start Opacity
    /// Represents the following attribute in the schema: :stA
    #[xml(attr = "stA")]
    pub start_opacity: Option<i32>,
    /// Start Position
    /// Represents the following attribute in the schema: :stPos
    #[xml(attr = "stPos")]
    pub start_position: Option<i32>,
    /// End Alpha
    /// Represents the following attribute in the schema: :endA
    #[xml(attr = "endA")]
    pub end_alpha: Option<i32>,
    /// End Position
    /// Represents the following attribute in the schema: :endPos
    #[xml(attr = "endPos")]
    pub end_position: Option<i32>,
    /// Distance
    /// Represents the following attribute in the schema: :dist
    #[xml(attr = "dist")]
    pub distance: Option<i64>,
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: Option<i32>,
    /// Fade Direction
    /// Represents the following attribute in the schema: :fadeDir
    #[xml(attr = "fadeDir")]
    pub fade_direction: Option<i32>,
    /// Horizontal Ratio
    /// Represents the following attribute in the schema: :sx
    #[xml(attr = "sx")]
    pub horizontal_ratio: Option<i32>,
    /// Vertical Ratio
    /// Represents the following attribute in the schema: :sy
    #[xml(attr = "sy")]
    pub vertical_ratio: Option<i32>,
    /// Horizontal Skew
    /// Represents the following attribute in the schema: :kx
    #[xml(attr = "kx")]
    pub horizontal_skew: Option<i32>,
    /// Vertical Skew
    /// Represents the following attribute in the schema: :ky
    #[xml(attr = "ky")]
    pub vertical_skew: Option<i32>,
    /// Shadow Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<RectangleAlignmentValues>,
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
}
/// Relative Offset Effect.
/// When the object is serialized out as xml, it's qualified name is a:relOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:relOff")]
pub struct RelativeOffset {
    /// Offset X
    /// Represents the following attribute in the schema: :tx
    #[xml(attr = "tx")]
    pub offset_x: Option<i32>,
    /// Offset Y
    /// Represents the following attribute in the schema: :ty
    #[xml(attr = "ty")]
    pub offset_y: Option<i32>,
}
/// Soft Edge Effect.
/// When the object is serialized out as xml, it's qualified name is a:softEdge.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:softEdge")]
pub struct SoftEdge {
    /// Radius
    /// Represents the following attribute in the schema: :rad
    #[xml(attr = "rad")]
    pub radius: i64,
}
/// Defines the TintEffect Class.
/// When the object is serialized out as xml, it's qualified name is a:tint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tint")]
pub struct TintEffect {
    /// Hue
    /// Represents the following attribute in the schema: :hue
    #[xml(attr = "hue")]
    pub hue: Option<i32>,
    /// Amount
    /// Represents the following attribute in the schema: :amt
    #[xml(attr = "amt")]
    pub amount: Option<i32>,
}
/// Transform Effect.
/// When the object is serialized out as xml, it's qualified name is a:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:xfrm")]
pub struct TransformEffect {
    /// Horizontal Ratio
    /// Represents the following attribute in the schema: :sx
    #[xml(attr = "sx")]
    pub horizontal_ratio: Option<i32>,
    /// Vertical Ratio
    /// Represents the following attribute in the schema: :sy
    #[xml(attr = "sy")]
    pub vertical_ratio: Option<i32>,
    /// Horizontal Skew
    /// Represents the following attribute in the schema: :kx
    #[xml(attr = "kx")]
    pub horizontal_skew: Option<i32>,
    /// Vertical Skew
    /// Represents the following attribute in the schema: :ky
    #[xml(attr = "ky")]
    pub vertical_skew: Option<i32>,
    /// Horizontal Shift
    /// Represents the following attribute in the schema: :tx
    #[xml(attr = "tx")]
    pub horizontal_shift: Option<i64>,
    /// Vertical Shift
    /// Represents the following attribute in the schema: :ty
    #[xml(attr = "ty")]
    pub vertical_shift: Option<i64>,
}
/// Effect Container.
/// When the object is serialized out as xml, it's qualified name is a:effectLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effectLst")]
pub struct EffectList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Blur Effect
    #[xml(child = "a:blur")]
    pub blur: Option<Blur>,
    /// _
    #[xml(child = "a:fillOverlay")]
    pub fill_overlay: Option<FillOverlay>,
    /// _
    #[xml(child = "a:glow")]
    pub glow: Option<Glow>,
    /// _
    #[xml(child = "a:innerShdw")]
    pub inner_shadow: Option<InnerShadow>,
    /// _
    #[xml(child = "a:outerShdw")]
    pub outer_shadow: Option<OuterShadow>,
    /// _
    #[xml(child = "a:prstShdw")]
    pub preset_shadow: Option<PresetShadow>,
    /// _
    #[xml(child = "a:reflection")]
    pub reflection: Option<Reflection>,
    /// _
    #[xml(child = "a:softEdge")]
    pub soft_edge: Option<SoftEdge>,
}
/// Custom geometry.
/// When the object is serialized out as xml, it's qualified name is a:custGeom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:custGeom")]
pub struct CustomGeometry {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Adjust Value List
    #[xml(child = "a:avLst")]
    pub adjust_value_list: Option<AdjustValueList>,
    ///List of Shape Guides
    #[xml(child = "a:gdLst")]
    pub shape_guide_list: Option<ShapeGuideList>,
    ///List of Shape Adjust Handles
    #[xml(child = "a:ahLst")]
    pub adjust_handle_list: Option<AdjustHandleList>,
    ///List of Shape Connection Sites
    #[xml(child = "a:cxnLst")]
    pub connection_site_list: Option<ConnectionSiteList>,
    ///Shape Text Rectangle
    #[xml(child = "a:rect")]
    pub rectangle: Option<Rectangle>,
    ///List of Shape Paths
    #[xml(child = "a:pathLst")]
    pub path_list: PathList,
}
/// Preset geometry.
/// When the object is serialized out as xml, it's qualified name is a:prstGeom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:prstGeom")]
pub struct PresetGeometry {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Preset Shape
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: ShapeTypeValues,
    ///List of Shape Adjust Values
    #[xml(child = "a:avLst")]
    pub adjust_value_list: Option<AdjustValueList>,
}
/// Preset Text Warp.
/// When the object is serialized out as xml, it's qualified name is a:prstTxWarp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:prstTxWarp")]
pub struct PresetTextWarp {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Preset Warp Shape
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: TextShapeValues,
    ///Adjust Value List
    #[xml(child = "a:avLst")]
    pub adjust_value_list: Option<AdjustValueList>,
}
/// Round Line Join.
/// When the object is serialized out as xml, it's qualified name is a:round.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:round")]
pub struct Round {}
/// Line Join Bevel.
/// When the object is serialized out as xml, it's qualified name is a:bevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bevel")]
pub struct LineJoinBevel {}
/// Miter Line Join.
/// When the object is serialized out as xml, it's qualified name is a:miter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:miter")]
pub struct Miter {
    /// Miter Join Limit
    /// Represents the following attribute in the schema: :lim
    #[xml(attr = "lim")]
    pub limit: Option<i32>,
}
/// Preset Dash.
/// When the object is serialized out as xml, it's qualified name is a:prstDash.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:prstDash")]
pub struct PresetDash {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<PresetLineDashValues>,
}
/// Custom Dash.
/// When the object is serialized out as xml, it's qualified name is a:custDash.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:custDash")]
pub struct CustomDash {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ds")]
    pub a_ds: Vec<DashStop>,
}
/// Fill.
/// When the object is serialized out as xml, it's qualified name is a:fill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fill")]
pub struct FillProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<FillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillPropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
}
/// Fill Reference.
/// When the object is serialized out as xml, it's qualified name is a:fillRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fillRef")]
pub struct FillReference {
    /// Style Matrix Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<FillReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillReferenceChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Effect Reference.
/// When the object is serialized out as xml, it's qualified name is a:effectRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effectRef")]
pub struct EffectReference {
    /// Style Matrix Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<EffectReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectReferenceChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Defines the LineReference Class.
/// When the object is serialized out as xml, it's qualified name is a:lnRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnRef")]
pub struct LineReference {
    /// Style Matrix Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<LineReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineReferenceChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Defines the StyleMatrixReferenceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StyleMatrixReferenceType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Style Matrix Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<StyleMatrixReferenceTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StyleMatrixReferenceTypeChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Effect.
/// When the object is serialized out as xml, it's qualified name is a:effect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effect")]
pub struct EffectPropertiesType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:effectLst", child = "a:effectDag")]
    pub children: Vec<EffectPropertiesTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectPropertiesTypeChildChoice {
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
}
/// Font.
/// When the object is serialized out as xml, it's qualified name is a:font.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:font")]
pub struct Fonts {
    ///Latin Font
    #[xml(child = "a:latin")]
    pub latin_font: LatinFont,
    ///East Asian Font
    #[xml(child = "a:ea")]
    pub east_asian_font: EastAsianFont,
    ///Complex Script Font
    #[xml(child = "a:cs")]
    pub complex_script_font: ComplexScriptFont,
    /// _
    #[xml(child = "a:font")]
    pub a_font: Vec<SupplementalFont>,
    /// _
    #[xml(child = "a:extLst")]
    pub a_ext_lst: Option<ExtensionList>,
}
/// Major Font.
/// When the object is serialized out as xml, it's qualified name is a:majorFont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:majorFont")]
pub struct MajorFont {
    ///Latin Font
    #[xml(child = "a:latin")]
    pub latin_font: LatinFont,
    ///East Asian Font
    #[xml(child = "a:ea")]
    pub east_asian_font: EastAsianFont,
    ///Complex Script Font
    #[xml(child = "a:cs")]
    pub complex_script_font: ComplexScriptFont,
    /// _
    #[xml(child = "a:font")]
    pub a_font: Vec<SupplementalFont>,
    /// _
    #[xml(child = "a:extLst")]
    pub a_ext_lst: Option<ExtensionList>,
}
/// Minor fonts.
/// When the object is serialized out as xml, it's qualified name is a:minorFont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:minorFont")]
pub struct MinorFont {
    ///Latin Font
    #[xml(child = "a:latin")]
    pub latin_font: LatinFont,
    ///East Asian Font
    #[xml(child = "a:ea")]
    pub east_asian_font: EastAsianFont,
    ///Complex Script Font
    #[xml(child = "a:cs")]
    pub complex_script_font: ComplexScriptFont,
    /// _
    #[xml(child = "a:font")]
    pub a_font: Vec<SupplementalFont>,
    /// _
    #[xml(child = "a:extLst")]
    pub a_ext_lst: Option<ExtensionList>,
}
/// Defines the FontCollectionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct FontCollectionType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
}
/// Defines the FontReference Class.
/// When the object is serialized out as xml, it's qualified name is a:fontRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fontRef")]
pub struct FontReference {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Identifier
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: FontCollectionIndexValues,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<FontReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FontReferenceChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// No AutoFit.
/// When the object is serialized out as xml, it's qualified name is a:noAutofit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:noAutofit")]
pub struct NoAutoFit {}
/// Normal AutoFit.
/// When the object is serialized out as xml, it's qualified name is a:normAutofit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:normAutofit")]
pub struct NormalAutoFit {
    /// Font Scale
    /// Represents the following attribute in the schema: :fontScale
    #[xml(attr = "fontScale")]
    pub font_scale: Option<i32>,
    /// Line Space Reduction
    /// Represents the following attribute in the schema: :lnSpcReduction
    #[xml(attr = "lnSpcReduction")]
    pub line_space_reduction: Option<i32>,
}
/// Shape AutoFit.
/// When the object is serialized out as xml, it's qualified name is a:spAutoFit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spAutoFit")]
pub struct ShapeAutoFit {}
/// Follow Text.
/// When the object is serialized out as xml, it's qualified name is a:buClrTx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buClrTx")]
pub struct BulletColorText {}
/// Color Specified.
/// When the object is serialized out as xml, it's qualified name is a:buClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buClr")]
pub struct BulletColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<BulletColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BulletColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Extrusion Color.
/// When the object is serialized out as xml, it's qualified name is a:extrusionClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extrusionClr")]
pub struct ExtrusionColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ExtrusionColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtrusionColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Contour Color.
/// When the object is serialized out as xml, it's qualified name is a:contourClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:contourClr")]
pub struct ContourColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ContourColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ContourColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Change Color From.
/// When the object is serialized out as xml, it's qualified name is a:clrFrom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:clrFrom")]
pub struct ColorFrom {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorFromChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorFromChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Change Color To.
/// When the object is serialized out as xml, it's qualified name is a:clrTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:clrTo")]
pub struct ColorTo {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorToChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorToChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Foreground color.
/// When the object is serialized out as xml, it's qualified name is a:fgClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fgClr")]
pub struct ForegroundColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ForegroundColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ForegroundColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Background color.
/// When the object is serialized out as xml, it's qualified name is a:bgClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bgClr")]
pub struct BackgroundColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<BackgroundColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Defines the Highlight Class.
/// When the object is serialized out as xml, it's qualified name is a:highlight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:highlight")]
pub struct Highlight {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<HighlightChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HighlightChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
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
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Bullet Size Follows Text.
/// When the object is serialized out as xml, it's qualified name is a:buSzTx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buSzTx")]
pub struct BulletSizeText {}
/// Bullet Size Percentage.
/// When the object is serialized out as xml, it's qualified name is a:buSzPct.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buSzPct")]
pub struct BulletSizePercentage {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Bullet Size Points.
/// When the object is serialized out as xml, it's qualified name is a:buSzPts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buSzPts")]
pub struct BulletSizePoints {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Follow text.
/// When the object is serialized out as xml, it's qualified name is a:buFontTx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buFontTx")]
pub struct BulletFontText {}
/// Specified.
/// When the object is serialized out as xml, it's qualified name is a:buFont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buFont")]
pub struct BulletFont {
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
/// Latin Font.
/// When the object is serialized out as xml, it's qualified name is a:latin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:latin")]
pub struct LatinFont {
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
/// East Asian Font.
/// When the object is serialized out as xml, it's qualified name is a:ea.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ea")]
pub struct EastAsianFont {
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
/// Complex Script Font.
/// When the object is serialized out as xml, it's qualified name is a:cs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cs")]
pub struct ComplexScriptFont {
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
/// Defines the SymbolFont Class.
/// When the object is serialized out as xml, it's qualified name is a:sym.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sym")]
pub struct SymbolFont {
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
/// Defines the TextFontType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextFontType {
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
/// No Bullet.
/// When the object is serialized out as xml, it's qualified name is a:buNone.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buNone")]
pub struct NoBullet {}
/// Auto-Numbered Bullet.
/// When the object is serialized out as xml, it's qualified name is a:buAutoNum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buAutoNum")]
pub struct AutoNumberedBullet {
    /// Bullet Autonumbering Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: TextAutoNumberSchemeValues,
    /// Start Numbering At
    /// Represents the following attribute in the schema: :startAt
    #[xml(attr = "startAt")]
    pub start_at: Option<i32>,
}
/// Character Bullet.
/// When the object is serialized out as xml, it's qualified name is a:buChar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buChar")]
pub struct CharacterBullet {
    /// Bullet Character
    /// Represents the following attribute in the schema: :char
    #[xml(attr = "char")]
    pub char: String,
}
/// Picture Bullet.
/// When the object is serialized out as xml, it's qualified name is a:buBlip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:buBlip")]
pub struct PictureBullet {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Blip
    #[xml(child = "a:blip")]
    pub blip: Blip,
}
/// Underline Follows Text.
/// When the object is serialized out as xml, it's qualified name is a:uLnTx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:uLnTx")]
pub struct UnderlineFollowsText {}
/// Underline Stroke.
/// When the object is serialized out as xml, it's qualified name is a:uLn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:uLn")]
pub struct Underline {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<UnderlineChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum UnderlineChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Defines the Outline Class.
/// When the object is serialized out as xml, it's qualified name is a:ln.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ln")]
pub struct Outline {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<OutlineChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OutlineChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Left Border Line Properties.
/// When the object is serialized out as xml, it's qualified name is a:lnL.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnL")]
pub struct LeftBorderLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<LeftBorderLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LeftBorderLinePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Right Border Line Properties.
/// When the object is serialized out as xml, it's qualified name is a:lnR.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnR")]
pub struct RightBorderLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<RightBorderLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RightBorderLinePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Top Border Line Properties.
/// When the object is serialized out as xml, it's qualified name is a:lnT.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnT")]
pub struct TopBorderLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<TopBorderLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TopBorderLinePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Bottom Border Line Properties.
/// When the object is serialized out as xml, it's qualified name is a:lnB.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnB")]
pub struct BottomBorderLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<BottomBorderLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BottomBorderLinePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Top-Left to Bottom-Right Border Line Properties.
/// When the object is serialized out as xml, it's qualified name is a:lnTlToBr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnTlToBr")]
pub struct TopLeftToBottomRightBorderLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<TopLeftToBottomRightBorderLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TopLeftToBottomRightBorderLinePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Bottom-Left to Top-Right Border Line Properties.
/// When the object is serialized out as xml, it's qualified name is a:lnBlToTr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnBlToTr")]
pub struct BottomLeftToTopRightBorderLineProperties {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<BottomLeftToTopRightBorderLinePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BottomLeftToTopRightBorderLinePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Defines the LinePropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LinePropertiesType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<LineCapValues>,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<CompoundLineValues>,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:pattFill",
        child = "a:prstDash",
        child = "a:custDash",
        child = "a:round",
        child = "a:bevel",
        child = "a:miter",
        child = "a:headEnd",
        child = "a:tailEnd",
        child = "a:extLst",
    )]
    pub children: Vec<LinePropertiesTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LinePropertiesTypeChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:prstDash")]
    APrstDash(PresetDash),
    #[xml(tag = "a:custDash")]
    ACustDash(CustomDash),
    #[xml(tag = "a:round")]
    ARound(Round),
    #[xml(tag = "a:bevel")]
    ABevel(LineJoinBevel),
    #[xml(tag = "a:miter")]
    AMiter(Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(LinePropertiesExtensionList),
}
/// Underline Fill Properties Follow Text.
/// When the object is serialized out as xml, it's qualified name is a:uFillTx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:uFillTx")]
pub struct UnderlineFillText {}
/// Underline Fill.
/// When the object is serialized out as xml, it's qualified name is a:uFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:uFill")]
pub struct UnderlineFill {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<UnderlineFillChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum UnderlineFillChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
}
/// Text Run.
/// When the object is serialized out as xml, it's qualified name is a:r.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:r")]
pub struct Run {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Text Character Properties
    #[xml(child = "a:rPr")]
    pub run_properties: Option<RunProperties>,
    ///Text String
    #[xml(child = "a:t")]
    pub text: Text,
}
/// Text Line Break.
/// When the object is serialized out as xml, it's qualified name is a:br.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:br")]
pub struct Break {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Text Run Properties
    #[xml(child = "a:rPr")]
    pub run_properties: Option<RunProperties>,
}
/// Text Field.
/// When the object is serialized out as xml, it's qualified name is a:fld.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fld")]
pub struct Field {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Field ID
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// Field Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    ///Text Character Properties
    #[xml(child = "a:rPr")]
    pub run_properties: Option<RunProperties>,
    ///Text Paragraph Properties
    #[xml(child = "a:pPr")]
    pub paragraph_properties: Option<ParagraphProperties>,
    /// _
    #[xml(child = "a:t")]
    pub text: Option<Text>,
}
/// Graphic Object.
/// When the object is serialized out as xml, it's qualified name is a:graphic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:graphic")]
pub struct Graphic {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Graphic Object Data
    #[xml(child = "a:graphicData")]
    pub graphic_data: GraphicData,
}
/// Defines the Blip Class.
/// When the object is serialized out as xml, it's qualified name is a:blip.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:blip")]
pub struct Blip {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
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
    pub compression_state: Option<BlipCompressionValues>,
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
    pub children: Vec<BlipChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipChildChoice {
    #[xml(tag = "a:alphaBiLevel")]
    AAlphaBiLevel(AlphaBiLevel),
    #[xml(tag = "a:alphaCeiling")]
    AAlphaCeiling(AlphaCeiling),
    #[xml(tag = "a:alphaFloor")]
    AAlphaFloor(AlphaFloor),
    #[xml(tag = "a:alphaInv")]
    AAlphaInv(AlphaInverse),
    #[xml(tag = "a:alphaMod")]
    AAlphaMod(AlphaModulationEffect),
    #[xml(tag = "a:alphaModFix")]
    AAlphaModFix(AlphaModulationFixed),
    #[xml(tag = "a:alphaRepl")]
    AAlphaRepl(AlphaReplace),
    #[xml(tag = "a:biLevel")]
    ABiLevel(BiLevel),
    #[xml(tag = "a:blur")]
    ABlur(Blur),
    #[xml(tag = "a:clrChange")]
    AClrChange(ColorChange),
    #[xml(tag = "a:clrRepl")]
    AClrRepl(ColorReplacement),
    #[xml(tag = "a:duotone")]
    ADuotone(Duotone),
    #[xml(tag = "a:fillOverlay")]
    AFillOverlay(FillOverlay),
    #[xml(tag = "a:grayscl")]
    AGrayscl(Grayscale),
    #[xml(tag = "a:hsl")]
    AHsl(Hsl),
    #[xml(tag = "a:lum")]
    ALum(LuminanceEffect),
    #[xml(tag = "a:tint")]
    ATint(TintEffect),
    #[xml(tag = "a:extLst")]
    AExtLst(BlipExtensionList),
}
/// Theme.
/// When the object is serialized out as xml, it's qualified name is a:theme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:theme")]
pub struct Theme {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// id
    /// Represents the following attribute in the schema: thm15:id
    #[xml(attr = "thm15:id")]
    pub theme_id: Option<String>,
    /// _
    #[xml(child = "a:themeElements")]
    pub theme_elements: ThemeElements,
    /// _
    #[xml(child = "a:objectDefaults")]
    pub object_defaults: Option<ObjectDefaults>,
    /// _
    #[xml(child = "a:extraClrSchemeLst")]
    pub extra_color_scheme_list: Option<ExtraColorSchemeList>,
    /// _
    #[xml(child = "a:custClrLst")]
    pub custom_color_list: Option<CustomColorList>,
    /// _
    #[xml(child = "a:extLst")]
    pub office_style_sheet_extension_list: Option<OfficeStyleSheetExtensionList>,
}
/// Theme Override.
/// When the object is serialized out as xml, it's qualified name is a:themeOverride.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:themeOverride")]
pub struct ThemeOverride {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Color Scheme
    #[xml(child = "a:clrScheme")]
    pub color_scheme: Option<ColorScheme>,
    /// _
    #[xml(child = "a:fontScheme")]
    pub font_scheme: Option<FontScheme>,
    /// _
    #[xml(child = "a:fmtScheme")]
    pub format_scheme: Option<FormatScheme>,
}
/// Theme Manager.
/// When the object is serialized out as xml, it's qualified name is a:themeManager.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:themeManager")]
pub struct ThemeManager {}
/// Master Color Mapping.
/// When the object is serialized out as xml, it's qualified name is a:masterClrMapping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:masterClrMapping")]
pub struct MasterColorMapping {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Table.
/// When the object is serialized out as xml, it's qualified name is a:tbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tbl")]
pub struct Table {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Table Properties
    #[xml(child = "a:tblPr")]
    pub table_properties: Option<TableProperties>,
    ///Table Grid
    #[xml(child = "a:tblGrid")]
    pub table_grid: TableGrid,
    /// _
    #[xml(child = "a:tr")]
    pub a_tr: Vec<TableRow>,
}
/// Table Style List.
/// When the object is serialized out as xml, it's qualified name is a:tblStyleLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tblStyleLst")]
pub struct TableStyleList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Default
    /// Represents the following attribute in the schema: :def
    #[xml(attr = "def")]
    pub default: String,
    /// _
    #[xml(child = "a:tblStyle")]
    pub a_tbl_style: Vec<TableStyleEntry>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct ExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(Extension),
}
/// Audio Start Time.
/// When the object is serialized out as xml, it's qualified name is a:st.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:st")]
pub struct StartTime {
    /// Track
    /// Represents the following attribute in the schema: :track
    #[xml(attr = "track")]
    pub track: u8,
    /// Time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: Option<i32>,
}
/// Audio End Time.
/// When the object is serialized out as xml, it's qualified name is a:end.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:end")]
pub struct EndTime {
    /// Track
    /// Represents the following attribute in the schema: :track
    #[xml(attr = "track")]
    pub track: u8,
    /// Time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: Option<i32>,
}
/// Defines the AudioCDTimeType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct AudioCdTimeType {
    /// Track
    /// Represents the following attribute in the schema: :track
    #[xml(attr = "track")]
    pub track: u8,
    /// Time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: Option<i32>,
}
/// Custom color.
/// When the object is serialized out as xml, it's qualified name is a:custClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:custClr")]
pub struct CustomColor {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<CustomColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum CustomColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Font.
/// When the object is serialized out as xml, it's qualified name is a:font.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:font")]
pub struct SupplementalFont {
    /// Script
    /// Represents the following attribute in the schema: :script
    #[xml(attr = "script")]
    pub script: String,
    /// Typeface
    /// Represents the following attribute in the schema: :typeface
    #[xml(attr = "typeface")]
    pub typeface: String,
}
/// 3D Scene Properties.
/// When the object is serialized out as xml, it's qualified name is a:scene3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:scene3d")]
pub struct Scene3DType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Camera
    #[xml(child = "a:camera")]
    pub camera: Camera,
    ///Light Rig
    #[xml(child = "a:lightRig")]
    pub light_rig: LightRig,
    ///Backdrop Plane
    #[xml(child = "a:backdrop")]
    pub backdrop: Option<Backdrop>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Effect Style.
/// When the object is serialized out as xml, it's qualified name is a:effectStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effectStyle")]
pub struct EffectStyle {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:scene3d",
        child = "a:sp3d",
    )]
    pub children: Vec<EffectStyleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectStyleChildChoice {
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:scene3d")]
    AScene3d(Scene3DType),
    #[xml(tag = "a:sp3d")]
    ASp3d(Shape3DType),
}
/// Fill Style List.
/// When the object is serialized out as xml, it's qualified name is a:fillStyleLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fillStyleLst")]
pub struct FillStyleList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<FillStyleListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillStyleListChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
}
/// Line Style List.
/// When the object is serialized out as xml, it's qualified name is a:lnStyleLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnStyleLst")]
pub struct LineStyleList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ln")]
    pub a_ln: Vec<Outline>,
}
/// Effect Style List.
/// When the object is serialized out as xml, it's qualified name is a:effectStyleLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:effectStyleLst")]
pub struct EffectStyleList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:effectStyle")]
    pub a_effect_style: Vec<EffectStyle>,
}
/// Background Fill Style List.
/// When the object is serialized out as xml, it's qualified name is a:bgFillStyleLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bgFillStyleLst")]
pub struct BackgroundFillStyleList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
    )]
    pub children: Vec<BackgroundFillStyleListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundFillStyleListChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
}
/// Defines the ColorScheme Class.
/// When the object is serialized out as xml, it's qualified name is a:clrScheme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:clrScheme")]
pub struct ColorScheme {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    ///Dark 1
    #[xml(child = "a:dk1")]
    pub dark1_color: Dark1Color,
    ///Light 1
    #[xml(child = "a:lt1")]
    pub light1_color: Light1Color,
    ///Dark 2
    #[xml(child = "a:dk2")]
    pub dark2_color: Dark2Color,
    ///Light 2
    #[xml(child = "a:lt2")]
    pub light2_color: Light2Color,
    ///Accent 1
    #[xml(child = "a:accent1")]
    pub accent1_color: Accent1Color,
    ///Accent 2
    #[xml(child = "a:accent2")]
    pub accent2_color: Accent2Color,
    ///Accent 3
    #[xml(child = "a:accent3")]
    pub accent3_color: Accent3Color,
    ///Accent 4
    #[xml(child = "a:accent4")]
    pub accent4_color: Accent4Color,
    ///Accent 5
    #[xml(child = "a:accent5")]
    pub accent5_color: Accent5Color,
    ///Accent 6
    #[xml(child = "a:accent6")]
    pub accent6_color: Accent6Color,
    ///Hyperlink
    #[xml(child = "a:hlink")]
    pub hyperlink: Hyperlink,
    ///Followed Hyperlink
    #[xml(child = "a:folHlink")]
    pub followed_hyperlink_color: FollowedHyperlinkColor,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Font Scheme.
/// When the object is serialized out as xml, it's qualified name is a:fontScheme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fontScheme")]
pub struct FontScheme {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    ///Major Font
    #[xml(child = "a:majorFont")]
    pub major_font: MajorFont,
    ///Minor fonts
    #[xml(child = "a:minorFont")]
    pub minor_font: MinorFont,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Format Scheme.
/// When the object is serialized out as xml, it's qualified name is a:fmtScheme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fmtScheme")]
pub struct FormatScheme {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    ///Fill Style List
    #[xml(child = "a:fillStyleLst")]
    pub fill_style_list: FillStyleList,
    ///Line Style List
    #[xml(child = "a:lnStyleLst")]
    pub line_style_list: LineStyleList,
    ///Effect Style List
    #[xml(child = "a:effectStyleLst")]
    pub effect_style_list: EffectStyleList,
    ///Background Fill Style List
    #[xml(child = "a:bgFillStyleLst")]
    pub background_fill_style_list: BackgroundFillStyleList,
}
/// Dark 1.
/// When the object is serialized out as xml, it's qualified name is a:dk1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:dk1")]
pub struct Dark1Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Dark1ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Dark1ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Light 1.
/// When the object is serialized out as xml, it's qualified name is a:lt1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lt1")]
pub struct Light1Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Light1ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Light1ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Dark 2.
/// When the object is serialized out as xml, it's qualified name is a:dk2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:dk2")]
pub struct Dark2Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Dark2ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Dark2ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Light 2.
/// When the object is serialized out as xml, it's qualified name is a:lt2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lt2")]
pub struct Light2Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Light2ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Light2ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Accent 1.
/// When the object is serialized out as xml, it's qualified name is a:accent1.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:accent1")]
pub struct Accent1Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Accent1ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Accent1ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Accent 2.
/// When the object is serialized out as xml, it's qualified name is a:accent2.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:accent2")]
pub struct Accent2Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Accent2ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Accent2ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Accent 3.
/// When the object is serialized out as xml, it's qualified name is a:accent3.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:accent3")]
pub struct Accent3Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Accent3ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Accent3ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Accent 4.
/// When the object is serialized out as xml, it's qualified name is a:accent4.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:accent4")]
pub struct Accent4Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Accent4ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Accent4ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Accent 5.
/// When the object is serialized out as xml, it's qualified name is a:accent5.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:accent5")]
pub struct Accent5Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Accent5ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Accent5ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Accent 6.
/// When the object is serialized out as xml, it's qualified name is a:accent6.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:accent6")]
pub struct Accent6Color {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Accent6ColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Accent6ColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Hyperlink.
/// When the object is serialized out as xml, it's qualified name is a:hlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hlink")]
pub struct Hyperlink {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<HyperlinkChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HyperlinkChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Followed Hyperlink.
/// When the object is serialized out as xml, it's qualified name is a:folHlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:folHlink")]
pub struct FollowedHyperlinkColor {
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<FollowedHyperlinkColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FollowedHyperlinkColorChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Defines the Color2Type Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Color2Type {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:prstClr",
    )]
    pub children: Vec<Color2TypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Color2TypeChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Horizontal Ratio.
/// When the object is serialized out as xml, it's qualified name is a:sx.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sx")]
pub struct ScaleX {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Vertical Ratio.
/// When the object is serialized out as xml, it's qualified name is a:sy.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sy")]
pub struct ScaleY {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Defines the RatioType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RatioType {
    /// Numerator
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub numerator: i32,
    /// Denominator
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub denominator: i32,
}
/// Offset.
/// When the object is serialized out as xml, it's qualified name is a:off.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:off")]
pub struct Offset {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Child Offset.
/// When the object is serialized out as xml, it's qualified name is a:chOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:chOff")]
pub struct ChildOffset {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Defines the Point2DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Point2DType {
    /// X-Axis Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Axis Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
}
/// Extents.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct Extents {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i64,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i64,
}
/// Child Extents.
/// When the object is serialized out as xml, it's qualified name is a:chExt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:chExt")]
pub struct ChildExtents {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i64,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i64,
}
/// Defines the PositiveSize2DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PositiveSize2DType {
    /// Extent Length
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: i64,
    /// Extent Width
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: i64,
}
/// Shape Locks.
/// When the object is serialized out as xml, it's qualified name is a:spLocks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spLocks")]
pub struct ShapeLocks {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Disallow Shape Grouping
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grouping: Option<bool>,
    /// Disallow Shape Selection
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_selection: Option<bool>,
    /// Disallow Shape Rotation
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rotation: Option<bool>,
    /// Disallow Aspect Ratio Change
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// Disallow Shape Movement
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// Disallow Shape Resize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// Disallow Shape Point Editing
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// Disallow Showing Adjust Handles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// Disallow Arrowhead Changes
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// Disallow Shape Type Change
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// Disallow Shape Text Editing
    /// Represents the following attribute in the schema: :noTextEdit
    #[xml(attr = "noTextEdit")]
    pub no_text_edit: Option<bool>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Connection Shape Locks.
/// When the object is serialized out as xml, it's qualified name is a:cxnSpLocks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cxnSpLocks")]
pub struct ConnectionShapeLocks {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Disallow Shape Grouping
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grouping: Option<bool>,
    /// Disallow Shape Selection
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_selection: Option<bool>,
    /// Disallow Shape Rotation
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rotation: Option<bool>,
    /// Disallow Aspect Ratio Change
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// Disallow Shape Movement
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// Disallow Shape Resize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// Disallow Shape Point Editing
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// Disallow Showing Adjust Handles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// Disallow Arrowhead Changes
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// Disallow Shape Type Change
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// _
    #[xml(child = "a:extLst")]
    pub connector_locking_extension_list: Option<ConnectorLockingExtensionList>,
}
/// Connection Start.
/// When the object is serialized out as xml, it's qualified name is a:stCxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:stCxn")]
pub struct StartConnection {
    /// Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
}
/// Connection End.
/// When the object is serialized out as xml, it's qualified name is a:endCxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:endCxn")]
pub struct EndConnection {
    /// Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
}
/// Defines the ConnectionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ConnectionType {
    /// Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
}
/// Graphic Frame Locks.
/// When the object is serialized out as xml, it's qualified name is a:graphicFrameLocks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:graphicFrameLocks")]
pub struct GraphicFrameLocks {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Disallow Shape Grouping
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grouping: Option<bool>,
    /// Disallow Selection of Child Shapes
    /// Represents the following attribute in the schema: :noDrilldown
    #[xml(attr = "noDrilldown")]
    pub no_drilldown: Option<bool>,
    /// Disallow Shape Selection
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_selection: Option<bool>,
    /// Disallow Aspect Ratio Change
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// Disallow Shape Movement
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// Disallow Shape Resize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Graphic Object Data.
/// When the object is serialized out as xml, it's qualified name is a:graphicData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:graphicData")]
pub struct GraphicData {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Uniform Resource Identifier
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "a:graphic",
        child = "a:blip",
        child = "a:theme",
        child = "a:themeOverride",
        child = "a:themeManager",
        child = "a:tbl",
        child = "a:tblStyleLst",
        child = "aoe:oembedShared",
        child = "woe:oembed",
        child = "aif:imageFormula",
        child = "alf:liveFeedProps",
        child = "asl:scriptLink",
        child = "aclsh:classification",
        child = "ask:lineSketchStyleProps",
        child = "a1611:picAttrSrcUrl",
        child = "asvg:svgBlip",
        child = "adec:decorative",
        child = "a16:creationId",
        child = "a16:predDERef",
        child = "a16:cxnDERefs",
        child = "a16:rowId",
        child = "a16:colId",
        child = "ahyp:hlinkClr",
        child = "wp15:webVideoPr",
        child = "thm15:themeFamily",
        child = "a15:backgroundPr",
        child = "a15:nonVisualGroupProps",
        child = "a15:objectPr",
        child = "a15:signatureLine",
        child = "a14:cameraTool",
        child = "a14:compatExt",
        child = "a14:isCanvas",
        child = "a14:contentPart",
        child = "a14:shadowObscured",
        child = "a14:hiddenFill",
        child = "a14:hiddenLine",
        child = "a14:hiddenEffects",
        child = "a14:hiddenScene3d",
        child = "a14:hiddenSp3d",
        child = "a14:imgProps",
        child = "a14:useLocalDpi",
        child = "a14:m",
        child = "dgm14:cNvPr",
        child = "dgm14:recolorImg",
        child = "dsp:drawing",
        child = "dsp:dataModelExt",
        child = "dgm:colorsDef",
        child = "dgm:colorsDefHdr",
        child = "dgm:colorsDefHdrLst",
        child = "dgm:dataModel",
        child = "dgm:layoutDef",
        child = "dgm:layoutDefHdr",
        child = "dgm:layoutDefHdrLst",
        child = "dgm:relIds",
        child = "dgm:styleDef",
        child = "dgm:styleDefHdr",
        child = "dgm:styleDefHdrLst",
        child = "dgm1612:spPr",
        child = "dgm1612:lstStyle",
        child = "dgm1611:autoBuNodeInfoLst",
        child = "c:chartSpace",
        child = "c:userShapes",
        child = "c:chart",
        child = "c16r3:dataDisplayOptions16",
        child = "c16:spPr",
        child = "c16:explosion",
        child = "c16:invertIfNegative",
        child = "c16:bubble3D",
        child = "c16:marker",
        child = "c16:dLbl",
        child = "c16:categoryFilterExceptions",
        child = "c16:pivotOptions16",
        child = "c16:datapointuniqueidmap",
        child = "c16:uniqueId",
        child = "c15:pivotSource",
        child = "c15:numFmt",
        child = "c15:spPr",
        child = "c15:layout",
        child = "c15:fullRef",
        child = "c15:levelRef",
        child = "c15:formulaRef",
        child = "c15:filteredSeriesTitle",
        child = "c15:filteredCategoryTitle",
        child = "c15:filteredAreaSeries",
        child = "c15:filteredBarSeries",
        child = "c15:filteredBubbleSeries",
        child = "c15:filteredLineSeries",
        child = "c15:filteredPieSeries",
        child = "c15:filteredRadarSeries",
        child = "c15:filteredScatterSeries",
        child = "c15:filteredSurfaceSeries",
        child = "c15:datalabelsRange",
        child = "c15:categoryFilterExceptions",
        child = "c15:dlblFieldTable",
        child = "c15:xForSave",
        child = "c15:showDataLabelsRange",
        child = "c15:tx",
        child = "c15:showLeaderLines",
        child = "c15:leaderLines",
        child = "c15:autoCat",
        child = "c14:pivotOptions",
        child = "c14:sketchOptions",
        child = "c14:invertSolidFillFmt",
        child = "c14:style",
        child = "cdr14:contentPart",
        child = "comp:legacyDrawing",
        child = "lc:lockedCanvas",
        child = "wp:inline",
        child = "wp:anchor",
        child = "wp14:pctPosHOffset",
        child = "wp14:pctPosVOffset",
        child = "wp14:sizeRelH",
        child = "wp14:sizeRelV",
        child = "pic:pic",
        child = "pic14:style",
        child = "pic14:extLst",
        child = "xdr:wsDr",
        child = "xdr:contentPart",
        child = "xdr14:contentPart",
        child = "pc:cmAuthorMkLst",
        child = "pc:cmMkLst",
        child = "pc:tagMkLst",
        child = "pc:custShowMkLst",
        child = "pc:docMkLst",
        child = "pc:sectionMkLst",
        child = "pc:sldBaseMkLst",
        child = "pc:sldLayoutMkLst",
        child = "pc:sldMasterMkLst",
        child = "pc:sldMkLst",
        child = "pc:sldPosMkLst",
        child = "pc:notesMkLst",
        child = "pc:notesTxtMkLst",
        child = "pc:notesMasterMkLst",
        child = "pc:handoutMkLst",
        child = "pc:animEffectMkLst",
        child = "pc:animEffectParentMkLst",
        child = "pc:tkAppMkLst",
        child = "pc:tocMkLst",
        child = "pc:sectionLnkObjMkLst",
        child = "pc:designTagMkLst",
        child = "pc:cXmlMkLst",
        child = "p:cmAuthorLst",
        child = "p:cmLst",
        child = "p:oleObj",
        child = "p:presentation",
        child = "p:presentationPr",
        child = "p:sld",
        child = "p:sldLayout",
        child = "p:sldMaster",
        child = "p:handoutMaster",
        child = "p:notesMaster",
        child = "p:notes",
        child = "p:sldSyncPr",
        child = "p:tagLst",
        child = "p:viewPr",
        child = "p:contentPart",
        child = "p232:phTypeExt",
        child = "p188:authorLst",
        child = "p188:cmLst",
        child = "p188:commentRel",
        child = "p223:reactions",
        child = "p228:taskDetails",
        child = "p1912:taskHistoryDetails",
        child = "oac:txBodyPkg",
        child = "oac:grpCmd",
        child = "oac:imgData",
        child = "oac:origImgData",
        child = "oac:imgLink",
        child = "oac:dgMkLst",
        child = "oac:dcMkLst",
        child = "oac:graphicParentMkLst",
        child = "oac:deMkLst",
        child = "oac:deMasterMkLst",
        child = "oac:spMkLst",
        child = "oac:grpSpMkLst",
        child = "oac:graphicFrameMkLst",
        child = "oac:cxnSpMkLst",
        child = "oac:picMkLst",
        child = "oac:inkMkLst",
        child = "oac:txBodyMkLst",
        child = "oac:txMkLst",
        child = "oac:hlinkMkLst",
        child = "oac:model3DMkLst",
        child = "oac:viewSelLst",
        child = "oac:editorSelLst",
        child = "oac:drSelLst",
        child = "oac:tblMkLst",
        child = "oac:tcMkLst",
        child = "oac:trMkLst",
        child = "oac:gridColMkLst",
        child = "inkml:ink",
        child = "emma:one-of",
        child = "emma:group",
        child = "emma:sequence",
        child = "emma:endpoint",
        child = "emma:endpoint-info",
        child = "emma:info",
        child = "emma:grammar",
        child = "emma:derived-from",
        child = "emma:node",
        child = "emma:arc",
        child = "emma:lattice",
        child = "emma:literal",
        child = "emma:interpretation",
        child = "emma:group-info",
        child = "emma:derivation",
        child = "emma:model",
        child = "emma:emma",
        child = "msink:context",
        child = "p15:prstTrans",
        child = "p15:presenceInfo",
        child = "p15:threadingInfo",
        child = "p15:sldGuideLst",
        child = "p15:notesGuideLst",
        child = "p15:chartTrackingRefBased",
        child = "p14:nvContentPartPr",
        child = "p14:xfrm",
        child = "p14:extLst",
        child = "p14:media",
        child = "p14:vortex",
        child = "p14:switch",
        child = "p14:flip",
        child = "p14:ripple",
        child = "p14:honeycomb",
        child = "p14:prism",
        child = "p14:doors",
        child = "p14:window",
        child = "p14:ferris",
        child = "p14:gallery",
        child = "p14:conveyor",
        child = "p14:pan",
        child = "p14:glitter",
        child = "p14:warp",
        child = "p14:flythrough",
        child = "p14:flash",
        child = "p14:shred",
        child = "p14:reveal",
        child = "p14:wheelReverse",
        child = "p14:bmkTgt",
        child = "p14:sectionPr",
        child = "p14:sectionLst",
        child = "p14:browseMode",
        child = "p14:laserClr",
        child = "p14:defaultImageDpi",
        child = "p14:discardImageEditData",
        child = "p14:showMediaCtrls",
        child = "p14:laserTraceLst",
        child = "p14:creationId",
        child = "p14:modId",
        child = "p14:showEvtLst",
        child = "sl:schemaLibrary",
        child = "m:mathPr",
        child = "m:oMathPara",
        child = "m:oMath",
        child = "w:recipients",
        child = "w:txbxContent",
        child = "w:comments",
        child = "w:footnotes",
        child = "w:endnotes",
        child = "w:hdr",
        child = "w:ftr",
        child = "w:settings",
        child = "w:webSettings",
        child = "w:fonts",
        child = "w:numbering",
        child = "w:styles",
        child = "w:document",
        child = "w:glossaryDocument",
        child = "w15:color",
        child = "w15:dataBinding",
        child = "w15:appearance",
        child = "w15:commentsEx",
        child = "w15:people",
        child = "w15:repeatingSection",
        child = "w15:repeatingSectionItem",
        child = "w15:chartTrackingRefBased",
        child = "w15:collapsed",
        child = "w15:docId",
        child = "w15:footnoteColumns",
        child = "w15:webExtensionLinked",
        child = "w15:webExtensionCreated",
        child = "w14:contentPart",
        child = "w14:docId",
        child = "w14:conflictMode",
        child = "w14:customXmlConflictInsRangeStart",
        child = "w14:customXmlConflictInsRangeEnd",
        child = "w14:customXmlConflictDelRangeStart",
        child = "w14:customXmlConflictDelRangeEnd",
        child = "w14:discardImageEditingData",
        child = "w14:defaultImageDpi",
        child = "w14:entityPicker",
        child = "w14:checkbox",
        child = "v:shape",
        child = "v:shapetype",
        child = "v:group",
        child = "v:background",
        child = "v:fill",
        child = "v:formulas",
        child = "v:handles",
        child = "v:imagedata",
        child = "v:path",
        child = "v:textbox",
        child = "v:shadow",
        child = "v:stroke",
        child = "v:textpath",
        child = "v:arc",
        child = "v:curve",
        child = "v:image",
        child = "v:line",
        child = "v:oval",
        child = "v:polyline",
        child = "v:rect",
        child = "v:roundrect",
        child = "o:shapedefaults",
        child = "o:shapelayout",
        child = "o:signatureline",
        child = "o:ink",
        child = "o:diagram",
        child = "o:skew",
        child = "o:extrusion",
        child = "o:callout",
        child = "o:lock",
        child = "o:OLEObject",
        child = "o:complex",
        child = "o:left",
        child = "o:top",
        child = "o:right",
        child = "o:bottom",
        child = "o:column",
        child = "o:clippath",
        child = "o:fill",
        child = "w10:bordertop",
        child = "w10:borderleft",
        child = "w10:borderright",
        child = "w10:borderbottom",
        child = "w10:wrap",
        child = "w10:anchorlock",
        child = "xvml:ClientData",
        child = "pvml:iscomment",
        child = "pvml:textdata",
        child = "wpc:wpc",
        child = "wpg:wgp",
        child = "wps:wsp",
        child = "sle:slicer",
        child = "cs:colorStyle",
        child = "cs:chartStyle",
        child = "we:webextension",
        child = "we:webextensionref",
        child = "tsle:timeslicer",
    )]
    pub children: Vec<GraphicDataChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GraphicDataChildChoice {
    #[xml(tag = "a:graphic")]
    AGraphic(Graphic),
    #[xml(tag = "a:blip")]
    ABlip(Blip),
    #[xml(tag = "a:theme")]
    ATheme(Theme),
    #[xml(tag = "a:themeOverride")]
    AThemeOverride(ThemeOverride),
    #[xml(tag = "a:themeManager")]
    AThemeManager(ThemeManager),
    #[xml(tag = "a:tbl")]
    ATbl(Table),
    #[xml(tag = "a:tblStyleLst")]
    ATblStyleLst(TableStyleList),
    #[xml(tag = "aoe:oembedShared")]
    AoeOembedShared(
        crate::schemas::schemas_microsoft_com_office_drawing_2021_oembed::OEmbedShared,
    ),
    #[xml(tag = "woe:oembed")]
    WoeOembed(crate::schemas::schemas_microsoft_com_office_word_2020_oembed::OEmbed),
    #[xml(tag = "aif:imageFormula")]
    AifImageFormula(
        crate::schemas::schemas_microsoft_com_office_drawing_2022_imageformula::ImageFormula,
    ),
    #[xml(tag = "alf:liveFeedProps")]
    AlfLiveFeedProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2021_livefeed::LiveFeedProperties,
    ),
    #[xml(tag = "asl:scriptLink")]
    AslScriptLink(
        crate::schemas::schemas_microsoft_com_office_drawing_2021_scriptlink::ScriptLink,
    ),
    #[xml(tag = "aclsh:classification")]
    AclshClassification(
        crate::schemas::schemas_microsoft_com_office_drawing_2020_classification_shape::ClassificationOutcome,
    ),
    #[xml(tag = "ask:lineSketchStyleProps")]
    AskLineSketchStyleProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2018_sketchyshapes::LineSketchStyleProperties,
    ),
    #[xml(tag = "a1611:picAttrSrcUrl")]
    A1611PicAttrSrcUrl(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_11_main::PictureAttributionSourceUrl,
    ),
    #[xml(tag = "asvg:svgBlip")]
    AsvgSvgBlip(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_svg_main::SvgBlip,
    ),
    #[xml(tag = "adec:decorative")]
    AdecDecorative(
        crate::schemas::schemas_microsoft_com_office_drawing_2017_decorative::Decorative,
    ),
    #[xml(tag = "a16:creationId")]
    A16CreationId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::CreationId,
    ),
    #[xml(tag = "a16:predDERef")]
    A16PredDeRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::PredecessorDrawingElementReference,
    ),
    #[xml(tag = "a16:cxnDERefs")]
    A16CxnDeRefs(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::ConnectableReferences,
    ),
    #[xml(tag = "a16:rowId")]
    A16RowId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::RowIdIdentifier,
    ),
    #[xml(tag = "a16:colId")]
    A16ColId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::ColIdIdentifier,
    ),
    #[xml(tag = "ahyp:hlinkClr")]
    AhypHlinkClr(
        crate::schemas::schemas_microsoft_com_office_drawing_2018_hyperlinkcolor::HyperlinkColor,
    ),
    #[xml(tag = "wp15:webVideoPr")]
    Wp15WebVideoPr(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordprocessing_drawing::WebVideoProperty,
    ),
    #[xml(tag = "thm15:themeFamily")]
    Thm15ThemeFamily(
        crate::schemas::schemas_microsoft_com_office_thememl_2012_main::ThemeFamily,
    ),
    #[xml(tag = "a15:backgroundPr")]
    A15BackgroundPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::BackgroundProperties,
    ),
    #[xml(tag = "a15:nonVisualGroupProps")]
    A15NonVisualGroupProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::NonVisualGroupProperties,
    ),
    #[xml(tag = "a15:objectPr")]
    A15ObjectPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::ObjectProperties,
    ),
    #[xml(tag = "a15:signatureLine")]
    A15SignatureLine(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::SignatureLine,
    ),
    #[xml(tag = "a14:cameraTool")]
    A14CameraTool(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::CameraTool,
    ),
    #[xml(tag = "a14:compatExt")]
    A14CompatExt(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::CompatExtension,
    ),
    #[xml(tag = "a14:isCanvas")]
    A14IsCanvas(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::IsCanvas,
    ),
    #[xml(tag = "a14:contentPart")]
    A14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::GvmlContentPart,
    ),
    #[xml(tag = "a14:shadowObscured")]
    A14ShadowObscured(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ShadowObscured,
    ),
    #[xml(tag = "a14:hiddenFill")]
    A14HiddenFill(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenFillProperties,
    ),
    #[xml(tag = "a14:hiddenLine")]
    A14HiddenLine(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenLineProperties,
    ),
    #[xml(tag = "a14:hiddenEffects")]
    A14HiddenEffects(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenEffectsProperties,
    ),
    #[xml(tag = "a14:hiddenScene3d")]
    A14HiddenScene3d(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenScene3D,
    ),
    #[xml(tag = "a14:hiddenSp3d")]
    A14HiddenSp3d(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenShape3D,
    ),
    #[xml(tag = "a14:imgProps")]
    A14ImgProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ImageProperties,
    ),
    #[xml(tag = "a14:useLocalDpi")]
    A14UseLocalDpi(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::UseLocalDpi,
    ),
    #[xml(tag = "a14:m")]
    A14M(crate::schemas::schemas_microsoft_com_office_drawing_2010_main::TextMath),
    #[xml(tag = "dgm14:cNvPr")]
    Dgm14CNvPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_diagram::NonVisualDrawingProperties,
    ),
    #[xml(tag = "dgm14:recolorImg")]
    Dgm14RecolorImg(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_diagram::RecolorImages,
    ),
    #[xml(tag = "dsp:drawing")]
    DspDrawing(
        crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing,
    ),
    #[xml(tag = "dsp:dataModelExt")]
    DspDataModelExt(
        crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::DataModelExtensionBlock,
    ),
    #[xml(tag = "dgm:colorsDef")]
    DgmColorsDef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition,
    ),
    #[xml(tag = "dgm:colorsDefHdr")]
    DgmColorsDefHdr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinitionHeader,
    ),
    #[xml(tag = "dgm:colorsDefHdrLst")]
    DgmColorsDefHdrLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinitionHeaderList,
    ),
    #[xml(tag = "dgm:dataModel")]
    DgmDataModel(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
    ),
    #[xml(tag = "dgm:layoutDef")]
    DgmLayoutDef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition,
    ),
    #[xml(tag = "dgm:layoutDefHdr")]
    DgmLayoutDefHdr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinitionHeader,
    ),
    #[xml(tag = "dgm:layoutDefHdrLst")]
    DgmLayoutDefHdrLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinitionHeaderList,
    ),
    #[xml(tag = "dgm:relIds")]
    DgmRelIds(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::RelationshipIds,
    ),
    #[xml(tag = "dgm:styleDef")]
    DgmStyleDef(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition,
    ),
    #[xml(tag = "dgm:styleDefHdr")]
    DgmStyleDefHdr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinitionHeader,
    ),
    #[xml(tag = "dgm:styleDefHdrLst")]
    DgmStyleDefHdrLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinitionHeaderList,
    ),
    #[xml(tag = "dgm1612:spPr")]
    Dgm1612SpPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_12_diagram::ShapeProperties,
    ),
    #[xml(tag = "dgm1612:lstStyle")]
    Dgm1612LstStyle(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_12_diagram::TextListStyleType,
    ),
    #[xml(tag = "dgm1611:autoBuNodeInfoLst")]
    Dgm1611AutoBuNodeInfoLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_11_diagram::NumberDiagramInfoList,
    ),
    #[xml(tag = "c:chartSpace")]
    CChartSpace(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
    ),
    #[xml(tag = "c:userShapes")]
    CUserShapes(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
    ),
    #[xml(tag = "c:chart")]
    CChart(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartReference,
    ),
    #[xml(tag = "c16r3:dataDisplayOptions16")]
    C16r3DataDisplayOptions16(
        crate::schemas::schemas_microsoft_com_office_drawing_2017_03_chart::DataDisplayOptions16,
    ),
    #[xml(tag = "c16:spPr")]
    C16SpPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ShapeProperties,
    ),
    #[xml(tag = "c16:explosion")]
    C16Explosion(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UnsignedIntegerType,
    ),
    #[xml(tag = "c16:invertIfNegative")]
    C16InvertIfNegative(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::InvertIfNegativeBoolean,
    ),
    #[xml(tag = "c16:bubble3D")]
    C16Bubble3D(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::Bubble3DBoolean,
    ),
    #[xml(tag = "c16:marker")]
    C16Marker(crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::Marker),
    #[xml(tag = "c16:dLbl")]
    C16DLbl(crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::DLbl),
    #[xml(tag = "c16:categoryFilterExceptions")]
    C16CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c16:pivotOptions16")]
    C16PivotOptions16(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::PivotOptions16,
    ),
    #[xml(tag = "c16:datapointuniqueidmap")]
    C16Datapointuniqueidmap(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::ChartDataPointUniqueIdMap,
    ),
    #[xml(tag = "c16:uniqueId")]
    C16UniqueId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_chart::UniqueIdChartUniqueId,
    ),
    #[xml(tag = "c15:pivotSource")]
    C15PivotSource(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::PivotSource,
    ),
    #[xml(tag = "c15:numFmt")]
    C15NumFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::NumberingFormat,
    ),
    #[xml(tag = "c15:spPr")]
    C15SpPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShapeProperties,
    ),
    #[xml(tag = "c15:layout")]
    C15Layout(crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::Layout),
    #[xml(tag = "c15:fullRef")]
    C15FullRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FullReference,
    ),
    #[xml(tag = "c15:levelRef")]
    C15LevelRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LevelReference,
    ),
    #[xml(tag = "c15:formulaRef")]
    C15FormulaRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FormulaReference,
    ),
    #[xml(tag = "c15:filteredSeriesTitle")]
    C15FilteredSeriesTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSeriesTitle,
    ),
    #[xml(tag = "c15:filteredCategoryTitle")]
    C15FilteredCategoryTitle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredCategoryTitle,
    ),
    #[xml(tag = "c15:filteredAreaSeries")]
    C15FilteredAreaSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredAreaSeries,
    ),
    #[xml(tag = "c15:filteredBarSeries")]
    C15FilteredBarSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBarSeries,
    ),
    #[xml(tag = "c15:filteredBubbleSeries")]
    C15FilteredBubbleSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredBubbleSeries,
    ),
    #[xml(tag = "c15:filteredLineSeries")]
    C15FilteredLineSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredLineSeriesExtension,
    ),
    #[xml(tag = "c15:filteredPieSeries")]
    C15FilteredPieSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredPieSeries,
    ),
    #[xml(tag = "c15:filteredRadarSeries")]
    C15FilteredRadarSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredRadarSeries,
    ),
    #[xml(tag = "c15:filteredScatterSeries")]
    C15FilteredScatterSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredScatterSeries,
    ),
    #[xml(tag = "c15:filteredSurfaceSeries")]
    C15FilteredSurfaceSeries(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::FilteredSurfaceSeries,
    ),
    #[xml(tag = "c15:datalabelsRange")]
    C15DatalabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelsRange,
    ),
    #[xml(tag = "c15:categoryFilterExceptions")]
    C15CategoryFilterExceptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::CategoryFilterExceptions,
    ),
    #[xml(tag = "c15:dlblFieldTable")]
    C15DlblFieldTable(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::DataLabelFieldTable,
    ),
    #[xml(tag = "c15:xForSave")]
    C15XForSave(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ExceptionForSave,
    ),
    #[xml(tag = "c15:showDataLabelsRange")]
    C15ShowDataLabelsRange(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowDataLabelsRange,
    ),
    #[xml(tag = "c15:tx")]
    C15Tx(crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ChartText),
    #[xml(tag = "c15:showLeaderLines")]
    C15ShowLeaderLines(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::ShowLeaderLines,
    ),
    #[xml(tag = "c15:leaderLines")]
    C15LeaderLines(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::LeaderLines,
    ),
    #[xml(tag = "c15:autoCat")]
    C15AutoCat(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart::AutoGeneneratedCategories,
    ),
    #[xml(tag = "c14:pivotOptions")]
    C14PivotOptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::PivotOptions,
    ),
    #[xml(tag = "c14:sketchOptions")]
    C14SketchOptions(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::SketchOptions,
    ),
    #[xml(tag = "c14:invertSolidFillFmt")]
    C14InvertSolidFillFmt(
        crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::InvertSolidFillFormat,
    ),
    #[xml(tag = "c14:style")]
    C14Style(crate::schemas::schemas_microsoft_com_office_drawing_2007_8_2_chart::Style),
    #[xml(tag = "cdr14:contentPart")]
    Cdr14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_chart_drawing::ContentPart,
    ),
    #[xml(tag = "comp:legacyDrawing")]
    CompLegacyDrawing(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_compatibility::LegacyDrawing,
    ),
    #[xml(tag = "lc:lockedCanvas")]
    LcLockedCanvas(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_locked_canvas::LockedCanvas,
    ),
    #[xml(tag = "wp:inline")]
    WpInline(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing::Inline,
    ),
    #[xml(tag = "wp:anchor")]
    WpAnchor(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing::Anchor,
    ),
    #[xml(tag = "wp14:pctPosHOffset")]
    Wp14PctPosHOffset(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::PercentagePositionHeightOffset,
    ),
    #[xml(tag = "wp14:pctPosVOffset")]
    Wp14PctPosVOffset(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::PercentagePositionVerticalOffset,
    ),
    #[xml(tag = "wp14:sizeRelH")]
    Wp14SizeRelH(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeWidth,
    ),
    #[xml(tag = "wp14:sizeRelV")]
    Wp14SizeRelV(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_drawing::RelativeHeight,
    ),
    #[xml(tag = "pic:pic")]
    PicPic(crate::schemas::schemas_openxmlformats_org_drawingml_2006_picture::Picture),
    #[xml(tag = "pic14:style")]
    Pic14Style(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_picture::ShapeStyle,
    ),
    #[xml(tag = "pic14:extLst")]
    Pic14ExtLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_picture::OfficeArtExtensionList,
    ),
    #[xml(tag = "xdr:wsDr")]
    XdrWsDr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
    ),
    #[xml(tag = "xdr:contentPart")]
    XdrContentPart(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::ContentPart,
    ),
    #[xml(tag = "xdr14:contentPart")]
    Xdr14ContentPart(
        crate::schemas::schemas_microsoft_com_office_excel_2010_spreadsheet_drawing::ContentPart,
    ),
    #[xml(tag = "pc:cmAuthorMkLst")]
    PcCmAuthorMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::CommentAuthorMonikerList,
    ),
    #[xml(tag = "pc:cmMkLst")]
    PcCmMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::CommentMonikerList,
    ),
    #[xml(tag = "pc:tagMkLst")]
    PcTagMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::StringTagMonikerList,
    ),
    #[xml(tag = "pc:custShowMkLst")]
    PcCustShowMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::CustomShowMonikerList,
    ),
    #[xml(tag = "pc:docMkLst")]
    PcDocMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::DocumentMonikerList,
    ),
    #[xml(tag = "pc:sectionMkLst")]
    PcSectionMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SectionMonikerList,
    ),
    #[xml(tag = "pc:sldBaseMkLst")]
    PcSldBaseMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideBaseMonikerList,
    ),
    #[xml(tag = "pc:sldLayoutMkLst")]
    PcSldLayoutMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideLayoutMonikerList,
    ),
    #[xml(tag = "pc:sldMasterMkLst")]
    PcSldMasterMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::MainMasterMonikerList,
    ),
    #[xml(tag = "pc:sldMkLst")]
    PcSldMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlideMonikerList,
    ),
    #[xml(tag = "pc:sldPosMkLst")]
    PcSldPosMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SlidePosMonikerList,
    ),
    #[xml(tag = "pc:notesMkLst")]
    PcNotesMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::NotesMonikerList,
    ),
    #[xml(tag = "pc:notesTxtMkLst")]
    PcNotesTxtMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::NotesTextMonikerList,
    ),
    #[xml(tag = "pc:notesMasterMkLst")]
    PcNotesMasterMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::NotesMasterMonikerList,
    ),
    #[xml(tag = "pc:handoutMkLst")]
    PcHandoutMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::HandoutMonikerList,
    ),
    #[xml(tag = "pc:animEffectMkLst")]
    PcAnimEffectMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::AnimEffectMkLstAnimationEffectMonikerList,
    ),
    #[xml(tag = "pc:animEffectParentMkLst")]
    PcAnimEffectParentMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::AnimEffectParentMkLstAnimationEffectMonikerList,
    ),
    #[xml(tag = "pc:tkAppMkLst")]
    PcTkAppMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::OsfTaskPaneAppMonikerList,
    ),
    #[xml(tag = "pc:tocMkLst")]
    PcTocMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SummaryZoomMonikerList,
    ),
    #[xml(tag = "pc:sectionLnkObjMkLst")]
    PcSectionLnkObjMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::SectionLinkObjMonikerList,
    ),
    #[xml(tag = "pc:designTagMkLst")]
    PcDesignTagMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::DesignerTagMonikerList,
    ),
    #[xml(tag = "pc:cXmlMkLst")]
    PcCXmlMkLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2013_main_command::CustomXmlPartMonikerList,
    ),
    #[xml(tag = "p:cmAuthorLst")]
    PCmAuthorLst(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList,
    ),
    #[xml(tag = "p:cmLst")]
    PCmLst(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList,
    ),
    #[xml(tag = "p:oleObj")]
    POleObj(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::OleObject,
    ),
    #[xml(tag = "p:presentation")]
    PPresentation(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
    ),
    #[xml(tag = "p:presentationPr")]
    PPresentationPr(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties,
    ),
    #[xml(tag = "p:sld")]
    PSld(crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide),
    #[xml(tag = "p:sldLayout")]
    PSldLayout(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout,
    ),
    #[xml(tag = "p:sldMaster")]
    PSldMaster(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster,
    ),
    #[xml(tag = "p:handoutMaster")]
    PHandoutMaster(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster,
    ),
    #[xml(tag = "p:notesMaster")]
    PNotesMaster(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster,
    ),
    #[xml(tag = "p:notes")]
    PNotes(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide,
    ),
    #[xml(tag = "p:sldSyncPr")]
    PSldSyncPr(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties,
    ),
    #[xml(tag = "p:tagLst")]
    PTagLst(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList,
    ),
    #[xml(tag = "p:viewPr")]
    PViewPr(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
    ),
    #[xml(tag = "p:contentPart")]
    PContentPart(
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ContentPart,
    ),
    #[xml(tag = "p232:phTypeExt")]
    P232PhTypeExt(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2023_02_main::PlaceholderTypeExtension,
    ),
    #[xml(tag = "p188:authorLst")]
    P188AuthorLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList,
    ),
    #[xml(tag = "p188:cmLst")]
    P188CmLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList,
    ),
    #[xml(tag = "p188:commentRel")]
    P188CommentRel(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentRelationship,
    ),
    #[xml(tag = "p223:reactions")]
    P223Reactions(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2022_03_main::Reactions,
    ),
    #[xml(tag = "p228:taskDetails")]
    P228TaskDetails(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2022_08_main::TaskDetails,
    ),
    #[xml(tag = "p1912:taskHistoryDetails")]
    P1912TaskHistoryDetails(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2019_12_main::TaskHistoryDetails,
    ),
    #[xml(tag = "oac:txBodyPkg")]
    OacTxBodyPkg(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextBodyPackage,
    ),
    #[xml(tag = "oac:grpCmd")]
    OacGrpCmd(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::GroupCommand,
    ),
    #[xml(tag = "oac:imgData")]
    OacImgData(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::ImgDataImgData,
    ),
    #[xml(tag = "oac:origImgData")]
    OacOrigImgData(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::OrigImgDataImgData,
    ),
    #[xml(tag = "oac:imgLink")]
    OacImgLink(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::ImgLink,
    ),
    #[xml(tag = "oac:dgMkLst")]
    OacDgMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DrawingMonikerList,
    ),
    #[xml(tag = "oac:dcMkLst")]
    OacDcMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DocumentContextMonikerList,
    ),
    #[xml(tag = "oac:graphicParentMkLst")]
    OacGraphicParentMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::GraphicParentMonikerList,
    ),
    #[xml(tag = "oac:deMkLst")]
    OacDeMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DeMkLstDrawingElementMonikerList,
    ),
    #[xml(tag = "oac:deMasterMkLst")]
    OacDeMasterMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DeMasterMkLstDrawingElementMonikerList,
    ),
    #[xml(tag = "oac:spMkLst")]
    OacSpMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::ShapeMonikerList,
    ),
    #[xml(tag = "oac:grpSpMkLst")]
    OacGrpSpMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::GroupShapeMonikerList,
    ),
    #[xml(tag = "oac:graphicFrameMkLst")]
    OacGraphicFrameMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::GraphicFrameMonikerList,
    ),
    #[xml(tag = "oac:cxnSpMkLst")]
    OacCxnSpMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::ConnectorMonikerList,
    ),
    #[xml(tag = "oac:picMkLst")]
    OacPicMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::PictureMonikerList,
    ),
    #[xml(tag = "oac:inkMkLst")]
    OacInkMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::InkMonikerList,
    ),
    #[xml(tag = "oac:txBodyMkLst")]
    OacTxBodyMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextBodyMonikerList,
    ),
    #[xml(tag = "oac:txMkLst")]
    OacTxMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TextCharRangeMonikerList,
    ),
    #[xml(tag = "oac:hlinkMkLst")]
    OacHlinkMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::HyperlinkMonikerList,
    ),
    #[xml(tag = "oac:model3DMkLst")]
    OacModel3DMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::Model3DMonikerList,
    ),
    #[xml(tag = "oac:viewSelLst")]
    OacViewSelLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::ViewSelectionStgList,
    ),
    #[xml(tag = "oac:editorSelLst")]
    OacEditorSelLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::EditorSelectionStgList,
    ),
    #[xml(tag = "oac:drSelLst")]
    OacDrSelLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::DrawingSelectionStgList,
    ),
    #[xml(tag = "oac:tblMkLst")]
    OacTblMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableMonikerList,
    ),
    #[xml(tag = "oac:tcMkLst")]
    OacTcMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableCellMonikerList,
    ),
    #[xml(tag = "oac:trMkLst")]
    OacTrMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableRowMonikerList,
    ),
    #[xml(tag = "oac:gridColMkLst")]
    OacGridColMkLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2013_main_command::TableColumnMonikerList,
    ),
    #[xml(tag = "inkml:ink")]
    InkmlInk(crate::schemas::www_w3_org_2003_ink_ml::Ink),
    #[xml(tag = "emma:one-of")]
    EmmaOneOf(crate::schemas::www_w3_org_2003_04_emma::OneOf),
    #[xml(tag = "emma:group")]
    EmmaGroup(crate::schemas::www_w3_org_2003_04_emma::Group),
    #[xml(tag = "emma:sequence")]
    EmmaSequence(crate::schemas::www_w3_org_2003_04_emma::Sequence),
    #[xml(tag = "emma:endpoint")]
    EmmaEndpoint(crate::schemas::www_w3_org_2003_04_emma::EndPoint),
    #[xml(tag = "emma:endpoint-info")]
    EmmaEndpointInfo(crate::schemas::www_w3_org_2003_04_emma::EndPointInfo),
    #[xml(tag = "emma:info")]
    EmmaInfo(crate::schemas::www_w3_org_2003_04_emma::Info),
    #[xml(tag = "emma:grammar")]
    EmmaGrammar(crate::schemas::www_w3_org_2003_04_emma::Grammar),
    #[xml(tag = "emma:derived-from")]
    EmmaDerivedFrom(crate::schemas::www_w3_org_2003_04_emma::DerivedFrom),
    #[xml(tag = "emma:node")]
    EmmaNode(crate::schemas::www_w3_org_2003_04_emma::Node),
    #[xml(tag = "emma:arc")]
    EmmaArc(crate::schemas::www_w3_org_2003_04_emma::Arc),
    #[xml(tag = "emma:lattice")]
    EmmaLattice(crate::schemas::www_w3_org_2003_04_emma::Lattice),
    #[xml(tag = "emma:literal")]
    EmmaLiteral(crate::schemas::www_w3_org_2003_04_emma::Literal),
    #[xml(tag = "emma:interpretation")]
    EmmaInterpretation(crate::schemas::www_w3_org_2003_04_emma::Interpretation),
    #[xml(tag = "emma:group-info")]
    EmmaGroupInfo(crate::schemas::www_w3_org_2003_04_emma::GroupInfo),
    #[xml(tag = "emma:derivation")]
    EmmaDerivation(crate::schemas::www_w3_org_2003_04_emma::Derivation),
    #[xml(tag = "emma:model")]
    EmmaModel(crate::schemas::www_w3_org_2003_04_emma::Model),
    #[xml(tag = "emma:emma")]
    EmmaEmma(crate::schemas::www_w3_org_2003_04_emma::Emma),
    #[xml(tag = "msink:context")]
    MsinkContext(crate::schemas::schemas_microsoft_com_ink_2010_main::ContextNode),
    #[xml(tag = "p15:prstTrans")]
    P15PrstTrans(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::PresetTransition,
    ),
    #[xml(tag = "p15:presenceInfo")]
    P15PresenceInfo(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::PresenceInfo,
    ),
    #[xml(tag = "p15:threadingInfo")]
    P15ThreadingInfo(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::ThreadingInfo,
    ),
    #[xml(tag = "p15:sldGuideLst")]
    P15SldGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::SlideGuideList,
    ),
    #[xml(tag = "p15:notesGuideLst")]
    P15NotesGuideLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::NotesGuideList,
    ),
    #[xml(tag = "p15:chartTrackingRefBased")]
    P15ChartTrackingRefBased(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2012_main::ChartTrackingReferenceBased,
    ),
    #[xml(tag = "p14:nvContentPartPr")]
    P14NvContentPartPr(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::NonVisualContentPartProperties,
    ),
    #[xml(tag = "p14:xfrm")]
    P14Xfrm(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Transform2D,
    ),
    #[xml(tag = "p14:extLst")]
    P14ExtLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ExtensionListModify,
    ),
    #[xml(tag = "p14:media")]
    P14Media(crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::Media),
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
    #[xml(tag = "p14:ferris")]
    P14Ferris(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FerrisTransition,
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
    #[xml(tag = "p14:glitter")]
    P14Glitter(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::GlitterTransition,
    ),
    #[xml(tag = "p14:warp")]
    P14Warp(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WarpTransition,
    ),
    #[xml(tag = "p14:flythrough")]
    P14Flythrough(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlythroughTransition,
    ),
    #[xml(tag = "p14:flash")]
    P14Flash(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::FlashTransition,
    ),
    #[xml(tag = "p14:shred")]
    P14Shred(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShredTransition,
    ),
    #[xml(tag = "p14:reveal")]
    P14Reveal(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::RevealTransition,
    ),
    #[xml(tag = "p14:wheelReverse")]
    P14WheelReverse(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::WheelReverseTransition,
    ),
    #[xml(tag = "p14:bmkTgt")]
    P14BmkTgt(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::BookmarkTarget,
    ),
    #[xml(tag = "p14:sectionPr")]
    P14SectionPr(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SectionProperties,
    ),
    #[xml(tag = "p14:sectionLst")]
    P14SectionLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::SectionList,
    ),
    #[xml(tag = "p14:browseMode")]
    P14BrowseMode(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::BrowseMode,
    ),
    #[xml(tag = "p14:laserClr")]
    P14LaserClr(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::LaserColor,
    ),
    #[xml(tag = "p14:defaultImageDpi")]
    P14DefaultImageDpi(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DefaultImageDpi,
    ),
    #[xml(tag = "p14:discardImageEditData")]
    P14DiscardImageEditData(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::DiscardImageEditData,
    ),
    #[xml(tag = "p14:showMediaCtrls")]
    P14ShowMediaCtrls(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShowMediaControls,
    ),
    #[xml(tag = "p14:laserTraceLst")]
    P14LaserTraceLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::LaserTraceList,
    ),
    #[xml(tag = "p14:creationId")]
    P14CreationId(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::CreationId,
    ),
    #[xml(tag = "p14:modId")]
    P14ModId(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ModificationId,
    ),
    #[xml(tag = "p14:showEvtLst")]
    P14ShowEvtLst(
        crate::schemas::schemas_microsoft_com_office_powerpoint_2010_main::ShowEventRecordList,
    ),
    #[xml(tag = "sl:schemaLibrary")]
    SlSchemaLibrary(
        crate::schemas::schemas_openxmlformats_org_schema_library_2006_main::SchemaLibrary,
    ),
    #[xml(tag = "m:mathPr")]
    MMathPr(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::MathProperties,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::Paragraph,
    ),
    #[xml(tag = "m:oMath")]
    MOMath(
        crate::schemas::schemas_openxmlformats_org_office_document_2006_math::OfficeMath,
    ),
    #[xml(tag = "w:recipients")]
    WRecipients(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Recipients,
    ),
    #[xml(tag = "w:txbxContent")]
    WTxbxContent(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TextBoxContent,
    ),
    #[xml(tag = "w:comments")]
    WComments(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments,
    ),
    #[xml(tag = "w:footnotes")]
    WFootnotes(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes,
    ),
    #[xml(tag = "w:endnotes")]
    WEndnotes(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes,
    ),
    #[xml(tag = "w:hdr")]
    WHdr(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header),
    #[xml(tag = "w:ftr")]
    WFtr(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer),
    #[xml(tag = "w:settings")]
    WSettings(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
    ),
    #[xml(tag = "w:webSettings")]
    WWebSettings(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings,
    ),
    #[xml(tag = "w:fonts")]
    WFonts(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts),
    #[xml(tag = "w:numbering")]
    WNumbering(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering,
    ),
    #[xml(tag = "w:styles")]
    WStyles(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
    ),
    #[xml(tag = "w:document")]
    WDocument(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
    ),
    #[xml(tag = "w:glossaryDocument")]
    WGlossaryDocument(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
    ),
    #[xml(tag = "w15:color")]
    W15Color(crate::schemas::schemas_microsoft_com_office_word_2012_wordml::Color),
    #[xml(tag = "w15:dataBinding")]
    W15DataBinding(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::DataBinding,
    ),
    #[xml(tag = "w15:appearance")]
    W15Appearance(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::Appearance,
    ),
    #[xml(tag = "w15:commentsEx")]
    W15CommentsEx(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx,
    ),
    #[xml(tag = "w15:people")]
    W15People(crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People),
    #[xml(tag = "w15:repeatingSection")]
    W15RepeatingSection(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::SdtRepeatedSection,
    ),
    #[xml(tag = "w15:repeatingSectionItem")]
    W15RepeatingSectionItem(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::SdtRepeatedSectionItem,
    ),
    #[xml(tag = "w15:chartTrackingRefBased")]
    W15ChartTrackingRefBased(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::ChartTrackingRefBased,
    ),
    #[xml(tag = "w15:collapsed")]
    W15Collapsed(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::DefaultCollapsed,
    ),
    #[xml(tag = "w15:docId")]
    W15DocId(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::PersistentDocumentId,
    ),
    #[xml(tag = "w15:footnoteColumns")]
    W15FootnoteColumns(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::FootnoteColumns,
    ),
    #[xml(tag = "w15:webExtensionLinked")]
    W15WebExtensionLinked(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::WebExtensionLinked,
    ),
    #[xml(tag = "w15:webExtensionCreated")]
    W15WebExtensionCreated(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordml::WebExtensionCreated,
    ),
    #[xml(tag = "w14:contentPart")]
    W14ContentPart(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ContentPart,
    ),
    #[xml(tag = "w14:docId")]
    W14DocId(crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DocumentId),
    #[xml(tag = "w14:conflictMode")]
    W14ConflictMode(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::ConflictMode,
    ),
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
    #[xml(tag = "w14:discardImageEditingData")]
    W14DiscardImageEditingData(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DiscardImageEditingData,
    ),
    #[xml(tag = "w14:defaultImageDpi")]
    W14DefaultImageDpi(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::DefaultImageDpi,
    ),
    #[xml(tag = "w14:entityPicker")]
    W14EntityPicker(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::EntityPickerEmpty,
    ),
    #[xml(tag = "w14:checkbox")]
    W14Checkbox(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::SdtContentCheckBox,
    ),
    #[xml(tag = "v:shape")]
    VShape(crate::schemas::schemas_microsoft_com_vml::Shape),
    #[xml(tag = "v:shapetype")]
    VShapetype(crate::schemas::schemas_microsoft_com_vml::Shapetype),
    #[xml(tag = "v:group")]
    VGroup(crate::schemas::schemas_microsoft_com_vml::Group),
    #[xml(tag = "v:background")]
    VBackground(crate::schemas::schemas_microsoft_com_vml::Background),
    #[xml(tag = "v:fill")]
    VFill(crate::schemas::schemas_microsoft_com_vml::Fill),
    #[xml(tag = "v:formulas")]
    VFormulas(crate::schemas::schemas_microsoft_com_vml::Formulas),
    #[xml(tag = "v:handles")]
    VHandles(crate::schemas::schemas_microsoft_com_vml::ShapeHandles),
    #[xml(tag = "v:imagedata")]
    VImagedata(crate::schemas::schemas_microsoft_com_vml::ImageData),
    #[xml(tag = "v:path")]
    VPath(crate::schemas::schemas_microsoft_com_vml::Path),
    #[xml(tag = "v:textbox")]
    VTextbox(crate::schemas::schemas_microsoft_com_vml::TextBox),
    #[xml(tag = "v:shadow")]
    VShadow(crate::schemas::schemas_microsoft_com_vml::Shadow),
    #[xml(tag = "v:stroke")]
    VStroke(crate::schemas::schemas_microsoft_com_vml::Stroke),
    #[xml(tag = "v:textpath")]
    VTextpath(crate::schemas::schemas_microsoft_com_vml::TextPath),
    #[xml(tag = "v:arc")]
    VArc(crate::schemas::schemas_microsoft_com_vml::Arc),
    #[xml(tag = "v:curve")]
    VCurve(crate::schemas::schemas_microsoft_com_vml::Curve),
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
    #[xml(tag = "o:shapedefaults")]
    OShapedefaults(crate::schemas::schemas_microsoft_com_office_office::ShapeDefaults),
    #[xml(tag = "o:shapelayout")]
    OShapelayout(crate::schemas::schemas_microsoft_com_office_office::ShapeLayout),
    #[xml(tag = "o:signatureline")]
    OSignatureline(crate::schemas::schemas_microsoft_com_office_office::SignatureLine),
    #[xml(tag = "o:ink")]
    OInk(crate::schemas::schemas_microsoft_com_office_office::Ink),
    #[xml(tag = "o:diagram")]
    ODiagram(crate::schemas::schemas_microsoft_com_office_office::Diagram),
    #[xml(tag = "o:skew")]
    OSkew(crate::schemas::schemas_microsoft_com_office_office::Skew),
    #[xml(tag = "o:extrusion")]
    OExtrusion(crate::schemas::schemas_microsoft_com_office_office::Extrusion),
    #[xml(tag = "o:callout")]
    OCallout(crate::schemas::schemas_microsoft_com_office_office::Callout),
    #[xml(tag = "o:lock")]
    OLock(crate::schemas::schemas_microsoft_com_office_office::Lock),
    #[xml(tag = "o:OLEObject")]
    OOleObject(crate::schemas::schemas_microsoft_com_office_office::OleObject),
    #[xml(tag = "o:complex")]
    OComplex(crate::schemas::schemas_microsoft_com_office_office::Complex),
    #[xml(tag = "o:left")]
    OLeft(crate::schemas::schemas_microsoft_com_office_office::LeftStroke),
    #[xml(tag = "o:top")]
    OTop(crate::schemas::schemas_microsoft_com_office_office::TopStroke),
    #[xml(tag = "o:right")]
    ORight(crate::schemas::schemas_microsoft_com_office_office::RightStroke),
    #[xml(tag = "o:bottom")]
    OBottom(crate::schemas::schemas_microsoft_com_office_office::BottomStroke),
    #[xml(tag = "o:column")]
    OColumn(crate::schemas::schemas_microsoft_com_office_office::ColumnStroke),
    #[xml(tag = "o:clippath")]
    OClippath(crate::schemas::schemas_microsoft_com_office_office::ClipPath),
    #[xml(tag = "o:fill")]
    OFill(crate::schemas::schemas_microsoft_com_office_office::FillExtendedProperties),
    #[xml(tag = "w10:bordertop")]
    W10Bordertop(crate::schemas::schemas_microsoft_com_office_word::TopBorder),
    #[xml(tag = "w10:borderleft")]
    W10Borderleft(crate::schemas::schemas_microsoft_com_office_word::LeftBorder),
    #[xml(tag = "w10:borderright")]
    W10Borderright(crate::schemas::schemas_microsoft_com_office_word::RightBorder),
    #[xml(tag = "w10:borderbottom")]
    W10Borderbottom(crate::schemas::schemas_microsoft_com_office_word::BottomBorder),
    #[xml(tag = "w10:wrap")]
    W10Wrap(crate::schemas::schemas_microsoft_com_office_word::TextWrap),
    #[xml(tag = "w10:anchorlock")]
    W10Anchorlock(crate::schemas::schemas_microsoft_com_office_word::AnchorLock),
    #[xml(tag = "xvml:ClientData")]
    XvmlClientData(crate::schemas::schemas_microsoft_com_office_excel::ClientData),
    #[xml(tag = "pvml:iscomment")]
    PvmlIscomment(
        crate::schemas::schemas_microsoft_com_office_powerpoint::InkAnnotationFlag,
    ),
    #[xml(tag = "pvml:textdata")]
    PvmlTextdata(crate::schemas::schemas_microsoft_com_office_powerpoint::TextData),
    #[xml(tag = "wpc:wpc")]
    WpcWpc(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_canvas::WordprocessingCanvas,
    ),
    #[xml(tag = "wpg:wgp")]
    WpgWgp(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_group::WordprocessingGroup,
    ),
    #[xml(tag = "wps:wsp")]
    WpsWsp(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordprocessing_shape::WordprocessingShape,
    ),
    #[xml(tag = "sle:slicer")]
    SleSlicer(crate::schemas::schemas_microsoft_com_office_drawing_2010_slicer::Slicer),
    #[xml(tag = "cs:colorStyle")]
    CsColorStyle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle,
    ),
    #[xml(tag = "cs:chartStyle")]
    CsChartStyle(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle,
    ),
    #[xml(tag = "we:webextension")]
    WeWebextension(
        crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
    ),
    #[xml(tag = "we:webextensionref")]
    WeWebextensionref(
        crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtensionReference,
    ),
    #[xml(tag = "tsle:timeslicer")]
    TsleTimeslicer(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_timeslicer::TimeSlicer,
    ),
}
/// Diagram to Animate.
/// When the object is serialized out as xml, it's qualified name is a:dgm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:dgm")]
pub struct Diagram {
    /// Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
    /// Animation Build Step
    /// Represents the following attribute in the schema: :bldStep
    #[xml(attr = "bldStep")]
    pub build_step: Option<DiagramBuildStepValues>,
}
/// Chart to Animate.
/// When the object is serialized out as xml, it's qualified name is a:chart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:chart")]
pub struct Chart {
    /// Series Index
    /// Represents the following attribute in the schema: :seriesIdx
    #[xml(attr = "seriesIdx")]
    pub series_index: Option<i32>,
    /// Category Index
    /// Represents the following attribute in the schema: :categoryIdx
    #[xml(attr = "categoryIdx")]
    pub category_index: Option<i32>,
    /// Animation Build Step
    /// Represents the following attribute in the schema: :bldStep
    #[xml(attr = "bldStep")]
    pub build_step: ChartBuildStepValues,
}
/// Build Diagram.
/// When the object is serialized out as xml, it's qualified name is a:bldDgm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bldDgm")]
pub struct BuildDiagram {
    /// Build
    /// Represents the following attribute in the schema: :bld
    #[xml(attr = "bld")]
    pub build: Option<String>,
    /// Reverse Animation
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub reverse_animation: Option<bool>,
}
/// Build Chart.
/// When the object is serialized out as xml, it's qualified name is a:bldChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bldChart")]
pub struct BuildChart {
    /// Build
    /// Represents the following attribute in the schema: :bld
    #[xml(attr = "bld")]
    pub build: Option<String>,
    /// Animate Background
    /// Represents the following attribute in the schema: :animBg
    #[xml(attr = "animBg")]
    pub animate_background: Option<bool>,
}
/// Shape Text Body.
/// When the object is serialized out as xml, it's qualified name is a:txBody.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:txBody")]
pub struct TextBody {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Body Properties
    #[xml(child = "a:bodyPr")]
    pub body_properties: BodyProperties,
    ///Text List Styles
    #[xml(child = "a:lstStyle")]
    pub list_style: Option<ListStyle>,
    /// _
    #[xml(child = "a:p")]
    pub a_p: Vec<Paragraph>,
}
/// Use Shape Text Rectangle.
/// When the object is serialized out as xml, it's qualified name is a:useSpRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:useSpRect")]
pub struct UseShapeRectangle {}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is a:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:xfrm")]
pub struct Transform2D {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
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
    pub offset: Option<Offset>,
    ///Extents
    #[xml(child = "a:ext")]
    pub extents: Option<Extents>,
}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is a:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cNvPr")]
pub struct NonVisualDrawingProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Application defined unique identifier.
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
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
    pub hyperlink_on_click: Option<HyperlinkOnClick>,
    ///Hyperlink associated with hovering over the element.
    #[xml(child = "a:hlinkHover")]
    pub hyperlink_on_hover: Option<HyperlinkOnHover>,
    ///Future extension
    #[xml(child = "a:extLst")]
    pub non_visual_drawing_properties_extension_list: Option<
        NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Non-Visual Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is a:cNvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cNvSpPr")]
pub struct NonVisualShapeDrawingProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Text Box
    /// Represents the following attribute in the schema: :txBox
    #[xml(attr = "txBox")]
    pub text_box: Option<bool>,
    ///Shape Locks
    #[xml(child = "a:spLocks")]
    pub shape_locks: Option<ShapeLocks>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Shape.
/// When the object is serialized out as xml, it's qualified name is a:nvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:nvSpPr")]
pub struct NonVisualShapeProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Shape Drawing Properties
    #[xml(child = "a:cNvSpPr")]
    pub non_visual_shape_drawing_properties: NonVisualShapeDrawingProperties,
}
/// Visual Properties.
/// When the object is serialized out as xml, it's qualified name is a:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spPr")]
pub struct ShapeProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<BlackWhiteModeValues>,
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
    AXfrm(Transform2D),
    #[xml(tag = "a:custGeom")]
    ACustGeom(CustomGeometry),
    #[xml(tag = "a:prstGeom")]
    APrstGeom(PresetGeometry),
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:scene3d")]
    AScene3d(Scene3DType),
    #[xml(tag = "a:sp3d")]
    ASp3d(Shape3DType),
    #[xml(tag = "a:extLst")]
    AExtLst(ShapePropertiesExtensionList),
}
/// Text Shape.
/// When the object is serialized out as xml, it's qualified name is a:txSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:txSp")]
pub struct TextShape {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:txBody",
        child = "a:useSpRect",
        child = "a:xfrm",
        child = "a:extLst",
    )]
    pub children: Vec<TextShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextShapeChildChoice {
    #[xml(tag = "a:txBody")]
    ATxBody(TextBody),
    #[xml(tag = "a:useSpRect")]
    AUseSpRect(UseShapeRectangle),
    #[xml(tag = "a:xfrm")]
    AXfrm(Transform2D),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Style.
/// When the object is serialized out as xml, it's qualified name is a:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:style")]
pub struct ShapeStyle {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:lnRef")]
    pub line_reference: LineReference,
    /// _
    #[xml(child = "a:fillRef")]
    pub fill_reference: FillReference,
    /// _
    #[xml(child = "a:effectRef")]
    pub effect_reference: EffectReference,
    ///Font Reference
    #[xml(child = "a:fontRef")]
    pub font_reference: FontReference,
}
/// Non-Visual Connector Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is a:cNvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cNvCxnSpPr")]
pub struct NonVisualConnectorShapeDrawingProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Connection Shape Locks
    #[xml(child = "a:cxnSpLocks")]
    pub connection_shape_locks: Option<ConnectionShapeLocks>,
    ///Connection Start
    #[xml(child = "a:stCxn")]
    pub start_connection: Option<StartConnection>,
    ///Connection End
    #[xml(child = "a:endCxn")]
    pub end_connection: Option<EndConnection>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Connection Shape.
/// When the object is serialized out as xml, it's qualified name is a:nvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:nvCxnSpPr")]
pub struct NonVisualConnectionShapeProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Drawing Properties
    #[xml(child = "a:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Connector Shape Drawing Properties
    #[xml(child = "a:cNvCxnSpPr")]
    pub non_visual_connector_shape_drawing_properties: NonVisualConnectorShapeDrawingProperties,
}
/// Non-Visual Picture Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is a:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cNvPicPr")]
pub struct NonVisualPictureDrawingProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// preferRelativeResize
    /// Represents the following attribute in the schema: :preferRelativeResize
    #[xml(attr = "preferRelativeResize")]
    pub prefer_relative_resize: Option<bool>,
    /// _
    #[xml(child = "a:picLocks")]
    pub picture_locks: Option<PictureLocks>,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_picture_properties_extension_list: Option<
        NonVisualPicturePropertiesExtensionList,
    >,
}
/// Non-Visual Properties for a Picture.
/// When the object is serialized out as xml, it's qualified name is a:nvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:nvPicPr")]
pub struct NonVisualPictureProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Picture Drawing Properties
    #[xml(child = "a:cNvPicPr")]
    pub non_visual_picture_drawing_properties: NonVisualPictureDrawingProperties,
}
/// Non-Visual Graphic Frame Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is a:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cNvGraphicFramePr")]
pub struct NonVisualGraphicFrameDrawingProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Graphic Frame Locks
    #[xml(child = "a:graphicFrameLocks")]
    pub graphic_frame_locks: Option<GraphicFrameLocks>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Non-Visual Properties for a Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is a:nvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:nvGraphicFramePr")]
pub struct NonVisualGraphicFrameProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Graphic Frame Drawing Properties
    #[xml(child = "a:cNvGraphicFramePr")]
    pub non_visual_graphic_frame_drawing_properties: NonVisualGraphicFrameDrawingProperties,
}
/// Non-Visual Group Shape Drawing Properties.
/// When the object is serialized out as xml, it's qualified name is a:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cNvGrpSpPr")]
pub struct NonVisualGroupShapeDrawingProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:grpSpLocks")]
    pub group_shape_locks: Option<GroupShapeLocks>,
    /// _
    #[xml(child = "a:extLst")]
    pub non_visual_group_drawing_shape_props_extension_list: Option<
        NonVisualGroupDrawingShapePropsExtensionList,
    >,
}
/// Rotation.
/// When the object is serialized out as xml, it's qualified name is a:rot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:rot")]
pub struct Rotation {
    /// Latitude
    /// Represents the following attribute in the schema: :lat
    #[xml(attr = "lat")]
    pub latitude: i32,
    /// Longitude
    /// Represents the following attribute in the schema: :lon
    #[xml(attr = "lon")]
    pub longitude: i32,
    /// Revolution
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub revolution: i32,
}
/// Camera.
/// When the object is serialized out as xml, it's qualified name is a:camera.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:camera")]
pub struct Camera {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Preset Camera Type
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: PresetCameraValues,
    /// Field of View
    /// Represents the following attribute in the schema: :fov
    #[xml(attr = "fov")]
    pub field_of_view: Option<i32>,
    /// Zoom
    /// Represents the following attribute in the schema: :zoom
    #[xml(attr = "zoom")]
    pub zoom: Option<i32>,
    ///Rotation
    #[xml(child = "a:rot")]
    pub rotation: Option<Rotation>,
}
/// Light Rig.
/// When the object is serialized out as xml, it's qualified name is a:lightRig.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lightRig")]
pub struct LightRig {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Rig Preset
    /// Represents the following attribute in the schema: :rig
    #[xml(attr = "rig")]
    pub rig: LightRigValues,
    /// Direction
    /// Represents the following attribute in the schema: :dir
    #[xml(attr = "dir")]
    pub direction: LightRigDirectionValues,
    ///Rotation
    #[xml(child = "a:rot")]
    pub rotation: Option<Rotation>,
}
/// Backdrop Plane.
/// When the object is serialized out as xml, it's qualified name is a:backdrop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:backdrop")]
pub struct Backdrop {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Anchor Point
    #[xml(child = "a:anchor")]
    pub anchor: Anchor,
    ///Normal
    #[xml(child = "a:norm")]
    pub normal: Normal,
    ///Up Vector
    #[xml(child = "a:up")]
    pub up_vector: UpVector,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Anchor Point.
/// When the object is serialized out as xml, it's qualified name is a:anchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:anchor")]
pub struct Anchor {
    /// X-Coordinate in 3D
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: i64,
    /// Y-Coordinate in 3D
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: i64,
    /// Z-Coordinate in 3D
    /// Represents the following attribute in the schema: :z
    #[xml(attr = "z")]
    pub z: i64,
}
/// Normal.
/// When the object is serialized out as xml, it's qualified name is a:norm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:norm")]
pub struct Normal {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i64,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i64,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i64,
}
/// Up Vector.
/// When the object is serialized out as xml, it's qualified name is a:up.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:up")]
pub struct UpVector {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i64,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i64,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i64,
}
/// Defines the Vector3DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct Vector3DType {
    /// Distance along X-axis in 3D
    /// Represents the following attribute in the schema: :dx
    #[xml(attr = "dx")]
    pub dx: i64,
    /// Distance along Y-axis in 3D
    /// Represents the following attribute in the schema: :dy
    #[xml(attr = "dy")]
    pub dy: i64,
    /// Distance along Z-axis in 3D
    /// Represents the following attribute in the schema: :dz
    #[xml(attr = "dz")]
    pub dz: i64,
}
/// Top Bevel.
/// When the object is serialized out as xml, it's qualified name is a:bevelT.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bevelT")]
pub struct BevelTop {
    /// Width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i64>,
    /// Height
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: Option<i64>,
    /// Preset Bevel
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: Option<BevelPresetValues>,
}
/// Bottom Bevel.
/// When the object is serialized out as xml, it's qualified name is a:bevelB.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bevelB")]
pub struct BevelBottom {
    /// Width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i64>,
    /// Height
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: Option<i64>,
    /// Preset Bevel
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: Option<BevelPresetValues>,
}
/// Bevel.
/// When the object is serialized out as xml, it's qualified name is a:bevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bevel")]
pub struct Bevel {
    /// Width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i64>,
    /// Height
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: Option<i64>,
    /// Preset Bevel
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: Option<BevelPresetValues>,
}
/// Defines the BevelType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BevelType {
    /// Width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i64>,
    /// Height
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: Option<i64>,
    /// Preset Bevel
    /// Represents the following attribute in the schema: :prst
    #[xml(attr = "prst")]
    pub preset: Option<BevelPresetValues>,
}
/// Fill To Rectangle.
/// When the object is serialized out as xml, it's qualified name is a:fillToRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fillToRect")]
pub struct FillToRectangle {
    /// Left Offset
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: Option<i32>,
    /// Top Offset
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: Option<i32>,
    /// Right Offset
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: Option<i32>,
    /// Bottom Offset
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: Option<i32>,
}
/// Tile Rectangle.
/// When the object is serialized out as xml, it's qualified name is a:tileRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tileRect")]
pub struct TileRectangle {
    /// Left Offset
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: Option<i32>,
    /// Top Offset
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: Option<i32>,
    /// Right Offset
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: Option<i32>,
    /// Bottom Offset
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: Option<i32>,
}
/// Fill Rectangle.
/// When the object is serialized out as xml, it's qualified name is a:fillRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:fillRect")]
pub struct FillRectangle {
    /// Left Offset
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: Option<i32>,
    /// Top Offset
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: Option<i32>,
    /// Right Offset
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: Option<i32>,
    /// Bottom Offset
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: Option<i32>,
}
/// Source Rectangle.
/// When the object is serialized out as xml, it's qualified name is a:srcRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:srcRect")]
pub struct SourceRectangle {
    /// Left Offset
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: Option<i32>,
    /// Top Offset
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: Option<i32>,
    /// Right Offset
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: Option<i32>,
    /// Bottom Offset
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: Option<i32>,
}
/// Defines the RelativeRectangleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct RelativeRectangleType {
    /// Left Offset
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: Option<i32>,
    /// Top Offset
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: Option<i32>,
    /// Right Offset
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: Option<i32>,
    /// Bottom Offset
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: Option<i32>,
}
/// Gradient stops.
/// When the object is serialized out as xml, it's qualified name is a:gs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gs")]
pub struct GradientStop {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Position
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub position: i32,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<GradientStopChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GradientStopChildChoice {
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
}
/// Gradient Stop List.
/// When the object is serialized out as xml, it's qualified name is a:gsLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gsLst")]
pub struct GradientStopList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:gs")]
    pub a_gs: Vec<GradientStop>,
}
/// Shape Guide.
/// When the object is serialized out as xml, it's qualified name is a:gd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gd")]
pub struct ShapeGuide {
    /// Shape Guide Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// Shape Guide Formula
    /// Represents the following attribute in the schema: :fmla
    #[xml(attr = "fmla")]
    pub formula: String,
}
/// Position.
/// When the object is serialized out as xml, it's qualified name is a:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:pos")]
pub struct Position {
    /// X-Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: String,
    /// Y-Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: String,
}
/// Move end point.
/// When the object is serialized out as xml, it's qualified name is a:pt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:pt")]
pub struct Point {
    /// X-Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: String,
    /// Y-Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: String,
}
/// Defines the AdjustPoint2DType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct AdjustPoint2DType {
    /// X-Coordinate
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: String,
    /// Y-Coordinate
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: String,
}
/// XY Adjust Handle.
/// When the object is serialized out as xml, it's qualified name is a:ahXY.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ahXY")]
pub struct AdjustHandleXy {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Horizontal Adjustment Guide
    /// Represents the following attribute in the schema: :gdRefX
    #[xml(attr = "gdRefX")]
    pub x_adjustment_guide: Option<String>,
    /// Minimum Horizontal Adjustment
    /// Represents the following attribute in the schema: :minX
    #[xml(attr = "minX")]
    pub min_x: Option<String>,
    /// Maximum Horizontal Adjustment
    /// Represents the following attribute in the schema: :maxX
    #[xml(attr = "maxX")]
    pub max_x: Option<String>,
    /// Vertical Adjustment Guide
    /// Represents the following attribute in the schema: :gdRefY
    #[xml(attr = "gdRefY")]
    pub y_adjustment_guide: Option<String>,
    /// Minimum Vertical Adjustment
    /// Represents the following attribute in the schema: :minY
    #[xml(attr = "minY")]
    pub min_y: Option<String>,
    /// Maximum Vertical Adjustment
    /// Represents the following attribute in the schema: :maxY
    #[xml(attr = "maxY")]
    pub max_y: Option<String>,
    ///Position
    #[xml(child = "a:pos")]
    pub position: Position,
}
/// Polar Adjust Handle.
/// When the object is serialized out as xml, it's qualified name is a:ahPolar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ahPolar")]
pub struct AdjustHandlePolar {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Radial Adjustment Guide
    /// Represents the following attribute in the schema: :gdRefR
    #[xml(attr = "gdRefR")]
    pub radial_adjustment_guide: Option<String>,
    /// Minimum Radial Adjustment
    /// Represents the following attribute in the schema: :minR
    #[xml(attr = "minR")]
    pub min_radial: Option<String>,
    /// Maximum Radial Adjustment
    /// Represents the following attribute in the schema: :maxR
    #[xml(attr = "maxR")]
    pub max_radial: Option<String>,
    /// Angle Adjustment Guide
    /// Represents the following attribute in the schema: :gdRefAng
    #[xml(attr = "gdRefAng")]
    pub angle_adjustment_guide: Option<String>,
    /// Minimum Angle Adjustment
    /// Represents the following attribute in the schema: :minAng
    #[xml(attr = "minAng")]
    pub min_angle: Option<String>,
    /// Maximum Angle Adjustment
    /// Represents the following attribute in the schema: :maxAng
    #[xml(attr = "maxAng")]
    pub max_angle: Option<String>,
    ///Shape Position Coordinate
    #[xml(child = "a:pos")]
    pub position: Position,
}
/// Shape Connection Site.
/// When the object is serialized out as xml, it's qualified name is a:cxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cxn")]
pub struct ConnectionSite {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Connection Site Angle
    /// Represents the following attribute in the schema: :ang
    #[xml(attr = "ang")]
    pub angle: String,
    ///Position
    #[xml(child = "a:pos")]
    pub position: Position,
}
/// Close Shape Path.
/// When the object is serialized out as xml, it's qualified name is a:close.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:close")]
pub struct CloseShapePath {}
/// Move Path To.
/// When the object is serialized out as xml, it's qualified name is a:moveTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:moveTo")]
pub struct MoveTo {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Move end point
    #[xml(child = "a:pt")]
    pub point: Point,
}
/// Draw Line To.
/// When the object is serialized out as xml, it's qualified name is a:lnTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnTo")]
pub struct LineTo {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Line end point
    #[xml(child = "a:pt")]
    pub point: Point,
}
/// Draw Arc To.
/// When the object is serialized out as xml, it's qualified name is a:arcTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:arcTo")]
pub struct ArcTo {
    /// Shape Arc Width Radius
    /// Represents the following attribute in the schema: :wR
    #[xml(attr = "wR")]
    pub width_radius: String,
    /// Shape Arc Height Radius
    /// Represents the following attribute in the schema: :hR
    #[xml(attr = "hR")]
    pub height_radius: String,
    /// Shape Arc Start Angle
    /// Represents the following attribute in the schema: :stAng
    #[xml(attr = "stAng")]
    pub start_angle: String,
    /// Shape Arc Swing Angle
    /// Represents the following attribute in the schema: :swAng
    #[xml(attr = "swAng")]
    pub swing_angle: String,
}
/// Draw Quadratic Bezier Curve To.
/// When the object is serialized out as xml, it's qualified name is a:quadBezTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:quadBezTo")]
pub struct QuadraticBezierCurveTo {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:pt")]
    pub a_pt: Vec<Point>,
}
/// Draw Cubic Bezier Curve To.
/// When the object is serialized out as xml, it's qualified name is a:cubicBezTo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cubicBezTo")]
pub struct CubicBezierCurveTo {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:pt")]
    pub a_pt: Vec<Point>,
}
/// Shape Path.
/// When the object is serialized out as xml, it's qualified name is a:path.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:path")]
pub struct Path {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Path Width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i64>,
    /// Path Height
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: Option<i64>,
    /// Path Fill
    /// Represents the following attribute in the schema: :fill
    #[xml(attr = "fill")]
    pub fill: Option<PathFillModeValues>,
    /// Path Stroke
    /// Represents the following attribute in the schema: :stroke
    #[xml(attr = "stroke")]
    pub stroke: Option<bool>,
    /// 3D Extrusion Allowed
    /// Represents the following attribute in the schema: :extrusionOk
    #[xml(attr = "extrusionOk")]
    pub extrusion_ok: Option<bool>,
    #[xml(
        child = "a:close",
        child = "a:moveTo",
        child = "a:lnTo",
        child = "a:arcTo",
        child = "a:quadBezTo",
        child = "a:cubicBezTo",
    )]
    pub children: Vec<PathChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PathChildChoice {
    #[xml(tag = "a:close")]
    AClose(CloseShapePath),
    #[xml(tag = "a:moveTo")]
    AMoveTo(MoveTo),
    #[xml(tag = "a:lnTo")]
    ALnTo(LineTo),
    #[xml(tag = "a:arcTo")]
    AArcTo(ArcTo),
    #[xml(tag = "a:quadBezTo")]
    AQuadBezTo(QuadraticBezierCurveTo),
    #[xml(tag = "a:cubicBezTo")]
    ACubicBezTo(CubicBezierCurveTo),
}
/// List of Shape Adjust Values.
/// When the object is serialized out as xml, it's qualified name is a:avLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:avLst")]
pub struct AdjustValueList {
    #[xml(child = "a:gd")]
    pub children: Vec<AdjustValueListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AdjustValueListChildChoice {
    #[xml(tag = "a:gd")]
    AGd(ShapeGuide),
}
/// List of Shape Guides.
/// When the object is serialized out as xml, it's qualified name is a:gdLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gdLst")]
pub struct ShapeGuideList {
    #[xml(child = "a:gd")]
    pub children: Vec<ShapeGuideListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapeGuideListChildChoice {
    #[xml(tag = "a:gd")]
    AGd(ShapeGuide),
}
/// Defines the GeometryGuideListType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct GeometryGuideListType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:gd")]
    pub children: Vec<GeometryGuideListTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GeometryGuideListTypeChildChoice {
    #[xml(tag = "a:gd")]
    AGd(ShapeGuide),
}
/// List of Shape Adjust Handles.
/// When the object is serialized out as xml, it's qualified name is a:ahLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ahLst")]
pub struct AdjustHandleList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:ahXY", child = "a:ahPolar")]
    pub children: Vec<AdjustHandleListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum AdjustHandleListChildChoice {
    #[xml(tag = "a:ahXY")]
    AAhXy(AdjustHandleXy),
    #[xml(tag = "a:ahPolar")]
    AAhPolar(AdjustHandlePolar),
}
/// List of Shape Connection Sites.
/// When the object is serialized out as xml, it's qualified name is a:cxnLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cxnLst")]
pub struct ConnectionSiteList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:cxn")]
    pub a_cxn: Vec<ConnectionSite>,
}
/// Shape Text Rectangle.
/// When the object is serialized out as xml, it's qualified name is a:rect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:rect")]
pub struct Rectangle {
    /// Left
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub left: String,
    /// Top
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub top: String,
    /// Right
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub right: String,
    /// Bottom Position
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bottom: String,
}
/// List of Shape Paths.
/// When the object is serialized out as xml, it's qualified name is a:pathLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:pathLst")]
pub struct PathList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:path")]
    pub a_path: Vec<Path>,
}
/// Dash Stop.
/// When the object is serialized out as xml, it's qualified name is a:ds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ds")]
pub struct DashStop {
    /// Dash Length
    /// Represents the following attribute in the schema: :d
    #[xml(attr = "d")]
    pub dash_length: i32,
    /// Space Length
    /// Represents the following attribute in the schema: :sp
    #[xml(attr = "sp")]
    pub space_length: i32,
}
/// 2D Transform for Grouped Objects.
/// When the object is serialized out as xml, it's qualified name is a:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:xfrm")]
pub struct TransformGroup {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
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
    pub offset: Option<Offset>,
    ///Extents
    #[xml(child = "a:ext")]
    pub extents: Option<Extents>,
    ///Child Offset
    #[xml(child = "a:chOff")]
    pub child_offset: Option<ChildOffset>,
    ///Child Extents
    #[xml(child = "a:chExt")]
    pub child_extents: Option<ChildExtents>,
}
/// Defines the BodyProperties Class.
/// When the object is serialized out as xml, it's qualified name is a:bodyPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bodyPr")]
pub struct BodyProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<i32>,
    /// Paragraph Spacing
    /// Represents the following attribute in the schema: :spcFirstLastPara
    #[xml(attr = "spcFirstLastPara")]
    pub use_paragraph_spacing: Option<bool>,
    /// Text Vertical Overflow
    /// Represents the following attribute in the schema: :vertOverflow
    #[xml(attr = "vertOverflow")]
    pub vertical_overflow: Option<TextVerticalOverflowValues>,
    /// Text Horizontal Overflow
    /// Represents the following attribute in the schema: :horzOverflow
    #[xml(attr = "horzOverflow")]
    pub horizontal_overflow: Option<TextHorizontalOverflowValues>,
    /// Vertical Text
    /// Represents the following attribute in the schema: :vert
    #[xml(attr = "vert")]
    pub vertical: Option<TextVerticalValues>,
    /// Text Wrapping Type
    /// Represents the following attribute in the schema: :wrap
    #[xml(attr = "wrap")]
    pub wrap: Option<TextWrappingValues>,
    /// Left Inset
    /// Represents the following attribute in the schema: :lIns
    #[xml(attr = "lIns")]
    pub left_inset: Option<i32>,
    /// Top Inset
    /// Represents the following attribute in the schema: :tIns
    #[xml(attr = "tIns")]
    pub top_inset: Option<i32>,
    /// Right Inset
    /// Represents the following attribute in the schema: :rIns
    #[xml(attr = "rIns")]
    pub right_inset: Option<i32>,
    /// Bottom Inset
    /// Represents the following attribute in the schema: :bIns
    #[xml(attr = "bIns")]
    pub bottom_inset: Option<i32>,
    /// Number of Columns
    /// Represents the following attribute in the schema: :numCol
    #[xml(attr = "numCol")]
    pub column_count: Option<i32>,
    /// Space Between Columns
    /// Represents the following attribute in the schema: :spcCol
    #[xml(attr = "spcCol")]
    pub column_spacing: Option<i32>,
    /// Columns Right-To-Left
    /// Represents the following attribute in the schema: :rtlCol
    #[xml(attr = "rtlCol")]
    pub right_to_left_columns: Option<bool>,
    /// From WordArt
    /// Represents the following attribute in the schema: :fromWordArt
    #[xml(attr = "fromWordArt")]
    pub from_word_art: Option<bool>,
    /// Anchor
    /// Represents the following attribute in the schema: :anchor
    #[xml(attr = "anchor")]
    pub anchor: Option<TextAnchoringTypeValues>,
    /// Anchor Center
    /// Represents the following attribute in the schema: :anchorCtr
    #[xml(attr = "anchorCtr")]
    pub anchor_center: Option<bool>,
    /// Force Anti-Alias
    /// Represents the following attribute in the schema: :forceAA
    #[xml(attr = "forceAA")]
    pub force_anti_alias: Option<bool>,
    /// Text Upright
    /// Represents the following attribute in the schema: :upright
    #[xml(attr = "upright")]
    pub up_right: Option<bool>,
    /// Compatible Line Spacing
    /// Represents the following attribute in the schema: :compatLnSpc
    #[xml(attr = "compatLnSpc")]
    pub compatible_line_spacing: Option<bool>,
    #[xml(
        child = "a:prstTxWarp",
        child = "a:noAutofit",
        child = "a:normAutofit",
        child = "a:spAutoFit",
        child = "a:scene3d",
        child = "a:sp3d",
        child = "a:flatTx",
        child = "a:extLst",
    )]
    pub children: Vec<BodyPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BodyPropertiesChildChoice {
    #[xml(tag = "a:prstTxWarp")]
    APrstTxWarp(PresetTextWarp),
    #[xml(tag = "a:noAutofit")]
    ANoAutofit(NoAutoFit),
    #[xml(tag = "a:normAutofit")]
    ANormAutofit(NormalAutoFit),
    #[xml(tag = "a:spAutoFit")]
    ASpAutoFit(ShapeAutoFit),
    #[xml(tag = "a:scene3d")]
    AScene3d(Scene3DType),
    #[xml(tag = "a:sp3d")]
    ASp3d(Shape3DType),
    #[xml(tag = "a:flatTx")]
    AFlatTx(FlatText),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Defines the ListStyle Class.
/// When the object is serialized out as xml, it's qualified name is a:lstStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lstStyle")]
pub struct ListStyle {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Default Paragraph Style
    #[xml(child = "a:defPPr")]
    pub default_paragraph_properties: Option<DefaultParagraphProperties>,
    ///List Level 1 Text Style
    #[xml(child = "a:lvl1pPr")]
    pub level1_paragraph_properties: Option<Level1ParagraphProperties>,
    ///List Level 2 Text Style
    #[xml(child = "a:lvl2pPr")]
    pub level2_paragraph_properties: Option<Level2ParagraphProperties>,
    ///List Level 3 Text Style
    #[xml(child = "a:lvl3pPr")]
    pub level3_paragraph_properties: Option<Level3ParagraphProperties>,
    ///List Level 4 Text Style
    #[xml(child = "a:lvl4pPr")]
    pub level4_paragraph_properties: Option<Level4ParagraphProperties>,
    ///List Level 5 Text Style
    #[xml(child = "a:lvl5pPr")]
    pub level5_paragraph_properties: Option<Level5ParagraphProperties>,
    ///List Level 6 Text Style
    #[xml(child = "a:lvl6pPr")]
    pub level6_paragraph_properties: Option<Level6ParagraphProperties>,
    ///List Level 7 Text Style
    #[xml(child = "a:lvl7pPr")]
    pub level7_paragraph_properties: Option<Level7ParagraphProperties>,
    ///List Level 8 Text Style
    #[xml(child = "a:lvl8pPr")]
    pub level8_paragraph_properties: Option<Level8ParagraphProperties>,
    ///List Level 9 Text Style
    #[xml(child = "a:lvl9pPr")]
    pub level9_paragraph_properties: Option<Level9ParagraphProperties>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Shape Default.
/// When the object is serialized out as xml, it's qualified name is a:spDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spDef")]
pub struct ShapeDefault {
    ///Visual Properties
    #[xml(child = "a:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "a:bodyPr")]
    pub body_properties: BodyProperties,
    /// _
    #[xml(child = "a:lstStyle")]
    pub list_style: ListStyle,
    /// _
    #[xml(child = "a:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Line Default.
/// When the object is serialized out as xml, it's qualified name is a:lnDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnDef")]
pub struct LineDefault {
    ///Visual Properties
    #[xml(child = "a:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "a:bodyPr")]
    pub body_properties: BodyProperties,
    /// _
    #[xml(child = "a:lstStyle")]
    pub list_style: ListStyle,
    /// _
    #[xml(child = "a:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Text Default.
/// When the object is serialized out as xml, it's qualified name is a:txDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:txDef")]
pub struct TextDefault {
    ///Visual Properties
    #[xml(child = "a:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "a:bodyPr")]
    pub body_properties: BodyProperties,
    /// _
    #[xml(child = "a:lstStyle")]
    pub list_style: ListStyle,
    /// _
    #[xml(child = "a:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the DefaultShapeDefinitionType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct DefaultShapeDefinitionType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
}
/// Override Color Mapping.
/// When the object is serialized out as xml, it's qualified name is a:overrideClrMapping.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:overrideClrMapping")]
pub struct OverrideColorMapping {
    /// Background 1
    /// Represents the following attribute in the schema: :bg1
    #[xml(attr = "bg1")]
    pub background1: ColorSchemeIndexValues,
    /// Text 1
    /// Represents the following attribute in the schema: :tx1
    #[xml(attr = "tx1")]
    pub text1: ColorSchemeIndexValues,
    /// Background 2
    /// Represents the following attribute in the schema: :bg2
    #[xml(attr = "bg2")]
    pub background2: ColorSchemeIndexValues,
    /// Text 2
    /// Represents the following attribute in the schema: :tx2
    #[xml(attr = "tx2")]
    pub text2: ColorSchemeIndexValues,
    /// Accent 1
    /// Represents the following attribute in the schema: :accent1
    #[xml(attr = "accent1")]
    pub accent1: ColorSchemeIndexValues,
    /// Accent 2
    /// Represents the following attribute in the schema: :accent2
    #[xml(attr = "accent2")]
    pub accent2: ColorSchemeIndexValues,
    /// Accent 3
    /// Represents the following attribute in the schema: :accent3
    #[xml(attr = "accent3")]
    pub accent3: ColorSchemeIndexValues,
    /// Accent 4
    /// Represents the following attribute in the schema: :accent4
    #[xml(attr = "accent4")]
    pub accent4: ColorSchemeIndexValues,
    /// Accent 5
    /// Represents the following attribute in the schema: :accent5
    #[xml(attr = "accent5")]
    pub accent5: ColorSchemeIndexValues,
    /// Accent 6
    /// Represents the following attribute in the schema: :accent6
    #[xml(attr = "accent6")]
    pub accent6: ColorSchemeIndexValues,
    /// Hyperlink
    /// Represents the following attribute in the schema: :hlink
    #[xml(attr = "hlink")]
    pub hyperlink: ColorSchemeIndexValues,
    /// Followed Hyperlink
    /// Represents the following attribute in the schema: :folHlink
    #[xml(attr = "folHlink")]
    pub followed_hyperlink: ColorSchemeIndexValues,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMap Class.
/// When the object is serialized out as xml, it's qualified name is a:clrMap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:clrMap")]
pub struct ColorMap {
    /// Background 1
    /// Represents the following attribute in the schema: :bg1
    #[xml(attr = "bg1")]
    pub background1: ColorSchemeIndexValues,
    /// Text 1
    /// Represents the following attribute in the schema: :tx1
    #[xml(attr = "tx1")]
    pub text1: ColorSchemeIndexValues,
    /// Background 2
    /// Represents the following attribute in the schema: :bg2
    #[xml(attr = "bg2")]
    pub background2: ColorSchemeIndexValues,
    /// Text 2
    /// Represents the following attribute in the schema: :tx2
    #[xml(attr = "tx2")]
    pub text2: ColorSchemeIndexValues,
    /// Accent 1
    /// Represents the following attribute in the schema: :accent1
    #[xml(attr = "accent1")]
    pub accent1: ColorSchemeIndexValues,
    /// Accent 2
    /// Represents the following attribute in the schema: :accent2
    #[xml(attr = "accent2")]
    pub accent2: ColorSchemeIndexValues,
    /// Accent 3
    /// Represents the following attribute in the schema: :accent3
    #[xml(attr = "accent3")]
    pub accent3: ColorSchemeIndexValues,
    /// Accent 4
    /// Represents the following attribute in the schema: :accent4
    #[xml(attr = "accent4")]
    pub accent4: ColorSchemeIndexValues,
    /// Accent 5
    /// Represents the following attribute in the schema: :accent5
    #[xml(attr = "accent5")]
    pub accent5: ColorSchemeIndexValues,
    /// Accent 6
    /// Represents the following attribute in the schema: :accent6
    #[xml(attr = "accent6")]
    pub accent6: ColorSchemeIndexValues,
    /// Hyperlink
    /// Represents the following attribute in the schema: :hlink
    #[xml(attr = "hlink")]
    pub hyperlink: ColorSchemeIndexValues,
    /// Followed Hyperlink
    /// Represents the following attribute in the schema: :folHlink
    #[xml(attr = "folHlink")]
    pub followed_hyperlink: ColorSchemeIndexValues,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ColorMappingType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorMappingType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Background 1
    /// Represents the following attribute in the schema: :bg1
    #[xml(attr = "bg1")]
    pub background1: ColorSchemeIndexValues,
    /// Text 1
    /// Represents the following attribute in the schema: :tx1
    #[xml(attr = "tx1")]
    pub text1: ColorSchemeIndexValues,
    /// Background 2
    /// Represents the following attribute in the schema: :bg2
    #[xml(attr = "bg2")]
    pub background2: ColorSchemeIndexValues,
    /// Text 2
    /// Represents the following attribute in the schema: :tx2
    #[xml(attr = "tx2")]
    pub text2: ColorSchemeIndexValues,
    /// Accent 1
    /// Represents the following attribute in the schema: :accent1
    #[xml(attr = "accent1")]
    pub accent1: ColorSchemeIndexValues,
    /// Accent 2
    /// Represents the following attribute in the schema: :accent2
    #[xml(attr = "accent2")]
    pub accent2: ColorSchemeIndexValues,
    /// Accent 3
    /// Represents the following attribute in the schema: :accent3
    #[xml(attr = "accent3")]
    pub accent3: ColorSchemeIndexValues,
    /// Accent 4
    /// Represents the following attribute in the schema: :accent4
    #[xml(attr = "accent4")]
    pub accent4: ColorSchemeIndexValues,
    /// Accent 5
    /// Represents the following attribute in the schema: :accent5
    #[xml(attr = "accent5")]
    pub accent5: ColorSchemeIndexValues,
    /// Accent 6
    /// Represents the following attribute in the schema: :accent6
    #[xml(attr = "accent6")]
    pub accent6: ColorSchemeIndexValues,
    /// Hyperlink
    /// Represents the following attribute in the schema: :hlink
    #[xml(attr = "hlink")]
    pub hyperlink: ColorSchemeIndexValues,
    /// Followed Hyperlink
    /// Represents the following attribute in the schema: :folHlink
    #[xml(attr = "folHlink")]
    pub followed_hyperlink: ColorSchemeIndexValues,
}
/// Extra Color Scheme.
/// When the object is serialized out as xml, it's qualified name is a:extraClrScheme.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extraClrScheme")]
pub struct ExtraColorScheme {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:clrScheme")]
    pub color_scheme: ColorScheme,
    /// _
    #[xml(child = "a:clrMap")]
    pub color_map: Option<ColorMap>,
}
/// Defines the ThemeElements Class.
/// When the object is serialized out as xml, it's qualified name is a:themeElements.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:themeElements")]
pub struct ThemeElements {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:clrScheme")]
    pub color_scheme: ColorScheme,
    ///Font Scheme
    #[xml(child = "a:fontScheme")]
    pub font_scheme: FontScheme,
    ///Format Scheme
    #[xml(child = "a:fmtScheme")]
    pub format_scheme: FormatScheme,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Cell 3-D.
/// When the object is serialized out as xml, it's qualified name is a:cell3D.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cell3D")]
pub struct Cell3DProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Preset Material
    /// Represents the following attribute in the schema: :prstMaterial
    #[xml(attr = "prstMaterial")]
    pub preset_material: Option<PresetMaterialTypeValues>,
    ///Bevel
    #[xml(child = "a:bevel")]
    pub bevel: Bevel,
    ///Light Rig
    #[xml(child = "a:lightRig")]
    pub light_rig: Option<LightRig>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Table Cell Properties.
/// When the object is serialized out as xml, it's qualified name is a:tcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tcPr")]
pub struct TableCellProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Top Margin
    /// Represents the following attribute in the schema: :marT
    #[xml(attr = "marT")]
    pub top_margin: Option<i32>,
    /// Bottom Margin
    /// Represents the following attribute in the schema: :marB
    #[xml(attr = "marB")]
    pub bottom_margin: Option<i32>,
    /// Text Direction
    /// Represents the following attribute in the schema: :vert
    #[xml(attr = "vert")]
    pub vertical: Option<TextVerticalValues>,
    /// Anchor
    /// Represents the following attribute in the schema: :anchor
    #[xml(attr = "anchor")]
    pub anchor: Option<TextAnchoringTypeValues>,
    /// Anchor Center
    /// Represents the following attribute in the schema: :anchorCtr
    #[xml(attr = "anchorCtr")]
    pub anchor_center: Option<bool>,
    /// Horizontal Overflow
    /// Represents the following attribute in the schema: :horzOverflow
    #[xml(attr = "horzOverflow")]
    pub horizontal_overflow: Option<TextHorizontalOverflowValues>,
    #[xml(
        child = "a:lnL",
        child = "a:lnR",
        child = "a:lnT",
        child = "a:lnB",
        child = "a:lnTlToBr",
        child = "a:lnBlToTr",
        child = "a:cell3D",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:extLst",
    )]
    pub children: Vec<TableCellPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableCellPropertiesChildChoice {
    #[xml(tag = "a:lnL")]
    ALnL(LeftBorderLineProperties),
    #[xml(tag = "a:lnR")]
    ALnR(RightBorderLineProperties),
    #[xml(tag = "a:lnT")]
    ALnT(TopBorderLineProperties),
    #[xml(tag = "a:lnB")]
    ALnB(BottomBorderLineProperties),
    #[xml(tag = "a:lnTlToBr")]
    ALnTlToBr(TopLeftToBottomRightBorderLineProperties),
    #[xml(tag = "a:lnBlToTr")]
    ALnBlToTr(BottomLeftToTopRightBorderLineProperties),
    #[xml(tag = "a:cell3D")]
    ACell3D(Cell3DProperties),
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Table Cell.
/// When the object is serialized out as xml, it's qualified name is a:tc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tc")]
pub struct TableCell {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Row Span
    /// Represents the following attribute in the schema: :rowSpan
    #[xml(attr = "rowSpan")]
    pub row_span: Option<i32>,
    /// Grid Span
    /// Represents the following attribute in the schema: :gridSpan
    #[xml(attr = "gridSpan")]
    pub grid_span: Option<i32>,
    /// Horizontal Merge
    /// Represents the following attribute in the schema: :hMerge
    #[xml(attr = "hMerge")]
    pub horizontal_merge: Option<bool>,
    /// Vertical Merge
    /// Represents the following attribute in the schema: :vMerge
    #[xml(attr = "vMerge")]
    pub vertical_merge: Option<bool>,
    ///Text Body
    #[xml(child = "a:txBody")]
    pub text_body: Option<TextBody>,
    ///Table Cell Properties
    #[xml(child = "a:tcPr")]
    pub table_cell_properties: Option<TableCellProperties>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Table Style.
/// When the object is serialized out as xml, it's qualified name is a:tableStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tableStyle")]
pub struct TableStyle {
    /// Style ID
    /// Represents the following attribute in the schema: :styleId
    #[xml(attr = "styleId")]
    pub style_id: String,
    /// Name
    /// Represents the following attribute in the schema: :styleName
    #[xml(attr = "styleName")]
    pub style_name: String,
    ///Table Background
    #[xml(child = "a:tblBg")]
    pub table_background: Option<TableBackground>,
    ///Whole Table
    #[xml(child = "a:wholeTbl")]
    pub whole_table: Option<WholeTable>,
    ///Band 1 Horizontal
    #[xml(child = "a:band1H")]
    pub band1_horizontal: Option<Band1Horizontal>,
    ///Band 2 Horizontal
    #[xml(child = "a:band2H")]
    pub band2_horizontal: Option<Band2Horizontal>,
    ///Band 1 Vertical
    #[xml(child = "a:band1V")]
    pub band1_vertical: Option<Band1Vertical>,
    ///Band 2 Vertical
    #[xml(child = "a:band2V")]
    pub band2_vertical: Option<Band2Vertical>,
    ///Last Column
    #[xml(child = "a:lastCol")]
    pub last_column: Option<LastColumn>,
    ///First Column
    #[xml(child = "a:firstCol")]
    pub first_column: Option<FirstColumn>,
    ///Last Row
    #[xml(child = "a:lastRow")]
    pub last_row: Option<LastRow>,
    ///Southeast Cell
    #[xml(child = "a:seCell")]
    pub southeast_cell: Option<SoutheastCell>,
    ///Southwest Cell
    #[xml(child = "a:swCell")]
    pub southwest_cell: Option<SouthwestCell>,
    ///First Row
    #[xml(child = "a:firstRow")]
    pub first_row: Option<FirstRow>,
    ///Northeast Cell
    #[xml(child = "a:neCell")]
    pub northeast_cell: Option<NortheastCell>,
    ///Northwest Cell
    #[xml(child = "a:nwCell")]
    pub northwest_cell: Option<NorthwestCell>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Table Style.
/// When the object is serialized out as xml, it's qualified name is a:tblStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tblStyle")]
pub struct TableStyleEntry {
    /// Style ID
    /// Represents the following attribute in the schema: :styleId
    #[xml(attr = "styleId")]
    pub style_id: String,
    /// Name
    /// Represents the following attribute in the schema: :styleName
    #[xml(attr = "styleName")]
    pub style_name: String,
    ///Table Background
    #[xml(child = "a:tblBg")]
    pub table_background: Option<TableBackground>,
    ///Whole Table
    #[xml(child = "a:wholeTbl")]
    pub whole_table: Option<WholeTable>,
    ///Band 1 Horizontal
    #[xml(child = "a:band1H")]
    pub band1_horizontal: Option<Band1Horizontal>,
    ///Band 2 Horizontal
    #[xml(child = "a:band2H")]
    pub band2_horizontal: Option<Band2Horizontal>,
    ///Band 1 Vertical
    #[xml(child = "a:band1V")]
    pub band1_vertical: Option<Band1Vertical>,
    ///Band 2 Vertical
    #[xml(child = "a:band2V")]
    pub band2_vertical: Option<Band2Vertical>,
    ///Last Column
    #[xml(child = "a:lastCol")]
    pub last_column: Option<LastColumn>,
    ///First Column
    #[xml(child = "a:firstCol")]
    pub first_column: Option<FirstColumn>,
    ///Last Row
    #[xml(child = "a:lastRow")]
    pub last_row: Option<LastRow>,
    ///Southeast Cell
    #[xml(child = "a:seCell")]
    pub southeast_cell: Option<SoutheastCell>,
    ///Southwest Cell
    #[xml(child = "a:swCell")]
    pub southwest_cell: Option<SouthwestCell>,
    ///First Row
    #[xml(child = "a:firstRow")]
    pub first_row: Option<FirstRow>,
    ///Northeast Cell
    #[xml(child = "a:neCell")]
    pub northeast_cell: Option<NortheastCell>,
    ///Northwest Cell
    #[xml(child = "a:nwCell")]
    pub northwest_cell: Option<NorthwestCell>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the TableStyleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TableStyleType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Style ID
    /// Represents the following attribute in the schema: :styleId
    #[xml(attr = "styleId")]
    pub style_id: String,
    /// Name
    /// Represents the following attribute in the schema: :styleName
    #[xml(attr = "styleName")]
    pub style_name: String,
}
/// Table Style ID.
/// When the object is serialized out as xml, it's qualified name is a:tableStyleId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tableStyleId")]
pub struct TableStyleId {
    #[xml(text)]
    pub child: String,
}
/// Table Grid Column.
/// When the object is serialized out as xml, it's qualified name is a:gridCol.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:gridCol")]
pub struct GridColumn {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: i64,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Table Properties.
/// When the object is serialized out as xml, it's qualified name is a:tblPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tblPr")]
pub struct TableProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Right-to-Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// First Row
    /// Represents the following attribute in the schema: :firstRow
    #[xml(attr = "firstRow")]
    pub first_row: Option<bool>,
    /// First Column
    /// Represents the following attribute in the schema: :firstCol
    #[xml(attr = "firstCol")]
    pub first_column: Option<bool>,
    /// Last Row
    /// Represents the following attribute in the schema: :lastRow
    #[xml(attr = "lastRow")]
    pub last_row: Option<bool>,
    /// Last Column
    /// Represents the following attribute in the schema: :lastCol
    #[xml(attr = "lastCol")]
    pub last_column: Option<bool>,
    /// Banded Rows
    /// Represents the following attribute in the schema: :bandRow
    #[xml(attr = "bandRow")]
    pub band_row: Option<bool>,
    /// Banded Columns
    /// Represents the following attribute in the schema: :bandCol
    #[xml(attr = "bandCol")]
    pub band_column: Option<bool>,
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:tableStyle",
        child = "a:tableStyleId",
        child = "a:extLst",
    )]
    pub children: Vec<TablePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TablePropertiesChildChoice {
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:tableStyle")]
    ATableStyle(TableStyle),
    #[xml(tag = "a:tableStyleId")]
    ATableStyleId(TableStyleId),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Table Grid.
/// When the object is serialized out as xml, it's qualified name is a:tblGrid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tblGrid")]
pub struct TableGrid {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:gridCol")]
    pub a_grid_col: Vec<GridColumn>,
}
/// Table Row.
/// When the object is serialized out as xml, it's qualified name is a:tr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tr")]
pub struct TableRow {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Height
    /// Represents the following attribute in the schema: :h
    #[xml(attr = "h")]
    pub height: i64,
    /// _
    #[xml(child = "a:tc")]
    pub a_tc: Vec<TableCell>,
    /// _
    #[xml(child = "a:extLst")]
    pub a_ext_lst: Option<ExtensionList>,
}
/// Left Border.
/// When the object is serialized out as xml, it's qualified name is a:left.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:left")]
pub struct LeftBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<LeftBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LeftBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Right Border.
/// When the object is serialized out as xml, it's qualified name is a:right.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:right")]
pub struct RightBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<RightBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RightBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Top Border.
/// When the object is serialized out as xml, it's qualified name is a:top.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:top")]
pub struct TopBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<TopBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TopBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Bottom Border.
/// When the object is serialized out as xml, it's qualified name is a:bottom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:bottom")]
pub struct BottomBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<BottomBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BottomBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Inside Horizontal Border.
/// When the object is serialized out as xml, it's qualified name is a:insideH.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:insideH")]
pub struct InsideHorizontalBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<InsideHorizontalBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InsideHorizontalBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Inside Vertical Border.
/// When the object is serialized out as xml, it's qualified name is a:insideV.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:insideV")]
pub struct InsideVerticalBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<InsideVerticalBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InsideVerticalBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Top Left to Bottom Right Border.
/// When the object is serialized out as xml, it's qualified name is a:tl2br.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tl2br")]
pub struct TopLeftToBottomRightBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<TopLeftToBottomRightBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TopLeftToBottomRightBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Top Right to Bottom Left Border.
/// When the object is serialized out as xml, it's qualified name is a:tr2bl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tr2bl")]
pub struct TopRightToBottomLeftBorder {
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<TopRightToBottomLeftBorderChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TopRightToBottomLeftBorderChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Defines the ThemeableLineStyleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ThemeableLineStyleType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:ln", child = "a:lnRef")]
    pub children: Vec<ThemeableLineStyleTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ThemeableLineStyleTypeChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:lnRef")]
    ALnRef(LineReference),
}
/// Table Cell Borders.
/// When the object is serialized out as xml, it's qualified name is a:tcBdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tcBdr")]
pub struct TableCellBorders {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Left Border
    #[xml(child = "a:left")]
    pub left_border: Option<LeftBorder>,
    ///Right Border
    #[xml(child = "a:right")]
    pub right_border: Option<RightBorder>,
    ///Top Border
    #[xml(child = "a:top")]
    pub top_border: Option<TopBorder>,
    ///Bottom Border
    #[xml(child = "a:bottom")]
    pub bottom_border: Option<BottomBorder>,
    ///Inside Horizontal Border
    #[xml(child = "a:insideH")]
    pub inside_horizontal_border: Option<InsideHorizontalBorder>,
    ///Inside Vertical Border
    #[xml(child = "a:insideV")]
    pub inside_vertical_border: Option<InsideVerticalBorder>,
    ///Top Left to Bottom Right Border
    #[xml(child = "a:tl2br")]
    pub top_left_to_bottom_right_border: Option<TopLeftToBottomRightBorder>,
    ///Top Right to Bottom Left Border
    #[xml(child = "a:tr2bl")]
    pub top_right_to_bottom_left_border: Option<TopRightToBottomLeftBorder>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Table Cell Text Style.
/// When the object is serialized out as xml, it's qualified name is a:tcTxStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tcTxStyle")]
pub struct TableCellTextStyle {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Bold
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<BooleanStyleValues>,
    /// Italic
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<BooleanStyleValues>,
    #[xml(
        child = "a:font",
        child = "a:fontRef",
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
        child = "a:extLst",
    )]
    pub children: Vec<TableCellTextStyleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableCellTextStyleChildChoice {
    #[xml(tag = "a:font")]
    AFont(Fonts),
    #[xml(tag = "a:fontRef")]
    AFontRef(FontReference),
    #[xml(tag = "a:scrgbClr")]
    AScrgbClr(RgbColorModelPercentage),
    #[xml(tag = "a:srgbClr")]
    ASrgbClr(RgbColorModelHex),
    #[xml(tag = "a:hslClr")]
    AHslClr(HslColor),
    #[xml(tag = "a:sysClr")]
    ASysClr(SystemColor),
    #[xml(tag = "a:schemeClr")]
    ASchemeClr(SchemeColor),
    #[xml(tag = "a:prstClr")]
    APrstClr(PresetColor),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Table Cell Style.
/// When the object is serialized out as xml, it's qualified name is a:tcStyle.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tcStyle")]
pub struct TableCellStyle {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:tcBdr", child = "a:fill", child = "a:fillRef", child = "a:cell3D")]
    pub children: Vec<TableCellStyleChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableCellStyleChildChoice {
    #[xml(tag = "a:tcBdr")]
    ATcBdr(TableCellBorders),
    #[xml(tag = "a:fill")]
    AFill(FillProperties),
    #[xml(tag = "a:fillRef")]
    AFillRef(FillReference),
    #[xml(tag = "a:cell3D")]
    ACell3D(Cell3DProperties),
}
/// Table Background.
/// When the object is serialized out as xml, it's qualified name is a:tblBg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tblBg")]
pub struct TableBackground {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:fill",
        child = "a:fillRef",
        child = "a:effect",
        child = "a:effectRef",
    )]
    pub children: Vec<TableBackgroundChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TableBackgroundChildChoice {
    #[xml(tag = "a:fill")]
    AFill(FillProperties),
    #[xml(tag = "a:fillRef")]
    AFillRef(FillReference),
    #[xml(tag = "a:effect")]
    AEffect(EffectPropertiesType),
    #[xml(tag = "a:effectRef")]
    AEffectRef(EffectReference),
}
/// Whole Table.
/// When the object is serialized out as xml, it's qualified name is a:wholeTbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:wholeTbl")]
pub struct WholeTable {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Band 1 Horizontal.
/// When the object is serialized out as xml, it's qualified name is a:band1H.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:band1H")]
pub struct Band1Horizontal {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Band 2 Horizontal.
/// When the object is serialized out as xml, it's qualified name is a:band2H.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:band2H")]
pub struct Band2Horizontal {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Band 1 Vertical.
/// When the object is serialized out as xml, it's qualified name is a:band1V.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:band1V")]
pub struct Band1Vertical {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Band 2 Vertical.
/// When the object is serialized out as xml, it's qualified name is a:band2V.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:band2V")]
pub struct Band2Vertical {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Last Column.
/// When the object is serialized out as xml, it's qualified name is a:lastCol.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lastCol")]
pub struct LastColumn {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// First Column.
/// When the object is serialized out as xml, it's qualified name is a:firstCol.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:firstCol")]
pub struct FirstColumn {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Last Row.
/// When the object is serialized out as xml, it's qualified name is a:lastRow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lastRow")]
pub struct LastRow {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Southeast Cell.
/// When the object is serialized out as xml, it's qualified name is a:seCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:seCell")]
pub struct SoutheastCell {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Southwest Cell.
/// When the object is serialized out as xml, it's qualified name is a:swCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:swCell")]
pub struct SouthwestCell {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// First Row.
/// When the object is serialized out as xml, it's qualified name is a:firstRow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:firstRow")]
pub struct FirstRow {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Northeast Cell.
/// When the object is serialized out as xml, it's qualified name is a:neCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:neCell")]
pub struct NortheastCell {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Northwest Cell.
/// When the object is serialized out as xml, it's qualified name is a:nwCell.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:nwCell")]
pub struct NorthwestCell {
    ///Table Cell Text Style
    #[xml(child = "a:tcTxStyle")]
    pub table_cell_text_style: Option<TableCellTextStyle>,
    ///Table Cell Style
    #[xml(child = "a:tcStyle")]
    pub table_cell_style: Option<TableCellStyle>,
}
/// Defines the TablePartStyleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TablePartStyleType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
}
/// Text Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is a:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:pPr")]
pub struct ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Default Paragraph Style.
/// When the object is serialized out as xml, it's qualified name is a:defPPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:defPPr")]
pub struct DefaultParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<DefaultParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DefaultParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 1 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl1pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl1pPr")]
pub struct Level1ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level1ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level1ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 2 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl2pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl2pPr")]
pub struct Level2ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level2ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level2ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 3 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl3pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl3pPr")]
pub struct Level3ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level3ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level3ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 4 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl4pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl4pPr")]
pub struct Level4ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level4ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level4ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 5 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl5pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl5pPr")]
pub struct Level5ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level5ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level5ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 6 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl6pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl6pPr")]
pub struct Level6ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level6ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level6ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 7 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl7pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl7pPr")]
pub struct Level7ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level7ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level7ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 8 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl8pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl8pPr")]
pub struct Level8ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level8ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level8ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// List Level 9 Text Style.
/// When the object is serialized out as xml, it's qualified name is a:lvl9pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lvl9pPr")]
pub struct Level9ParagraphProperties {
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
    #[xml(
        child = "a:lnSpc",
        child = "a:spcBef",
        child = "a:spcAft",
        child = "a:buClrTx",
        child = "a:buClr",
        child = "a:buSzTx",
        child = "a:buSzPct",
        child = "a:buSzPts",
        child = "a:buFontTx",
        child = "a:buFont",
        child = "a:buNone",
        child = "a:buAutoNum",
        child = "a:buChar",
        child = "a:buBlip",
        child = "a:tabLst",
        child = "a:defRPr",
        child = "a:extLst",
    )]
    pub children: Vec<Level9ParagraphPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum Level9ParagraphPropertiesChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(BulletColorText),
    #[xml(tag = "a:buClr")]
    ABuClr(BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(BulletSizeText),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(BulletSizePercentage),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(BulletSizePoints),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(BulletFontText),
    #[xml(tag = "a:buFont")]
    ABuFont(BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(AutoNumberedBullet),
    #[xml(tag = "a:buChar")]
    ABuChar(CharacterBullet),
    #[xml(tag = "a:buBlip")]
    ABuBlip(PictureBullet),
    #[xml(tag = "a:tabLst")]
    ATabLst(TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(DefaultRunProperties),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Defines the TextParagraphPropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextParagraphPropertiesType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Left Margin
    /// Represents the following attribute in the schema: :marL
    #[xml(attr = "marL")]
    pub left_margin: Option<i32>,
    /// Right Margin
    /// Represents the following attribute in the schema: :marR
    #[xml(attr = "marR")]
    pub right_margin: Option<i32>,
    /// Level
    /// Represents the following attribute in the schema: :lvl
    #[xml(attr = "lvl")]
    pub level: Option<i32>,
    /// Indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextAlignmentTypeValues>,
    /// Default Tab Size
    /// Represents the following attribute in the schema: :defTabSz
    #[xml(attr = "defTabSz")]
    pub default_tab_size: Option<i32>,
    /// Right To Left
    /// Represents the following attribute in the schema: :rtl
    #[xml(attr = "rtl")]
    pub right_to_left: Option<bool>,
    /// East Asian Line Break
    /// Represents the following attribute in the schema: :eaLnBrk
    #[xml(attr = "eaLnBrk")]
    pub east_asian_line_break: Option<bool>,
    /// Font Alignment
    /// Represents the following attribute in the schema: :fontAlgn
    #[xml(attr = "fontAlgn")]
    pub font_alignment: Option<TextFontAlignmentValues>,
    /// Latin Line Break
    /// Represents the following attribute in the schema: :latinLnBrk
    #[xml(attr = "latinLnBrk")]
    pub latin_line_break: Option<bool>,
    /// Hanging Punctuation
    /// Represents the following attribute in the schema: :hangingPunct
    #[xml(attr = "hangingPunct")]
    pub height: Option<bool>,
}
/// End Paragraph Run Properties.
/// When the object is serialized out as xml, it's qualified name is a:endParaRPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:endParaRPr")]
pub struct EndParagraphRunProperties {
    /// kumimoji
    /// Represents the following attribute in the schema: :kumimoji
    #[xml(attr = "kumimoji")]
    pub kumimoji: Option<bool>,
    /// lang
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// altLang
    /// Represents the following attribute in the schema: :altLang
    #[xml(attr = "altLang")]
    pub alternative_language: Option<String>,
    /// sz
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub font_size: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// u
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub underline: Option<TextUnderlineValues>,
    /// strike
    /// Represents the following attribute in the schema: :strike
    #[xml(attr = "strike")]
    pub strike: Option<TextStrikeValues>,
    /// kern
    /// Represents the following attribute in the schema: :kern
    #[xml(attr = "kern")]
    pub kerning: Option<i32>,
    /// cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub capital: Option<TextCapsValues>,
    /// spc
    /// Represents the following attribute in the schema: :spc
    #[xml(attr = "spc")]
    pub spacing: Option<i32>,
    /// normalizeH
    /// Represents the following attribute in the schema: :normalizeH
    #[xml(attr = "normalizeH")]
    pub normalize_height: Option<bool>,
    /// baseline
    /// Represents the following attribute in the schema: :baseline
    #[xml(attr = "baseline")]
    pub baseline: Option<i32>,
    /// noProof
    /// Represents the following attribute in the schema: :noProof
    #[xml(attr = "noProof")]
    pub no_proof: Option<bool>,
    /// dirty
    /// Represents the following attribute in the schema: :dirty
    #[xml(attr = "dirty")]
    pub dirty: Option<bool>,
    /// err
    /// Represents the following attribute in the schema: :err
    #[xml(attr = "err")]
    pub spelling_error: Option<bool>,
    /// smtClean
    /// Represents the following attribute in the schema: :smtClean
    #[xml(attr = "smtClean")]
    pub smart_tag_clean: Option<bool>,
    /// smtId
    /// Represents the following attribute in the schema: :smtId
    #[xml(attr = "smtId")]
    pub smart_tag_id: Option<i32>,
    /// bmk
    /// Represents the following attribute in the schema: :bmk
    #[xml(attr = "bmk")]
    pub bookmark: Option<String>,
    #[xml(
        child = "a:ln",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:highlight",
        child = "a:uLnTx",
        child = "a:uLn",
        child = "a:uFillTx",
        child = "a:uFill",
        child = "a:latin",
        child = "a:ea",
        child = "a:cs",
        child = "a:sym",
        child = "a:hlinkClick",
        child = "a:hlinkMouseOver",
        child = "a:rtl",
        child = "a:extLst",
    )]
    pub children: Vec<EndParagraphRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndParagraphRunPropertiesChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:highlight")]
    AHighlight(Highlight),
    #[xml(tag = "a:uLnTx")]
    AULnTx(UnderlineFollowsText),
    #[xml(tag = "a:uLn")]
    AULn(Underline),
    #[xml(tag = "a:uFillTx")]
    AUFillTx(UnderlineFillText),
    #[xml(tag = "a:uFill")]
    AUFill(UnderlineFill),
    #[xml(tag = "a:latin")]
    ALatin(LatinFont),
    #[xml(tag = "a:ea")]
    AEa(EastAsianFont),
    #[xml(tag = "a:cs")]
    ACs(ComplexScriptFont),
    #[xml(tag = "a:sym")]
    ASym(SymbolFont),
    #[xml(tag = "a:hlinkClick")]
    AHlinkClick(HyperlinkOnClick),
    #[xml(tag = "a:hlinkMouseOver")]
    AHlinkMouseOver(HyperlinkOnMouseOver),
    #[xml(tag = "a:rtl")]
    ARtl(RightToLeft),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Text Run Properties.
/// When the object is serialized out as xml, it's qualified name is a:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:rPr")]
pub struct RunProperties {
    /// kumimoji
    /// Represents the following attribute in the schema: :kumimoji
    #[xml(attr = "kumimoji")]
    pub kumimoji: Option<bool>,
    /// lang
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// altLang
    /// Represents the following attribute in the schema: :altLang
    #[xml(attr = "altLang")]
    pub alternative_language: Option<String>,
    /// sz
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub font_size: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// u
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub underline: Option<TextUnderlineValues>,
    /// strike
    /// Represents the following attribute in the schema: :strike
    #[xml(attr = "strike")]
    pub strike: Option<TextStrikeValues>,
    /// kern
    /// Represents the following attribute in the schema: :kern
    #[xml(attr = "kern")]
    pub kerning: Option<i32>,
    /// cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub capital: Option<TextCapsValues>,
    /// spc
    /// Represents the following attribute in the schema: :spc
    #[xml(attr = "spc")]
    pub spacing: Option<i32>,
    /// normalizeH
    /// Represents the following attribute in the schema: :normalizeH
    #[xml(attr = "normalizeH")]
    pub normalize_height: Option<bool>,
    /// baseline
    /// Represents the following attribute in the schema: :baseline
    #[xml(attr = "baseline")]
    pub baseline: Option<i32>,
    /// noProof
    /// Represents the following attribute in the schema: :noProof
    #[xml(attr = "noProof")]
    pub no_proof: Option<bool>,
    /// dirty
    /// Represents the following attribute in the schema: :dirty
    #[xml(attr = "dirty")]
    pub dirty: Option<bool>,
    /// err
    /// Represents the following attribute in the schema: :err
    #[xml(attr = "err")]
    pub spelling_error: Option<bool>,
    /// smtClean
    /// Represents the following attribute in the schema: :smtClean
    #[xml(attr = "smtClean")]
    pub smart_tag_clean: Option<bool>,
    /// smtId
    /// Represents the following attribute in the schema: :smtId
    #[xml(attr = "smtId")]
    pub smart_tag_id: Option<i32>,
    /// bmk
    /// Represents the following attribute in the schema: :bmk
    #[xml(attr = "bmk")]
    pub bookmark: Option<String>,
    #[xml(
        child = "a:ln",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:highlight",
        child = "a:uLnTx",
        child = "a:uLn",
        child = "a:uFillTx",
        child = "a:uFill",
        child = "a:latin",
        child = "a:ea",
        child = "a:cs",
        child = "a:sym",
        child = "a:hlinkClick",
        child = "a:hlinkMouseOver",
        child = "a:rtl",
        child = "a:extLst",
    )]
    pub children: Vec<RunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunPropertiesChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:highlight")]
    AHighlight(Highlight),
    #[xml(tag = "a:uLnTx")]
    AULnTx(UnderlineFollowsText),
    #[xml(tag = "a:uLn")]
    AULn(Underline),
    #[xml(tag = "a:uFillTx")]
    AUFillTx(UnderlineFillText),
    #[xml(tag = "a:uFill")]
    AUFill(UnderlineFill),
    #[xml(tag = "a:latin")]
    ALatin(LatinFont),
    #[xml(tag = "a:ea")]
    AEa(EastAsianFont),
    #[xml(tag = "a:cs")]
    ACs(ComplexScriptFont),
    #[xml(tag = "a:sym")]
    ASym(SymbolFont),
    #[xml(tag = "a:hlinkClick")]
    AHlinkClick(HyperlinkOnClick),
    #[xml(tag = "a:hlinkMouseOver")]
    AHlinkMouseOver(HyperlinkOnMouseOver),
    #[xml(tag = "a:rtl")]
    ARtl(RightToLeft),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Default Text Run Properties.
/// When the object is serialized out as xml, it's qualified name is a:defRPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:defRPr")]
pub struct DefaultRunProperties {
    /// kumimoji
    /// Represents the following attribute in the schema: :kumimoji
    #[xml(attr = "kumimoji")]
    pub kumimoji: Option<bool>,
    /// lang
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// altLang
    /// Represents the following attribute in the schema: :altLang
    #[xml(attr = "altLang")]
    pub alternative_language: Option<String>,
    /// sz
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub font_size: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// u
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub underline: Option<TextUnderlineValues>,
    /// strike
    /// Represents the following attribute in the schema: :strike
    #[xml(attr = "strike")]
    pub strike: Option<TextStrikeValues>,
    /// kern
    /// Represents the following attribute in the schema: :kern
    #[xml(attr = "kern")]
    pub kerning: Option<i32>,
    /// cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub capital: Option<TextCapsValues>,
    /// spc
    /// Represents the following attribute in the schema: :spc
    #[xml(attr = "spc")]
    pub spacing: Option<i32>,
    /// normalizeH
    /// Represents the following attribute in the schema: :normalizeH
    #[xml(attr = "normalizeH")]
    pub normalize_height: Option<bool>,
    /// baseline
    /// Represents the following attribute in the schema: :baseline
    #[xml(attr = "baseline")]
    pub baseline: Option<i32>,
    /// noProof
    /// Represents the following attribute in the schema: :noProof
    #[xml(attr = "noProof")]
    pub no_proof: Option<bool>,
    /// dirty
    /// Represents the following attribute in the schema: :dirty
    #[xml(attr = "dirty")]
    pub dirty: Option<bool>,
    /// err
    /// Represents the following attribute in the schema: :err
    #[xml(attr = "err")]
    pub spelling_error: Option<bool>,
    /// smtClean
    /// Represents the following attribute in the schema: :smtClean
    #[xml(attr = "smtClean")]
    pub smart_tag_clean: Option<bool>,
    /// smtId
    /// Represents the following attribute in the schema: :smtId
    #[xml(attr = "smtId")]
    pub smart_tag_id: Option<i32>,
    /// bmk
    /// Represents the following attribute in the schema: :bmk
    #[xml(attr = "bmk")]
    pub bookmark: Option<String>,
    #[xml(
        child = "a:ln",
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
        child = "a:highlight",
        child = "a:uLnTx",
        child = "a:uLn",
        child = "a:uFillTx",
        child = "a:uFill",
        child = "a:latin",
        child = "a:ea",
        child = "a:cs",
        child = "a:sym",
        child = "a:hlinkClick",
        child = "a:hlinkMouseOver",
        child = "a:rtl",
        child = "a:extLst",
    )]
    pub children: Vec<DefaultRunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DefaultRunPropertiesChildChoice {
    #[xml(tag = "a:ln")]
    ALn(Outline),
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:highlight")]
    AHighlight(Highlight),
    #[xml(tag = "a:uLnTx")]
    AULnTx(UnderlineFollowsText),
    #[xml(tag = "a:uLn")]
    AULn(Underline),
    #[xml(tag = "a:uFillTx")]
    AUFillTx(UnderlineFillText),
    #[xml(tag = "a:uFill")]
    AUFill(UnderlineFill),
    #[xml(tag = "a:latin")]
    ALatin(LatinFont),
    #[xml(tag = "a:ea")]
    AEa(EastAsianFont),
    #[xml(tag = "a:cs")]
    ACs(ComplexScriptFont),
    #[xml(tag = "a:sym")]
    ASym(SymbolFont),
    #[xml(tag = "a:hlinkClick")]
    AHlinkClick(HyperlinkOnClick),
    #[xml(tag = "a:hlinkMouseOver")]
    AHlinkMouseOver(HyperlinkOnMouseOver),
    #[xml(tag = "a:rtl")]
    ARtl(RightToLeft),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Defines the TextCharacterPropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextCharacterPropertiesType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// kumimoji
    /// Represents the following attribute in the schema: :kumimoji
    #[xml(attr = "kumimoji")]
    pub kumimoji: Option<bool>,
    /// lang
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// altLang
    /// Represents the following attribute in the schema: :altLang
    #[xml(attr = "altLang")]
    pub alternative_language: Option<String>,
    /// sz
    /// Represents the following attribute in the schema: :sz
    #[xml(attr = "sz")]
    pub font_size: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub bold: Option<bool>,
    /// i
    /// Represents the following attribute in the schema: :i
    #[xml(attr = "i")]
    pub italic: Option<bool>,
    /// u
    /// Represents the following attribute in the schema: :u
    #[xml(attr = "u")]
    pub underline: Option<TextUnderlineValues>,
    /// strike
    /// Represents the following attribute in the schema: :strike
    #[xml(attr = "strike")]
    pub strike: Option<TextStrikeValues>,
    /// kern
    /// Represents the following attribute in the schema: :kern
    #[xml(attr = "kern")]
    pub kerning: Option<i32>,
    /// cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub capital: Option<TextCapsValues>,
    /// spc
    /// Represents the following attribute in the schema: :spc
    #[xml(attr = "spc")]
    pub spacing: Option<i32>,
    /// normalizeH
    /// Represents the following attribute in the schema: :normalizeH
    #[xml(attr = "normalizeH")]
    pub normalize_height: Option<bool>,
    /// baseline
    /// Represents the following attribute in the schema: :baseline
    #[xml(attr = "baseline")]
    pub baseline: Option<i32>,
    /// noProof
    /// Represents the following attribute in the schema: :noProof
    #[xml(attr = "noProof")]
    pub no_proof: Option<bool>,
    /// dirty
    /// Represents the following attribute in the schema: :dirty
    #[xml(attr = "dirty")]
    pub dirty: Option<bool>,
    /// err
    /// Represents the following attribute in the schema: :err
    #[xml(attr = "err")]
    pub spelling_error: Option<bool>,
    /// smtClean
    /// Represents the following attribute in the schema: :smtClean
    #[xml(attr = "smtClean")]
    pub smart_tag_clean: Option<bool>,
    /// smtId
    /// Represents the following attribute in the schema: :smtId
    #[xml(attr = "smtId")]
    pub smart_tag_id: Option<i32>,
    /// bmk
    /// Represents the following attribute in the schema: :bmk
    #[xml(attr = "bmk")]
    pub bookmark: Option<String>,
}
/// Text Paragraphs.
/// When the object is serialized out as xml, it's qualified name is a:p.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:p")]
pub struct Paragraph {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:pPr",
        child = "a:r",
        child = "a:br",
        child = "a:fld",
        child = "a14:m",
        child = "a:endParaRPr",
    )]
    pub children: Vec<ParagraphChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphChildChoice {
    #[xml(tag = "a:pPr")]
    APPr(ParagraphProperties),
    #[xml(tag = "a:r")]
    AR(Run),
    #[xml(tag = "a:br")]
    ABr(Break),
    #[xml(tag = "a:fld")]
    AFld(Field),
    #[xml(tag = "a14:m")]
    A14M(crate::schemas::schemas_microsoft_com_office_drawing_2010_main::TextMath),
    #[xml(tag = "a:endParaRPr")]
    AEndParaRPr(EndParagraphRunProperties),
}
/// Tab Stop.
/// When the object is serialized out as xml, it's qualified name is a:tab.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tab")]
pub struct TabStop {
    /// Tab Position
    /// Represents the following attribute in the schema: :pos
    #[xml(attr = "pos")]
    pub position: Option<i32>,
    /// Tab Alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<TextTabAlignmentValues>,
}
/// Spacing Percent.
/// When the object is serialized out as xml, it's qualified name is a:spcPct.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spcPct")]
pub struct SpacingPercent {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Spacing Points.
/// When the object is serialized out as xml, it's qualified name is a:spcPts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spcPts")]
pub struct SpacingPoints {
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: i32,
}
/// Line Spacing.
/// When the object is serialized out as xml, it's qualified name is a:lnSpc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:lnSpc")]
pub struct LineSpacing {
    #[xml(child = "a:spcPct", child = "a:spcPts")]
    pub children: Vec<LineSpacingChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineSpacingChildChoice {
    #[xml(tag = "a:spcPct")]
    ASpcPct(SpacingPercent),
    #[xml(tag = "a:spcPts")]
    ASpcPts(SpacingPoints),
}
/// Space Before.
/// When the object is serialized out as xml, it's qualified name is a:spcBef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spcBef")]
pub struct SpaceBefore {
    #[xml(child = "a:spcPct", child = "a:spcPts")]
    pub children: Vec<SpaceBeforeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SpaceBeforeChildChoice {
    #[xml(tag = "a:spcPct")]
    ASpcPct(SpacingPercent),
    #[xml(tag = "a:spcPts")]
    ASpcPts(SpacingPoints),
}
/// Space After.
/// When the object is serialized out as xml, it's qualified name is a:spcAft.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:spcAft")]
pub struct SpaceAfter {
    #[xml(child = "a:spcPct", child = "a:spcPts")]
    pub children: Vec<SpaceAfterChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SpaceAfterChildChoice {
    #[xml(tag = "a:spcPct")]
    ASpcPct(SpacingPercent),
    #[xml(tag = "a:spcPts")]
    ASpcPts(SpacingPoints),
}
/// Defines the TextSpacingType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TextSpacingType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(child = "a:spcPct", child = "a:spcPts")]
    pub children: Vec<TextSpacingTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextSpacingTypeChildChoice {
    #[xml(tag = "a:spcPct")]
    ASpcPct(SpacingPercent),
    #[xml(tag = "a:spcPts")]
    ASpcPts(SpacingPoints),
}
/// Tab List.
/// When the object is serialized out as xml, it's qualified name is a:tabLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tabLst")]
pub struct TabStopList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:tab")]
    pub a_tab: Vec<TabStop>,
}
/// Defines the Text Class.
/// When the object is serialized out as xml, it's qualified name is a:t.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:t")]
pub struct Text {
    #[xml(text)]
    pub child: String,
}
/// Defines the ShapePropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct ShapePropertiesExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "a14:hiddenFill",
        child = "a14:hiddenLine",
        child = "a14:hiddenEffects",
        child = "a14:hiddenScene3d",
        child = "a14:hiddenSp3d",
        child = "a14:shadowObscured",
    )]
    pub children: Vec<ShapePropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShapePropertiesExtensionChildChoice {
    #[xml(tag = "a14:hiddenFill")]
    A14HiddenFill(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenFillProperties,
    ),
    #[xml(tag = "a14:hiddenLine")]
    A14HiddenLine(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenLineProperties,
    ),
    #[xml(tag = "a14:hiddenEffects")]
    A14HiddenEffects(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenEffectsProperties,
    ),
    #[xml(tag = "a14:hiddenScene3d")]
    A14HiddenScene3d(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenScene3D,
    ),
    #[xml(tag = "a14:hiddenSp3d")]
    A14HiddenSp3d(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::HiddenShape3D,
    ),
    #[xml(tag = "a14:shadowObscured")]
    A14ShadowObscured(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ShadowObscured,
    ),
}
/// Defines the GvmlGroupShapeExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct GvmlGroupShapeExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "a14:isCanvas")]
    pub children: Vec<GvmlGroupShapeExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GvmlGroupShapeExtensionChildChoice {
    #[xml(tag = "a14:isCanvas")]
    A14IsCanvas(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::IsCanvas,
    ),
}
/// Defines the ShapePropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct ShapePropertiesExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<ShapePropertiesExtension>,
}
/// Non-Visual Properties for a Group Shape.
/// When the object is serialized out as xml, it's qualified name is a:nvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:nvGrpSpPr")]
pub struct NonVisualGroupShapeProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:cNvPr")]
    pub non_visual_drawing_properties: NonVisualDrawingProperties,
    ///Non-Visual Group Shape Drawing Properties
    #[xml(child = "a:cNvGrpSpPr")]
    pub non_visual_group_shape_drawing_properties: NonVisualGroupShapeDrawingProperties,
}
/// Visual Group Shape Properties.
/// When the object is serialized out as xml, it's qualified name is a:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:grpSpPr")]
pub struct VisualGroupShapeProperties {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Black and White Mode
    /// Represents the following attribute in the schema: :bwMode
    #[xml(attr = "bwMode")]
    pub black_white_mode: Option<BlackWhiteModeValues>,
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
    pub children: Vec<VisualGroupShapePropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum VisualGroupShapePropertiesChildChoice {
    #[xml(tag = "a:xfrm")]
    AXfrm(TransformGroup),
    #[xml(tag = "a:noFill")]
    ANoFill(NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(SolidFill),
    #[xml(tag = "a:gradFill")]
    AGradFill(GradientFill),
    #[xml(tag = "a:blipFill")]
    ABlipFill(BlipFill),
    #[xml(tag = "a:pattFill")]
    APattFill(PatternFill),
    #[xml(tag = "a:grpFill")]
    AGrpFill(GroupFill),
    #[xml(tag = "a:effectLst")]
    AEffectLst(EffectList),
    #[xml(tag = "a:effectDag")]
    AEffectDag(EffectDag),
    #[xml(tag = "a:scene3d")]
    AScene3d(Scene3DType),
    #[xml(tag = "a:extLst")]
    AExtLst(ExtensionList),
}
/// Shape.
/// When the object is serialized out as xml, it's qualified name is a:sp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:sp")]
pub struct Shape {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Properties for a Shape
    #[xml(child = "a:nvSpPr")]
    pub non_visual_shape_properties: NonVisualShapeProperties,
    ///Visual Properties
    #[xml(child = "a:spPr")]
    pub shape_properties: ShapeProperties,
    ///Text Shape
    #[xml(child = "a:txSp")]
    pub text_shape: Option<TextShape>,
    ///Style
    #[xml(child = "a:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Connection Shape.
/// When the object is serialized out as xml, it's qualified name is a:cxnSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:cxnSp")]
pub struct ConnectionShape {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Properties for a Connection Shape
    #[xml(child = "a:nvCxnSpPr")]
    pub non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
    ///Visual Properties
    #[xml(child = "a:spPr")]
    pub shape_properties: ShapeProperties,
    ///Shape Style
    #[xml(child = "a:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Picture.
/// When the object is serialized out as xml, it's qualified name is a:pic.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:pic")]
pub struct Picture {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Properties for a Picture
    #[xml(child = "a:nvPicPr")]
    pub non_visual_picture_properties: NonVisualPictureProperties,
    ///Picture Fill
    #[xml(child = "a:blipFill")]
    pub blip_fill: BlipFill,
    ///Shape Properties
    #[xml(child = "a:spPr")]
    pub shape_properties: ShapeProperties,
    /// _
    #[xml(child = "a:style")]
    pub shape_style: Option<ShapeStyle>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Graphic Frame.
/// When the object is serialized out as xml, it's qualified name is a:graphicFrame.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:graphicFrame")]
pub struct GraphicFrame {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Non-Visual Properties for a Graphic Frame
    #[xml(child = "a:nvGraphicFramePr")]
    pub non_visual_graphic_frame_properties: NonVisualGraphicFrameProperties,
    /// _
    #[xml(child = "a:graphic")]
    pub graphic: Graphic,
    /// _
    #[xml(child = "a:xfrm")]
    pub transform2_d: Transform2D,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Group shape.
/// When the object is serialized out as xml, it's qualified name is a:grpSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:grpSp")]
pub struct GroupShape {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    #[xml(
        child = "a:nvGrpSpPr",
        child = "a:grpSpPr",
        child = "a:txSp",
        child = "a:sp",
        child = "a:cxnSp",
        child = "a:pic",
        child = "a14:contentPart",
        child = "a:graphicFrame",
        child = "a:grpSp",
        child = "a:extLst",
    )]
    pub children: Vec<GroupShapeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupShapeChildChoice {
    #[xml(tag = "a:nvGrpSpPr")]
    ANvGrpSpPr(NonVisualGroupShapeProperties),
    #[xml(tag = "a:grpSpPr")]
    AGrpSpPr(VisualGroupShapeProperties),
    #[xml(tag = "a:txSp")]
    ATxSp(TextShape),
    #[xml(tag = "a:sp")]
    ASp(Shape),
    #[xml(tag = "a:cxnSp")]
    ACxnSp(ConnectionShape),
    #[xml(tag = "a:pic")]
    APic(Picture),
    #[xml(tag = "a14:contentPart")]
    A14ContentPart(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::GvmlContentPart,
    ),
    #[xml(tag = "a:graphicFrame")]
    AGraphicFrame(GraphicFrame),
    #[xml(tag = "a:grpSp")]
    AGrpSp(GroupShape),
    #[xml(tag = "a:extLst")]
    AExtLst(GvmlGroupShapeExtensionList),
}
/// Defines the GvmlGroupShapeExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct GvmlGroupShapeExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<GvmlGroupShapeExtension>,
}
/// Defines the NonVisualGroupDrawingShapePropsExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct NonVisualGroupDrawingShapePropsExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "a15:nonVisualGroupProps")]
    pub children: Vec<NonVisualGroupDrawingShapePropsExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NonVisualGroupDrawingShapePropsExtensionChildChoice {
    #[xml(tag = "a15:nonVisualGroupProps")]
    A15NonVisualGroupProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::NonVisualGroupProperties,
    ),
}
/// Defines the OfficeStyleSheetExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct OfficeStyleSheetExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "thm15:themeFamily")]
    pub children: Vec<OfficeStyleSheetExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeStyleSheetExtensionChildChoice {
    #[xml(tag = "thm15:themeFamily")]
    Thm15ThemeFamily(
        crate::schemas::schemas_microsoft_com_office_thememl_2012_main::ThemeFamily,
    ),
}
/// Defines the ConnectorLockingExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct ConnectorLockingExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "a:graphic")]
    pub children: Vec<ConnectorLockingExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ConnectorLockingExtensionChildChoice {
    #[xml(tag = "a:graphic")]
    AGraphic(Graphic),
}
/// Defines the GroupShapeLocks Class.
/// When the object is serialized out as xml, it's qualified name is a:grpSpLocks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:grpSpLocks")]
pub struct GroupShapeLocks {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Disallow Shape Grouping
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grouping: Option<bool>,
    /// Disallow Shape Ungrouping
    /// Represents the following attribute in the schema: :noUngrp
    #[xml(attr = "noUngrp")]
    pub no_ungrouping: Option<bool>,
    /// Disallow Shape Selection
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_selection: Option<bool>,
    /// Disallow Shape Rotation
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rotation: Option<bool>,
    /// Disallow Aspect Ratio Change
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// Disallow Moving Shape
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// Disallow Shape Resizing
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the NonVisualGroupDrawingShapePropsExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct NonVisualGroupDrawingShapePropsExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<NonVisualGroupDrawingShapePropsExtension>,
}
/// Defines the ObjectDefaults Class.
/// When the object is serialized out as xml, it's qualified name is a:objectDefaults.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:objectDefaults")]
pub struct ObjectDefaults {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Shape Default
    #[xml(child = "a:spDef")]
    pub shape_default: Option<ShapeDefault>,
    ///Line Default
    #[xml(child = "a:lnDef")]
    pub line_default: Option<LineDefault>,
    ///Text Default
    #[xml(child = "a:txDef")]
    pub text_default: Option<TextDefault>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the ExtraColorSchemeList Class.
/// When the object is serialized out as xml, it's qualified name is a:extraClrSchemeLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extraClrSchemeLst")]
pub struct ExtraColorSchemeList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:extraClrScheme")]
    pub a_extra_clr_scheme: Vec<ExtraColorScheme>,
}
/// Defines the CustomColorList Class.
/// When the object is serialized out as xml, it's qualified name is a:custClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:custClrLst")]
pub struct CustomColorList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:custClr")]
    pub a_cust_clr: Vec<CustomColor>,
}
/// Defines the OfficeStyleSheetExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct OfficeStyleSheetExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<OfficeStyleSheetExtension>,
}
/// Defines the HyperlinkOnClick Class.
/// When the object is serialized out as xml, it's qualified name is a:hlinkClick.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hlinkClick")]
pub struct HyperlinkOnClick {
    /// relationship identifier to find target URI
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
    /// Represents the following attribute in the schema: :invalidUrl
    #[xml(attr = "invalidUrl")]
    pub invalid_url: Option<String>,
    /// Action to take, it may still need r:id to specify an action target
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// target frame for navigating to the URI
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub target_frame: Option<String>,
    /// tooltip for display
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// whether to add this URI to the history when navigating to it
    /// Represents the following attribute in the schema: :history
    #[xml(attr = "history")]
    pub history: Option<bool>,
    /// Whether to highlight it when click on a shape
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// Whether to stop previous sound when click on it
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_sound: Option<bool>,
    ///Sound to play.
    #[xml(child = "a:snd")]
    pub hyperlink_sound: Option<HyperlinkSound>,
    ///Future extensions.
    #[xml(child = "a:extLst")]
    pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkOnMouseOver Class.
/// When the object is serialized out as xml, it's qualified name is a:hlinkMouseOver.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hlinkMouseOver")]
pub struct HyperlinkOnMouseOver {
    /// relationship identifier to find target URI
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
    /// Represents the following attribute in the schema: :invalidUrl
    #[xml(attr = "invalidUrl")]
    pub invalid_url: Option<String>,
    /// Action to take, it may still need r:id to specify an action target
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// target frame for navigating to the URI
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub target_frame: Option<String>,
    /// tooltip for display
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// whether to add this URI to the history when navigating to it
    /// Represents the following attribute in the schema: :history
    #[xml(attr = "history")]
    pub history: Option<bool>,
    /// Whether to highlight it when click on a shape
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// Whether to stop previous sound when click on it
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_sound: Option<bool>,
    ///Sound to play.
    #[xml(child = "a:snd")]
    pub hyperlink_sound: Option<HyperlinkSound>,
    ///Future extensions.
    #[xml(child = "a:extLst")]
    pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkOnHover Class.
/// When the object is serialized out as xml, it's qualified name is a:hlinkHover.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:hlinkHover")]
pub struct HyperlinkOnHover {
    /// relationship identifier to find target URI
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
    /// Represents the following attribute in the schema: :invalidUrl
    #[xml(attr = "invalidUrl")]
    pub invalid_url: Option<String>,
    /// Action to take, it may still need r:id to specify an action target
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// target frame for navigating to the URI
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub target_frame: Option<String>,
    /// tooltip for display
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// whether to add this URI to the history when navigating to it
    /// Represents the following attribute in the schema: :history
    #[xml(attr = "history")]
    pub history: Option<bool>,
    /// Whether to highlight it when click on a shape
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// Whether to stop previous sound when click on it
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_sound: Option<bool>,
    ///Sound to play.
    #[xml(child = "a:snd")]
    pub hyperlink_sound: Option<HyperlinkSound>,
    ///Future extensions.
    #[xml(child = "a:extLst")]
    pub hyperlink_extension_list: Option<HyperlinkExtensionList>,
}
/// Defines the HyperlinkType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct HyperlinkType {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// relationship identifier to find target URI
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// In case the url is invalid so we can't create a relationship, we'll save it here, r:id will point to a NULL one
    /// Represents the following attribute in the schema: :invalidUrl
    #[xml(attr = "invalidUrl")]
    pub invalid_url: Option<String>,
    /// Action to take, it may still need r:id to specify an action target
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// target frame for navigating to the URI
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub target_frame: Option<String>,
    /// tooltip for display
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// whether to add this URI to the history when navigating to it
    /// Represents the following attribute in the schema: :history
    #[xml(attr = "history")]
    pub history: Option<bool>,
    /// Whether to highlight it when click on a shape
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// Whether to stop previous sound when click on it
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_sound: Option<bool>,
}
/// Defines the RightToLeft Class.
/// When the object is serialized out as xml, it's qualified name is a:rtl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:rtl")]
pub struct RightToLeft {
    /// val
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Defines the NonVisualDrawingPropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct NonVisualDrawingPropertiesExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<NonVisualDrawingPropertiesExtension>,
}
/// Defines the ConnectorLockingExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct ConnectorLockingExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<ConnectorLockingExtension>,
}
/// Defines the DataModelExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct DataModelExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "dsp:dataModelExt", child = "dgm14:recolorImg")]
    pub children: Vec<DataModelExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DataModelExtensionChildChoice {
    #[xml(tag = "dsp:dataModelExt")]
    DspDataModelExt(
        crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::DataModelExtensionBlock,
    ),
    #[xml(tag = "dgm14:recolorImg")]
    Dgm14RecolorImg(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_diagram::RecolorImages,
    ),
}
/// Defines the PtExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct PtExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "dgm14:cNvPr")]
    pub children: Vec<PtExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum PtExtensionChildChoice {
    #[xml(tag = "dgm14:cNvPr")]
    Dgm14CNvPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_diagram::NonVisualDrawingProperties,
    ),
}
/// Defines the HyperlinkExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct HyperlinkExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "ahyp:hlinkClr")]
    pub children: Vec<HyperlinkExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum HyperlinkExtensionChildChoice {
    #[xml(tag = "ahyp:hlinkClr")]
    AhypHlinkClr(
        crate::schemas::schemas_microsoft_com_office_drawing_2018_hyperlinkcolor::HyperlinkColor,
    ),
}
/// Future extensions..
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct HyperlinkExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<HyperlinkExtension>,
}
/// Defines the LinePropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct LinePropertiesExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "ask:lineSketchStyleProps")]
    pub children: Vec<LinePropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LinePropertiesExtensionChildChoice {
    #[xml(tag = "ask:lineSketchStyleProps")]
    AskLineSketchStyleProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2018_sketchyshapes::LineSketchStyleProperties,
    ),
}
/// default head line end style is none.
/// When the object is serialized out as xml, it's qualified name is a:headEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:headEnd")]
pub struct HeadEnd {
    /// Line Head/End Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<LineEndValues>,
    /// Width of Head/End
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<LineEndWidthValues>,
    /// Length of Head/End
    /// Represents the following attribute in the schema: :len
    #[xml(attr = "len")]
    pub length: Option<LineEndLengthValues>,
}
/// default tail line end style is none.
/// When the object is serialized out as xml, it's qualified name is a:tailEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:tailEnd")]
pub struct TailEnd {
    /// Line Head/End Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<LineEndValues>,
    /// Width of Head/End
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<LineEndWidthValues>,
    /// Length of Head/End
    /// Represents the following attribute in the schema: :len
    #[xml(attr = "len")]
    pub length: Option<LineEndLengthValues>,
}
/// Defines the LineEndPropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LineEndPropertiesType {
    /// Line Head/End Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<LineEndValues>,
    /// Width of Head/End
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<LineEndWidthValues>,
    /// Length of Head/End
    /// Represents the following attribute in the schema: :len
    #[xml(attr = "len")]
    pub length: Option<LineEndLengthValues>,
}
/// Future extensions..
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct LinePropertiesExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<LinePropertiesExtension>,
}
/// Defines the NonVisualDrawingPropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct NonVisualDrawingPropertiesExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "a14:compatExt",
        child = "a15:backgroundPr",
        child = "a16:creationId",
        child = "a16:predDERef",
        child = "adec:decorative",
        child = "aclsh:classification",
        child = "asl:scriptLink",
    )]
    pub children: Vec<NonVisualDrawingPropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NonVisualDrawingPropertiesExtensionChildChoice {
    #[xml(tag = "a14:compatExt")]
    A14CompatExt(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::CompatExtension,
    ),
    #[xml(tag = "a15:backgroundPr")]
    A15BackgroundPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::BackgroundProperties,
    ),
    #[xml(tag = "a16:creationId")]
    A16CreationId(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::CreationId,
    ),
    #[xml(tag = "a16:predDERef")]
    A16PredDeRef(
        crate::schemas::schemas_microsoft_com_office_drawing_2014_main::PredecessorDrawingElementReference,
    ),
    #[xml(tag = "adec:decorative")]
    AdecDecorative(
        crate::schemas::schemas_microsoft_com_office_drawing_2017_decorative::Decorative,
    ),
    #[xml(tag = "aclsh:classification")]
    AclshClassification(
        crate::schemas::schemas_microsoft_com_office_drawing_2020_classification_shape::ClassificationOutcome,
    ),
    #[xml(tag = "asl:scriptLink")]
    AslScriptLink(
        crate::schemas::schemas_microsoft_com_office_drawing_2021_scriptlink::ScriptLink,
    ),
}
/// Defines the PictureLocks Class.
/// When the object is serialized out as xml, it's qualified name is a:picLocks.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:picLocks")]
pub struct PictureLocks {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Disallow Shape Grouping
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grouping: Option<bool>,
    /// Disallow Shape Selection
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_selection: Option<bool>,
    /// Disallow Shape Rotation
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rotation: Option<bool>,
    /// Disallow Aspect Ratio Change
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// Disallow Shape Movement
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// Disallow Shape Resize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// Disallow Shape Point Editing
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// Disallow Showing Adjust Handles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// Disallow Arrowhead Changes
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// Disallow Shape Type Change
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// Disallow Crop Changes
    /// Represents the following attribute in the schema: :noCrop
    #[xml(attr = "noCrop")]
    pub no_crop: Option<bool>,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the NonVisualPicturePropertiesExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct NonVisualPicturePropertiesExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<NonVisualPicturePropertiesExtension>,
}
/// Defines the NonVisualPicturePropertiesExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct NonVisualPicturePropertiesExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "a14:cameraTool",
        child = "a15:signatureLine",
        child = "a15:objectPr",
        child = "alf:liveFeedProps",
        child = "aif:imageFormula",
    )]
    pub children: Vec<NonVisualPicturePropertiesExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NonVisualPicturePropertiesExtensionChildChoice {
    #[xml(tag = "a14:cameraTool")]
    A14CameraTool(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::CameraTool,
    ),
    #[xml(tag = "a15:signatureLine")]
    A15SignatureLine(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::SignatureLine,
    ),
    #[xml(tag = "a15:objectPr")]
    A15ObjectPr(
        crate::schemas::schemas_microsoft_com_office_drawing_2012_main::ObjectProperties,
    ),
    #[xml(tag = "alf:liveFeedProps")]
    AlfLiveFeedProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2021_livefeed::LiveFeedProperties,
    ),
    #[xml(tag = "aif:imageFormula")]
    AifImageFormula(
        crate::schemas::schemas_microsoft_com_office_drawing_2022_imageformula::ImageFormula,
    ),
}
/// Future extensions..
/// When the object is serialized out as xml, it's qualified name is a:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:extLst")]
pub struct BlipExtensionList {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<BlipExtension>,
}
/// Defines the BlipExtension Class.
/// When the object is serialized out as xml, it's qualified name is a:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "a:ext")]
pub struct BlipExtension {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(
        child = "a14:imgProps",
        child = "a14:useLocalDpi",
        child = "wp15:webVideoPr",
        child = "asvg:svgBlip",
        child = "a1611:picAttrSrcUrl",
        child = "woe:oembed",
        child = "aoe:oembedShared",
    )]
    pub children: Vec<BlipExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipExtensionChildChoice {
    #[xml(tag = "a14:imgProps")]
    A14ImgProps(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ImageProperties,
    ),
    #[xml(tag = "a14:useLocalDpi")]
    A14UseLocalDpi(
        crate::schemas::schemas_microsoft_com_office_drawing_2010_main::UseLocalDpi,
    ),
    #[xml(tag = "wp15:webVideoPr")]
    Wp15WebVideoPr(
        crate::schemas::schemas_microsoft_com_office_word_2012_wordprocessing_drawing::WebVideoProperty,
    ),
    #[xml(tag = "asvg:svgBlip")]
    AsvgSvgBlip(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_svg_main::SvgBlip,
    ),
    #[xml(tag = "a1611:picAttrSrcUrl")]
    A1611PicAttrSrcUrl(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_11_main::PictureAttributionSourceUrl,
    ),
    #[xml(tag = "woe:oembed")]
    WoeOembed(crate::schemas::schemas_microsoft_com_office_word_2020_oembed::OEmbed),
    #[xml(tag = "aoe:oembedShared")]
    AoeOembedShared(
        crate::schemas::schemas_microsoft_com_office_drawing_2021_oembed::OEmbedShared,
    ),
}
