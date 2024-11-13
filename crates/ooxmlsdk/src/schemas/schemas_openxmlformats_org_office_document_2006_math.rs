#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HorizontalAlignmentValues {
    #[default]
    Left,
    Center,
    Right,
}
crate::__string_enum! {
    HorizontalAlignmentValues { Left = "left", Center = "center", Right = "right", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ShapeDelimiterValues {
    #[default]
    Centered,
    Match,
}
crate::__string_enum! {
    ShapeDelimiterValues { Centered = "centered", Match = "match", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FractionTypeValues {
    #[default]
    Bar,
    Skewed,
    Linear,
    NoBar,
}
crate::__string_enum! {
    FractionTypeValues { Bar = "bar", Skewed = "skw", Linear = "lin", NoBar = "noBar", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum LimitLocationValues {
    #[default]
    UnderOver,
    SubscriptSuperscript,
}
crate::__string_enum! {
    LimitLocationValues { UnderOver = "undOvr", SubscriptSuperscript = "subSup", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalJustificationValues {
    #[default]
    Top,
    Bottom,
}
crate::__string_enum! {
    VerticalJustificationValues { Top = "top", Bottom = "bot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ScriptValues {
    #[default]
    Roman,
    Script,
    Fraktur,
    DoubleStruck,
    SansSerif,
    Monospace,
}
crate::__string_enum! {
    ScriptValues { Roman = "roman", Script = "script", Fraktur = "fraktur", DoubleStruck
    = "doubleStruck", SansSerif = "sansSerif", Monospace = "monospace", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StyleValues {
    #[default]
    Plain,
    Bold,
    Italic,
    BoldItalic,
}
crate::__string_enum! {
    StyleValues { Plain = "p", Bold = "b", Italic = "i", BoldItalic = "bi", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum JustificationValues {
    #[default]
    Left,
    Right,
    Center,
    CenterGroup,
}
crate::__string_enum! {
    JustificationValues { Left = "left", Right = "right", Center = "center", CenterGroup
    = "centerGroup", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BreakBinaryOperatorValues {
    #[default]
    Before,
    After,
    Repeat,
}
crate::__string_enum! {
    BreakBinaryOperatorValues { Before = "before", After = "after", Repeat = "repeat", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BreakBinarySubtractionValues {
    #[default]
    MinusMinus,
    MinusPlus,
    PlusMinus,
}
crate::__string_enum! {
    BreakBinarySubtractionValues { MinusMinus = "--", MinusPlus = "-+", PlusMinus = "+-", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum VerticalAlignmentValues {
    #[default]
    Top,
    Center,
    Bottom,
    Bot,
}
crate::__string_enum! {
    VerticalAlignmentValues { Top = "top", Center = "center", Bottom = "bottom", Bot =
    "bot", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BooleanValues {
    #[default]
    True,
    False,
    On,
    Off,
    Zero,
    One,
}
crate::__string_enum! {
    BooleanValues { True = "true", False = "false", On = "on", Off = "off", Zero = "0",
    One = "1", }
}
/// Script.
/// When the object is serialized out as xml, it's qualified name is m:scr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:scr")]
pub struct Script {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: ScriptValues,
}
/// style.
/// When the object is serialized out as xml, it's qualified name is m:sty.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sty")]
pub struct Style {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: StyleValues,
}
/// Defines the Run Class.
/// When the object is serialized out as xml, it's qualified name is m:r.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:r")]
pub struct Run {
    #[xml(
        child = "m:rPr",
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
        child = "m:t",
    )]
    pub children: Vec<RunChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunChildChoice {
    #[xml(tag = "m:rPr")]
    MRPr(RunProperties),
    #[xml(tag = "w:rPr")]
    WRPr(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::RunProperties,
    ),
    #[xml(tag = "w:br")]
    WBr(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Break),
    #[xml(tag = "w:t")]
    WT(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Text),
    #[xml(tag = "w:delText")]
    WDelText(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedText,
    ),
    #[xml(tag = "w:instrText")]
    WInstrText(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FieldCode,
    ),
    #[xml(tag = "w:delInstrText")]
    WDelInstrText(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedFieldCode,
    ),
    #[xml(tag = "w:noBreakHyphen")]
    WNoBreakHyphen(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::NoBreakHyphen,
    ),
    #[xml(tag = "w:softHyphen")]
    WSoftHyphen(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SoftHyphen,
    ),
    #[xml(tag = "w:dayShort")]
    WDayShort(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DayShort,
    ),
    #[xml(tag = "w:monthShort")]
    WMonthShort(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MonthShort,
    ),
    #[xml(tag = "w:yearShort")]
    WYearShort(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::YearShort,
    ),
    #[xml(tag = "w:dayLong")]
    WDayLong(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DayLong,
    ),
    #[xml(tag = "w:monthLong")]
    WMonthLong(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MonthLong,
    ),
    #[xml(tag = "w:yearLong")]
    WYearLong(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::YearLong,
    ),
    #[xml(tag = "w:annotationRef")]
    WAnnotationRef(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::AnnotationReferenceMark,
    ),
    #[xml(tag = "w:footnoteRef")]
    WFootnoteRef(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FootnoteReferenceMark,
    ),
    #[xml(tag = "w:endnoteRef")]
    WEndnoteRef(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::EndnoteReferenceMark,
    ),
    #[xml(tag = "w:separator")]
    WSeparator(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SeparatorMark,
    ),
    #[xml(tag = "w:continuationSeparator")]
    WContinuationSeparator(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::ContinuationSeparatorMark,
    ),
    #[xml(tag = "w:sym")]
    WSym(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SymbolChar,
    ),
    #[xml(tag = "w:pgNum")]
    WPgNum(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PageNumber,
    ),
    #[xml(tag = "w:cr")]
    WCr(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CarriageReturn,
    ),
    #[xml(tag = "w:tab")]
    WTab(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::TabChar),
    #[xml(tag = "w:object")]
    WObject(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::EmbeddedObject,
    ),
    #[xml(tag = "w:pict")]
    WPict(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Picture,
    ),
    #[xml(tag = "w:fldChar")]
    WFldChar(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FieldChar,
    ),
    #[xml(tag = "w:ruby")]
    WRuby(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Ruby),
    #[xml(tag = "w:footnoteReference")]
    WFootnoteReference(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::FootnoteReference,
    ),
    #[xml(tag = "w:endnoteReference")]
    WEndnoteReference(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::EndnoteReference,
    ),
    #[xml(tag = "w:commentReference")]
    WCommentReference(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CommentReference,
    ),
    #[xml(tag = "w:drawing")]
    WDrawing(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Drawing,
    ),
    #[xml(tag = "w:ptab")]
    WPtab(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::PositionalTab,
    ),
    #[xml(tag = "w:lastRenderedPageBreak")]
    WLastRenderedPageBreak(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::LastRenderedPageBreak,
    ),
    #[xml(tag = "m:t")]
    MT(Text),
}
/// Accent.
/// When the object is serialized out as xml, it's qualified name is m:acc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:acc")]
pub struct Accent {
    ///Accent Properties
    #[xml(child = "m:accPr")]
    pub accent_properties: Option<AccentProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Bar.
/// When the object is serialized out as xml, it's qualified name is m:bar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:bar")]
pub struct Bar {
    ///Bar Properties
    #[xml(child = "m:barPr")]
    pub bar_properties: Option<BarProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Box Function.
/// When the object is serialized out as xml, it's qualified name is m:box.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:box")]
pub struct Box {
    ///Box Properties
    #[xml(child = "m:boxPr")]
    pub box_properties: Option<BoxProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Border-Box Function.
/// When the object is serialized out as xml, it's qualified name is m:borderBox.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:borderBox")]
pub struct BorderBox {
    ///Border Box Properties
    #[xml(child = "m:borderBoxPr")]
    pub border_box_properties: Option<BorderBoxProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Delimiter Function.
/// When the object is serialized out as xml, it's qualified name is m:d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:d")]
pub struct Delimiter {
    ///Delimiter Properties
    #[xml(child = "m:dPr")]
    pub delimiter_properties: Option<DelimiterProperties>,
    /// _
    #[xml(child = "m:e")]
    pub m_e: Vec<Base>,
}
/// Equation-Array Function.
/// When the object is serialized out as xml, it's qualified name is m:eqArr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:eqArr")]
pub struct EquationArray {
    ///Equation Array Properties
    #[xml(child = "m:eqArrPr")]
    pub equation_array_properties: Option<EquationArrayProperties>,
    /// _
    #[xml(child = "m:e")]
    pub m_e: Vec<Base>,
}
/// Fraction Function.
/// When the object is serialized out as xml, it's qualified name is m:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:f")]
pub struct Fraction {
    ///Fraction Properties
    #[xml(child = "m:fPr")]
    pub fraction_properties: Option<FractionProperties>,
    ///Numerator
    #[xml(child = "m:num")]
    pub numerator: Numerator,
    ///Denominator
    #[xml(child = "m:den")]
    pub denominator: Denominator,
}
/// Function Apply Function.
/// When the object is serialized out as xml, it's qualified name is m:func.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:func")]
pub struct MathFunction {
    ///Function Properties
    #[xml(child = "m:funcPr")]
    pub function_properties: Option<FunctionProperties>,
    ///Function Name
    #[xml(child = "m:fName")]
    pub function_name: FunctionName,
    ///Base (Argument)
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Group-Character Function.
/// When the object is serialized out as xml, it's qualified name is m:groupChr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:groupChr")]
pub struct GroupChar {
    ///Group-Character Properties
    #[xml(child = "m:groupChrPr")]
    pub group_char_properties: Option<GroupCharProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Lower-Limit Function.
/// When the object is serialized out as xml, it's qualified name is m:limLow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:limLow")]
pub struct LimitLower {
    ///Lower Limit Properties
    #[xml(child = "m:limLowPr")]
    pub limit_lower_properties: Option<LimitLowerProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
    ///Limit (Lower)
    #[xml(child = "m:lim")]
    pub limit: Limit,
}
/// Upper-Limit Function.
/// When the object is serialized out as xml, it's qualified name is m:limUpp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:limUpp")]
pub struct LimitUpper {
    ///Upper Limit Properties
    #[xml(child = "m:limUppPr")]
    pub limit_upper_properties: Option<LimitUpperProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
    ///Limit (Upper)
    #[xml(child = "m:lim")]
    pub limit: Limit,
}
/// Matrix Function.
/// When the object is serialized out as xml, it's qualified name is m:m.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:m")]
pub struct Matrix {
    ///Matrix Properties
    #[xml(child = "m:mPr")]
    pub matrix_properties: Option<MatrixProperties>,
    /// _
    #[xml(child = "m:mr")]
    pub m_mr: Vec<MatrixRow>,
}
/// n-ary Operator Function.
/// When the object is serialized out as xml, it's qualified name is m:nary.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:nary")]
pub struct Nary {
    ///n-ary Properties
    #[xml(child = "m:naryPr")]
    pub nary_properties: Option<NaryProperties>,
    ///Lower limit (n-ary)
    #[xml(child = "m:sub")]
    pub sub_argument: SubArgument,
    ///Upper limit (n-ary)
    #[xml(child = "m:sup")]
    pub super_argument: SuperArgument,
    ///Base (Argument)
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Phantom Function.
/// When the object is serialized out as xml, it's qualified name is m:phant.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:phant")]
pub struct Phantom {
    ///Phantom Properties
    #[xml(child = "m:phantPr")]
    pub phantom_properties: Option<PhantomProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Radical Function.
/// When the object is serialized out as xml, it's qualified name is m:rad.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:rad")]
pub struct Radical {
    ///Radical Properties
    #[xml(child = "m:radPr")]
    pub radical_properties: Option<RadicalProperties>,
    ///Degree
    #[xml(child = "m:deg")]
    pub degree: Degree,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Pre-Sub-Superscript Function.
/// When the object is serialized out as xml, it's qualified name is m:sPre.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sPre")]
pub struct PreSubSuper {
    ///Pre-Sub-Superscript Properties
    #[xml(child = "m:sPrePr")]
    pub pre_sub_super_properties: Option<PreSubSuperProperties>,
    ///Subscript (Pre-Sub-Superscript)
    #[xml(child = "m:sub")]
    pub sub_argument: SubArgument,
    ///Superscript(Pre-Sub-Superscript function)
    #[xml(child = "m:sup")]
    pub super_argument: SuperArgument,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
}
/// Subscript Function.
/// When the object is serialized out as xml, it's qualified name is m:sSub.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sSub")]
pub struct Subscript {
    ///Subscript Properties
    #[xml(child = "m:sSubPr")]
    pub subscript_properties: Option<SubscriptProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
    ///Subscript (Subscript function)
    #[xml(child = "m:sub")]
    pub sub_argument: SubArgument,
}
/// Sub-Superscript Function.
/// When the object is serialized out as xml, it's qualified name is m:sSubSup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sSubSup")]
pub struct SubSuperscript {
    ///Sub-Superscript Properties
    #[xml(child = "m:sSubSupPr")]
    pub sub_superscript_properties: Option<SubSuperscriptProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
    ///Subscript (Sub-Superscript)
    #[xml(child = "m:sub")]
    pub sub_argument: SubArgument,
    ///Superscript (Sub-Superscript function)
    #[xml(child = "m:sup")]
    pub super_argument: SuperArgument,
}
/// Superscript Function.
/// When the object is serialized out as xml, it's qualified name is m:sSup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sSup")]
pub struct Superscript {
    ///Superscript Properties
    #[xml(child = "m:sSupPr")]
    pub superscript_properties: Option<SuperscriptProperties>,
    ///Base
    #[xml(child = "m:e")]
    pub base: Base,
    ///Superscript (Superscript function)
    #[xml(child = "m:sup")]
    pub super_argument: SuperArgument,
}
/// Defines the Paragraph Class.
/// When the object is serialized out as xml, it's qualified name is m:oMathPara.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:oMathPara")]
pub struct Paragraph {
    #[xml(
        child = "m:oMathParaPr",
        child = "m:oMath",
        child = "m:r",
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
        child = "w:r",
    )]
    pub children: Vec<ParagraphChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ParagraphChildChoice {
    #[xml(tag = "m:oMathParaPr")]
    MOMathParaPr(ParagraphProperties),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:r")]
    MR(Run),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "w:r")]
    WR(crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Run),
}
/// Defines the OfficeMath Class.
/// When the object is serialized out as xml, it's qualified name is m:oMath.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:oMath")]
pub struct OfficeMath {
    #[xml(
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
    )]
    pub children: Vec<OfficeMathChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OfficeMathChildChoice {
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
}
/// Math Properties.
/// When the object is serialized out as xml, it's qualified name is m:mathPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mathPr")]
pub struct MathProperties {
    #[xml(
        child = "m:mathFont",
        child = "m:brkBin",
        child = "m:brkBinSub",
        child = "m:smallFrac",
        child = "m:dispDef",
        child = "m:lMargin",
        child = "m:rMargin",
        child = "m:defJc",
        child = "m:preSp",
        child = "m:postSp",
        child = "m:interSp",
        child = "m:intraSp",
        child = "m:wrapIndent",
        child = "m:wrapRight",
        child = "m:intLim",
        child = "m:naryLim",
    )]
    pub children: Vec<MathPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum MathPropertiesChildChoice {
    #[xml(tag = "m:mathFont")]
    MMathFont(MathFont),
    #[xml(tag = "m:brkBin")]
    MBrkBin(BreakBinary),
    #[xml(tag = "m:brkBinSub")]
    MBrkBinSub(BreakBinarySubtraction),
    #[xml(tag = "m:smallFrac")]
    MSmallFrac(SmallFraction),
    #[xml(tag = "m:dispDef")]
    MDispDef(DisplayDefaults),
    #[xml(tag = "m:lMargin")]
    MLMargin(LeftMargin),
    #[xml(tag = "m:rMargin")]
    MRMargin(RightMargin),
    #[xml(tag = "m:defJc")]
    MDefJc(DefaultJustification),
    #[xml(tag = "m:preSp")]
    MPreSp(PreSpacing),
    #[xml(tag = "m:postSp")]
    MPostSp(PostSpacing),
    #[xml(tag = "m:interSp")]
    MInterSp(InterSpacing),
    #[xml(tag = "m:intraSp")]
    MIntraSp(IntraSpacing),
    #[xml(tag = "m:wrapIndent")]
    MWrapIndent(WrapIndent),
    #[xml(tag = "m:wrapRight")]
    MWrapRight(WrapRight),
    #[xml(tag = "m:intLim")]
    MIntLim(IntegralLimitLocation),
    #[xml(tag = "m:naryLim")]
    MNaryLim(NaryLimitLocation),
}
/// Literal.
/// When the object is serialized out as xml, it's qualified name is m:lit.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:lit")]
pub struct Literal {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Normal Text.
/// When the object is serialized out as xml, it's qualified name is m:nor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:nor")]
pub struct NormalText {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Align.
/// When the object is serialized out as xml, it's qualified name is m:aln.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:aln")]
pub struct Alignment {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Operator Emulator.
/// When the object is serialized out as xml, it's qualified name is m:opEmu.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:opEmu")]
pub struct OperatorEmulator {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// No Break.
/// When the object is serialized out as xml, it's qualified name is m:noBreak.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:noBreak")]
pub struct NoBreak {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Differential.
/// When the object is serialized out as xml, it's qualified name is m:diff.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:diff")]
pub struct Differential {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Top Edge.
/// When the object is serialized out as xml, it's qualified name is m:hideTop.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:hideTop")]
pub struct HideTop {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Bottom Edge.
/// When the object is serialized out as xml, it's qualified name is m:hideBot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:hideBot")]
pub struct HideBottom {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Left Edge.
/// When the object is serialized out as xml, it's qualified name is m:hideLeft.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:hideLeft")]
pub struct HideLeft {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Right Edge.
/// When the object is serialized out as xml, it's qualified name is m:hideRight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:hideRight")]
pub struct HideRight {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Horizontal.
/// When the object is serialized out as xml, it's qualified name is m:strikeH.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:strikeH")]
pub struct StrikeHorizontal {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Vertical.
/// When the object is serialized out as xml, it's qualified name is m:strikeV.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:strikeV")]
pub struct StrikeVertical {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Bottom-Left to Top-Right.
/// When the object is serialized out as xml, it's qualified name is m:strikeBLTR.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:strikeBLTR")]
pub struct StrikeBottomLeftToTopRight {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Border Box Strikethrough Top-Left to Bottom-Right.
/// When the object is serialized out as xml, it's qualified name is m:strikeTLBR.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:strikeTLBR")]
pub struct StrikeTopLeftToBottomRight {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Delimiter Grow.
/// When the object is serialized out as xml, it's qualified name is m:grow.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:grow")]
pub struct GrowOperators {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Maximum Distribution.
/// When the object is serialized out as xml, it's qualified name is m:maxDist.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:maxDist")]
pub struct MaxDistribution {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Object Distribution.
/// When the object is serialized out as xml, it's qualified name is m:objDist.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:objDist")]
pub struct ObjectDistribution {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Placeholders (Matrix).
/// When the object is serialized out as xml, it's qualified name is m:plcHide.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:plcHide")]
pub struct HidePlaceholder {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Subscript (n-ary).
/// When the object is serialized out as xml, it's qualified name is m:subHide.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:subHide")]
pub struct HideSubArgument {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Superscript (n-ary).
/// When the object is serialized out as xml, it's qualified name is m:supHide.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:supHide")]
pub struct HideSuperArgument {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Phantom Show.
/// When the object is serialized out as xml, it's qualified name is m:show.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:show")]
pub struct ShowPhantom {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Phantom Zero Width.
/// When the object is serialized out as xml, it's qualified name is m:zeroWid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:zeroWid")]
pub struct ZeroWidth {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Phantom Zero Ascent.
/// When the object is serialized out as xml, it's qualified name is m:zeroAsc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:zeroAsc")]
pub struct ZeroAscent {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Phantom Zero Descent.
/// When the object is serialized out as xml, it's qualified name is m:zeroDesc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:zeroDesc")]
pub struct ZeroDescent {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Transparent (Phantom).
/// When the object is serialized out as xml, it's qualified name is m:transp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:transp")]
pub struct Transparent {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Hide Degree.
/// When the object is serialized out as xml, it's qualified name is m:degHide.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:degHide")]
pub struct HideDegree {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Align Scripts.
/// When the object is serialized out as xml, it's qualified name is m:alnScr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:alnScr")]
pub struct AlignScripts {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Small Fraction.
/// When the object is serialized out as xml, it's qualified name is m:smallFrac.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:smallFrac")]
pub struct SmallFraction {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Use Display Math Defaults.
/// When the object is serialized out as xml, it's qualified name is m:dispDef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:dispDef")]
pub struct DisplayDefaults {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Wrap Right.
/// When the object is serialized out as xml, it's qualified name is m:wrapRight.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:wrapRight")]
pub struct WrapRight {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Defines the OnOffType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OnOffType {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BooleanValues>,
}
/// Break.
/// When the object is serialized out as xml, it's qualified name is m:brk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:brk")]
pub struct Break {
    /// Index of Operator to Align To
    /// Represents the following attribute in the schema: m:alnAt
    #[xml(attr = "m:alnAt")]
    pub align_at: Option<i64>,
    /// Index of Operator to Align To
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<i64>,
}
/// Run Properties.
/// When the object is serialized out as xml, it's qualified name is m:rPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:rPr")]
pub struct RunProperties {
    #[xml(
        child = "m:lit",
        child = "m:nor",
        child = "m:scr",
        child = "m:sty",
        child = "m:brk",
        child = "m:aln",
    )]
    pub children: Vec<RunPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RunPropertiesChildChoice {
    #[xml(tag = "m:lit")]
    MLit(Literal),
    #[xml(tag = "m:nor")]
    MNor(NormalText),
    #[xml(tag = "m:scr")]
    MScr(Script),
    #[xml(tag = "m:sty")]
    MSty(Style),
    #[xml(tag = "m:brk")]
    MBrk(Break),
    #[xml(tag = "m:aln")]
    MAln(Alignment),
}
/// Text.
/// When the object is serialized out as xml, it's qualified name is m:t.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:t")]
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
/// Accent Character.
/// When the object is serialized out as xml, it's qualified name is m:chr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:chr")]
pub struct AccentChar {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: String,
}
/// Delimiter Beginning Character.
/// When the object is serialized out as xml, it's qualified name is m:begChr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:begChr")]
pub struct BeginChar {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: String,
}
/// Delimiter Separator Character.
/// When the object is serialized out as xml, it's qualified name is m:sepChr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sepChr")]
pub struct SeparatorChar {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: String,
}
/// Delimiter Ending Character.
/// When the object is serialized out as xml, it's qualified name is m:endChr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:endChr")]
pub struct EndChar {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: String,
}
/// Defines the CharType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct CharType {
    /// value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: String,
}
/// Control Properties.
/// When the object is serialized out as xml, it's qualified name is m:ctrlPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:ctrlPr")]
pub struct ControlProperties {
    #[xml(
        child = "w:rPr",
        child = "w:ins",
        child = "w:del",
        child = "w:moveFrom",
        child = "w:moveTo",
    )]
    pub children: Vec<ControlPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ControlPropertiesChildChoice {
    #[xml(tag = "w:rPr")]
    WRPr(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::RunProperties,
    ),
    #[xml(tag = "w:ins")]
    WIns(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::InsertedMathControl,
    ),
    #[xml(tag = "w:del")]
    WDel(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::DeletedMathControl,
    ),
    #[xml(tag = "w:moveFrom")]
    WMoveFrom(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveFromMathControl,
    ),
    #[xml(tag = "w:moveTo")]
    WMoveTo(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::MoveToMathControl,
    ),
}
/// Accent Properties.
/// When the object is serialized out as xml, it's qualified name is m:accPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:accPr")]
pub struct AccentProperties {
    ///Accent Character
    #[xml(child = "m:chr")]
    pub accent_char: Option<AccentChar>,
    ///Control Properties
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Base.
/// When the object is serialized out as xml, it's qualified name is m:e.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:e")]
pub struct Base {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<BaseChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BaseChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Numerator.
/// When the object is serialized out as xml, it's qualified name is m:num.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:num")]
pub struct Numerator {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<NumeratorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NumeratorChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Denominator.
/// When the object is serialized out as xml, it's qualified name is m:den.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:den")]
pub struct Denominator {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<DenominatorChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DenominatorChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Function Name.
/// When the object is serialized out as xml, it's qualified name is m:fName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:fName")]
pub struct FunctionName {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<FunctionNameChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FunctionNameChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Limit (Lower).
/// When the object is serialized out as xml, it's qualified name is m:lim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:lim")]
pub struct Limit {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<LimitChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LimitChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Lower limit (n-ary) .
/// When the object is serialized out as xml, it's qualified name is m:sub.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sub")]
pub struct SubArgument {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<SubArgumentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SubArgumentChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Upper limit (n-ary).
/// When the object is serialized out as xml, it's qualified name is m:sup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sup")]
pub struct SuperArgument {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<SuperArgumentChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SuperArgumentChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Degree.
/// When the object is serialized out as xml, it's qualified name is m:deg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:deg")]
pub struct Degree {
    #[xml(
        child = "m:argPr",
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
        child = "m:ctrlPr",
    )]
    pub children: Vec<DegreeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DegreeChildChoice {
    #[xml(tag = "m:argPr")]
    MArgPr(ArgumentProperties),
    #[xml(tag = "m:acc")]
    MAcc(Accent),
    #[xml(tag = "m:bar")]
    MBar(Bar),
    #[xml(tag = "m:box")]
    MBox(Box),
    #[xml(tag = "m:borderBox")]
    MBorderBox(BorderBox),
    #[xml(tag = "m:d")]
    MD(Delimiter),
    #[xml(tag = "m:eqArr")]
    MEqArr(EquationArray),
    #[xml(tag = "m:f")]
    MF(Fraction),
    #[xml(tag = "m:func")]
    MFunc(MathFunction),
    #[xml(tag = "m:groupChr")]
    MGroupChr(GroupChar),
    #[xml(tag = "m:limLow")]
    MLimLow(LimitLower),
    #[xml(tag = "m:limUpp")]
    MLimUpp(LimitUpper),
    #[xml(tag = "m:m")]
    MM(Matrix),
    #[xml(tag = "m:nary")]
    MNary(Nary),
    #[xml(tag = "m:phant")]
    MPhant(Phantom),
    #[xml(tag = "m:rad")]
    MRad(Radical),
    #[xml(tag = "m:sPre")]
    MSPre(PreSubSuper),
    #[xml(tag = "m:sSub")]
    MSSub(Subscript),
    #[xml(tag = "m:sSubSup")]
    MSSubSup(SubSuperscript),
    #[xml(tag = "m:sSup")]
    MSSup(Superscript),
    #[xml(tag = "m:r")]
    MR(Run),
    #[xml(tag = "w:customXml")]
    WCustomXml(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::CustomXmlRun,
    ),
    #[xml(tag = "w:fldSimple")]
    WFldSimple(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::SimpleField,
    ),
    #[xml(tag = "w:hyperlink")]
    WHyperlink(
        crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Hyperlink,
    ),
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
    W14ConflictIns(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictInsertion,
    ),
    #[xml(tag = "w14:conflictDel")]
    W14ConflictDel(
        crate::schemas::schemas_microsoft_com_office_word_2010_wordml::RunConflictDeletion,
    ),
    #[xml(tag = "m:oMathPara")]
    MOMathPara(Paragraph),
    #[xml(tag = "m:oMath")]
    MOMath(OfficeMath),
    #[xml(tag = "m:ctrlPr")]
    MCtrlPr(ControlProperties),
}
/// Defines the OfficeMathArgumentType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OfficeMathArgumentType {}
/// Position (Bar).
/// When the object is serialized out as xml, it's qualified name is m:pos.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:pos")]
pub struct Position {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: VerticalJustificationValues,
}
/// Vertical Justification.
/// When the object is serialized out as xml, it's qualified name is m:vertJc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:vertJc")]
pub struct VerticalJustification {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: VerticalJustificationValues,
}
/// Defines the TopBottomType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TopBottomType {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: VerticalJustificationValues,
}
/// Bar Properties.
/// When the object is serialized out as xml, it's qualified name is m:barPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:barPr")]
pub struct BarProperties {
    ///Position (Bar)
    #[xml(child = "m:pos")]
    pub position: Option<Position>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Box Properties.
/// When the object is serialized out as xml, it's qualified name is m:boxPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:boxPr")]
pub struct BoxProperties {
    ///Operator Emulator
    #[xml(child = "m:opEmu")]
    pub operator_emulator: Option<OperatorEmulator>,
    ///No Break
    #[xml(child = "m:noBreak")]
    pub no_break: Option<NoBreak>,
    ///Differential
    #[xml(child = "m:diff")]
    pub differential: Option<Differential>,
    ///Break
    #[xml(child = "m:brk")]
    pub r#break: Option<Break>,
    ///Alignment
    #[xml(child = "m:aln")]
    pub alignment: Option<Alignment>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Border Box Properties.
/// When the object is serialized out as xml, it's qualified name is m:borderBoxPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:borderBoxPr")]
pub struct BorderBoxProperties {
    ///Hide Top Edge
    #[xml(child = "m:hideTop")]
    pub hide_top: Option<HideTop>,
    ///Hide Bottom Edge
    #[xml(child = "m:hideBot")]
    pub hide_bottom: Option<HideBottom>,
    ///Hide Left Edge
    #[xml(child = "m:hideLeft")]
    pub hide_left: Option<HideLeft>,
    ///Hide Right Edge
    #[xml(child = "m:hideRight")]
    pub hide_right: Option<HideRight>,
    ///Border Box Strikethrough Horizontal
    #[xml(child = "m:strikeH")]
    pub strike_horizontal: Option<StrikeHorizontal>,
    ///Border Box Strikethrough Vertical
    #[xml(child = "m:strikeV")]
    pub strike_vertical: Option<StrikeVertical>,
    ///Border Box Strikethrough Bottom-Left to Top-Right
    #[xml(child = "m:strikeBLTR")]
    pub strike_bottom_left_to_top_right: Option<StrikeBottomLeftToTopRight>,
    ///Border Box Strikethrough Top-Left to Bottom-Right
    #[xml(child = "m:strikeTLBR")]
    pub strike_top_left_to_bottom_right: Option<StrikeTopLeftToBottomRight>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Shape (Delimiters).
/// When the object is serialized out as xml, it's qualified name is m:shp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:shp")]
pub struct Shape {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: ShapeDelimiterValues,
}
/// Delimiter Properties.
/// When the object is serialized out as xml, it's qualified name is m:dPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:dPr")]
pub struct DelimiterProperties {
    ///Delimiter Beginning Character
    #[xml(child = "m:begChr")]
    pub begin_char: Option<BeginChar>,
    ///Delimiter Separator Character
    #[xml(child = "m:sepChr")]
    pub separator_char: Option<SeparatorChar>,
    ///Delimiter Ending Character
    #[xml(child = "m:endChr")]
    pub end_char: Option<EndChar>,
    ///Delimiter Grow
    #[xml(child = "m:grow")]
    pub grow_operators: Option<GrowOperators>,
    ///Shape (Delimiters)
    #[xml(child = "m:shp")]
    pub shape: Option<Shape>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Equation Array Base Justification.
/// When the object is serialized out as xml, it's qualified name is m:baseJc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:baseJc")]
pub struct BaseJustification {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: VerticalAlignmentValues,
}
/// Row Spacing Rule.
/// When the object is serialized out as xml, it's qualified name is m:rSpRule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:rSpRule")]
pub struct RowSpacingRule {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: i64,
}
/// Matrix Column Gap Rule.
/// When the object is serialized out as xml, it's qualified name is m:cGpRule.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:cGpRule")]
pub struct ColumnGapRule {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: i64,
}
/// Defines the SpacingRuleType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct SpacingRuleType {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: i64,
}
/// Row Spacing (Equation Array).
/// When the object is serialized out as xml, it's qualified name is m:rSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:rSp")]
pub struct RowSpacing {
    /// val
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u16,
}
/// Matrix Column Gap.
/// When the object is serialized out as xml, it's qualified name is m:cGp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:cGp")]
pub struct ColumnGap {
    /// val
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u16,
}
/// Defines the UnsignedShortType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct UnsignedShortType {
    /// val
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u16,
}
/// Equation Array Properties.
/// When the object is serialized out as xml, it's qualified name is m:eqArrPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:eqArrPr")]
pub struct EquationArrayProperties {
    ///Equation Array Base Justification
    #[xml(child = "m:baseJc")]
    pub base_justification: Option<BaseJustification>,
    ///Maximum Distribution
    #[xml(child = "m:maxDist")]
    pub max_distribution: Option<MaxDistribution>,
    ///Object Distribution
    #[xml(child = "m:objDist")]
    pub object_distribution: Option<ObjectDistribution>,
    ///Row Spacing Rule
    #[xml(child = "m:rSpRule")]
    pub row_spacing_rule: Option<RowSpacingRule>,
    ///Row Spacing (Equation Array)
    #[xml(child = "m:rSp")]
    pub row_spacing: Option<RowSpacing>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Fraction type.
/// When the object is serialized out as xml, it's qualified name is m:type.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:type")]
pub struct FractionType {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: FractionTypeValues,
}
/// Fraction Properties.
/// When the object is serialized out as xml, it's qualified name is m:fPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:fPr")]
pub struct FractionProperties {
    ///Fraction type
    #[xml(child = "m:type")]
    pub fraction_type: Option<FractionType>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Function Properties.
/// When the object is serialized out as xml, it's qualified name is m:funcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:funcPr")]
pub struct FunctionProperties {
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Group-Character Properties.
/// When the object is serialized out as xml, it's qualified name is m:groupChrPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:groupChrPr")]
pub struct GroupCharProperties {
    ///Group Character (Grouping Character)
    #[xml(child = "m:chr")]
    pub accent_char: Option<AccentChar>,
    ///Position (Group Character)
    #[xml(child = "m:pos")]
    pub position: Option<Position>,
    ///Vertical Justification
    #[xml(child = "m:vertJc")]
    pub vertical_justification: Option<VerticalJustification>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Lower Limit Properties.
/// When the object is serialized out as xml, it's qualified name is m:limLowPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:limLowPr")]
pub struct LimitLowerProperties {
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Upper Limit Properties.
/// When the object is serialized out as xml, it's qualified name is m:limUppPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:limUppPr")]
pub struct LimitUpperProperties {
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Matrix Column Count.
/// When the object is serialized out as xml, it's qualified name is m:count.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:count")]
pub struct MatrixColumnCount {
    /// val
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: i64,
}
/// Matrix Column Justification.
/// When the object is serialized out as xml, it's qualified name is m:mcJc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mcJc")]
pub struct MatrixColumnJustification {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: HorizontalAlignmentValues,
}
/// Matrix Column Properties.
/// When the object is serialized out as xml, it's qualified name is m:mcPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mcPr")]
pub struct MatrixColumnProperties {
    ///Matrix Column Count
    #[xml(child = "m:count")]
    pub matrix_column_count: Option<MatrixColumnCount>,
    ///Matrix Column Justification
    #[xml(child = "m:mcJc")]
    pub matrix_column_justification: Option<MatrixColumnJustification>,
}
/// Matrix Column.
/// When the object is serialized out as xml, it's qualified name is m:mc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mc")]
pub struct MatrixColumn {
    ///Matrix Column Properties
    #[xml(child = "m:mcPr")]
    pub matrix_column_properties: Option<MatrixColumnProperties>,
}
/// Matrix Column Spacing.
/// When the object is serialized out as xml, it's qualified name is m:cSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:cSp")]
pub struct ColumnSpacing {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Left Margin.
/// When the object is serialized out as xml, it's qualified name is m:lMargin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:lMargin")]
pub struct LeftMargin {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Right Margin.
/// When the object is serialized out as xml, it's qualified name is m:rMargin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:rMargin")]
pub struct RightMargin {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Pre-Equation Spacing.
/// When the object is serialized out as xml, it's qualified name is m:preSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:preSp")]
pub struct PreSpacing {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Post-Equation Spacing.
/// When the object is serialized out as xml, it's qualified name is m:postSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:postSp")]
pub struct PostSpacing {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Inter-Equation Spacing.
/// When the object is serialized out as xml, it's qualified name is m:interSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:interSp")]
pub struct InterSpacing {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Intra-Equation Spacing.
/// When the object is serialized out as xml, it's qualified name is m:intraSp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:intraSp")]
pub struct IntraSpacing {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Wrap Indent.
/// When the object is serialized out as xml, it's qualified name is m:wrapIndent.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:wrapIndent")]
pub struct WrapIndent {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Defines the TwipsMeasureType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct TwipsMeasureType {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: u32,
}
/// Matrix Columns.
/// When the object is serialized out as xml, it's qualified name is m:mcs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mcs")]
pub struct MatrixColumns {
    /// _
    #[xml(child = "m:mc")]
    pub m_mc: Vec<MatrixColumn>,
}
/// Matrix Properties.
/// When the object is serialized out as xml, it's qualified name is m:mPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mPr")]
pub struct MatrixProperties {
    ///Matrix Base Justification
    #[xml(child = "m:baseJc")]
    pub base_justification: Option<BaseJustification>,
    ///Hide Placeholders (Matrix)
    #[xml(child = "m:plcHide")]
    pub hide_placeholder: Option<HidePlaceholder>,
    ///Row Spacing Rule
    #[xml(child = "m:rSpRule")]
    pub row_spacing_rule: Option<RowSpacingRule>,
    ///Matrix Column Gap Rule
    #[xml(child = "m:cGpRule")]
    pub column_gap_rule: Option<ColumnGapRule>,
    ///Row Spacing (Matrix)
    #[xml(child = "m:rSp")]
    pub row_spacing: Option<RowSpacing>,
    ///Matrix Column Spacing
    #[xml(child = "m:cSp")]
    pub column_spacing: Option<ColumnSpacing>,
    ///Matrix Column Gap
    #[xml(child = "m:cGp")]
    pub column_gap: Option<ColumnGap>,
    ///Matrix Columns
    #[xml(child = "m:mcs")]
    pub matrix_columns: Option<MatrixColumns>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Matrix Row.
/// When the object is serialized out as xml, it's qualified name is m:mr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mr")]
pub struct MatrixRow {
    /// _
    #[xml(child = "m:e")]
    pub m_e: Vec<Base>,
}
/// n-ary Limit Location.
/// When the object is serialized out as xml, it's qualified name is m:limLoc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:limLoc")]
pub struct LimitLocation {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: LimitLocationValues,
}
/// Integral Limit Locations.
/// When the object is serialized out as xml, it's qualified name is m:intLim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:intLim")]
pub struct IntegralLimitLocation {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: LimitLocationValues,
}
/// n-ary Limit Location.
/// When the object is serialized out as xml, it's qualified name is m:naryLim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:naryLim")]
pub struct NaryLimitLocation {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: LimitLocationValues,
}
/// Defines the LimitLocationType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct LimitLocationType {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: LimitLocationValues,
}
/// n-ary Properties.
/// When the object is serialized out as xml, it's qualified name is m:naryPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:naryPr")]
pub struct NaryProperties {
    ///n-ary Operator Character
    #[xml(child = "m:chr")]
    pub accent_char: Option<AccentChar>,
    ///n-ary Limit Location
    #[xml(child = "m:limLoc")]
    pub limit_location: Option<LimitLocation>,
    ///n-ary Grow
    #[xml(child = "m:grow")]
    pub grow_operators: Option<GrowOperators>,
    ///Hide Subscript (n-ary)
    #[xml(child = "m:subHide")]
    pub hide_sub_argument: Option<HideSubArgument>,
    ///Hide Superscript (n-ary)
    #[xml(child = "m:supHide")]
    pub hide_super_argument: Option<HideSuperArgument>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Phantom Properties.
/// When the object is serialized out as xml, it's qualified name is m:phantPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:phantPr")]
pub struct PhantomProperties {
    ///Phantom Show
    #[xml(child = "m:show")]
    pub show_phantom: Option<ShowPhantom>,
    ///Phantom Zero Width
    #[xml(child = "m:zeroWid")]
    pub zero_width: Option<ZeroWidth>,
    ///Phantom Zero Ascent
    #[xml(child = "m:zeroAsc")]
    pub zero_ascent: Option<ZeroAscent>,
    ///Phantom Zero Descent
    #[xml(child = "m:zeroDesc")]
    pub zero_descent: Option<ZeroDescent>,
    ///Transparent (Phantom)
    #[xml(child = "m:transp")]
    pub transparent: Option<Transparent>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Radical Properties.
/// When the object is serialized out as xml, it's qualified name is m:radPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:radPr")]
pub struct RadicalProperties {
    ///Hide Degree
    #[xml(child = "m:degHide")]
    pub hide_degree: Option<HideDegree>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Pre-Sub-Superscript Properties.
/// When the object is serialized out as xml, it's qualified name is m:sPrePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sPrePr")]
pub struct PreSubSuperProperties {
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Subscript Properties.
/// When the object is serialized out as xml, it's qualified name is m:sSubPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sSubPr")]
pub struct SubscriptProperties {
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Sub-Superscript Properties.
/// When the object is serialized out as xml, it's qualified name is m:sSubSupPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sSubSupPr")]
pub struct SubSuperscriptProperties {
    ///Align Scripts
    #[xml(child = "m:alnScr")]
    pub align_scripts: Option<AlignScripts>,
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Superscript Properties.
/// When the object is serialized out as xml, it's qualified name is m:sSupPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:sSupPr")]
pub struct SuperscriptProperties {
    /// _
    #[xml(child = "m:ctrlPr")]
    pub control_properties: Option<ControlProperties>,
}
/// Argument Size.
/// When the object is serialized out as xml, it's qualified name is m:argSz.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:argSz")]
pub struct ArgumentSize {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: i64,
}
/// Argument Properties.
/// When the object is serialized out as xml, it's qualified name is m:argPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:argPr")]
pub struct ArgumentProperties {
    ///Argument Size
    #[xml(child = "m:argSz")]
    pub argument_size: Option<ArgumentSize>,
}
/// Justification.
/// When the object is serialized out as xml, it's qualified name is m:jc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:jc")]
pub struct Justification {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: JustificationValues,
}
/// Default Justification.
/// When the object is serialized out as xml, it's qualified name is m:defJc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:defJc")]
pub struct DefaultJustification {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: JustificationValues,
}
/// Defines the OfficeMathJustificationType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OfficeMathJustificationType {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: JustificationValues,
}
/// Math Font.
/// When the object is serialized out as xml, it's qualified name is m:mathFont.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:mathFont")]
pub struct MathFont {
    /// val
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: String,
}
/// Break on Binary Operators.
/// When the object is serialized out as xml, it's qualified name is m:brkBin.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:brkBin")]
pub struct BreakBinary {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: Option<BreakBinaryOperatorValues>,
}
/// Break on Binary Subtraction.
/// When the object is serialized out as xml, it's qualified name is m:brkBinSub.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:brkBinSub")]
pub struct BreakBinarySubtraction {
    /// Value
    /// Represents the following attribute in the schema: m:val
    #[xml(attr = "m:val")]
    pub val: BreakBinarySubtractionValues,
}
/// Office Math Paragraph Properties.
/// When the object is serialized out as xml, it's qualified name is m:oMathParaPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "m:oMathParaPr")]
pub struct ParagraphProperties {
    ///Justification
    #[xml(child = "m:jc")]
    pub justification: Option<Justification>,
}
