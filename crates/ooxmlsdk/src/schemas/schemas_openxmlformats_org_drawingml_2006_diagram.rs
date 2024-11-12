#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ColorApplicationMethodValues {
    #[default]
    Span,
    Cycle,
    Repeat,
}
crate::__string_enum! {
    ColorApplicationMethodValues { Span = "span", Cycle = "cycle", Repeat = "repeat", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HueDirectionValues {
    #[default]
    Clockwise,
    Counterclockwise,
}
crate::__string_enum! {
    HueDirectionValues { Clockwise = "cw", Counterclockwise = "ccw", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PointValues {
    #[default]
    Node,
    Assistant,
    Document,
    Presentation,
    ParentTransition,
    SiblingTransition,
}
crate::__string_enum! {
    PointValues { Node = "node", Assistant = "asst", Document = "doc", Presentation =
    "pres", ParentTransition = "parTrans", SiblingTransition = "sibTrans", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectionValues {
    #[default]
    ParentOf,
    PresentationOf,
    PresentationParentOf,
    UnknownRelationship,
}
crate::__string_enum! {
    ConnectionValues { ParentOf = "parOf", PresentationOf = "presOf",
    PresentationParentOf = "presParOf", UnknownRelationship = "unknownRelationship", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DirectionValues {
    #[default]
    Normal,
    Reversed,
}
crate::__string_enum! {
    DirectionValues { Normal = "norm", Reversed = "rev", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HierarchyBranchStyleValues {
    #[default]
    Left,
    Right,
    Hanging,
    Standard,
    Initial,
}
crate::__string_enum! {
    HierarchyBranchStyleValues { Left = "l", Right = "r", Hanging = "hang", Standard =
    "std", Initial = "init", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimateOneByOneValues {
    #[default]
    None,
    One,
    Branch,
}
crate::__string_enum! {
    AnimateOneByOneValues { None = "none", One = "one", Branch = "branch", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnimationLevelStringValues {
    #[default]
    None,
    Level,
    Center,
}
crate::__string_enum! {
    AnimationLevelStringValues { None = "none", Level = "lvl", Center = "ctr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ResizeHandlesStringValues {
    #[default]
    Exact,
    Relative,
}
crate::__string_enum! {
    ResizeHandlesStringValues { Exact = "exact", Relative = "rel", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AlgorithmValues {
    #[default]
    Composite,
    Connector,
    Cycle,
    HierarchyChild,
    HierarchyRoot,
    Pyramid,
    Linear,
    Space,
    Text,
    Snake,
}
crate::__string_enum! {
    AlgorithmValues { Composite = "composite", Connector = "conn", Cycle = "cycle",
    HierarchyChild = "hierChild", HierarchyRoot = "hierRoot", Pyramid = "pyra", Linear =
    "lin", Space = "sp", Text = "tx", Snake = "snake", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AxisValues {
    #[default]
    _Self,
    Child,
    Descendant,
    DescendantOrSelf,
    Parent,
    Ancestor,
    AncestorOrSelf,
    FollowSibling,
    PrecedingSibling,
    Follow,
    Preceding,
    Root,
    None,
}
crate::__string_enum! {
    AxisValues { _Self = "self", Child = "ch", Descendant = "des", DescendantOrSelf =
    "desOrSelf", Parent = "par", Ancestor = "ancst", AncestorOrSelf = "ancstOrSelf",
    FollowSibling = "followSib", PrecedingSibling = "precedSib", Follow = "follow",
    Preceding = "preced", Root = "root", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BoolOperatorValues {
    #[default]
    None,
    Equal,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}
crate::__string_enum! {
    BoolOperatorValues { None = "none", Equal = "equ", GreaterThanOrEqualTo = "gte",
    LessThanOrEqualTo = "lte", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChildOrderValues {
    #[default]
    Bottom,
    Top,
}
crate::__string_enum! {
    ChildOrderValues { Bottom = "b", Top = "t", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConstraintValues {
    #[default]
    None,
    AlignmentOffset,
    BeginningMargin,
    BendingDistance,
    BeginningPadding,
    Bottom,
    BottomMargin,
    BottomOffset,
    CenterHeight,
    CenterXOffset,
    CenterWidth,
    CenterYOffset,
    ConnectionDistance,
    Diameter,
    EndMargin,
    EndPadding,
    Height,
    ArrowheadHeight,
    HeightOffset,
    Left,
    LeftMargin,
    LeftOffset,
    Right,
    RightMargin,
    RightOffset,
    PrimaryFontSize,
    PyramidAccentRatio,
    SecondaryFontSize,
    SiblingSpacing,
    SecondarySiblingSpacing,
    Spacing,
    StemThickness,
    Top,
    TopMargin,
    TopOffset,
    UserDefinedA,
    UserDefinedB,
    UserDefinedC,
    UserDefinedD,
    UserDefinedE,
    UserDefinedF,
    UserDefinedG,
    UserDefinedH,
    UserDefinedI,
    UserDefinedJ,
    UserDefinedK,
    UserDefinedL,
    UserDefinedM,
    UserDefinedN,
    UserDefinedO,
    UserDefinedP,
    UserDefinedQ,
    UserDefinedR,
    UserDefinedS,
    UserDefinedT,
    UserDefinedU,
    UserDefinedV,
    UserDefinedW,
    UserDefinedX,
    UserDefinedY,
    UserDefinedZ,
    Width,
    ArrowheadWidth,
    WidthOffset,
}
crate::__string_enum! {
    ConstraintValues { None = "none", AlignmentOffset = "alignOff", BeginningMargin =
    "begMarg", BendingDistance = "bendDist", BeginningPadding = "begPad", Bottom = "b",
    BottomMargin = "bMarg", BottomOffset = "bOff", CenterHeight = "ctrX", CenterXOffset =
    "ctrXOff", CenterWidth = "ctrY", CenterYOffset = "ctrYOff", ConnectionDistance =
    "connDist", Diameter = "diam", EndMargin = "endMarg", EndPadding = "endPad", Height =
    "h", ArrowheadHeight = "hArH", HeightOffset = "hOff", Left = "l", LeftMargin =
    "lMarg", LeftOffset = "lOff", Right = "r", RightMargin = "rMarg", RightOffset =
    "rOff", PrimaryFontSize = "primFontSz", PyramidAccentRatio = "pyraAcctRatio",
    SecondaryFontSize = "secFontSz", SiblingSpacing = "sibSp", SecondarySiblingSpacing =
    "secSibSp", Spacing = "sp", StemThickness = "stemThick", Top = "t", TopMargin =
    "tMarg", TopOffset = "tOff", UserDefinedA = "userA", UserDefinedB = "userB",
    UserDefinedC = "userC", UserDefinedD = "userD", UserDefinedE = "userE", UserDefinedF
    = "userF", UserDefinedG = "userG", UserDefinedH = "userH", UserDefinedI = "userI",
    UserDefinedJ = "userJ", UserDefinedK = "userK", UserDefinedL = "userL", UserDefinedM
    = "userM", UserDefinedN = "userN", UserDefinedO = "userO", UserDefinedP = "userP",
    UserDefinedQ = "userQ", UserDefinedR = "userR", UserDefinedS = "userS", UserDefinedT
    = "userT", UserDefinedU = "userU", UserDefinedV = "userV", UserDefinedW = "userW",
    UserDefinedX = "userX", UserDefinedY = "userY", UserDefinedZ = "userZ", Width = "w",
    ArrowheadWidth = "wArH", WidthOffset = "wOff", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConstraintRelationshipValues {
    #[default]
    _Self,
    Child,
    Descendant,
}
crate::__string_enum! {
    ConstraintRelationshipValues { _Self = "self", Child = "ch", Descendant = "des", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ElementValues {
    #[default]
    All,
    Document,
    Node,
    Normal,
    NonNormal,
    Assistant,
    NonAssistant,
    ParentTransition,
    Presentation,
    SiblingTransition,
}
crate::__string_enum! {
    ElementValues { All = "all", Document = "doc", Node = "node", Normal = "norm",
    NonNormal = "nonNorm", Assistant = "asst", NonAssistant = "nonAsst", ParentTransition
    = "parTrans", Presentation = "pres", SiblingTransition = "sibTrans", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ParameterIdValues {
    #[default]
    HorizontalAlignment,
    VerticalAlignment,
    ChildDirection,
    ChildAlignment,
    SecondaryChildAlignment,
    LinearDirection,
    SecondaryLinearDirection,
    StartElement,
    BendPoint,
    ConnectionRoute,
    BeginningArrowheadStyle,
    EndStyle,
    ConnectorDimension,
    RotationPath,
    CenterShapeMapping,
    NodeHorizontalAlignment,
    NodeVerticalAlignment,
    FallbackScale,
    TextDirection,
    PyramidAccentPosition,
    PyramidAccentTextMargin,
    TextBlockDirection,
    TextAnchorHorizontal,
    TextAnchorVertical,
    TextAnchorHorizontalWithChildren,
    TextAnchorVerticalWithChildren,
    ParentTextLeftToRightAlignment,
    ParentTextRightToLeftAlignment,
    ShapeTextLeftToRightAlignment,
    ShapeTextRightToLeftAlignment,
    AutoTextRotation,
    GrowDirection,
    FlowDirection,
    ContinueDirection,
    Breakpoint,
    Offset,
    HierarchyAlignment,
    BreakpointFixedValue,
    StartBulletsAtLevel,
    StartAngle,
    SpanAngle,
    AspectRatio,
    LineSpacingParent,
    LineSpacingAfterParentParagraph,
    LineSpacingChildren,
    LineSpacingAfterChildrenParagraph,
    RouteShortestDistance,
    TextAlignment,
    PyramidLevelNode,
    PyramidAccentBackgroundNode,
    PyramidAccentTextNode,
    SourceNode,
    DestinationNode,
    BeginningPoints,
    EndPoints,
}
crate::__string_enum! {
    ParameterIdValues { HorizontalAlignment = "horzAlign", VerticalAlignment =
    "vertAlign", ChildDirection = "chDir", ChildAlignment = "chAlign",
    SecondaryChildAlignment = "secChAlign", LinearDirection = "linDir",
    SecondaryLinearDirection = "secLinDir", StartElement = "stElem", BendPoint =
    "bendPt", ConnectionRoute = "connRout", BeginningArrowheadStyle = "begSty", EndStyle
    = "endSty", ConnectorDimension = "dim", RotationPath = "rotPath", CenterShapeMapping
    = "ctrShpMap", NodeHorizontalAlignment = "nodeHorzAlign", NodeVerticalAlignment =
    "nodeVertAlign", FallbackScale = "fallback", TextDirection = "txDir",
    PyramidAccentPosition = "pyraAcctPos", PyramidAccentTextMargin = "pyraAcctTxMar",
    TextBlockDirection = "txBlDir", TextAnchorHorizontal = "txAnchorHorz",
    TextAnchorVertical = "txAnchorVert", TextAnchorHorizontalWithChildren =
    "txAnchorHorzCh", TextAnchorVerticalWithChildren = "txAnchorVertCh",
    ParentTextLeftToRightAlignment = "parTxLtrAlign", ParentTextRightToLeftAlignment =
    "parTxRtlAlign", ShapeTextLeftToRightAlignment = "shpTxLtrAlignCh",
    ShapeTextRightToLeftAlignment = "shpTxRtlAlignCh", AutoTextRotation = "autoTxRot",
    GrowDirection = "grDir", FlowDirection = "flowDir", ContinueDirection = "contDir",
    Breakpoint = "bkpt", Offset = "off", HierarchyAlignment = "hierAlign",
    BreakpointFixedValue = "bkPtFixedVal", StartBulletsAtLevel = "stBulletLvl",
    StartAngle = "stAng", SpanAngle = "spanAng", AspectRatio = "ar", LineSpacingParent =
    "lnSpPar", LineSpacingAfterParentParagraph = "lnSpAfParP", LineSpacingChildren =
    "lnSpCh", LineSpacingAfterChildrenParagraph = "lnSpAfChP", RouteShortestDistance =
    "rtShortDist", TextAlignment = "alignTx", PyramidLevelNode = "pyraLvlNode",
    PyramidAccentBackgroundNode = "pyraAcctBkgdNode", PyramidAccentTextNode =
    "pyraAcctTxNode", SourceNode = "srcNode", DestinationNode = "dstNode",
    BeginningPoints = "begPts", EndPoints = "endPts", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FunctionValues {
    #[default]
    Count,
    Position,
    ReversePosition,
    PositionEven,
    PositionOdd,
    Variable,
    Depth,
    MaxDepth,
}
crate::__string_enum! {
    FunctionValues { Count = "cnt", Position = "pos", ReversePosition = "revPos",
    PositionEven = "posEven", PositionOdd = "posOdd", Variable = "var", Depth = "depth",
    MaxDepth = "maxDepth", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FunctionOperatorValues {
    #[default]
    Equal,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
}
crate::__string_enum! {
    FunctionOperatorValues { Equal = "equ", NotEqualTo = "neq", GreaterThan = "gt",
    LessThan = "lt", GreaterThanOrEqualTo = "gte", LessThanOrEqualTo = "lte", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
    None,
}
crate::__string_enum! {
    HorizontalAlignmentValues { Left = "l", Center = "ctr", Right = "r", None = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChildDirectionValues {
    #[default]
    Horizontal,
    Vertical,
}
crate::__string_enum! {
    ChildDirectionValues { Horizontal = "horz", Vertical = "vert", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChildAlignmentValues {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}
crate::__string_enum! {
    ChildAlignmentValues { Top = "t", Bottom = "b", Left = "l", Right = "r", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SecondaryChildAlignmentValues {
    #[default]
    None,
    Top,
    Bottom,
    Left,
    Right,
}
crate::__string_enum! {
    SecondaryChildAlignmentValues { None = "none", Top = "t", Bottom = "b", Left = "l",
    Right = "r", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LinearDirectionValues {
    #[default]
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
}
crate::__string_enum! {
    LinearDirectionValues { FromLeft = "fromL", FromRight = "fromR", FromTop = "fromT",
    FromBottom = "fromB", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SecondaryLinearDirectionValues {
    #[default]
    None,
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
}
crate::__string_enum! {
    SecondaryLinearDirectionValues { None = "none", FromLeft = "fromL", FromRight =
    "fromR", FromTop = "fromT", FromBottom = "fromB", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StartingElementValues {
    #[default]
    Node,
    Transition,
}
crate::__string_enum! {
    StartingElementValues { Node = "node", Transition = "trans", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RotationPathValues {
    #[default]
    None,
    AlongPath,
}
crate::__string_enum! {
    RotationPathValues { None = "none", AlongPath = "alongPath", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CenterShapeMappingValues {
    #[default]
    None,
    FirstNode,
}
crate::__string_enum! {
    CenterShapeMappingValues { None = "none", FirstNode = "fNode", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BendPointValues {
    #[default]
    Beginning,
    Default,
    End,
}
crate::__string_enum! {
    BendPointValues { Beginning = "beg", Default = "def", End = "end", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectorRoutingValues {
    #[default]
    Straight,
    Bending,
    Curve,
    LongCurve,
}
crate::__string_enum! {
    ConnectorRoutingValues { Straight = "stra", Bending = "bend", Curve = "curve",
    LongCurve = "longCurve", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ArrowheadStyleValues {
    #[default]
    Auto,
    Arrow,
    NoArrow,
}
crate::__string_enum! {
    ArrowheadStyleValues { Auto = "auto", Arrow = "arr", NoArrow = "noArr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectorDimensionValues {
    #[default]
    OneDimension,
    TwoDimension,
    Custom,
}
crate::__string_enum! {
    ConnectorDimensionValues { OneDimension = "1d", TwoDimension = "2d", Custom = "cust",
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConnectorPointValues {
    #[default]
    Auto,
    BottomCenter,
    Center,
    MiddleLeft,
    MiddleRight,
    TopCenter,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
    Radial,
}
crate::__string_enum! {
    ConnectorPointValues { Auto = "auto", BottomCenter = "bCtr", Center = "ctr",
    MiddleLeft = "midL", MiddleRight = "midR", TopCenter = "tCtr", BottomLeft = "bL",
    BottomRight = "bR", TopLeft = "tL", TopRight = "tR", Radial = "radial", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NodeHorizontalAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    NodeHorizontalAlignmentValues { Left = "l", Center = "ctr", Right = "r", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NodeVerticalAlignmentValues {
    #[default]
    Top,
    Middle,
    Bottom,
}
crate::__string_enum! {
    NodeVerticalAlignmentValues { Top = "t", Middle = "mid", Bottom = "b", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FallbackDimensionValues {
    #[default]
    OneDimension,
    TwoDimension,
}
crate::__string_enum! {
    FallbackDimensionValues { OneDimension = "1d", TwoDimension = "2d", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextDirectionValues {
    #[default]
    FromTop,
    FromBottom,
}
crate::__string_enum! {
    TextDirectionValues { FromTop = "fromT", FromBottom = "fromB", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PyramidAccentPositionValues {
    #[default]
    Before,
    After,
}
crate::__string_enum! {
    PyramidAccentPositionValues { Before = "bef", After = "aft", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PyramidAccentTextMarginValues {
    #[default]
    Step,
    Stack,
}
crate::__string_enum! {
    PyramidAccentTextMarginValues { Step = "step", Stack = "stack", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextBlockDirectionValues {
    #[default]
    Horizontal,
    Vertical,
}
crate::__string_enum! {
    TextBlockDirectionValues { Horizontal = "horz", Vertical = "vert", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextAnchorHorizontalValues {
    #[default]
    None,
    Center,
}
crate::__string_enum! {
    TextAnchorHorizontalValues { None = "none", Center = "ctr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextAnchorVerticalValues {
    #[default]
    Top,
    Middle,
    Bottom,
}
crate::__string_enum! {
    TextAnchorVerticalValues { Top = "t", Middle = "mid", Bottom = "b", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TextAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    TextAlignmentValues { Left = "l", Center = "ctr", Right = "r", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AutoTextRotationValues {
    #[default]
    None,
    Upright,
    Gravity,
}
crate::__string_enum! {
    AutoTextRotationValues { None = "none", Upright = "upr", Gravity = "grav", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GrowDirectionValues {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}
crate::__string_enum! {
    GrowDirectionValues { TopLeft = "tL", TopRight = "tR", BottomLeft = "bL", BottomRight
    = "bR", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FlowDirectionValues {
    #[default]
    Row,
    Column,
}
crate::__string_enum! {
    FlowDirectionValues { Row = "row", Column = "col", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ContinueDirectionValues {
    #[default]
    ReverseDirection,
    SameDirection,
}
crate::__string_enum! {
    ContinueDirectionValues { ReverseDirection = "revDir", SameDirection = "sameDir", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BreakpointValues {
    #[default]
    EndCanvas,
    Balanced,
    Fixed,
}
crate::__string_enum! {
    BreakpointValues { EndCanvas = "endCnv", Balanced = "bal", Fixed = "fixed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OffsetValues {
    #[default]
    Center,
    Offset,
}
crate::__string_enum! {
    OffsetValues { Center = "ctr", Offset = "off", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HierarchyAlignmentValues {
    #[default]
    TopLeft,
    TopRight,
    TopCenterChildren,
    TopCenterDescendants,
    BottomLeft,
    BottomRight,
    BottomCenterChild,
    BottomCenterDescendant,
    LeftTop,
    LeftBottom,
    LeftCenterChild,
    LeftCenterDescendant,
    RightTop,
    RightBottom,
    RightCenterChildren,
    RightCenterDescendants,
}
crate::__string_enum! {
    HierarchyAlignmentValues { TopLeft = "tL", TopRight = "tR", TopCenterChildren =
    "tCtrCh", TopCenterDescendants = "tCtrDes", BottomLeft = "bL", BottomRight = "bR",
    BottomCenterChild = "bCtrCh", BottomCenterDescendant = "bCtrDes", LeftTop = "lT",
    LeftBottom = "lB", LeftCenterChild = "lCtrCh", LeftCenterDescendant = "lCtrDes",
    RightTop = "rT", RightBottom = "rB", RightCenterChildren = "rCtrCh",
    RightCenterDescendants = "rCtrDes", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VariableValues {
    #[default]
    None,
    OrganizationalChart,
    ChildMax,
    ChildPreference,
    BulletsEnabled,
    Direction,
    HierarchyBranch,
    AnimateOne,
    AnimationLevel,
    ResizeHandles,
}
crate::__string_enum! {
    VariableValues { None = "none", OrganizationalChart = "orgChart", ChildMax = "chMax",
    ChildPreference = "chPref", BulletsEnabled = "bulEnabled", Direction = "dir",
    HierarchyBranch = "hierBranch", AnimateOne = "animOne", AnimationLevel = "animLvl",
    ResizeHandles = "resizeHandles", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OutputShapeValues {
    #[default]
    None,
    Connection,
}
crate::__string_enum! {
    OutputShapeValues { None = "none", Connection = "conn", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAlignmentValues {
    #[default]
    Top,
    Middle,
    Bottom,
    None,
    Top2010,
    Middle2010,
    Bottom2010,
}
crate::__string_enum! {
    VerticalAlignmentValues { Top = "t", Middle = "mid", Bottom = "b", None = "none",
    Top2010 = "top", Middle2010 = "center", Bottom2010 = "bottom", }
}
/// Color Transform Definitions.
/// When the object is serialized out as xml, it's qualified name is dgm:colorsDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:colorsDef")]
pub struct ColorsDefinition {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Unique ID
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: Option<String>,
    /// Minimum Version
    /// Represents the following attribute in the schema: :minVer
    #[xml(attr = "minVer")]
    pub min_version: Option<String>,
    /// _
    #[xml(child = "dgm:title")]
    pub dgm_title: Vec<ColorDefinitionTitle>,
    /// _
    #[xml(child = "dgm:desc")]
    pub dgm_desc: Vec<ColorTransformDescription>,
    /// _
    #[xml(child = "dgm:catLst")]
    pub dgm_cat_lst: Option<ColorTransformCategories>,
    /// _
    #[xml(child = "dgm:styleLbl")]
    pub dgm_style_lbl: Vec<ColorTransformStyleLabel>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<ExtensionList>,
}
/// Color Transform Header.
/// When the object is serialized out as xml, it's qualified name is dgm:colorsDefHdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:colorsDefHdr")]
pub struct ColorsDefinitionHeader {
    /// Unique ID
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: String,
    /// Minimum Version
    /// Represents the following attribute in the schema: :minVer
    #[xml(attr = "minVer")]
    pub min_version: Option<String>,
    /// Resource ID
    /// Represents the following attribute in the schema: :resId
    #[xml(attr = "resId")]
    pub resource_id: Option<i32>,
    /// _
    #[xml(child = "dgm:title")]
    pub dgm_title: Vec<ColorDefinitionTitle>,
    /// _
    #[xml(child = "dgm:desc")]
    pub dgm_desc: Vec<ColorTransformDescription>,
    /// _
    #[xml(child = "dgm:catLst")]
    pub dgm_cat_lst: Option<ColorTransformCategories>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<ExtensionList>,
}
/// Color Transform Header List.
/// When the object is serialized out as xml, it's qualified name is dgm:colorsDefHdrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:colorsDefHdrLst")]
pub struct ColorsDefinitionHeaderList {
    /// _
    #[xml(child = "dgm:colorsDefHdr")]
    pub dgm_colors_def_hdr: Vec<ColorsDefinitionHeader>,
}
/// Data Model.
/// When the object is serialized out as xml, it's qualified name is dgm:dataModel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:dataModel")]
pub struct DataModelRoot {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    ///Point List
    #[xml(child = "dgm:ptLst")]
    pub point_list: PointList,
    ///Connection List
    #[xml(child = "dgm:cxnLst")]
    pub connection_list: Option<ConnectionList>,
    ///Background Formatting
    #[xml(child = "dgm:bg")]
    pub background: Option<Background>,
    ///Whole E2O Formatting
    #[xml(child = "dgm:whole")]
    pub whole: Option<Whole>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub data_model_extension_list: Option<DataModelExtensionList>,
}
/// Layout Definition.
/// When the object is serialized out as xml, it's qualified name is dgm:layoutDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:layoutDef")]
pub struct LayoutDefinition {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// uniqueId
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: Option<String>,
    /// minVer
    /// Represents the following attribute in the schema: :minVer
    #[xml(attr = "minVer")]
    pub min_version: Option<String>,
    /// defStyle
    /// Represents the following attribute in the schema: :defStyle
    #[xml(attr = "defStyle")]
    pub default_style: Option<String>,
    /// _
    #[xml(child = "dgm:title")]
    pub dgm_title: Vec<Title>,
    /// _
    #[xml(child = "dgm:desc")]
    pub dgm_desc: Vec<Description>,
    /// _
    #[xml(child = "dgm:catLst")]
    pub dgm_cat_lst: Option<CategoryList>,
    /// _
    #[xml(child = "dgm:sampData")]
    pub dgm_samp_data: Option<SampleData>,
    /// _
    #[xml(child = "dgm:styleData")]
    pub dgm_style_data: Option<StyleData>,
    /// _
    #[xml(child = "dgm:clrData")]
    pub dgm_clr_data: Option<ColorData>,
    /// _
    #[xml(child = "dgm:layoutNode")]
    pub dgm_layout_node: LayoutNode,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<DiagramDefinitionExtensionList>,
}
/// Layout Definition Header.
/// When the object is serialized out as xml, it's qualified name is dgm:layoutDefHdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:layoutDefHdr")]
pub struct LayoutDefinitionHeader {
    /// Unique Identifier
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: String,
    /// Minimum Version
    /// Represents the following attribute in the schema: :minVer
    #[xml(attr = "minVer")]
    pub min_version: Option<String>,
    /// Default Style
    /// Represents the following attribute in the schema: :defStyle
    #[xml(attr = "defStyle")]
    pub default_style: Option<String>,
    /// Resource Identifier
    /// Represents the following attribute in the schema: :resId
    #[xml(attr = "resId")]
    pub resource_id: Option<i32>,
    /// _
    #[xml(child = "dgm:title")]
    pub dgm_title: Vec<Title>,
    /// _
    #[xml(child = "dgm:desc")]
    pub dgm_desc: Vec<Description>,
    /// _
    #[xml(child = "dgm:catLst")]
    pub dgm_cat_lst: Option<CategoryList>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<ExtensionList>,
}
/// Diagram Layout Header List.
/// When the object is serialized out as xml, it's qualified name is dgm:layoutDefHdrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:layoutDefHdrLst")]
pub struct LayoutDefinitionHeaderList {
    /// _
    #[xml(child = "dgm:layoutDefHdr")]
    pub dgm_layout_def_hdr: Vec<LayoutDefinitionHeader>,
}
/// Explicit Relationships to Diagram Parts.
/// When the object is serialized out as xml, it's qualified name is dgm:relIds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:relIds")]
pub struct RelationshipIds {
    /// Explicit Relationship to Diagram Data Part
    /// Represents the following attribute in the schema: r:dm
    #[xml(attr = "r:dm")]
    pub data_part: String,
    /// Explicit Relationship to Diagram Layout Definition Part
    /// Represents the following attribute in the schema: r:lo
    #[xml(attr = "r:lo")]
    pub layout_part: String,
    /// Explicit Relationship to Style Definition Part
    /// Represents the following attribute in the schema: r:qs
    #[xml(attr = "r:qs")]
    pub style_part: String,
    /// Explicit Relationship to Diagram Colors Part
    /// Represents the following attribute in the schema: r:cs
    #[xml(attr = "r:cs")]
    pub color_part: String,
}
/// Style Definition.
/// When the object is serialized out as xml, it's qualified name is dgm:styleDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:styleDef")]
pub struct StyleDefinition {
    #[xml(attr = "xmlns")]
    pub xmlns: Option<String>,
    #[xml(prefix = "xmlns")]
    pub xmlns_map: std::collections::HashMap<String, String>,
    #[xml(attr = "mc:Ignorable")]
    pub mc_ignorable: Option<String>,
    /// Unique Style ID
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: Option<String>,
    /// Minimum Version
    /// Represents the following attribute in the schema: :minVer
    #[xml(attr = "minVer")]
    pub min_version: Option<String>,
    /// _
    #[xml(child = "dgm:title")]
    pub dgm_title: Vec<StyleDefinitionTitle>,
    /// _
    #[xml(child = "dgm:desc")]
    pub dgm_desc: Vec<StyleLabelDescription>,
    /// _
    #[xml(child = "dgm:catLst")]
    pub dgm_cat_lst: Option<StyleDisplayCategories>,
    /// _
    #[xml(child = "dgm:scene3d")]
    pub dgm_scene3d: Option<Scene3D>,
    /// _
    #[xml(child = "dgm:styleLbl")]
    pub dgm_style_lbl: Vec<StyleLabel>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<ExtensionList>,
}
/// Style Definition Header.
/// When the object is serialized out as xml, it's qualified name is dgm:styleDefHdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:styleDefHdr")]
pub struct StyleDefinitionHeader {
    /// Unique Style ID
    /// Represents the following attribute in the schema: :uniqueId
    #[xml(attr = "uniqueId")]
    pub unique_id: String,
    /// Minimum Version
    /// Represents the following attribute in the schema: :minVer
    #[xml(attr = "minVer")]
    pub min_version: Option<String>,
    /// Resource ID
    /// Represents the following attribute in the schema: :resId
    #[xml(attr = "resId")]
    pub resource_id: Option<i32>,
    /// _
    #[xml(child = "dgm:title")]
    pub dgm_title: Vec<StyleDefinitionTitle>,
    /// _
    #[xml(child = "dgm:desc")]
    pub dgm_desc: Vec<StyleLabelDescription>,
    /// _
    #[xml(child = "dgm:catLst")]
    pub dgm_cat_lst: Option<StyleDisplayCategories>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<ExtensionList>,
}
/// List of Style Definition Headers.
/// When the object is serialized out as xml, it's qualified name is dgm:styleDefHdrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:styleDefHdrLst")]
pub struct StyleDefinitionHeaderList {
    /// _
    #[xml(child = "dgm:styleDefHdr")]
    pub dgm_style_def_hdr: Vec<StyleDefinitionHeader>,
}
/// Color Transform Category.
/// When the object is serialized out as xml, it's qualified name is dgm:cat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:cat")]
pub struct ColorTransformCategory {
    /// Category Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// Priority
    /// Represents the following attribute in the schema: :pri
    #[xml(attr = "pri")]
    pub priority: i32,
}
/// Fill Color List.
/// When the object is serialized out as xml, it's qualified name is dgm:fillClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:fillClrLst")]
pub struct FillColorList {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<FillColorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillColorListChildChoice {
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
/// Line Color List.
/// When the object is serialized out as xml, it's qualified name is dgm:linClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:linClrLst")]
pub struct LineColorList {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<LineColorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LineColorListChildChoice {
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
/// Effect Color List.
/// When the object is serialized out as xml, it's qualified name is dgm:effectClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:effectClrLst")]
pub struct EffectColorList {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<EffectColorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectColorListChildChoice {
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
/// Text Line Color List.
/// When the object is serialized out as xml, it's qualified name is dgm:txLinClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:txLinClrLst")]
pub struct TextLineColorList {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<TextLineColorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextLineColorListChildChoice {
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
/// Text Fill Color List.
/// When the object is serialized out as xml, it's qualified name is dgm:txFillClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:txFillClrLst")]
pub struct TextFillColorList {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<TextFillColorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextFillColorListChildChoice {
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
/// Text Effect Color List.
/// When the object is serialized out as xml, it's qualified name is dgm:txEffectClrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:txEffectClrLst")]
pub struct TextEffectColorList {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<TextEffectColorListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextEffectColorListChildChoice {
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
/// Defines the ColorsType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorsType {
    /// Color Application Method Type
    /// Represents the following attribute in the schema: :meth
    #[xml(attr = "meth")]
    pub method: Option<ColorApplicationMethodValues>,
    /// Hue Direction
    /// Represents the following attribute in the schema: :hueDir
    #[xml(attr = "hueDir")]
    pub hue_direction: Option<HueDirectionValues>,
    #[xml(
        child = "a:scrgbClr",
        child = "a:srgbClr",
        child = "a:hslClr",
        child = "a:sysClr",
        child = "a:schemeClr",
        child = "a:prstClr",
    )]
    pub children: Vec<ColorsTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorsTypeChildChoice {
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
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is dgm:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:extLst")]
pub struct ExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
/// Title.
/// When the object is serialized out as xml, it's qualified name is dgm:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:title")]
pub struct ColorDefinitionTitle {
    /// Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Description Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Description.
/// When the object is serialized out as xml, it's qualified name is dgm:desc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:desc")]
pub struct ColorTransformDescription {
    /// Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Description Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Color Transform Category List.
/// When the object is serialized out as xml, it's qualified name is dgm:catLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:catLst")]
pub struct ColorTransformCategories {
    /// _
    #[xml(child = "dgm:cat")]
    pub dgm_cat: Vec<ColorTransformCategory>,
}
/// Style Label.
/// When the object is serialized out as xml, it's qualified name is dgm:styleLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:styleLbl")]
pub struct ColorTransformStyleLabel {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    ///Fill Color List
    #[xml(child = "dgm:fillClrLst")]
    pub fill_color_list: Option<FillColorList>,
    ///Line Color List
    #[xml(child = "dgm:linClrLst")]
    pub line_color_list: Option<LineColorList>,
    ///Effect Color List
    #[xml(child = "dgm:effectClrLst")]
    pub effect_color_list: Option<EffectColorList>,
    ///Text Line Color List
    #[xml(child = "dgm:txLinClrLst")]
    pub text_line_color_list: Option<TextLineColorList>,
    ///Text Fill Color List
    #[xml(child = "dgm:txFillClrLst")]
    pub text_fill_color_list: Option<TextFillColorList>,
    ///Text Effect Color List
    #[xml(child = "dgm:txEffectClrLst")]
    pub text_effect_color_list: Option<TextEffectColorList>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Point.
/// When the object is serialized out as xml, it's qualified name is dgm:pt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:pt")]
pub struct Point {
    /// Model Identifier
    /// Represents the following attribute in the schema: :modelId
    #[xml(attr = "modelId")]
    pub model_id: String,
    /// Point Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<PointValues>,
    /// Connection Identifier
    /// Represents the following attribute in the schema: :cxnId
    #[xml(attr = "cxnId")]
    pub connection_id: Option<String>,
    ///Property Set
    #[xml(child = "dgm:prSet")]
    pub property_set: Option<PropertySet>,
    ///Shape Properties
    #[xml(child = "dgm:spPr")]
    pub shape_properties: Option<ShapeProperties>,
    ///Text Body
    #[xml(child = "dgm:t")]
    pub text_body: Option<TextBody>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub pt_extension_list: Option<PtExtensionList>,
}
/// Connection.
/// When the object is serialized out as xml, it's qualified name is dgm:cxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:cxn")]
pub struct Connection {
    /// Model Identifier
    /// Represents the following attribute in the schema: :modelId
    #[xml(attr = "modelId")]
    pub model_id: String,
    /// Point Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<ConnectionValues>,
    /// Source Identifier
    /// Represents the following attribute in the schema: :srcId
    #[xml(attr = "srcId")]
    pub source_id: String,
    /// Destination Identifier
    /// Represents the following attribute in the schema: :destId
    #[xml(attr = "destId")]
    pub destination_id: String,
    /// Source Position
    /// Represents the following attribute in the schema: :srcOrd
    #[xml(attr = "srcOrd")]
    pub source_position: i32,
    /// Destination Position
    /// Represents the following attribute in the schema: :destOrd
    #[xml(attr = "destOrd")]
    pub destination_position: i32,
    /// Parent Transition Identifier
    /// Represents the following attribute in the schema: :parTransId
    #[xml(attr = "parTransId")]
    pub parent_transition_id: Option<String>,
    /// Sibling Transition Identifier
    /// Represents the following attribute in the schema: :sibTransId
    #[xml(attr = "sibTransId")]
    pub sibling_transition_id: Option<String>,
    /// Presentation Identifier
    /// Represents the following attribute in the schema: :presId
    #[xml(attr = "presId")]
    pub presentation_id: Option<String>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Constraint.
/// When the object is serialized out as xml, it's qualified name is dgm:constr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:constr")]
pub struct Constraint {
    /// Constraint Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ConstraintValues,
    /// For
    /// Represents the following attribute in the schema: :for
    #[xml(attr = "for")]
    pub r#for: Option<ConstraintRelationshipValues>,
    /// For Name
    /// Represents the following attribute in the schema: :forName
    #[xml(attr = "forName")]
    pub for_name: Option<String>,
    /// Data Point Type
    /// Represents the following attribute in the schema: :ptType
    #[xml(attr = "ptType")]
    pub point_type: Option<ElementValues>,
    /// Reference Type
    /// Represents the following attribute in the schema: :refType
    #[xml(attr = "refType")]
    pub reference_type: Option<ConstraintValues>,
    /// Reference For
    /// Represents the following attribute in the schema: :refFor
    #[xml(attr = "refFor")]
    pub reference_for: Option<ConstraintRelationshipValues>,
    /// Reference For Name
    /// Represents the following attribute in the schema: :refForName
    #[xml(attr = "refForName")]
    pub reference_for_name: Option<String>,
    /// Reference Point Type
    /// Represents the following attribute in the schema: :refPtType
    #[xml(attr = "refPtType")]
    pub reference_point_type: Option<ElementValues>,
    /// Operator
    /// Represents the following attribute in the schema: :op
    #[xml(attr = "op")]
    pub operator: Option<BoolOperatorValues>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<f64>,
    /// Factor
    /// Represents the following attribute in the schema: :fact
    #[xml(attr = "fact")]
    pub fact: Option<f64>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Rule.
/// When the object is serialized out as xml, it's qualified name is dgm:rule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:rule")]
pub struct Rule {
    /// Constraint Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ConstraintValues,
    /// For
    /// Represents the following attribute in the schema: :for
    #[xml(attr = "for")]
    pub r#for: Option<ConstraintRelationshipValues>,
    /// For Name
    /// Represents the following attribute in the schema: :forName
    #[xml(attr = "forName")]
    pub for_name: Option<String>,
    /// Data Point Type
    /// Represents the following attribute in the schema: :ptType
    #[xml(attr = "ptType")]
    pub point_type: Option<ElementValues>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<f64>,
    /// Factor
    /// Represents the following attribute in the schema: :fact
    #[xml(attr = "fact")]
    pub fact: Option<f64>,
    /// Max Value
    /// Represents the following attribute in the schema: :max
    #[xml(attr = "max")]
    pub max: Option<f64>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Shape Adjust.
/// When the object is serialized out as xml, it's qualified name is dgm:adj.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:adj")]
pub struct Adjust {
    /// Adjust Handle Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: f64,
}
/// Shape Adjust List.
/// When the object is serialized out as xml, it's qualified name is dgm:adjLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:adjLst")]
pub struct AdjustList {
    /// _
    #[xml(child = "dgm:adj")]
    pub dgm_adj: Vec<Adjust>,
}
/// Parameter.
/// When the object is serialized out as xml, it's qualified name is dgm:param.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:param")]
pub struct Parameter {
    /// Parameter Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: ParameterIdValues,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<String>,
}
/// Algorithm.
/// When the object is serialized out as xml, it's qualified name is dgm:alg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:alg")]
pub struct Algorithm {
    /// Algorithm Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: AlgorithmValues,
    /// Revision Number
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub revision: Option<i32>,
    /// _
    #[xml(child = "dgm:param")]
    pub dgm_param: Vec<Parameter>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub dgm_ext_lst: Option<ExtensionList>,
}
/// Shape.
/// When the object is serialized out as xml, it's qualified name is dgm:shape.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:shape")]
pub struct Shape {
    /// Rotation
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rotation: Option<f64>,
    /// Shape Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<String>,
    /// Relationship to Image Part
    /// Represents the following attribute in the schema: r:blip
    #[xml(attr = "r:blip")]
    pub blip: Option<String>,
    /// Z-Order Offset
    /// Represents the following attribute in the schema: :zOrderOff
    #[xml(attr = "zOrderOff")]
    pub z_order_offset: Option<i32>,
    /// Hide Geometry
    /// Represents the following attribute in the schema: :hideGeom
    #[xml(attr = "hideGeom")]
    pub hide_geometry: Option<bool>,
    /// Prevent Text Editing
    /// Represents the following attribute in the schema: :lkTxEntry
    #[xml(attr = "lkTxEntry")]
    pub locked_text: Option<bool>,
    /// Image Placeholder
    /// Represents the following attribute in the schema: :blipPhldr
    #[xml(attr = "blipPhldr")]
    pub blip_placeholder: Option<bool>,
    ///Shape Adjust List
    #[xml(child = "dgm:adjLst")]
    pub adjust_list: Option<AdjustList>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Presentation Of.
/// When the object is serialized out as xml, it's qualified name is dgm:presOf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:presOf")]
pub struct PresentationOf {
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<String>,
    /// Data Point Type
    /// Represents the following attribute in the schema: :ptType
    #[xml(attr = "ptType")]
    pub point_type: Option<String>,
    /// Hide Last Transition
    /// Represents the following attribute in the schema: :hideLastTrans
    #[xml(attr = "hideLastTrans")]
    pub hide_last_trans: Option<String>,
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: Option<String>,
    /// Count
    /// Represents the following attribute in the schema: :cnt
    #[xml(attr = "cnt")]
    pub count: Option<String>,
    /// Step
    /// Represents the following attribute in the schema: :step
    #[xml(attr = "step")]
    pub step: Option<String>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Constraint List.
/// When the object is serialized out as xml, it's qualified name is dgm:constrLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:constrLst")]
pub struct Constraints {
    /// _
    #[xml(child = "dgm:constr")]
    pub dgm_constr: Vec<Constraint>,
}
/// Rule List.
/// When the object is serialized out as xml, it's qualified name is dgm:ruleLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:ruleLst")]
pub struct RuleList {
    /// _
    #[xml(child = "dgm:rule")]
    pub dgm_rule: Vec<Rule>,
}
/// Variable List.
/// When the object is serialized out as xml, it's qualified name is dgm:varLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:varLst")]
pub struct VariableList {
    ///Show Organization Chart User Interface
    #[xml(child = "dgm:orgChart")]
    pub organization_chart: Option<OrganizationChart>,
    ///Maximum Children
    #[xml(child = "dgm:chMax")]
    pub max_number_of_children: Option<MaxNumberOfChildren>,
    ///Preferred Number of Children
    #[xml(child = "dgm:chPref")]
    pub preferred_number_of_children: Option<PreferredNumberOfChildren>,
    ///Show Insert Bullet
    #[xml(child = "dgm:bulletEnabled")]
    pub bullet_enabled: Option<BulletEnabled>,
    ///Diagram Direction
    #[xml(child = "dgm:dir")]
    pub direction: Option<Direction>,
    ///Organization Chart Branch Style
    #[xml(child = "dgm:hierBranch")]
    pub hierarchy_branch: Option<HierarchyBranch>,
    ///One by One Animation String
    #[xml(child = "dgm:animOne")]
    pub animate_one_by_one: Option<AnimateOneByOne>,
    ///Level Animation
    #[xml(child = "dgm:animLvl")]
    pub animation_level: Option<AnimationLevel>,
    ///Shape Resize Style
    #[xml(child = "dgm:resizeHandles")]
    pub resize_handles: Option<ResizeHandles>,
}
/// Presentation Layout Variables.
/// When the object is serialized out as xml, it's qualified name is dgm:presLayoutVars.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:presLayoutVars")]
pub struct PresentationLayoutVariables {
    ///Show Organization Chart User Interface
    #[xml(child = "dgm:orgChart")]
    pub organization_chart: Option<OrganizationChart>,
    ///Maximum Children
    #[xml(child = "dgm:chMax")]
    pub max_number_of_children: Option<MaxNumberOfChildren>,
    ///Preferred Number of Children
    #[xml(child = "dgm:chPref")]
    pub preferred_number_of_children: Option<PreferredNumberOfChildren>,
    ///Show Insert Bullet
    #[xml(child = "dgm:bulletEnabled")]
    pub bullet_enabled: Option<BulletEnabled>,
    ///Diagram Direction
    #[xml(child = "dgm:dir")]
    pub direction: Option<Direction>,
    ///Organization Chart Branch Style
    #[xml(child = "dgm:hierBranch")]
    pub hierarchy_branch: Option<HierarchyBranch>,
    ///One by One Animation String
    #[xml(child = "dgm:animOne")]
    pub animate_one_by_one: Option<AnimateOneByOne>,
    ///Level Animation
    #[xml(child = "dgm:animLvl")]
    pub animation_level: Option<AnimationLevel>,
    ///Shape Resize Style
    #[xml(child = "dgm:resizeHandles")]
    pub resize_handles: Option<ResizeHandles>,
}
/// Defines the LayoutVariablePropertySetType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LayoutVariablePropertySetType {}
/// For Each.
/// When the object is serialized out as xml, it's qualified name is dgm:forEach.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:forEach")]
pub struct ForEach {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<String>,
    /// Data Point Type
    /// Represents the following attribute in the schema: :ptType
    #[xml(attr = "ptType")]
    pub point_type: Option<String>,
    /// Hide Last Transition
    /// Represents the following attribute in the schema: :hideLastTrans
    #[xml(attr = "hideLastTrans")]
    pub hide_last_trans: Option<String>,
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: Option<String>,
    /// Count
    /// Represents the following attribute in the schema: :cnt
    #[xml(attr = "cnt")]
    pub count: Option<String>,
    /// Step
    /// Represents the following attribute in the schema: :step
    #[xml(attr = "step")]
    pub step: Option<String>,
    #[xml(
        child = "dgm:alg",
        child = "dgm:shape",
        child = "dgm:presOf",
        child = "dgm:constrLst",
        child = "dgm:ruleLst",
        child = "dgm:forEach",
        child = "dgm:layoutNode",
        child = "dgm:choose",
        child = "dgm:extLst",
    )]
    pub children: Vec<ForEachChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ForEachChildChoice {
    #[xml(tag = "dgm:alg")]
    DgmAlg(Algorithm),
    #[xml(tag = "dgm:shape")]
    DgmShape(Shape),
    #[xml(tag = "dgm:presOf")]
    DgmPresOf(PresentationOf),
    #[xml(tag = "dgm:constrLst")]
    DgmConstrLst(Constraints),
    #[xml(tag = "dgm:ruleLst")]
    DgmRuleLst(RuleList),
    #[xml(tag = "dgm:forEach")]
    DgmForEach(ForEach),
    #[xml(tag = "dgm:layoutNode")]
    DgmLayoutNode(LayoutNode),
    #[xml(tag = "dgm:choose")]
    DgmChoose(Choose),
    #[xml(tag = "dgm:extLst")]
    DgmExtLst(ExtensionList),
}
/// Layout Node.
/// When the object is serialized out as xml, it's qualified name is dgm:layoutNode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:layoutNode")]
pub struct LayoutNode {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Style Label
    /// Represents the following attribute in the schema: :styleLbl
    #[xml(attr = "styleLbl")]
    pub style_label: Option<String>,
    /// Child Order
    /// Represents the following attribute in the schema: :chOrder
    #[xml(attr = "chOrder")]
    pub child_order: Option<ChildOrderValues>,
    /// Move With
    /// Represents the following attribute in the schema: :moveWith
    #[xml(attr = "moveWith")]
    pub move_with: Option<String>,
    #[xml(
        child = "dgm:alg",
        child = "dgm:shape",
        child = "dgm:presOf",
        child = "dgm:constrLst",
        child = "dgm:ruleLst",
        child = "dgm:varLst",
        child = "dgm:forEach",
        child = "dgm:layoutNode",
        child = "dgm:choose",
        child = "dgm:extLst",
    )]
    pub children: Vec<LayoutNodeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LayoutNodeChildChoice {
    #[xml(tag = "dgm:alg")]
    DgmAlg(Algorithm),
    #[xml(tag = "dgm:shape")]
    DgmShape(Shape),
    #[xml(tag = "dgm:presOf")]
    DgmPresOf(PresentationOf),
    #[xml(tag = "dgm:constrLst")]
    DgmConstrLst(Constraints),
    #[xml(tag = "dgm:ruleLst")]
    DgmRuleLst(RuleList),
    #[xml(tag = "dgm:varLst")]
    DgmVarLst(VariableList),
    #[xml(tag = "dgm:forEach")]
    DgmForEach(ForEach),
    #[xml(tag = "dgm:layoutNode")]
    DgmLayoutNode(LayoutNode),
    #[xml(tag = "dgm:choose")]
    DgmChoose(Choose),
    #[xml(tag = "dgm:extLst")]
    DgmExtLst(ExtensionList),
}
/// Choose Element.
/// When the object is serialized out as xml, it's qualified name is dgm:choose.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:choose")]
pub struct Choose {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// _
    #[xml(child = "dgm:if")]
    pub dgm_if: Vec<DiagramChooseIf>,
    /// _
    #[xml(child = "dgm:else")]
    pub dgm_else: Option<DiagramChooseElse>,
}
/// If.
/// When the object is serialized out as xml, it's qualified name is dgm:if.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:if")]
pub struct DiagramChooseIf {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// Axis
    /// Represents the following attribute in the schema: :axis
    #[xml(attr = "axis")]
    pub axis: Option<String>,
    /// Data Point Type
    /// Represents the following attribute in the schema: :ptType
    #[xml(attr = "ptType")]
    pub point_type: Option<String>,
    /// Hide Last Transition
    /// Represents the following attribute in the schema: :hideLastTrans
    #[xml(attr = "hideLastTrans")]
    pub hide_last_trans: Option<String>,
    /// Start
    /// Represents the following attribute in the schema: :st
    #[xml(attr = "st")]
    pub start: Option<String>,
    /// Count
    /// Represents the following attribute in the schema: :cnt
    #[xml(attr = "cnt")]
    pub count: Option<String>,
    /// Step
    /// Represents the following attribute in the schema: :step
    #[xml(attr = "step")]
    pub step: Option<String>,
    /// Function
    /// Represents the following attribute in the schema: :func
    #[xml(attr = "func")]
    pub function: FunctionValues,
    /// Argument
    /// Represents the following attribute in the schema: :arg
    #[xml(attr = "arg")]
    pub argument: Option<String>,
    /// Operator
    /// Represents the following attribute in the schema: :op
    #[xml(attr = "op")]
    pub operator: FunctionOperatorValues,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
    #[xml(
        child = "dgm:alg",
        child = "dgm:shape",
        child = "dgm:presOf",
        child = "dgm:constrLst",
        child = "dgm:ruleLst",
        child = "dgm:forEach",
        child = "dgm:layoutNode",
        child = "dgm:choose",
        child = "dgm:extLst",
    )]
    pub children: Vec<DiagramChooseIfChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DiagramChooseIfChildChoice {
    #[xml(tag = "dgm:alg")]
    DgmAlg(Algorithm),
    #[xml(tag = "dgm:shape")]
    DgmShape(Shape),
    #[xml(tag = "dgm:presOf")]
    DgmPresOf(PresentationOf),
    #[xml(tag = "dgm:constrLst")]
    DgmConstrLst(Constraints),
    #[xml(tag = "dgm:ruleLst")]
    DgmRuleLst(RuleList),
    #[xml(tag = "dgm:forEach")]
    DgmForEach(ForEach),
    #[xml(tag = "dgm:layoutNode")]
    DgmLayoutNode(LayoutNode),
    #[xml(tag = "dgm:choose")]
    DgmChoose(Choose),
    #[xml(tag = "dgm:extLst")]
    DgmExtLst(ExtensionList),
}
/// Else.
/// When the object is serialized out as xml, it's qualified name is dgm:else.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:else")]
pub struct DiagramChooseElse {
    /// Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    #[xml(
        child = "dgm:alg",
        child = "dgm:shape",
        child = "dgm:presOf",
        child = "dgm:constrLst",
        child = "dgm:ruleLst",
        child = "dgm:forEach",
        child = "dgm:layoutNode",
        child = "dgm:choose",
        child = "dgm:extLst",
    )]
    pub children: Vec<DiagramChooseElseChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DiagramChooseElseChildChoice {
    #[xml(tag = "dgm:alg")]
    DgmAlg(Algorithm),
    #[xml(tag = "dgm:shape")]
    DgmShape(Shape),
    #[xml(tag = "dgm:presOf")]
    DgmPresOf(PresentationOf),
    #[xml(tag = "dgm:constrLst")]
    DgmConstrLst(Constraints),
    #[xml(tag = "dgm:ruleLst")]
    DgmRuleLst(RuleList),
    #[xml(tag = "dgm:forEach")]
    DgmForEach(ForEach),
    #[xml(tag = "dgm:layoutNode")]
    DgmLayoutNode(LayoutNode),
    #[xml(tag = "dgm:choose")]
    DgmChoose(Choose),
    #[xml(tag = "dgm:extLst")]
    DgmExtLst(ExtensionList),
}
/// Data Model.
/// When the object is serialized out as xml, it's qualified name is dgm:dataModel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:dataModel")]
pub struct DataModel {
    ///Point List
    #[xml(child = "dgm:ptLst")]
    pub point_list: PointList,
    ///Connection List
    #[xml(child = "dgm:cxnLst")]
    pub connection_list: Option<ConnectionList>,
    ///Background Formatting
    #[xml(child = "dgm:bg")]
    pub background: Option<Background>,
    ///Whole E2O Formatting
    #[xml(child = "dgm:whole")]
    pub whole: Option<Whole>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub data_model_extension_list: Option<DataModelExtensionList>,
}
/// Category.
/// When the object is serialized out as xml, it's qualified name is dgm:cat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:cat")]
pub struct Category {
    /// Category Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// Priority
    /// Represents the following attribute in the schema: :pri
    #[xml(attr = "pri")]
    pub priority: i32,
}
/// Title.
/// When the object is serialized out as xml, it's qualified name is dgm:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:title")]
pub struct Title {
    /// Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Description.
/// When the object is serialized out as xml, it's qualified name is dgm:desc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:desc")]
pub struct Description {
    /// Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Category List.
/// When the object is serialized out as xml, it's qualified name is dgm:catLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:catLst")]
pub struct CategoryList {
    /// _
    #[xml(child = "dgm:cat")]
    pub dgm_cat: Vec<Category>,
}
/// Shape Style.
/// When the object is serialized out as xml, it's qualified name is dgm:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:style")]
pub struct Style {
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
/// Show Organization Chart User Interface.
/// When the object is serialized out as xml, it's qualified name is dgm:orgChart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:orgChart")]
pub struct OrganizationChart {
    /// Show Organization Chart User Interface Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Maximum Children.
/// When the object is serialized out as xml, it's qualified name is dgm:chMax.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:chMax")]
pub struct MaxNumberOfChildren {
    /// Maximum Children Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i32>,
}
/// Preferred Number of Children.
/// When the object is serialized out as xml, it's qualified name is dgm:chPref.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:chPref")]
pub struct PreferredNumberOfChildren {
    /// Preferred Number of CHildren Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<i32>,
}
/// Show Insert Bullet.
/// When the object is serialized out as xml, it's qualified name is dgm:bulletEnabled.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:bulletEnabled")]
pub struct BulletEnabled {
    /// Show Insert Bullet Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<bool>,
}
/// Diagram Direction.
/// When the object is serialized out as xml, it's qualified name is dgm:dir.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:dir")]
pub struct Direction {
    /// Diagram Direction Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<DirectionValues>,
}
/// Organization Chart Branch Style.
/// When the object is serialized out as xml, it's qualified name is dgm:hierBranch.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:hierBranch")]
pub struct HierarchyBranch {
    /// Organization Chart Branch Style Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<HierarchyBranchStyleValues>,
}
/// One by One Animation String.
/// When the object is serialized out as xml, it's qualified name is dgm:animOne.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:animOne")]
pub struct AnimateOneByOne {
    /// One By One Animation Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<AnimateOneByOneValues>,
}
/// Level Animation.
/// When the object is serialized out as xml, it's qualified name is dgm:animLvl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:animLvl")]
pub struct AnimationLevel {
    /// Level Animation Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<AnimationLevelStringValues>,
}
/// Shape Resize Style.
/// When the object is serialized out as xml, it's qualified name is dgm:resizeHandles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:resizeHandles")]
pub struct ResizeHandles {
    /// Shape Resize Style Type
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: Option<ResizeHandlesStringValues>,
}
/// Category.
/// When the object is serialized out as xml, it's qualified name is dgm:cat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:cat")]
pub struct StyleDisplayCategory {
    /// Category Type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: String,
    /// Priority
    /// Represents the following attribute in the schema: :pri
    #[xml(attr = "pri")]
    pub priority: i32,
}
/// 3-D Scene.
/// When the object is serialized out as xml, it's qualified name is dgm:scene3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:scene3d")]
pub struct Scene3D {
    ///Camera
    #[xml(child = "a:camera")]
    pub camera: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Camera,
    ///Light Rig
    #[xml(child = "a:lightRig")]
    pub light_rig: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LightRig,
    ///Backdrop Plane
    #[xml(child = "a:backdrop")]
    pub backdrop: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Backdrop,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// 3-D Shape Properties.
/// When the object is serialized out as xml, it's qualified name is dgm:sp3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:sp3d")]
pub struct Shape3D {
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
    pub preset_material: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetMaterialTypeValues,
    >,
    ///Top Bevel
    #[xml(child = "a:bevelT")]
    pub bevel_top: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelTop,
    >,
    ///Bottom Bevel
    #[xml(child = "a:bevelB")]
    pub bevel_bottom: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BevelBottom,
    >,
    ///Extrusion Color
    #[xml(child = "a:extrusionClr")]
    pub extrusion_color: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtrusionColor,
    >,
    ///Contour Color
    #[xml(child = "a:contourClr")]
    pub contour_color: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ContourColor,
    >,
    /// _
    #[xml(child = "a:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    >,
}
/// Text Properties.
/// When the object is serialized out as xml, it's qualified name is dgm:txPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:txPr")]
pub struct TextProperties {
    #[xml(child = "a:sp3d", child = "a:flatTx")]
    pub children: Vec<TextPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextPropertiesChildChoice {
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:flatTx")]
    AFlatTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText),
}
/// Title.
/// When the object is serialized out as xml, it's qualified name is dgm:title.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:title")]
pub struct StyleDefinitionTitle {
    /// Natural Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Description Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Style Label Description.
/// When the object is serialized out as xml, it's qualified name is dgm:desc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:desc")]
pub struct StyleLabelDescription {
    /// Natural Language
    /// Represents the following attribute in the schema: :lang
    #[xml(attr = "lang")]
    pub language: Option<String>,
    /// Description Value
    /// Represents the following attribute in the schema: :val
    #[xml(attr = "val")]
    pub val: String,
}
/// Category List.
/// When the object is serialized out as xml, it's qualified name is dgm:catLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:catLst")]
pub struct StyleDisplayCategories {
    /// _
    #[xml(child = "dgm:cat")]
    pub dgm_cat: Vec<StyleDisplayCategory>,
}
/// Style Label.
/// When the object is serialized out as xml, it's qualified name is dgm:styleLbl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:styleLbl")]
pub struct StyleLabel {
    /// Style Name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    ///3-D Scene
    #[xml(child = "dgm:scene3d")]
    pub scene3_d: Option<Scene3D>,
    ///3-D Shape Properties
    #[xml(child = "dgm:sp3d")]
    pub shape3_d: Option<Shape3D>,
    ///Text Properties
    #[xml(child = "dgm:txPr")]
    pub text_properties: Option<TextProperties>,
    ///Shape Style
    #[xml(child = "dgm:style")]
    pub style: Option<Style>,
    /// _
    #[xml(child = "dgm:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Point List.
/// When the object is serialized out as xml, it's qualified name is dgm:ptLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:ptLst")]
pub struct PointList {
    /// _
    #[xml(child = "dgm:pt")]
    pub dgm_pt: Vec<Point>,
}
/// Connection List.
/// When the object is serialized out as xml, it's qualified name is dgm:cxnLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:cxnLst")]
pub struct ConnectionList {
    /// _
    #[xml(child = "dgm:cxn")]
    pub dgm_cxn: Vec<Connection>,
}
/// Background Formatting.
/// When the object is serialized out as xml, it's qualified name is dgm:bg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:bg")]
pub struct Background {
    #[xml(
        child = "a:noFill",
        child = "a:solidFill",
        child = "a:gradFill",
        child = "a:blipFill",
        child = "a:pattFill",
        child = "a:grpFill",
        child = "a:effectLst",
        child = "a:effectDag",
    )]
    pub children: Vec<BackgroundChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BackgroundChildChoice {
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
}
/// Whole E2O Formatting.
/// When the object is serialized out as xml, it's qualified name is dgm:whole.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:whole")]
pub struct Whole {
    #[xml(child = "a:ln", child = "a:effectLst", child = "a:effectDag")]
    pub children: Vec<WholeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum WholeChildChoice {
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
}
/// Defines the DataModelExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is dgm:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:extLst")]
pub struct DataModelExtensionList {
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DataModelExtension,
    >,
}
/// Property Set.
/// When the object is serialized out as xml, it's qualified name is dgm:prSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:prSet")]
pub struct PropertySet {
    /// Presentation Element Identifier
    /// Represents the following attribute in the schema: :presAssocID
    #[xml(attr = "presAssocID")]
    pub presentation_element_id: Option<String>,
    /// Presentation Name
    /// Represents the following attribute in the schema: :presName
    #[xml(attr = "presName")]
    pub presentation_name: Option<String>,
    /// Presentation Style Label
    /// Represents the following attribute in the schema: :presStyleLbl
    #[xml(attr = "presStyleLbl")]
    pub presentation_style_label: Option<String>,
    /// Presentation Style Index
    /// Represents the following attribute in the schema: :presStyleIdx
    #[xml(attr = "presStyleIdx")]
    pub presentation_style_index: Option<i32>,
    /// Presentation Style Count
    /// Represents the following attribute in the schema: :presStyleCnt
    #[xml(attr = "presStyleCnt")]
    pub presentation_style_count: Option<i32>,
    /// Current Diagram Type
    /// Represents the following attribute in the schema: :loTypeId
    #[xml(attr = "loTypeId")]
    pub layout_type_id: Option<String>,
    /// Current Diagram Category
    /// Represents the following attribute in the schema: :loCatId
    #[xml(attr = "loCatId")]
    pub layout_category_id: Option<String>,
    /// Current Style Type
    /// Represents the following attribute in the schema: :qsTypeId
    #[xml(attr = "qsTypeId")]
    pub quick_style_type_id: Option<String>,
    /// Current Style Category
    /// Represents the following attribute in the schema: :qsCatId
    #[xml(attr = "qsCatId")]
    pub quick_style_category_id: Option<String>,
    /// Color Transform Type Identifier
    /// Represents the following attribute in the schema: :csTypeId
    #[xml(attr = "csTypeId")]
    pub color_type: Option<String>,
    /// Color Transform Category
    /// Represents the following attribute in the schema: :csCatId
    #[xml(attr = "csCatId")]
    pub color_category_id: Option<String>,
    /// Coherent 3D Behavior
    /// Represents the following attribute in the schema: :coherent3DOff
    #[xml(attr = "coherent3DOff")]
    pub coherent3_d: Option<bool>,
    /// Placeholder Text
    /// Represents the following attribute in the schema: :phldrT
    #[xml(attr = "phldrT")]
    pub placeholder_text: Option<String>,
    /// Placeholder
    /// Represents the following attribute in the schema: :phldr
    #[xml(attr = "phldr")]
    pub placeholder: Option<bool>,
    /// Custom Rotation
    /// Represents the following attribute in the schema: :custAng
    #[xml(attr = "custAng")]
    pub rotation: Option<i32>,
    /// Custom Vertical Flip
    /// Represents the following attribute in the schema: :custFlipVert
    #[xml(attr = "custFlipVert")]
    pub vertical_flip: Option<bool>,
    /// Custom Horizontal Flip
    /// Represents the following attribute in the schema: :custFlipHor
    #[xml(attr = "custFlipHor")]
    pub horizontal_flip: Option<bool>,
    /// Fixed Width Override
    /// Represents the following attribute in the schema: :custSzX
    #[xml(attr = "custSzX")]
    pub fixed_width_override: Option<i32>,
    /// Fixed Height Override
    /// Represents the following attribute in the schema: :custSzY
    #[xml(attr = "custSzY")]
    pub fixed_height_override: Option<i32>,
    /// Width Scale
    /// Represents the following attribute in the schema: :custScaleX
    #[xml(attr = "custScaleX")]
    pub width_scale: Option<i32>,
    /// Height Scale
    /// Represents the following attribute in the schema: :custScaleY
    #[xml(attr = "custScaleY")]
    pub height_scale: Option<i32>,
    /// Text Changed
    /// Represents the following attribute in the schema: :custT
    #[xml(attr = "custT")]
    pub text_changed: Option<bool>,
    /// Custom Factor Width
    /// Represents the following attribute in the schema: :custLinFactX
    #[xml(attr = "custLinFactX")]
    pub factor_width: Option<i32>,
    /// Custom Factor Height
    /// Represents the following attribute in the schema: :custLinFactY
    #[xml(attr = "custLinFactY")]
    pub factor_height: Option<i32>,
    /// Neighbor Offset Width
    /// Represents the following attribute in the schema: :custLinFactNeighborX
    #[xml(attr = "custLinFactNeighborX")]
    pub neighbor_offset_width: Option<i32>,
    /// Neighbor Offset Height
    /// Represents the following attribute in the schema: :custLinFactNeighborY
    #[xml(attr = "custLinFactNeighborY")]
    pub neighbor_offset_height: Option<i32>,
    /// Radius Scale
    /// Represents the following attribute in the schema: :custRadScaleRad
    #[xml(attr = "custRadScaleRad")]
    pub radius_scale: Option<i32>,
    /// Include Angle Scale
    /// Represents the following attribute in the schema: :custRadScaleInc
    #[xml(attr = "custRadScaleInc")]
    pub include_angle_scale: Option<i32>,
    ///Presentation Layout Variables
    #[xml(child = "dgm:presLayoutVars")]
    pub presentation_layout_variables: Option<PresentationLayoutVariables>,
    ///Shape Style
    #[xml(child = "dgm:style")]
    pub style: Option<Style>,
}
/// Shape Properties.
/// When the object is serialized out as xml, it's qualified name is dgm:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:spPr")]
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
/// Text Body.
/// When the object is serialized out as xml, it's qualified name is dgm:t.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:t")]
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
/// Defines the PtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is dgm:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:extLst")]
pub struct PtExtensionList {
    /// _
    #[xml(child = "a:ext")]
    pub a_ext: Vec<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PtExtension,
    >,
}
/// Defines the DiagramDefinitionExtension Class.
/// When the object is serialized out as xml, it's qualified name is dgm:ext.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:ext")]
pub struct DiagramDefinitionExtension {
    /// URI
    /// Represents the following attribute in the schema: :uri
    #[xml(attr = "uri")]
    pub uri: String,
    #[xml(child = "dgm1611:autoBuNodeInfoLst", child = "dgm1612:lstStyle")]
    pub children: Vec<DiagramDefinitionExtensionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DiagramDefinitionExtensionChildChoice {
    #[xml(tag = "dgm1611:autoBuNodeInfoLst")]
    Dgm1611AutoBuNodeInfoLst(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_11_diagram::NumberDiagramInfoList,
    ),
    #[xml(tag = "dgm1612:lstStyle")]
    Dgm1612LstStyle(
        crate::schemas::schemas_microsoft_com_office_drawing_2016_12_diagram::TextListStyleType,
    ),
}
/// Defines the SampleData Class.
/// When the object is serialized out as xml, it's qualified name is dgm:sampData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:sampData")]
pub struct SampleData {
    /// Use Default
    /// Represents the following attribute in the schema: :useDef
    #[xml(attr = "useDef")]
    pub use_default: Option<bool>,
    ///Data Model
    #[xml(child = "dgm:dataModel")]
    pub data_model: Option<DataModel>,
}
/// Defines the StyleData Class.
/// When the object is serialized out as xml, it's qualified name is dgm:styleData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:styleData")]
pub struct StyleData {
    /// Use Default
    /// Represents the following attribute in the schema: :useDef
    #[xml(attr = "useDef")]
    pub use_default: Option<bool>,
    ///Data Model
    #[xml(child = "dgm:dataModel")]
    pub data_model: Option<DataModel>,
}
/// Defines the ColorData Class.
/// When the object is serialized out as xml, it's qualified name is dgm:clrData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:clrData")]
pub struct ColorData {
    /// Use Default
    /// Represents the following attribute in the schema: :useDef
    #[xml(attr = "useDef")]
    pub use_default: Option<bool>,
    ///Data Model
    #[xml(child = "dgm:dataModel")]
    pub data_model: Option<DataModel>,
}
/// Defines the SampleDataType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SampleDataType {
    /// Use Default
    /// Represents the following attribute in the schema: :useDef
    #[xml(attr = "useDef")]
    pub use_default: Option<bool>,
}
/// List of extensions to the CT_DiagramDefintions type..
/// When the object is serialized out as xml, it's qualified name is dgm:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "dgm:extLst")]
pub struct DiagramDefinitionExtensionList {
    /// _
    #[xml(child = "dgm:ext")]
    pub dgm_ext: Vec<DiagramDefinitionExtension>,
}
