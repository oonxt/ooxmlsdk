#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OnOffValues {
    #[default]
    True,
    False,
    Zero,
    One,
}
crate::__string_enum! {
    OnOffValues { True = "true", False = "false", Zero = "0", One = "1", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SchemeColorValues {
    #[default]
    BackgroundColor,
    TextColor,
    AdditionalBackgroundColor,
    AdditionalTextColor,
    ExtraSchemeColor1,
    ExtraSchemeColor2,
    ExtraSchemeColor3,
    ExtraSchemeColor4,
    ExtraSchemeColor5,
    ExtraSchemeColor6,
    HyperlinkColor,
    FollowedHyperlinkColor,
    MainDarkColor1,
    MainLightColor1,
    MainDarkColor2,
    MainLightColor2,
    AutoColor,
}
crate::__string_enum! {
    SchemeColorValues { BackgroundColor = "bg1", TextColor = "tx1",
    AdditionalBackgroundColor = "bg2", AdditionalTextColor = "tx2", ExtraSchemeColor1 =
    "accent1", ExtraSchemeColor2 = "accent2", ExtraSchemeColor3 = "accent3",
    ExtraSchemeColor4 = "accent4", ExtraSchemeColor5 = "accent5", ExtraSchemeColor6 =
    "accent6", HyperlinkColor = "hlink", FollowedHyperlinkColor = "folHlink",
    MainDarkColor1 = "dk1", MainLightColor1 = "lt1", MainDarkColor2 = "dk2",
    MainLightColor2 = "lt2", AutoColor = "phClr", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RectangleAlignmentValues {
    #[default]
    None,
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
    RectangleAlignmentValues { None = "none", TopLeft = "tl", Top = "t", TopRight = "tr",
    Left = "l", Center = "ctr", Right = "r", BottomLeft = "bl", Bottom = "b", BottomRight
    = "br", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PathShadeTypeValues {
    #[default]
    Shape,
    Circle,
    Rect,
}
crate::__string_enum! {
    PathShadeTypeValues { Shape = "shape", Circle = "circle", Rect = "rect", }
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
pub enum PresetLineDashValues {
    #[default]
    Solid,
    Dot,
    SysDot,
    Dash,
    SysDash,
    LongDash,
    DashDot,
    SystemDashDot,
    LongDashDot,
    LongDashDotDot,
    SystemDashDotDot,
}
crate::__string_enum! {
    PresetLineDashValues { Solid = "solid", Dot = "dot", SysDot = "sysDot", Dash =
    "dash", SysDash = "sysDash", LongDash = "lgDash", DashDot = "dashDot", SystemDashDot
    = "sysDashDot", LongDashDot = "lgDashDot", LongDashDotDot = "lgDashDotDot",
    SystemDashDotDot = "sysDashDotDot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PenAlignmentValues {
    #[default]
    Center,
    Inset,
}
crate::__string_enum! {
    PenAlignmentValues { Center = "ctr", Inset = "in", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum CompoundLineValues {
    #[default]
    Simple,
    Double,
    ThickThin,
    ThinThick,
    Triple,
}
crate::__string_enum! {
    CompoundLineValues { Simple = "sng", Double = "dbl", ThickThin = "thickThin",
    ThinThick = "thinThick", Triple = "tri", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PresetCameraTypeValues {
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
    PresetCameraTypeValues { LegacyObliqueTopLeft = "legacyObliqueTopLeft",
    LegacyObliqueTop = "legacyObliqueTop", LegacyObliqueTopRight =
    "legacyObliqueTopRight", LegacyObliqueLeft = "legacyObliqueLeft", LegacyObliqueFront
    = "legacyObliqueFront", LegacyObliqueRight = "legacyObliqueRight",
    LegacyObliqueBottomLeft = "legacyObliqueBottomLeft", LegacyObliqueBottom =
    "legacyObliqueBottom", LegacyObliqueBottomRight = "legacyObliqueBottomRight",
    LegacyPerspectiveTopLeft = "legacyPerspectiveTopLeft", LegacyPerspectiveTop =
    "legacyPerspectiveTop", LegacyPerspectiveTopRight = "legacyPerspectiveTopRight",
    LegacyPerspectiveLeft = "legacyPerspectiveLeft", LegacyPerspectiveFront =
    "legacyPerspectiveFront", LegacyPerspectiveRight = "legacyPerspectiveRight",
    LegacyPerspectiveBottomLeft = "legacyPerspectiveBottomLeft", LegacyPerspectiveBottom
    = "legacyPerspectiveBottom", LegacyPerspectiveBottomRight =
    "legacyPerspectiveBottomRight", OrthographicFront = "orthographicFront",
    IsometricTopUp = "isometricTopUp", IsometricTopDown = "isometricTopDown",
    IsometricBottomUp = "isometricBottomUp", IsometricBottomDown = "isometricBottomDown",
    IsometricLeftUp = "isometricLeftUp", IsometricLeftDown = "isometricLeftDown",
    IsometricRightUp = "isometricRightUp", IsometricRightDown = "isometricRightDown",
    IsometricOffAxis1Left = "isometricOffAxis1Left", IsometricOffAxis1Right =
    "isometricOffAxis1Right", IsometricOffAxis1Top = "isometricOffAxis1Top",
    IsometricOffAxis2Left = "isometricOffAxis2Left", IsometricOffAxis2Right =
    "isometricOffAxis2Right", IsometricOffAxis2Top = "isometricOffAxis2Top",
    IsometricOffAxis3Left = "isometricOffAxis3Left", IsometricOffAxis3Right =
    "isometricOffAxis3Right", IsometricOffAxis3Bottom = "isometricOffAxis3Bottom",
    IsometricOffAxis4Left = "isometricOffAxis4Left", IsometricOffAxis4Right =
    "isometricOffAxis4Right", IsometricOffAxis4Bottom = "isometricOffAxis4Bottom",
    ObliqueTopLeft = "obliqueTopLeft", ObliqueTop = "obliqueTop", ObliqueTopRight =
    "obliqueTopRight", ObliqueLeft = "obliqueLeft", ObliqueRight = "obliqueRight",
    ObliqueBottomLeft = "obliqueBottomLeft", ObliqueBottom = "obliqueBottom",
    ObliqueBottomRight = "obliqueBottomRight", PerspectiveFront = "perspectiveFront",
    PerspectiveLeft = "perspectiveLeft", PerspectiveRight = "perspectiveRight",
    PerspectiveAbove = "perspectiveAbove", PerspectiveBelow = "perspectiveBelow",
    PerspectiveAboveLeftFacing = "perspectiveAboveLeftFacing",
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
pub enum LightRigTypeValues {
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
    ThreePoint,
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
    TwoPoint,
    Glow,
    BrightRoom,
}
crate::__string_enum! {
    LightRigTypeValues { LegacyFlat1 = "legacyFlat1", LegacyFlat2 = "legacyFlat2",
    LegacyFlat3 = "legacyFlat3", LegacyFlat4 = "legacyFlat4", LegacyNormal1 =
    "legacyNormal1", LegacyNormal2 = "legacyNormal2", LegacyNormal3 = "legacyNormal3",
    LegacyNormal4 = "legacyNormal4", LegacyHarsh1 = "legacyHarsh1", LegacyHarsh2 =
    "legacyHarsh2", LegacyHarsh3 = "legacyHarsh3", LegacyHarsh4 = "legacyHarsh4",
    ThreePoint = "threePt", Balanced = "balanced", Soft = "soft", Harsh = "harsh", Flood
    = "flood", Contrasting = "contrasting", Morning = "morning", Sunrise = "sunrise",
    Sunset = "sunset", Chilly = "chilly", Freezing = "freezing", Flat = "flat", TwoPoint
    = "twoPt", Glow = "glow", BrightRoom = "brightRoom", }
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
pub enum BevelPresetTypeValues {
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
    BevelPresetTypeValues { RelaxedInset = "relaxedInset", Circle = "circle", Slope =
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
    None,
}
crate::__string_enum! {
    PresetMaterialTypeValues { LegacyMatte = "legacyMatte", LegacyPlastic =
    "legacyPlastic", LegacyMetal = "legacyMetal", LegacyWireframe = "legacyWireframe",
    Matte = "matte", Plastic = "plastic", Metal = "metal", WarmMatte = "warmMatte",
    TranslucentPowder = "translucentPowder", Powder = "powder", DarkEdge = "dkEdge",
    SoftEdge = "softEdge", Clear = "clear", Flat = "flat", SoftMetal = "softmetal", None
    = "none", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LigaturesValues {
    #[default]
    None,
    Standard,
    Contextual,
    Historical,
    Discretional,
    StandardContextual,
    StandardHistorical,
    ContextualHistorical,
    StandardDiscretional,
    ContextualDiscretional,
    HistoricalDiscretional,
    StandardContextualHistorical,
    StandardContextualDiscretional,
    StandardHistoricalDiscretional,
    ContextualHistoricalDiscretional,
    All,
}
crate::__string_enum! {
    LigaturesValues { None = "none", Standard = "standard", Contextual = "contextual",
    Historical = "historical", Discretional = "discretional", StandardContextual =
    "standardContextual", StandardHistorical = "standardHistorical", ContextualHistorical
    = "contextualHistorical", StandardDiscretional = "standardDiscretional",
    ContextualDiscretional = "contextualDiscretional", HistoricalDiscretional =
    "historicalDiscretional", StandardContextualHistorical =
    "standardContextualHistorical", StandardContextualDiscretional =
    "standardContextualDiscretional", StandardHistoricalDiscretional =
    "standardHistoricalDiscretional", ContextualHistoricalDiscretional =
    "contextualHistoricalDiscretional", All = "all", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NumberFormValues {
    #[default]
    Default,
    Lining,
    OldStyle,
}
crate::__string_enum! {
    NumberFormValues { Default = "default", Lining = "lining", OldStyle = "oldStyle", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum NumberSpacingValues {
    #[default]
    Default,
    Proportional,
    Tabular,
}
crate::__string_enum! {
    NumberSpacingValues { Default = "default", Proportional = "proportional", Tabular =
    "tabular", }
}
/// Defines the RunConflictInsertion Class.
/// When the object is serialized out as xml, it's qualified name is w14:conflictIns.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:conflictIns")]
pub struct RunConflictInsertion {
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
    pub children: Vec<RunConflictInsertionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunConflictInsertionChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
    ),
    #[xml(tag = "w:permStart")]
    WPermStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
    ),
    #[xml(tag = "w:permEnd")]
    WPermEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
    ),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
    ),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
    ),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
    ),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
    ),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
    ),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
    ),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
    ),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
    ),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
    ),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
    ),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
    ),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
    ),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
    ),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
    ),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
    ),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(CustomXmlConflictInsertionRangeStart),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(CustomXmlConflictInsertionRangeEnd),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(CustomXmlConflictDeletionRangeStart),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(CustomXmlConflictDeletionRangeEnd),
    #[xml(tag = "w:ins")]
    WIns(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
    ),
    #[xml(tag = "w:del")]
    WDel(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
    ),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
    ),
    #[xml(tag = "w:moveTo")]
    WMoveTo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
    ),
    #[xml(tag = "w:contentPart")]
    WContentPart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
    ),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(RunConflictInsertion),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(RunConflictDeletion),
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
    WR(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run),
    #[xml(tag = "w:bdo")]
    WBdo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalOverride,
    ),
    #[xml(tag = "w:dir")]
    WDir(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalEmbedding,
    ),
}
/// Defines the RunConflictDeletion Class.
/// When the object is serialized out as xml, it's qualified name is w14:conflictDel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:conflictDel")]
pub struct RunConflictDeletion {
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
    pub children: Vec<RunConflictDeletionChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunConflictDeletionChildChoice {
    #[xml(tag = "w:sdt")]
    WSdt(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
    ),
    #[xml(tag = "w:permStart")]
    WPermStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
    ),
    #[xml(tag = "w:permEnd")]
    WPermEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
    ),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
    ),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
    ),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
    ),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
    ),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
    ),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
    ),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
    ),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
    ),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
    ),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
    ),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
    ),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
    ),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
    ),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
    ),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
    ),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(CustomXmlConflictInsertionRangeStart),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(CustomXmlConflictInsertionRangeEnd),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(CustomXmlConflictDeletionRangeStart),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(CustomXmlConflictDeletionRangeEnd),
    #[xml(tag = "w:ins")]
    WIns(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
    ),
    #[xml(tag = "w:del")]
    WDel(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
    ),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
    ),
    #[xml(tag = "w:moveTo")]
    WMoveTo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
    ),
    #[xml(tag = "w:contentPart")]
    WContentPart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
    ),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(RunConflictInsertion),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(RunConflictDeletion),
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
    WR(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run),
    #[xml(tag = "w:bdo")]
    WBdo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalOverride,
    ),
    #[xml(tag = "w:dir")]
    WDir(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalEmbedding,
    ),
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
    WSdt(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SdtRun),
    #[xml(tag = "w:proofErr")]
    WProofErr(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ProofError,
    ),
    #[xml(tag = "w:permStart")]
    WPermStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermStart,
    ),
    #[xml(tag = "w:permEnd")]
    WPermEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PermEnd,
    ),
    #[xml(tag = "w:bookmarkStart")]
    WBookmarkStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkStart,
    ),
    #[xml(tag = "w:bookmarkEnd")]
    WBookmarkEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BookmarkEnd,
    ),
    #[xml(tag = "w:commentRangeStart")]
    WCommentRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeStart,
    ),
    #[xml(tag = "w:commentRangeEnd")]
    WCommentRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentRangeEnd,
    ),
    #[xml(tag = "w:moveFromRangeStart")]
    WMoveFromRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeStart,
    ),
    #[xml(tag = "w:moveFromRangeEnd")]
    WMoveFromRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRangeEnd,
    ),
    #[xml(tag = "w:moveToRangeStart")]
    WMoveToRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeStart,
    ),
    #[xml(tag = "w:moveToRangeEnd")]
    WMoveToRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRangeEnd,
    ),
    #[xml(tag = "w:customXmlInsRangeStart")]
    WCustomXmlInsRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeStart,
    ),
    #[xml(tag = "w:customXmlInsRangeEnd")]
    WCustomXmlInsRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlInsRangeEnd,
    ),
    #[xml(tag = "w:customXmlDelRangeStart")]
    WCustomXmlDelRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeStart,
    ),
    #[xml(tag = "w:customXmlDelRangeEnd")]
    WCustomXmlDelRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlDelRangeEnd,
    ),
    #[xml(tag = "w:customXmlMoveFromRangeStart")]
    WCustomXmlMoveFromRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeStart,
    ),
    #[xml(tag = "w:customXmlMoveFromRangeEnd")]
    WCustomXmlMoveFromRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveFromRangeEnd,
    ),
    #[xml(tag = "w:customXmlMoveToRangeStart")]
    WCustomXmlMoveToRangeStart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeStart,
    ),
    #[xml(tag = "w:customXmlMoveToRangeEnd")]
    WCustomXmlMoveToRangeEnd(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlMoveToRangeEnd,
    ),
    #[xml(tag = "w14:customXmlConflictInsRangeStart")]
    W14CustomXmlConflictInsRangeStart(CustomXmlConflictInsertionRangeStart),
    #[xml(tag = "w14:customXmlConflictInsRangeEnd")]
    W14CustomXmlConflictInsRangeEnd(CustomXmlConflictInsertionRangeEnd),
    #[xml(tag = "w14:customXmlConflictDelRangeStart")]
    W14CustomXmlConflictDelRangeStart(CustomXmlConflictDeletionRangeStart),
    #[xml(tag = "w14:customXmlConflictDelRangeEnd")]
    W14CustomXmlConflictDelRangeEnd(CustomXmlConflictDeletionRangeEnd),
    #[xml(tag = "w:ins")]
    WIns(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedRun,
    ),
    #[xml(tag = "w:del")]
    WDel(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedRun,
    ),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromRun,
    ),
    #[xml(tag = "w:moveTo")]
    WMoveTo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToRun,
    ),
    #[xml(tag = "w:contentPart")]
    WContentPart(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContentPart,
    ),
    #[xml(tag = "w14:conflictIns")]
    W14ConflictIns(RunConflictInsertion),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(RunConflictDeletion),
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
    WR(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run),
    #[xml(tag = "w:bdo")]
    WBdo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalOverride,
    ),
    #[xml(tag = "w:dir")]
    WDir(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::BidirectionalEmbedding,
    ),
}
/// Defines the ConflictInsertion Class.
/// When the object is serialized out as xml, it's qualified name is w14:conflictIns.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:conflictIns")]
pub struct ConflictInsertion {
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
/// Defines the ConflictDeletion Class.
/// When the object is serialized out as xml, it's qualified name is w14:conflictDel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:conflictDel")]
pub struct ConflictDeletion {
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
/// Defines the CustomXmlConflictInsertionRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w14:customXmlConflictInsRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:customXmlConflictInsRangeStart")]
pub struct CustomXmlConflictInsertionRangeStart {
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
/// Defines the CustomXmlConflictDeletionRangeStart Class.
/// When the object is serialized out as xml, it's qualified name is w14:customXmlConflictDelRangeStart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:customXmlConflictDelRangeStart")]
pub struct CustomXmlConflictDeletionRangeStart {
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
/// Defines the Tint Class.
/// When the object is serialized out as xml, it's qualified name is w14:tint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:tint")]
pub struct Tint {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the Shade Class.
/// When the object is serialized out as xml, it's qualified name is w14:shade.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:shade")]
pub struct Shade {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the Alpha Class.
/// When the object is serialized out as xml, it's qualified name is w14:alpha.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:alpha")]
pub struct Alpha {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the PositiveFixedPercentageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PositiveFixedPercentageType {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the HueModulation Class.
/// When the object is serialized out as xml, it's qualified name is w14:hueMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:hueMod")]
pub struct HueModulation {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the Saturation Class.
/// When the object is serialized out as xml, it's qualified name is w14:sat.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:sat")]
pub struct Saturation {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the SaturationOffset Class.
/// When the object is serialized out as xml, it's qualified name is w14:satOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:satOff")]
pub struct SaturationOffset {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the SaturationModulation Class.
/// When the object is serialized out as xml, it's qualified name is w14:satMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:satMod")]
pub struct SaturationModulation {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the Luminance Class.
/// When the object is serialized out as xml, it's qualified name is w14:lum.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:lum")]
pub struct Luminance {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the LuminanceOffset Class.
/// When the object is serialized out as xml, it's qualified name is w14:lumOff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:lumOff")]
pub struct LuminanceOffset {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the LuminanceModulation Class.
/// When the object is serialized out as xml, it's qualified name is w14:lumMod.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:lumMod")]
pub struct LuminanceModulation {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the PercentageType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct PercentageType {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the RgbColorModelHex Class.
/// When the object is serialized out as xml, it's qualified name is w14:srgbClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:srgbClr")]
pub struct RgbColorModelHex {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: String,
    #[xml(
        child = "w14:tint",
        child = "w14:shade",
        child = "w14:alpha",
        child = "w14:hueMod",
        child = "w14:sat",
        child = "w14:satOff",
        child = "w14:satMod",
        child = "w14:lum",
        child = "w14:lumOff",
        child = "w14:lumMod",
    )]
    pub children: Vec<RgbColorModelHexChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RgbColorModelHexChildChoice {
    #[xml(tag = "w14:tint")]
    W14Tint(Tint),
    #[xml(tag = "w14:shade")]
    W14Shade(Shade),
    #[xml(tag = "w14:alpha")]
    W14Alpha(Alpha),
    #[xml(tag = "w14:hueMod")]
    W14HueMod(HueModulation),
    #[xml(tag = "w14:sat")]
    W14Sat(Saturation),
    #[xml(tag = "w14:satOff")]
    W14SatOff(SaturationOffset),
    #[xml(tag = "w14:satMod")]
    W14SatMod(SaturationModulation),
    #[xml(tag = "w14:lum")]
    W14Lum(Luminance),
    #[xml(tag = "w14:lumOff")]
    W14LumOff(LuminanceOffset),
    #[xml(tag = "w14:lumMod")]
    W14LumMod(LuminanceModulation),
}
/// Defines the SchemeColor Class.
/// When the object is serialized out as xml, it's qualified name is w14:schemeClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:schemeClr")]
pub struct SchemeColor {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: SchemeColorValues,
    #[xml(
        child = "w14:tint",
        child = "w14:shade",
        child = "w14:alpha",
        child = "w14:hueMod",
        child = "w14:sat",
        child = "w14:satOff",
        child = "w14:satMod",
        child = "w14:lum",
        child = "w14:lumOff",
        child = "w14:lumMod",
    )]
    pub children: Vec<SchemeColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SchemeColorChildChoice {
    #[xml(tag = "w14:tint")]
    W14Tint(Tint),
    #[xml(tag = "w14:shade")]
    W14Shade(Shade),
    #[xml(tag = "w14:alpha")]
    W14Alpha(Alpha),
    #[xml(tag = "w14:hueMod")]
    W14HueMod(HueModulation),
    #[xml(tag = "w14:sat")]
    W14Sat(Saturation),
    #[xml(tag = "w14:satOff")]
    W14SatOff(SaturationOffset),
    #[xml(tag = "w14:satMod")]
    W14SatMod(SaturationModulation),
    #[xml(tag = "w14:lum")]
    W14Lum(Luminance),
    #[xml(tag = "w14:lumOff")]
    W14LumOff(LuminanceOffset),
    #[xml(tag = "w14:lumMod")]
    W14LumMod(LuminanceModulation),
}
/// Defines the LinearShadeProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:lin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:lin")]
pub struct LinearShadeProperties {
    /// ang
    /// Represents the following attribute in the schema: w14:ang
    #[xml(attr = "w14:ang")]
    pub angle: Option<i32>,
    /// scaled
    /// Represents the following attribute in the schema: w14:scaled
    #[xml(attr = "w14:scaled")]
    pub scaled: Option<OnOffValues>,
}
/// Defines the PathShadeProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:path.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:path")]
pub struct PathShadeProperties {
    /// path
    /// Represents the following attribute in the schema: w14:path
    #[xml(attr = "w14:path")]
    pub path: Option<PathShadeTypeValues>,
    /// _
    #[xml(child = "w14:fillToRect")]
    pub fill_to_rectangle: Option<FillToRectangle>,
}
/// Defines the NoFillEmpty Class.
/// When the object is serialized out as xml, it's qualified name is w14:noFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:noFill")]
pub struct NoFillEmpty {}
/// Defines the RoundEmpty Class.
/// When the object is serialized out as xml, it's qualified name is w14:round.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:round")]
pub struct RoundEmpty {}
/// Defines the BevelEmpty Class.
/// When the object is serialized out as xml, it's qualified name is w14:bevel.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:bevel")]
pub struct BevelEmpty {}
/// Defines the EntityPickerEmpty Class.
/// When the object is serialized out as xml, it's qualified name is w14:entityPicker.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:entityPicker")]
pub struct EntityPickerEmpty {}
/// Defines the EmptyType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct EmptyType {}
/// Defines the SolidColorFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:solidFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:solidFill")]
pub struct SolidColorFillProperties {
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<SolidColorFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SolidColorFillPropertiesChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the GradientFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:gradFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:gradFill")]
pub struct GradientFillProperties {
    #[xml(child = "w14:gsLst", child = "w14:lin", child = "w14:path")]
    pub children: Vec<GradientFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GradientFillPropertiesChildChoice {
    #[xml(tag = "w14:gsLst")]
    W14GsLst(GradientStopList),
    #[xml(tag = "w14:lin")]
    W14Lin(LinearShadeProperties),
    #[xml(tag = "w14:path")]
    W14Path(PathShadeProperties),
}
/// Defines the PresetLineDashProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:prstDash.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:prstDash")]
pub struct PresetLineDashProperties {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<PresetLineDashValues>,
}
/// Defines the LineJoinMiterProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:miter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:miter")]
pub struct LineJoinMiterProperties {
    /// lim
    /// Represents the following attribute in the schema: w14:lim
    #[xml(attr = "w14:lim")]
    pub limit: Option<i32>,
}
/// Defines the Glow Class.
/// When the object is serialized out as xml, it's qualified name is w14:glow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:glow")]
pub struct Glow {
    /// rad
    /// Represents the following attribute in the schema: w14:rad
    #[xml(attr = "w14:rad")]
    pub glow_radius: Option<i32>,
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<GlowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GlowChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the Shadow Class.
/// When the object is serialized out as xml, it's qualified name is w14:shadow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:shadow")]
pub struct Shadow {
    /// blurRad
    /// Represents the following attribute in the schema: w14:blurRad
    #[xml(attr = "w14:blurRad")]
    pub blur_radius: Option<i32>,
    /// dist
    /// Represents the following attribute in the schema: w14:dist
    #[xml(attr = "w14:dist")]
    pub distance_from_text: Option<i32>,
    /// dir
    /// Represents the following attribute in the schema: w14:dir
    #[xml(attr = "w14:dir")]
    pub direction_angle: Option<i32>,
    /// sx
    /// Represents the following attribute in the schema: w14:sx
    #[xml(attr = "w14:sx")]
    pub horizontal_scaling_factor: Option<i32>,
    /// sy
    /// Represents the following attribute in the schema: w14:sy
    #[xml(attr = "w14:sy")]
    pub vertical_scaling_factor: Option<i32>,
    /// kx
    /// Represents the following attribute in the schema: w14:kx
    #[xml(attr = "w14:kx")]
    pub horizontal_skew_angle: Option<i32>,
    /// ky
    /// Represents the following attribute in the schema: w14:ky
    #[xml(attr = "w14:ky")]
    pub vertical_skew_angle: Option<i32>,
    /// algn
    /// Represents the following attribute in the schema: w14:algn
    #[xml(attr = "w14:algn")]
    pub alignment: Option<RectangleAlignmentValues>,
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<ShadowChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ShadowChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the Reflection Class.
/// When the object is serialized out as xml, it's qualified name is w14:reflection.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:reflection")]
pub struct Reflection {
    /// blurRad
    /// Represents the following attribute in the schema: w14:blurRad
    #[xml(attr = "w14:blurRad")]
    pub blur_radius: Option<i32>,
    /// stA
    /// Represents the following attribute in the schema: w14:stA
    #[xml(attr = "w14:stA")]
    pub starting_opacity: Option<i32>,
    /// stPos
    /// Represents the following attribute in the schema: w14:stPos
    #[xml(attr = "w14:stPos")]
    pub start_position: Option<i32>,
    /// endA
    /// Represents the following attribute in the schema: w14:endA
    #[xml(attr = "w14:endA")]
    pub ending_opacity: Option<i32>,
    /// endPos
    /// Represents the following attribute in the schema: w14:endPos
    #[xml(attr = "w14:endPos")]
    pub end_position: Option<i32>,
    /// dist
    /// Represents the following attribute in the schema: w14:dist
    #[xml(attr = "w14:dist")]
    pub distance_from_text: Option<i32>,
    /// dir
    /// Represents the following attribute in the schema: w14:dir
    #[xml(attr = "w14:dir")]
    pub direction_angle: Option<i32>,
    /// fadeDir
    /// Represents the following attribute in the schema: w14:fadeDir
    #[xml(attr = "w14:fadeDir")]
    pub fade_direction: Option<i32>,
    /// sx
    /// Represents the following attribute in the schema: w14:sx
    #[xml(attr = "w14:sx")]
    pub horizontal_scaling_factor: Option<i32>,
    /// sy
    /// Represents the following attribute in the schema: w14:sy
    #[xml(attr = "w14:sy")]
    pub vertical_scaling_factor: Option<i32>,
    /// kx
    /// Represents the following attribute in the schema: w14:kx
    #[xml(attr = "w14:kx")]
    pub horizontal_skew_angle: Option<i32>,
    /// ky
    /// Represents the following attribute in the schema: w14:ky
    #[xml(attr = "w14:ky")]
    pub vertical_skew_angle: Option<i32>,
    /// algn
    /// Represents the following attribute in the schema: w14:algn
    #[xml(attr = "w14:algn")]
    pub alignment: Option<RectangleAlignmentValues>,
}
/// Defines the TextOutlineEffect Class.
/// When the object is serialized out as xml, it's qualified name is w14:textOutline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:textOutline")]
pub struct TextOutlineEffect {
    /// w
    /// Represents the following attribute in the schema: w14:w
    #[xml(attr = "w14:w")]
    pub line_width: Option<i32>,
    /// cap
    /// Represents the following attribute in the schema: w14:cap
    #[xml(attr = "w14:cap")]
    pub cap_type: Option<LineCapValues>,
    /// cmpd
    /// Represents the following attribute in the schema: w14:cmpd
    #[xml(attr = "w14:cmpd")]
    pub compound: Option<CompoundLineValues>,
    /// algn
    /// Represents the following attribute in the schema: w14:algn
    #[xml(attr = "w14:algn")]
    pub alignment: Option<PenAlignmentValues>,
    #[xml(
        child = "w14:noFill",
        child = "w14:solidFill",
        child = "w14:gradFill",
        child = "w14:prstDash",
        child = "w14:round",
        child = "w14:bevel",
        child = "w14:miter",
    )]
    pub children: Vec<TextOutlineEffectChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextOutlineEffectChildChoice {
    #[xml(tag = "w14:noFill")]
    W14NoFill(NoFillEmpty),
    #[xml(tag = "w14:solidFill")]
    W14SolidFill(SolidColorFillProperties),
    #[xml(tag = "w14:gradFill")]
    W14GradFill(GradientFillProperties),
    #[xml(tag = "w14:prstDash")]
    W14PrstDash(PresetLineDashProperties),
    #[xml(tag = "w14:round")]
    W14Round(RoundEmpty),
    #[xml(tag = "w14:bevel")]
    W14Bevel(BevelEmpty),
    #[xml(tag = "w14:miter")]
    W14Miter(LineJoinMiterProperties),
}
/// Defines the FillTextEffect Class.
/// When the object is serialized out as xml, it's qualified name is w14:textFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:textFill")]
pub struct FillTextEffect {
    #[xml(child = "w14:noFill", child = "w14:solidFill", child = "w14:gradFill")]
    pub children: Vec<FillTextEffectChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillTextEffectChildChoice {
    #[xml(tag = "w14:noFill")]
    W14NoFill(NoFillEmpty),
    #[xml(tag = "w14:solidFill")]
    W14SolidFill(SolidColorFillProperties),
    #[xml(tag = "w14:gradFill")]
    W14GradFill(GradientFillProperties),
}
/// Defines the Scene3D Class.
/// When the object is serialized out as xml, it's qualified name is w14:scene3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:scene3d")]
pub struct Scene3D {
    /// _
    #[xml(child = "w14:camera")]
    pub camera: Camera,
    /// _
    #[xml(child = "w14:lightRig")]
    pub light_rig: LightRig,
}
/// Defines the Properties3D Class.
/// When the object is serialized out as xml, it's qualified name is w14:props3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:props3d")]
pub struct Properties3D {
    /// extrusionH
    /// Represents the following attribute in the schema: w14:extrusionH
    #[xml(attr = "w14:extrusionH")]
    pub extrusion_height: Option<i32>,
    /// contourW
    /// Represents the following attribute in the schema: w14:contourW
    #[xml(attr = "w14:contourW")]
    pub contour_width: Option<i32>,
    /// prstMaterial
    /// Represents the following attribute in the schema: w14:prstMaterial
    #[xml(attr = "w14:prstMaterial")]
    pub preset_material_type: Option<PresetMaterialTypeValues>,
    /// _
    #[xml(child = "w14:bevelT")]
    pub bevel_top: Option<BevelTop>,
    /// _
    #[xml(child = "w14:bevelB")]
    pub bevel_bottom: Option<BevelBottom>,
    /// _
    #[xml(child = "w14:extrusionClr")]
    pub extrusion_color: Option<ExtrusionColor>,
    /// _
    #[xml(child = "w14:contourClr")]
    pub contour_color: Option<ContourColor>,
}
/// Defines the Ligatures Class.
/// When the object is serialized out as xml, it's qualified name is w14:ligatures.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:ligatures")]
pub struct Ligatures {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: LigaturesValues,
}
/// Defines the NumberingFormat Class.
/// When the object is serialized out as xml, it's qualified name is w14:numForm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:numForm")]
pub struct NumberingFormat {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: NumberFormValues,
}
/// Defines the NumberSpacing Class.
/// When the object is serialized out as xml, it's qualified name is w14:numSpacing.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:numSpacing")]
pub struct NumberSpacing {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: NumberSpacingValues,
}
/// Defines the StylisticSets Class.
/// When the object is serialized out as xml, it's qualified name is w14:stylisticSets.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:stylisticSets")]
pub struct StylisticSets {
    /// _
    #[xml(child = "w14:styleSet")]
    pub w14_style_set: Vec<StyleSet>,
}
/// Defines the ContextualAlternatives Class.
/// When the object is serialized out as xml, it's qualified name is w14:cntxtAlts.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:cntxtAlts")]
pub struct ContextualAlternatives {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<OnOffValues>,
}
/// Defines the ConflictMode Class.
/// When the object is serialized out as xml, it's qualified name is w14:conflictMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:conflictMode")]
pub struct ConflictMode {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<OnOffValues>,
}
/// Defines the DiscardImageEditingData Class.
/// When the object is serialized out as xml, it's qualified name is w14:discardImageEditingData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:discardImageEditingData")]
pub struct DiscardImageEditingData {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<OnOffValues>,
}
/// Defines the Checked Class.
/// When the object is serialized out as xml, it's qualified name is w14:checked.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:checked")]
pub struct Checked {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<OnOffValues>,
}
/// Defines the OnOffType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OnOffType {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<OnOffValues>,
}
/// Defines the ContentPart Class.
/// When the object is serialized out as xml, it's qualified name is w14:contentPart.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:contentPart")]
pub struct ContentPart {
    /// bwMode
    /// Represents the following attribute in the schema: w14:bwMode
    #[xml(attr = "w14:bwMode")]
    pub black_white_mode: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlackWhiteModeValues,
    >,
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub relationship_id: String,
    /// _
    #[xml(child = "w14:nvContentPartPr")]
    pub word_non_visual_content_part_shape_properties: Option<
        WordNonVisualContentPartShapeProperties,
    >,
    /// _
    #[xml(child = "w14:xfrm")]
    pub transform2_d: Option<Transform2D>,
    /// _
    #[xml(child = "w14:extLst")]
    pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the DocumentId Class.
/// When the object is serialized out as xml, it's qualified name is w14:docId.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:docId")]
pub struct DocumentId {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: String,
}
/// Defines the CustomXmlConflictInsertionRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w14:customXmlConflictInsRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:customXmlConflictInsRangeEnd")]
pub struct CustomXmlConflictInsertionRangeEnd {
    /// Annotation Identifier
    /// Represents the following attribute in the schema: w:id
    #[xml(attr = "w:id")]
    pub id: String,
}
/// Defines the CustomXmlConflictDeletionRangeEnd Class.
/// When the object is serialized out as xml, it's qualified name is w14:customXmlConflictDelRangeEnd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:customXmlConflictDelRangeEnd")]
pub struct CustomXmlConflictDeletionRangeEnd {
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
/// Defines the DefaultImageDpi Class.
/// When the object is serialized out as xml, it's qualified name is w14:defaultImageDpi.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:defaultImageDpi")]
pub struct DefaultImageDpi {
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: i32,
}
/// Defines the SdtContentCheckBox Class.
/// When the object is serialized out as xml, it's qualified name is w14:checkbox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:checkbox")]
pub struct SdtContentCheckBox {
    /// _
    #[xml(child = "w14:checked")]
    pub checked: Option<Checked>,
    /// _
    #[xml(child = "w14:checkedState")]
    pub checked_state: Option<CheckedState>,
    /// _
    #[xml(child = "w14:uncheckedState")]
    pub unchecked_state: Option<UncheckedState>,
}
/// Defines the GradientStop Class.
/// When the object is serialized out as xml, it's qualified name is w14:gs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:gs")]
pub struct GradientStop {
    /// pos
    /// Represents the following attribute in the schema: w14:pos
    #[xml(attr = "w14:pos")]
    pub stop_position: i32,
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<GradientStopChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GradientStopChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the FillToRectangle Class.
/// When the object is serialized out as xml, it's qualified name is w14:fillToRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:fillToRect")]
pub struct FillToRectangle {
    /// l
    /// Represents the following attribute in the schema: w14:l
    #[xml(attr = "w14:l")]
    pub left: Option<i32>,
    /// t
    /// Represents the following attribute in the schema: w14:t
    #[xml(attr = "w14:t")]
    pub top: Option<i32>,
    /// r
    /// Represents the following attribute in the schema: w14:r
    #[xml(attr = "w14:r")]
    pub right: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: w14:b
    #[xml(attr = "w14:b")]
    pub bottom: Option<i32>,
}
/// Defines the GradientStopList Class.
/// When the object is serialized out as xml, it's qualified name is w14:gsLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:gsLst")]
pub struct GradientStopList {
    /// _
    #[xml(child = "w14:gs")]
    pub w14_gs: Vec<GradientStop>,
}
/// Defines the SphereCoordinates Class.
/// When the object is serialized out as xml, it's qualified name is w14:rot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:rot")]
pub struct SphereCoordinates {
    /// lat
    /// Represents the following attribute in the schema: w14:lat
    #[xml(attr = "w14:lat")]
    pub lattitude: i32,
    /// lon
    /// Represents the following attribute in the schema: w14:lon
    #[xml(attr = "w14:lon")]
    pub longitude: i32,
    /// rev
    /// Represents the following attribute in the schema: w14:rev
    #[xml(attr = "w14:rev")]
    pub revolution: i32,
}
/// Defines the Camera Class.
/// When the object is serialized out as xml, it's qualified name is w14:camera.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:camera")]
pub struct Camera {
    /// prst
    /// Represents the following attribute in the schema: w14:prst
    #[xml(attr = "w14:prst")]
    pub preset_camera_type: PresetCameraTypeValues,
}
/// Defines the LightRig Class.
/// When the object is serialized out as xml, it's qualified name is w14:lightRig.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:lightRig")]
pub struct LightRig {
    /// rig
    /// Represents the following attribute in the schema: w14:rig
    #[xml(attr = "w14:rig")]
    pub light_rig_type: LightRigTypeValues,
    /// dir
    /// Represents the following attribute in the schema: w14:dir
    #[xml(attr = "w14:dir")]
    pub light_direction_type: LightRigDirectionValues,
    /// _
    #[xml(child = "w14:rot")]
    pub sphere_coordinates: Option<SphereCoordinates>,
}
/// Defines the BevelTop Class.
/// When the object is serialized out as xml, it's qualified name is w14:bevelT.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:bevelT")]
pub struct BevelTop {
    /// w
    /// Represents the following attribute in the schema: w14:w
    #[xml(attr = "w14:w")]
    pub width: Option<i32>,
    /// h
    /// Represents the following attribute in the schema: w14:h
    #[xml(attr = "w14:h")]
    pub height: Option<i32>,
    /// prst
    /// Represents the following attribute in the schema: w14:prst
    #[xml(attr = "w14:prst")]
    pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the BevelBottom Class.
/// When the object is serialized out as xml, it's qualified name is w14:bevelB.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:bevelB")]
pub struct BevelBottom {
    /// w
    /// Represents the following attribute in the schema: w14:w
    #[xml(attr = "w14:w")]
    pub width: Option<i32>,
    /// h
    /// Represents the following attribute in the schema: w14:h
    #[xml(attr = "w14:h")]
    pub height: Option<i32>,
    /// prst
    /// Represents the following attribute in the schema: w14:prst
    #[xml(attr = "w14:prst")]
    pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the BevelType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct BevelType {
    /// w
    /// Represents the following attribute in the schema: w14:w
    #[xml(attr = "w14:w")]
    pub width: Option<i32>,
    /// h
    /// Represents the following attribute in the schema: w14:h
    #[xml(attr = "w14:h")]
    pub height: Option<i32>,
    /// prst
    /// Represents the following attribute in the schema: w14:prst
    #[xml(attr = "w14:prst")]
    pub preset_profile_type: Option<BevelPresetTypeValues>,
}
/// Defines the ExtrusionColor Class.
/// When the object is serialized out as xml, it's qualified name is w14:extrusionClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:extrusionClr")]
pub struct ExtrusionColor {
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<ExtrusionColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtrusionColorChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the ContourColor Class.
/// When the object is serialized out as xml, it's qualified name is w14:contourClr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:contourClr")]
pub struct ContourColor {
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<ContourColorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ContourColorChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the ColorType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct ColorType {
    #[xml(child = "w14:srgbClr", child = "w14:schemeClr")]
    pub children: Vec<ColorTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ColorTypeChildChoice {
    #[xml(tag = "w14:srgbClr")]
    W14SrgbClr(RgbColorModelHex),
    #[xml(tag = "w14:schemeClr")]
    W14SchemeClr(SchemeColor),
}
/// Defines the StyleSet Class.
/// When the object is serialized out as xml, it's qualified name is w14:styleSet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:styleSet")]
pub struct StyleSet {
    /// id
    /// Represents the following attribute in the schema: w14:id
    #[xml(attr = "w14:id")]
    pub id: u32,
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<OnOffValues>,
}
/// Defines the CheckedState Class.
/// When the object is serialized out as xml, it's qualified name is w14:checkedState.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:checkedState")]
pub struct CheckedState {
    /// font
    /// Represents the following attribute in the schema: w14:font
    #[xml(attr = "w14:font")]
    pub font: Option<String>,
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<String>,
}
/// Defines the UncheckedState Class.
/// When the object is serialized out as xml, it's qualified name is w14:uncheckedState.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:uncheckedState")]
pub struct UncheckedState {
    /// font
    /// Represents the following attribute in the schema: w14:font
    #[xml(attr = "w14:font")]
    pub font: Option<String>,
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<String>,
}
/// Defines the CheckBoxSymbolType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct CheckBoxSymbolType {
    /// font
    /// Represents the following attribute in the schema: w14:font
    #[xml(attr = "w14:font")]
    pub font: Option<String>,
    /// val
    /// Represents the following attribute in the schema: w14:val
    #[xml(attr = "w14:val")]
    pub val: Option<String>,
}
/// Defines the NonVisualDrawingProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:cNvPr")]
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
/// When the object is serialized out as xml, it's qualified name is w14:cNvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:cNvContentPartPr")]
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
/// Defines the WordNonVisualContentPartShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is w14:nvContentPartPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:nvContentPartPr")]
pub struct WordNonVisualContentPartShapeProperties {
    /// _
    #[xml(child = "w14:cNvPr")]
    pub non_visual_drawing_properties: Option<NonVisualDrawingProperties>,
    /// _
    #[xml(child = "w14:cNvContentPartPr")]
    pub non_visual_ink_content_part_properties: Option<
        NonVisualInkContentPartProperties,
    >,
}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is w14:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:xfrm")]
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
/// Defines the OfficeArtExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is w14:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "w14:extLst")]
pub struct OfficeArtExtensionList {
    #[xml(child = "a:ext")]
    pub children: Vec<OfficeArtExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeArtExtensionListChildChoice {
    #[xml(tag = "a:ext")]
    AExt(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension),
}
