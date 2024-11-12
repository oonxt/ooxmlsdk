#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RevisionContext {
    #[default]
    Normal,
    Undo,
    Redo,
    Copy,
}
crate::__string_enum! {
    RevisionContext { Normal = "normal", Undo = "undo", Redo = "redo", Copy = "copy", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RwColAction {
    #[default]
    Insr,
    Delr,
    Insc,
    Delc,
}
crate::__string_enum! {
    RwColAction { Insr = "insr", Delr = "delr", Insc = "insc", Delc = "delc", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FeatureType {
    #[default]
    DataValidation,
    Hyperlink,
    RowColVisualOps,
    FreezePanes,
    Sparklines,
    HideUnhideSheet,
    ShowGridlinesHeadings,
    Comment,
    Outlines,
    DrawingElement,
    AutoFilter,
    PivotTable,
    Future,
}
crate::__string_enum! {
    FeatureType { DataValidation = "dataValidation", Hyperlink = "hyperlink",
    RowColVisualOps = "rowColVisualOps", FreezePanes = "freezePanes", Sparklines =
    "sparklines", HideUnhideSheet = "hideUnhideSheet", ShowGridlinesHeadings =
    "showGridlinesHeadings", Comment = "comment", Outlines = "outlines", DrawingElement =
    "drawingElement", AutoFilter = "autoFilter", PivotTable = "pivotTable", Future =
    "future", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtFeatureType {
    #[default]
    Reserved,
}
crate::__string_enum! {
    ExtFeatureType { Reserved = "reserved", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SubFeatureType {
    #[default]
    None,
    Future,
}
crate::__string_enum! {
    SubFeatureType { None = "none", Future = "future", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtSubFeatureType {
    #[default]
    Reserved,
}
crate::__string_enum! {
    ExtSubFeatureType { Reserved = "reserved", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum RowColVisualOp {
    #[default]
    Hide,
    Unhide,
    Resize,
    Autosize,
}
crate::__string_enum! {
    RowColVisualOp { Hide = "hide", Unhide = "unhide", Resize = "resize", Autosize =
    "autosize", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SheetOp {
    #[default]
    Insert,
    Delete,
    Reorder,
    Rename,
}
crate::__string_enum! {
    SheetOp { Insert = "insert", Delete = "delete", Reorder = "reorder", Rename =
    "rename", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FillType {
    #[default]
    Fill,
    Array,
    Future,
}
crate::__string_enum! {
    FillType { Fill = "fill", Array = "array", Future = "future", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum FillTypeExt {
    #[default]
    Test,
}
crate::__string_enum! {
    FillTypeExt { Test = "test", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AdjustType {
    #[default]
    Fmla,
    Format,
    CondFmt,
    Sparkline,
    Anchor,
    FmlaNoSticky,
    NoAdj,
    Fragile,
    Future,
}
crate::__string_enum! {
    AdjustType { Fmla = "fmla", Format = "format", CondFmt = "condFmt", Sparkline =
    "sparkline", Anchor = "anchor", FmlaNoSticky = "fmlaNoSticky", NoAdj = "noAdj",
    Fragile = "fragile", Future = "future", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AdjustTypeExt {
    #[default]
    Test,
}
crate::__string_enum! {
    AdjustTypeExt { Test = "test", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OartAnchorType {
    #[default]
    TwoCell,
    OneCell,
    Absolute,
}
crate::__string_enum! {
    OartAnchorType { TwoCell = "twoCell", OneCell = "oneCell", Absolute = "absolute", }
}
/// Defines the RevExHeaders Class.
/// When the object is serialized out as xml, it's qualified name is xr:revHdrs.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:revHdrs")]
pub struct RevExHeaders {
    /// minRev
    /// Represents the following attribute in the schema: :minRev
    #[xml(attr = "minRev")]
    pub min_rev: i64,
    /// maxRev
    /// Represents the following attribute in the schema: :maxRev
    #[xml(attr = "maxRev")]
    pub max_rev: i64,
    /// docId
    /// Represents the following attribute in the schema: :docId
    #[xml(attr = "docId")]
    pub doc_id: String,
    /// endpointId
    /// Represents the following attribute in the schema: :endpointId
    #[xml(attr = "endpointId")]
    pub endpoint_id: String,
    /// _
    #[xml(child = "xr:hdr")]
    pub xr_hdr: Vec<RevExHeader>,
}
/// Defines the RevExStream Class.
/// When the object is serialized out as xml, it's qualified name is xr:revStream.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:revStream")]
pub struct RevExStream {
    #[xml(
        child = "xr:xrrftr",
        child = "xr:xrrUspt",
        child = "xr:xrrTrim",
        child = "xr:xrrrc",
        child = "xr:xrrm",
        child = "xr:xrrc",
        child = "xr:xrrf",
        child = "xr:xrrDefName",
        child = "xr:xrrdo",
        child = "xr:xrrco",
        child = "xr:xrrSheet",
        child = "xr:xrrList",
        child = "xr:xrrListExpR",
        child = "xr:xrrg",
    )]
    pub children: Vec<RevExStreamChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevExStreamChildChoice {
    #[xml(tag = "xr:xrrftr")]
    XrXrrftr(RevExFuture),
    #[xml(tag = "xr:xrrUspt")]
    XrXrrUspt(RevExUnsupported),
    #[xml(tag = "xr:xrrTrim")]
    XrXrrTrim(RevExTrimmed),
    #[xml(tag = "xr:xrrrc")]
    XrXrrrc(RevExRowColumn),
    #[xml(tag = "xr:xrrm")]
    XrXrrm(RevExMove),
    #[xml(tag = "xr:xrrc")]
    XrXrrc(RevExChangeCell),
    #[xml(tag = "xr:xrrf")]
    XrXrrf(RevExFormatting),
    #[xml(tag = "xr:xrrDefName")]
    XrXrrDefName(RevExDefinedName),
    #[xml(tag = "xr:xrrdo")]
    XrXrrdo(RevExDelObj),
    #[xml(tag = "xr:xrrco")]
    XrXrrco(RevExChgObj),
    #[xml(tag = "xr:xrrSheet")]
    XrXrrSheet(RevExSheetOp),
    #[xml(tag = "xr:xrrList")]
    XrXrrList(RevisionList),
    #[xml(tag = "xr:xrrListExpR")]
    XrXrrListExpR(RevListAutoExpandRw),
    #[xml(tag = "xr:xrrg")]
    XrXrrg(RevGroup),
}
/// Defines the DifferentialFormatType Class.
/// When the object is serialized out as xml, it's qualified name is xr:dxf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:dxf")]
pub struct DifferentialFormatType {
    ///Font Properties
    #[xml(child = "x:font")]
    pub font: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Font,
    >,
    ///Number Format
    #[xml(child = "x:numFmt")]
    pub numbering_format: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::NumberingFormat,
    >,
    ///Fill
    #[xml(child = "x:fill")]
    pub fill: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Fill,
    >,
    ///Alignment
    #[xml(child = "x:alignment")]
    pub alignment: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Alignment,
    >,
    ///Border Properties
    #[xml(child = "x:border")]
    pub border: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Border,
    >,
    ///Protection Properties
    #[xml(child = "x:protection")]
    pub protection: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Protection,
    >,
    ///Future Feature Data Storage Area
    #[xml(child = "x:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Defines the RevisionPtr Class.
/// When the object is serialized out as xml, it's qualified name is xr:revisionPtr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:revisionPtr")]
pub struct RevisionPtr {
    /// revIDLastSave
    /// Represents the following attribute in the schema: :revIDLastSave
    #[xml(attr = "revIDLastSave")]
    pub rev_id_last_save: i64,
    /// documentId
    /// Represents the following attribute in the schema: :documentId
    #[xml(attr = "documentId")]
    pub document_id: String,
}
/// Defines the StateBasedObject Class.
/// When the object is serialized out as xml, it's qualified name is xr:objectState.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:objectState")]
pub struct StateBasedObject {
    #[xml(
        child = "xr:dataValidation",
        child = "xr:hyperlink",
        child = "xr:sparklineGroup",
        child = "xr:comments",
        child = "xr:autoFilter",
        child = "xr:pivotTableDefinition",
    )]
    pub children: Vec<StateBasedObjectChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum StateBasedObjectChildChoice {
    #[xml(tag = "xr:dataValidation")]
    XrDataValidation(DataValidation),
    #[xml(tag = "xr:hyperlink")]
    XrHyperlink(Hyperlink),
    #[xml(tag = "xr:sparklineGroup")]
    XrSparklineGroup(SparklineGroup),
    #[xml(tag = "xr:comments")]
    XrComments(Comments),
    #[xml(tag = "xr:autoFilter")]
    XrAutoFilter(AutoFilter),
    #[xml(tag = "xr:pivotTableDefinition")]
    XrPivotTableDefinition(PivotTableDefinition),
}
/// Defines the RevExHeader Class.
/// When the object is serialized out as xml, it's qualified name is xr:hdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:hdr")]
pub struct RevExHeader {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
    /// minRev
    /// Represents the following attribute in the schema: :minRev
    #[xml(attr = "minRev")]
    pub min_rev: Option<i64>,
    /// maxRev
    /// Represents the following attribute in the schema: :maxRev
    #[xml(attr = "maxRev")]
    pub max_rev: Option<i64>,
    /// time
    /// Represents the following attribute in the schema: :time
    #[xml(attr = "time")]
    pub time: String,
}
/// Defines the RevExFuture Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrftr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrftr")]
pub struct RevExFuture {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// sti
    /// Represents the following attribute in the schema: :sti
    #[xml(attr = "sti")]
    pub sti: Option<bool>,
    #[xml(child = "xr:xrrtest")]
    pub children: Vec<RevExFutureChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevExFutureChildChoice {
    #[xml(tag = "xr:xrrtest")]
    XrXrrtest(RevExTest),
}
/// Defines the RevExUnsupported Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrUspt.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrUspt")]
pub struct RevExUnsupported {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
}
/// Defines the RevExTrimmed Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrTrim.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrTrim")]
pub struct RevExTrimmed {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
}
/// Defines the RevExRowColumn Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrrc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrrc")]
pub struct RevExRowColumn {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// eol
    /// Represents the following attribute in the schema: :eol
    #[xml(attr = "eol")]
    pub eol: Option<bool>,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub r#ref: String,
    /// action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: RwColAction,
}
/// Defines the RevExMove Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrm")]
pub struct RevExMove {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// src
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub src: String,
    /// dst
    /// Represents the following attribute in the schema: :dst
    #[xml(attr = "dst")]
    pub dst: String,
    /// srcSh
    /// Represents the following attribute in the schema: :srcSh
    #[xml(attr = "srcSh")]
    pub src_sh: Option<String>,
}
/// Defines the RevExChangeCell Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrc")]
pub struct RevExChangeCell {
    /// listUid
    /// Represents the following attribute in the schema: :listUid
    #[xml(attr = "listUid")]
    pub list_uid: Option<String>,
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: String,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<FillType>,
    /// x
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: Option<FillTypeExt>,
    /// w
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub w: Option<i32>,
    /// _
    #[xml(child = "xr:c")]
    pub xr_c: Vec<RevCell>,
    /// _
    #[xml(child = "xr:ccse")]
    pub xr_ccse: Vec<ChangeCellSubEdit>,
}
/// Defines the RevExFormatting Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrf.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrf")]
pub struct RevExFormatting {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// numFmtId
    /// Represents the following attribute in the schema: :numFmtId
    #[xml(attr = "numFmtId")]
    pub num_fmt_id: Option<i32>,
    /// xfDxf
    /// Represents the following attribute in the schema: :xfDxf
    #[xml(attr = "xfDxf")]
    pub xf_dxf: Option<bool>,
    /// style
    /// Represents the following attribute in the schema: :style
    #[xml(attr = "style")]
    pub style: Option<bool>,
    /// sqref
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sqref: String,
    /// start
    /// Represents the following attribute in the schema: :start
    #[xml(attr = "start")]
    pub start: Option<i32>,
    /// length
    /// Represents the following attribute in the schema: :length
    #[xml(attr = "length")]
    pub length: Option<i32>,
    /// styleUid
    /// Represents the following attribute in the schema: :styleUid
    #[xml(attr = "styleUid")]
    pub style_uid: Option<String>,
    /// fBlankCell
    /// Represents the following attribute in the schema: :fBlankCell
    #[xml(attr = "fBlankCell")]
    pub f_blank_cell: Option<bool>,
    /// applyNumberFormat
    /// Represents the following attribute in the schema: :applyNumberFormat
    #[xml(attr = "applyNumberFormat")]
    pub apply_number_format: Option<bool>,
    /// applyFont
    /// Represents the following attribute in the schema: :applyFont
    #[xml(attr = "applyFont")]
    pub apply_font: Option<bool>,
    /// applyFill
    /// Represents the following attribute in the schema: :applyFill
    #[xml(attr = "applyFill")]
    pub apply_fill: Option<bool>,
    /// applyBorder
    /// Represents the following attribute in the schema: :applyBorder
    #[xml(attr = "applyBorder")]
    pub apply_border: Option<bool>,
    /// applyAlignment
    /// Represents the following attribute in the schema: :applyAlignment
    #[xml(attr = "applyAlignment")]
    pub apply_alignment: Option<bool>,
    /// applyProtection
    /// Represents the following attribute in the schema: :applyProtection
    #[xml(attr = "applyProtection")]
    pub apply_protection: Option<bool>,
    /// _
    #[xml(child = "xr:dxf")]
    pub differential_format_type: Option<DifferentialFormatType>,
    /// _
    #[xml(child = "xr:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDefinedName Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrDefName.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrDefName")]
pub struct RevExDefinedName {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// customView
    /// Represents the following attribute in the schema: :customView
    #[xml(attr = "customView")]
    pub custom_view: Option<bool>,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// function
    /// Represents the following attribute in the schema: :function
    #[xml(attr = "function")]
    pub function: Option<bool>,
    /// functionGroupId
    /// Represents the following attribute in the schema: :functionGroupId
    #[xml(attr = "functionGroupId")]
    pub function_group_id: Option<u8>,
    /// shortcutKey
    /// Represents the following attribute in the schema: :shortcutKey
    #[xml(attr = "shortcutKey")]
    pub shortcut_key: Option<u8>,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// customMenu
    /// Represents the following attribute in the schema: :customMenu
    #[xml(attr = "customMenu")]
    pub custom_menu: Option<String>,
    /// description
    /// Represents the following attribute in the schema: :description
    #[xml(attr = "description")]
    pub description: Option<String>,
    /// help
    /// Represents the following attribute in the schema: :help
    #[xml(attr = "help")]
    pub help: Option<String>,
    /// statusBar
    /// Represents the following attribute in the schema: :statusBar
    #[xml(attr = "statusBar")]
    pub status_bar: Option<String>,
    /// comment
    /// Represents the following attribute in the schema: :comment
    #[xml(attr = "comment")]
    pub comment: Option<String>,
    /// _
    #[xml(child = "xr:formula")]
    pub formula_formula: Option<FormulaFormula>,
    /// _
    #[xml(child = "xr:extLst")]
    pub extension_list: Option<ExtensionList>,
}
/// Defines the RevExDelObj Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrdo.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrdo")]
pub struct RevExDelObj {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// _
    #[xml(child = "xr:hdr")]
    pub state_based_header: StateBasedHeader,
}
/// Defines the RevExChgObj Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrco.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrco")]
pub struct RevExChgObj {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    #[xml(child = "xr:hdr", child = "xr:link", child = "xr:body")]
    pub children: Vec<RevExChgObjChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevExChgObjChildChoice {
    #[xml(tag = "xr:hdr")]
    XrHdr(StateBasedHeader),
    #[xml(tag = "xr:link")]
    XrLink(RevisionStateLink),
    #[xml(tag = "xr:body")]
    XrBody(RevisionState),
}
/// Defines the RevExSheetOp Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrSheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrSheet")]
pub struct RevExSheetOp {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// op
    /// Represents the following attribute in the schema: :op
    #[xml(attr = "op")]
    pub op: SheetOp,
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// idOrig
    /// Represents the following attribute in the schema: :idOrig
    #[xml(attr = "idOrig")]
    pub id_orig: Option<i32>,
    /// idNew
    /// Represents the following attribute in the schema: :idNew
    #[xml(attr = "idNew")]
    pub id_new: Option<i32>,
}
/// Defines the RevisionList Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrList.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrList")]
pub struct RevisionList {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// Data
    /// Represents the following attribute in the schema: :Data
    #[xml(attr = "Data")]
    pub data: Option<bool>,
    /// Formatting
    /// Represents the following attribute in the schema: :Formatting
    #[xml(attr = "Formatting")]
    pub formatting: Option<bool>,
    /// RangeBased
    /// Represents the following attribute in the schema: :RangeBased
    #[xml(attr = "RangeBased")]
    pub range_based: Option<bool>,
    /// Fake
    /// Represents the following attribute in the schema: :Fake
    #[xml(attr = "Fake")]
    pub fake: Option<bool>,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub r#ref: String,
    /// Headers
    /// Represents the following attribute in the schema: :Headers
    #[xml(attr = "Headers")]
    pub headers: Option<bool>,
    /// InsDelHeaders
    /// Represents the following attribute in the schema: :InsDelHeaders
    #[xml(attr = "InsDelHeaders")]
    pub ins_del_headers: Option<bool>,
    /// rId
    /// Represents the following attribute in the schema: :rId
    #[xml(attr = "rId")]
    pub r_id: i32,
}
/// Defines the RevListAutoExpandRw Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrListExpR.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrListExpR")]
pub struct RevListAutoExpandRw {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    /// refAdded
    /// Represents the following attribute in the schema: :refAdded
    #[xml(attr = "refAdded")]
    pub ref_added: String,
    /// listGuid
    /// Represents the following attribute in the schema: :listGuid
    #[xml(attr = "listGuid")]
    pub list_guid: String,
}
/// Defines the RevGroup Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrg")]
pub struct RevGroup {
    /// rev
    /// Represents the following attribute in the schema: :rev
    #[xml(attr = "rev")]
    pub rev: i64,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// sh
    /// Represents the following attribute in the schema: :sh
    #[xml(attr = "sh")]
    pub sh: String,
    /// uidp
    /// Represents the following attribute in the schema: :uidp
    #[xml(attr = "uidp")]
    pub uidp: Option<String>,
    /// ctx
    /// Represents the following attribute in the schema: :ctx
    #[xml(attr = "ctx")]
    pub ctx: Option<RevisionContext>,
    #[xml(
        child = "xr:xrrftr",
        child = "xr:xrrUspt",
        child = "xr:xrrTrim",
        child = "xr:xrrrc",
        child = "xr:xrrm",
        child = "xr:xrrc",
        child = "xr:xrrf",
        child = "xr:xrrDefName",
        child = "xr:xrrdo",
        child = "xr:xrrco",
        child = "xr:xrrSheet",
        child = "xr:xrrList",
        child = "xr:xrrListExpR",
    )]
    pub children: Vec<RevGroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevGroupChildChoice {
    #[xml(tag = "xr:xrrftr")]
    XrXrrftr(RevExFuture),
    #[xml(tag = "xr:xrrUspt")]
    XrXrrUspt(RevExUnsupported),
    #[xml(tag = "xr:xrrTrim")]
    XrXrrTrim(RevExTrimmed),
    #[xml(tag = "xr:xrrrc")]
    XrXrrrc(RevExRowColumn),
    #[xml(tag = "xr:xrrm")]
    XrXrrm(RevExMove),
    #[xml(tag = "xr:xrrc")]
    XrXrrc(RevExChangeCell),
    #[xml(tag = "xr:xrrf")]
    XrXrrf(RevExFormatting),
    #[xml(tag = "xr:xrrDefName")]
    XrXrrDefName(RevExDefinedName),
    #[xml(tag = "xr:xrrdo")]
    XrXrrdo(RevExDelObj),
    #[xml(tag = "xr:xrrco")]
    XrXrrco(RevExChgObj),
    #[xml(tag = "xr:xrrSheet")]
    XrXrrSheet(RevExSheetOp),
    #[xml(tag = "xr:xrrList")]
    XrXrrList(RevisionList),
    #[xml(tag = "xr:xrrListExpR")]
    XrXrrListExpR(RevListAutoExpandRw),
}
/// Defines the RevExTest Class.
/// When the object is serialized out as xml, it's qualified name is xr:xrrtest.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:xrrtest")]
pub struct RevExTest {}
/// Defines the RevCell Class.
/// When the object is serialized out as xml, it's qualified name is xr:c.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:c")]
pub struct RevCell {
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues,
    >,
    /// nop
    /// Represents the following attribute in the schema: :nop
    #[xml(attr = "nop")]
    pub nop: Option<bool>,
    /// tick
    /// Represents the following attribute in the schema: :tick
    #[xml(attr = "tick")]
    pub tick: Option<bool>,
    /// rep
    /// Represents the following attribute in the schema: :rep
    #[xml(attr = "rep")]
    pub rep: Option<i32>,
    /// _
    #[xml(child = "xr:f")]
    pub f_formula: Option<FFormula>,
    /// _
    #[xml(child = "xr:v")]
    pub xstring: Option<Xstring>,
    /// _
    #[xml(child = "xr:is")]
    pub rst_type: Option<RstType>,
}
/// Defines the ChangeCellSubEdit Class.
/// When the object is serialized out as xml, it's qualified name is xr:ccse.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:ccse")]
pub struct ChangeCellSubEdit {
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: String,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<FillType>,
    /// x
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: Option<FillTypeExt>,
    /// w
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub w: Option<i32>,
    /// _
    #[xml(child = "xr:c")]
    pub xr_c: Vec<RevCell>,
}
/// Defines the ExtensionList Class.
/// When the object is serialized out as xml, it's qualified name is xr:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:extLst")]
pub struct ExtensionList {
    #[xml(child = "x:ext")]
    pub children: Vec<ExtensionListChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ExtensionListChildChoice {
    #[xml(tag = "x:ext")]
    XExt(crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension),
}
/// Defines the FormulaFormula Class.
/// When the object is serialized out as xml, it's qualified name is xr:formula.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:formula")]
pub struct FormulaFormula {
    #[xml(text)]
    pub child: String,
}
/// Defines the FFormula Class.
/// When the object is serialized out as xml, it's qualified name is xr:f.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:f")]
pub struct FFormula {
    #[xml(text)]
    pub child: String,
}
/// Defines the StateBasedHeader Class.
/// When the object is serialized out as xml, it's qualified name is xr:hdr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:hdr")]
pub struct StateBasedHeader {
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
    /// eft
    /// Represents the following attribute in the schema: :eft
    #[xml(attr = "eft")]
    pub eft: FeatureType,
    /// eftx
    /// Represents the following attribute in the schema: :eftx
    #[xml(attr = "eftx")]
    pub eftx: Option<ExtFeatureType>,
    /// seft
    /// Represents the following attribute in the schema: :seft
    #[xml(attr = "seft")]
    pub seft: Option<SubFeatureType>,
    /// seftx
    /// Represents the following attribute in the schema: :seftx
    #[xml(attr = "seftx")]
    pub seftx: Option<ExtSubFeatureType>,
    /// _
    #[xml(child = "xr:refmap")]
    pub ref_map: Option<RefMap>,
}
/// Defines the RevisionStateLink Class.
/// When the object is serialized out as xml, it's qualified name is xr:link.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:link")]
pub struct RevisionStateLink {
    /// id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub r_id: String,
}
/// Defines the RevisionState Class.
/// When the object is serialized out as xml, it's qualified name is xr:body.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:body")]
pub struct RevisionState {
    #[xml(
        child = "xr:rowColVisualOps",
        child = "xr:hideUnhideSheet",
        child = "xr:showGridlinesHeadings",
        child = "xr:freezePanes",
        child = "xr:outlines",
    )]
    pub children: Vec<RevisionStateChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RevisionStateChildChoice {
    #[xml(tag = "xr:rowColVisualOps")]
    XrRowColVisualOps(RowColVisualOps),
    #[xml(tag = "xr:hideUnhideSheet")]
    XrHideUnhideSheet(HideUnhideSheet),
    #[xml(tag = "xr:showGridlinesHeadings")]
    XrShowGridlinesHeadings(ShowGridlinesHeadings),
    #[xml(tag = "xr:freezePanes")]
    XrFreezePanes(FreezePanes),
    #[xml(tag = "xr:outlines")]
    XrOutlines(Outlines),
}
/// Defines the RefMap Class.
/// When the object is serialized out as xml, it's qualified name is xr:refmap.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:refmap")]
pub struct RefMap {
    #[xml(
        child = "xr:ref",
        child = "xr:sheetUid",
        child = "xr:oartAnchor",
        child = "xr:future",
        child = "xr:test",
    )]
    pub children: Vec<RefMapChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum RefMapChildChoice {
    #[xml(tag = "xr:ref")]
    XrRef(RefCell),
    #[xml(tag = "xr:sheetUid")]
    XrSheetUid(SheetXluid),
    #[xml(tag = "xr:oartAnchor")]
    XrOartAnchor(RefOartAnchor),
    #[xml(tag = "xr:future")]
    XrFuture(RefFuture),
    #[xml(tag = "xr:test")]
    XrTest(RefTest),
}
/// Defines the RowColVisualOps Class.
/// When the object is serialized out as xml, it's qualified name is xr:rowColVisualOps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:rowColVisualOps")]
pub struct RowColVisualOps {
    /// action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: RowColVisualOp,
    /// isRow
    /// Represents the following attribute in the schema: :isRow
    #[xml(attr = "isRow")]
    pub is_row: bool,
    /// size
    /// Represents the following attribute in the schema: :size
    #[xml(attr = "size")]
    pub size: Option<i32>,
    /// userSized
    /// Represents the following attribute in the schema: :userSized
    #[xml(attr = "userSized")]
    pub user_sized: Option<bool>,
}
/// Defines the HideUnhideSheet Class.
/// When the object is serialized out as xml, it's qualified name is xr:hideUnhideSheet.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:hideUnhideSheet")]
pub struct HideUnhideSheet {
    /// hide
    /// Represents the following attribute in the schema: :hide
    #[xml(attr = "hide")]
    pub hide: bool,
}
/// Defines the ShowGridlinesHeadings Class.
/// When the object is serialized out as xml, it's qualified name is xr:showGridlinesHeadings.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:showGridlinesHeadings")]
pub struct ShowGridlinesHeadings {
    /// showGridLines
    /// Represents the following attribute in the schema: :showGridLines
    #[xml(attr = "showGridLines")]
    pub show_grid_lines: bool,
    /// showRowCol
    /// Represents the following attribute in the schema: :showRowCol
    #[xml(attr = "showRowCol")]
    pub show_row_col: bool,
}
/// Defines the FreezePanes Class.
/// When the object is serialized out as xml, it's qualified name is xr:freezePanes.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:freezePanes")]
pub struct FreezePanes {
    /// sheetViewUid
    /// Represents the following attribute in the schema: :sheetViewUid
    #[xml(attr = "sheetViewUid")]
    pub sheet_view_uid: Option<String>,
}
/// Defines the Outlines Class.
/// When the object is serialized out as xml, it's qualified name is xr:outlines.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:outlines")]
pub struct Outlines {
    /// isRow
    /// Represents the following attribute in the schema: :isRow
    #[xml(attr = "isRow")]
    pub is_row: bool,
    /// _
    #[xml(child = "xr:outline")]
    pub xr_outline: Vec<Outline>,
}
/// Defines the Outline Class.
/// When the object is serialized out as xml, it's qualified name is xr:outline.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:outline")]
pub struct Outline {
    /// isCollapsed
    /// Represents the following attribute in the schema: :isCollapsed
    #[xml(attr = "isCollapsed")]
    pub is_collapsed: bool,
    /// level
    /// Represents the following attribute in the schema: :level
    #[xml(attr = "level")]
    pub level: u8,
}
/// Defines the Xstring Class.
/// When the object is serialized out as xml, it's qualified name is xr:v.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:v")]
pub struct Xstring {
    #[xml(text)]
    pub child: String,
}
/// Defines the RstType Class.
/// When the object is serialized out as xml, it's qualified name is xr:is.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:is")]
pub struct RstType {
    ///Text
    #[xml(child = "x:t")]
    pub text: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Text,
    >,
    /// _
    #[xml(child = "x:r")]
    pub x_r: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Run,
    >,
    /// _
    #[xml(child = "x:rPh")]
    pub x_r_ph: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PhoneticRun,
    >,
    /// _
    #[xml(child = "x:phoneticPr")]
    pub x_phonetic_pr: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PhoneticProperties,
    >,
}
/// Defines the RefCell Class.
/// When the object is serialized out as xml, it's qualified name is xr:ref.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:ref")]
pub struct RefCell {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// ajt
    /// Represents the following attribute in the schema: :ajt
    #[xml(attr = "ajt")]
    pub ajt: AdjustType,
    /// ajtx
    /// Represents the following attribute in the schema: :ajtx
    #[xml(attr = "ajtx")]
    pub ajtx: Option<AdjustTypeExt>,
    /// homeRef
    /// Represents the following attribute in the schema: :homeRef
    #[xml(attr = "homeRef")]
    pub home_ref: Option<bool>,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: String,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: Option<String>,
    /// uidLast
    /// Represents the following attribute in the schema: :uidLast
    #[xml(attr = "uidLast")]
    pub uid_last: Option<String>,
}
/// Defines the SheetXluid Class.
/// When the object is serialized out as xml, it's qualified name is xr:sheetUid.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:sheetUid")]
pub struct SheetXluid {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// ajt
    /// Represents the following attribute in the schema: :ajt
    #[xml(attr = "ajt")]
    pub ajt: AdjustType,
    /// ajtx
    /// Represents the following attribute in the schema: :ajtx
    #[xml(attr = "ajtx")]
    pub ajtx: Option<AdjustTypeExt>,
    /// homeRef
    /// Represents the following attribute in the schema: :homeRef
    #[xml(attr = "homeRef")]
    pub home_ref: Option<bool>,
    /// uid
    /// Represents the following attribute in the schema: :uid
    #[xml(attr = "uid")]
    pub uid: String,
}
/// Defines the RefOartAnchor Class.
/// When the object is serialized out as xml, it's qualified name is xr:oartAnchor.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:oartAnchor")]
pub struct RefOartAnchor {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// ajt
    /// Represents the following attribute in the schema: :ajt
    #[xml(attr = "ajt")]
    pub ajt: AdjustType,
    /// ajtx
    /// Represents the following attribute in the schema: :ajtx
    #[xml(attr = "ajtx")]
    pub ajtx: Option<AdjustTypeExt>,
    /// homeRef
    /// Represents the following attribute in the schema: :homeRef
    #[xml(attr = "homeRef")]
    pub home_ref: Option<bool>,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: Option<String>,
    /// fromRowOff
    /// Represents the following attribute in the schema: :fromRowOff
    #[xml(attr = "fromRowOff")]
    pub from_row_off: Option<i64>,
    /// fromColOff
    /// Represents the following attribute in the schema: :fromColOff
    #[xml(attr = "fromColOff")]
    pub from_col_off: Option<i64>,
    /// toRowOff
    /// Represents the following attribute in the schema: :toRowOff
    #[xml(attr = "toRowOff")]
    pub to_row_off: Option<i64>,
    /// toColOff
    /// Represents the following attribute in the schema: :toColOff
    #[xml(attr = "toColOff")]
    pub to_col_off: Option<i64>,
    /// cx
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: Option<i64>,
    /// cy
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: Option<i64>,
    /// x
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: Option<i64>,
    /// y
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: Option<i64>,
    /// oat
    /// Represents the following attribute in the schema: :oat
    #[xml(attr = "oat")]
    pub oat: OartAnchorType,
}
/// Defines the RefFuture Class.
/// When the object is serialized out as xml, it's qualified name is xr:future.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:future")]
pub struct RefFuture {}
/// Defines the RefTest Class.
/// When the object is serialized out as xml, it's qualified name is xr:test.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:test")]
pub struct RefTest {
    /// n
    /// Represents the following attribute in the schema: :n
    #[xml(attr = "n")]
    pub n: String,
    /// ajt
    /// Represents the following attribute in the schema: :ajt
    #[xml(attr = "ajt")]
    pub ajt: AdjustType,
    /// ajtx
    /// Represents the following attribute in the schema: :ajtx
    #[xml(attr = "ajtx")]
    pub ajtx: Option<AdjustTypeExt>,
    /// homeRef
    /// Represents the following attribute in the schema: :homeRef
    #[xml(attr = "homeRef")]
    pub home_ref: Option<bool>,
}
/// Represents an external link to another workbook..
/// When the object is serialized out as xml, it's qualified name is xr:dataValidation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:dataValidation")]
pub struct DataValidation {
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationValues,
    >,
    /// errorStyle
    /// Represents the following attribute in the schema: :errorStyle
    #[xml(attr = "errorStyle")]
    pub error_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationErrorStyleValues,
    >,
    /// imeMode
    /// Represents the following attribute in the schema: :imeMode
    #[xml(attr = "imeMode")]
    pub ime_mode: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationImeModeValues,
    >,
    /// operator
    /// Represents the following attribute in the schema: :operator
    #[xml(attr = "operator")]
    pub operator: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataValidationOperatorValues,
    >,
    /// allowBlank
    /// Represents the following attribute in the schema: :allowBlank
    #[xml(attr = "allowBlank")]
    pub allow_blank: Option<bool>,
    /// showDropDown
    /// Represents the following attribute in the schema: :showDropDown
    #[xml(attr = "showDropDown")]
    pub show_drop_down: Option<bool>,
    /// showInputMessage
    /// Represents the following attribute in the schema: :showInputMessage
    #[xml(attr = "showInputMessage")]
    pub show_input_message: Option<bool>,
    /// showErrorMessage
    /// Represents the following attribute in the schema: :showErrorMessage
    #[xml(attr = "showErrorMessage")]
    pub show_error_message: Option<bool>,
    /// errorTitle
    /// Represents the following attribute in the schema: :errorTitle
    #[xml(attr = "errorTitle")]
    pub error_title: Option<String>,
    /// error
    /// Represents the following attribute in the schema: :error
    #[xml(attr = "error")]
    pub error: Option<String>,
    /// promptTitle
    /// Represents the following attribute in the schema: :promptTitle
    #[xml(attr = "promptTitle")]
    pub prompt_title: Option<String>,
    /// prompt
    /// Represents the following attribute in the schema: :prompt
    #[xml(attr = "prompt")]
    pub prompt: Option<String>,
    /// sqref
    /// Represents the following attribute in the schema: :sqref
    #[xml(attr = "sqref")]
    pub sequence_of_references: String,
    /// _
    #[xml(child = "x12ac:list")]
    pub list: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2011_1_ac::List,
    >,
    /// _
    #[xml(child = "x:formula1")]
    pub formula1: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Formula1,
    >,
    /// _
    #[xml(child = "x:formula2")]
    pub formula2: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Formula2,
    >,
}
/// Represents a hyperlink within a cell..
/// When the object is serialized out as xml, it's qualified name is xr:hyperlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:hyperlink")]
pub struct Hyperlink {
    /// Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
    /// Relationship Id
    /// Represents the following attribute in the schema: r:id
    #[xml(attr = "r:id")]
    pub id: Option<String>,
    /// Location
    /// Represents the following attribute in the schema: :location
    #[xml(attr = "location")]
    pub location: Option<String>,
    /// Tool Tip
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// Display String
    /// Represents the following attribute in the schema: :display
    #[xml(attr = "display")]
    pub display: Option<String>,
}
/// Represents a sparkline group of 1 or more sparklines..
/// When the object is serialized out as xml, it's qualified name is xr:sparklineGroup.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:sparklineGroup")]
pub struct SparklineGroup {
    /// manualMax
    /// Represents the following attribute in the schema: :manualMax
    #[xml(attr = "manualMax")]
    pub manual_max: Option<f64>,
    /// manualMin
    /// Represents the following attribute in the schema: :manualMin
    #[xml(attr = "manualMin")]
    pub manual_min: Option<f64>,
    /// lineWeight
    /// Represents the following attribute in the schema: :lineWeight
    #[xml(attr = "lineWeight")]
    pub line_weight: Option<f64>,
    /// type
    /// Represents the following attribute in the schema: :type
    #[xml(attr = "type")]
    pub r#type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineTypeValues,
    >,
    /// dateAxis
    /// Represents the following attribute in the schema: :dateAxis
    #[xml(attr = "dateAxis")]
    pub date_axis: Option<bool>,
    /// displayEmptyCellsAs
    /// Represents the following attribute in the schema: :displayEmptyCellsAs
    #[xml(attr = "displayEmptyCellsAs")]
    pub display_empty_cells_as: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DisplayBlanksAsValues,
    >,
    /// markers
    /// Represents the following attribute in the schema: :markers
    #[xml(attr = "markers")]
    pub markers: Option<bool>,
    /// high
    /// Represents the following attribute in the schema: :high
    #[xml(attr = "high")]
    pub high: Option<bool>,
    /// low
    /// Represents the following attribute in the schema: :low
    #[xml(attr = "low")]
    pub low: Option<bool>,
    /// first
    /// Represents the following attribute in the schema: :first
    #[xml(attr = "first")]
    pub first: Option<bool>,
    /// last
    /// Represents the following attribute in the schema: :last
    #[xml(attr = "last")]
    pub last: Option<bool>,
    /// negative
    /// Represents the following attribute in the schema: :negative
    #[xml(attr = "negative")]
    pub negative: Option<bool>,
    /// displayXAxis
    /// Represents the following attribute in the schema: :displayXAxis
    #[xml(attr = "displayXAxis")]
    pub display_x_axis: Option<bool>,
    /// displayHidden
    /// Represents the following attribute in the schema: :displayHidden
    #[xml(attr = "displayHidden")]
    pub display_hidden: Option<bool>,
    /// minAxisType
    /// Represents the following attribute in the schema: :minAxisType
    #[xml(attr = "minAxisType")]
    pub min_axis_type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineAxisMinMaxValues,
    >,
    /// maxAxisType
    /// Represents the following attribute in the schema: :maxAxisType
    #[xml(attr = "maxAxisType")]
    pub max_axis_type: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SparklineAxisMinMaxValues,
    >,
    /// rightToLeft
    /// Represents the following attribute in the schema: :rightToLeft
    #[xml(attr = "rightToLeft")]
    pub right_to_left: Option<bool>,
    /// _
    #[xml(child = "x14:colorSeries")]
    pub series_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SeriesColor,
    >,
    /// _
    #[xml(child = "x14:colorNegative")]
    pub negative_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::NegativeColor,
    >,
    /// _
    #[xml(child = "x14:colorAxis")]
    pub axis_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::AxisColor,
    >,
    /// _
    #[xml(child = "x14:colorMarkers")]
    pub markers_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::MarkersColor,
    >,
    /// _
    #[xml(child = "x14:colorFirst")]
    pub first_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FirstMarkerColor,
    >,
    /// _
    #[xml(child = "x14:colorLast")]
    pub last_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::LastMarkerColor,
    >,
    /// _
    #[xml(child = "x14:colorHigh")]
    pub high_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::HighMarkerColor,
    >,
    /// _
    #[xml(child = "x14:colorLow")]
    pub low_marker_color: Option<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::LowMarkerColor,
    >,
    /// _
    #[xml(child = "xne:f")]
    pub formula: Option<
        crate::schemas::schemas_microsoft_com_office_excel_2006_main::Formula,
    >,
    /// _
    #[xml(child = "x14:sparklines")]
    pub sparklines: crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Sparklines,
}
/// Represents one comment within a cell..
/// When the object is serialized out as xml, it's qualified name is xr:comments.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:comments")]
pub struct Comments {
    ///Authors
    #[xml(child = "x:authors")]
    pub authors: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Authors,
    ///List of Comments
    #[xml(child = "x:commentList")]
    pub comment_list: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CommentList,
    /// _
    #[xml(child = "x:extLst")]
    pub extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Represents an autofilter..
/// When the object is serialized out as xml, it's qualified name is xr:autoFilter.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:autoFilter")]
pub struct AutoFilter {
    /// Cell or Range Reference
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
    /// _
    #[xml(child = "x:filterColumn")]
    pub x_filter_column: Vec<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::FilterColumn,
    >,
    /// _
    #[xml(child = "x:sortState")]
    pub x_sort_state: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SortState,
    >,
    /// _
    #[xml(child = "x:extLst")]
    pub x_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExtensionList,
    >,
}
/// Represents a PivotTable View..
/// When the object is serialized out as xml, it's qualified name is xr:pivotTableDefinition.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "xr:pivotTableDefinition")]
pub struct PivotTableDefinition {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: String,
    /// cacheId
    /// Represents the following attribute in the schema: :cacheId
    #[xml(attr = "cacheId")]
    pub cache_id: i32,
    /// dataOnRows
    /// Represents the following attribute in the schema: :dataOnRows
    #[xml(attr = "dataOnRows")]
    pub data_on_rows: Option<bool>,
    /// dataPosition
    /// Represents the following attribute in the schema: :dataPosition
    #[xml(attr = "dataPosition")]
    pub data_position: Option<i32>,
    /// Auto Format Id
    /// Represents the following attribute in the schema: :autoFormatId
    #[xml(attr = "autoFormatId")]
    pub auto_format_id: Option<i32>,
    /// Apply Number Formats
    /// Represents the following attribute in the schema: :applyNumberFormats
    #[xml(attr = "applyNumberFormats")]
    pub apply_number_formats: Option<bool>,
    /// Apply Border Formats
    /// Represents the following attribute in the schema: :applyBorderFormats
    #[xml(attr = "applyBorderFormats")]
    pub apply_border_formats: Option<bool>,
    /// Apply Font Formats
    /// Represents the following attribute in the schema: :applyFontFormats
    #[xml(attr = "applyFontFormats")]
    pub apply_font_formats: Option<bool>,
    /// Apply Pattern Formats
    /// Represents the following attribute in the schema: :applyPatternFormats
    #[xml(attr = "applyPatternFormats")]
    pub apply_pattern_formats: Option<bool>,
    /// Apply Alignment Formats
    /// Represents the following attribute in the schema: :applyAlignmentFormats
    #[xml(attr = "applyAlignmentFormats")]
    pub apply_alignment_formats: Option<bool>,
    /// Apply Width / Height Formats
    /// Represents the following attribute in the schema: :applyWidthHeightFormats
    #[xml(attr = "applyWidthHeightFormats")]
    pub apply_width_height_formats: Option<bool>,
    /// dataCaption
    /// Represents the following attribute in the schema: :dataCaption
    #[xml(attr = "dataCaption")]
    pub data_caption: String,
    /// grandTotalCaption
    /// Represents the following attribute in the schema: :grandTotalCaption
    #[xml(attr = "grandTotalCaption")]
    pub grand_total_caption: Option<String>,
    /// errorCaption
    /// Represents the following attribute in the schema: :errorCaption
    #[xml(attr = "errorCaption")]
    pub error_caption: Option<String>,
    /// showError
    /// Represents the following attribute in the schema: :showError
    #[xml(attr = "showError")]
    pub show_error: Option<bool>,
    /// missingCaption
    /// Represents the following attribute in the schema: :missingCaption
    #[xml(attr = "missingCaption")]
    pub missing_caption: Option<String>,
    /// showMissing
    /// Represents the following attribute in the schema: :showMissing
    #[xml(attr = "showMissing")]
    pub show_missing: Option<bool>,
    /// pageStyle
    /// Represents the following attribute in the schema: :pageStyle
    #[xml(attr = "pageStyle")]
    pub page_style: Option<String>,
    /// pivotTableStyle
    /// Represents the following attribute in the schema: :pivotTableStyle
    #[xml(attr = "pivotTableStyle")]
    pub pivot_table_style_name: Option<String>,
    /// vacatedStyle
    /// Represents the following attribute in the schema: :vacatedStyle
    #[xml(attr = "vacatedStyle")]
    pub vacated_style: Option<String>,
    /// tag
    /// Represents the following attribute in the schema: :tag
    #[xml(attr = "tag")]
    pub tag: Option<String>,
    /// updatedVersion
    /// Represents the following attribute in the schema: :updatedVersion
    #[xml(attr = "updatedVersion")]
    pub updated_version: Option<u8>,
    /// minRefreshableVersion
    /// Represents the following attribute in the schema: :minRefreshableVersion
    #[xml(attr = "minRefreshableVersion")]
    pub min_refreshable_version: Option<u8>,
    /// asteriskTotals
    /// Represents the following attribute in the schema: :asteriskTotals
    #[xml(attr = "asteriskTotals")]
    pub asterisk_totals: Option<bool>,
    /// showItems
    /// Represents the following attribute in the schema: :showItems
    #[xml(attr = "showItems")]
    pub show_items: Option<bool>,
    /// editData
    /// Represents the following attribute in the schema: :editData
    #[xml(attr = "editData")]
    pub edit_data: Option<bool>,
    /// disableFieldList
    /// Represents the following attribute in the schema: :disableFieldList
    #[xml(attr = "disableFieldList")]
    pub disable_field_list: Option<bool>,
    /// showCalcMbrs
    /// Represents the following attribute in the schema: :showCalcMbrs
    #[xml(attr = "showCalcMbrs")]
    pub show_calculated_members: Option<bool>,
    /// visualTotals
    /// Represents the following attribute in the schema: :visualTotals
    #[xml(attr = "visualTotals")]
    pub visual_totals: Option<bool>,
    /// showMultipleLabel
    /// Represents the following attribute in the schema: :showMultipleLabel
    #[xml(attr = "showMultipleLabel")]
    pub show_multiple_label: Option<bool>,
    /// showDataDropDown
    /// Represents the following attribute in the schema: :showDataDropDown
    #[xml(attr = "showDataDropDown")]
    pub show_data_drop_down: Option<bool>,
    /// showDrill
    /// Represents the following attribute in the schema: :showDrill
    #[xml(attr = "showDrill")]
    pub show_drill: Option<bool>,
    /// printDrill
    /// Represents the following attribute in the schema: :printDrill
    #[xml(attr = "printDrill")]
    pub print_drill: Option<bool>,
    /// showMemberPropertyTips
    /// Represents the following attribute in the schema: :showMemberPropertyTips
    #[xml(attr = "showMemberPropertyTips")]
    pub show_member_property_tips: Option<bool>,
    /// showDataTips
    /// Represents the following attribute in the schema: :showDataTips
    #[xml(attr = "showDataTips")]
    pub show_data_tips: Option<bool>,
    /// enableWizard
    /// Represents the following attribute in the schema: :enableWizard
    #[xml(attr = "enableWizard")]
    pub enable_wizard: Option<bool>,
    /// enableDrill
    /// Represents the following attribute in the schema: :enableDrill
    #[xml(attr = "enableDrill")]
    pub enable_drill: Option<bool>,
    /// enableFieldProperties
    /// Represents the following attribute in the schema: :enableFieldProperties
    #[xml(attr = "enableFieldProperties")]
    pub enable_field_properties: Option<bool>,
    /// preserveFormatting
    /// Represents the following attribute in the schema: :preserveFormatting
    #[xml(attr = "preserveFormatting")]
    pub preserve_formatting: Option<bool>,
    /// useAutoFormatting
    /// Represents the following attribute in the schema: :useAutoFormatting
    #[xml(attr = "useAutoFormatting")]
    pub use_auto_formatting: Option<bool>,
    /// pageWrap
    /// Represents the following attribute in the schema: :pageWrap
    #[xml(attr = "pageWrap")]
    pub page_wrap: Option<i32>,
    /// pageOverThenDown
    /// Represents the following attribute in the schema: :pageOverThenDown
    #[xml(attr = "pageOverThenDown")]
    pub page_over_then_down: Option<bool>,
    /// subtotalHiddenItems
    /// Represents the following attribute in the schema: :subtotalHiddenItems
    #[xml(attr = "subtotalHiddenItems")]
    pub subtotal_hidden_items: Option<bool>,
    /// rowGrandTotals
    /// Represents the following attribute in the schema: :rowGrandTotals
    #[xml(attr = "rowGrandTotals")]
    pub row_grand_totals: Option<bool>,
    /// colGrandTotals
    /// Represents the following attribute in the schema: :colGrandTotals
    #[xml(attr = "colGrandTotals")]
    pub column_grand_totals: Option<bool>,
    /// fieldPrintTitles
    /// Represents the following attribute in the schema: :fieldPrintTitles
    #[xml(attr = "fieldPrintTitles")]
    pub field_print_titles: Option<bool>,
    /// itemPrintTitles
    /// Represents the following attribute in the schema: :itemPrintTitles
    #[xml(attr = "itemPrintTitles")]
    pub item_print_titles: Option<bool>,
    /// mergeItem
    /// Represents the following attribute in the schema: :mergeItem
    #[xml(attr = "mergeItem")]
    pub merge_item: Option<bool>,
    /// showDropZones
    /// Represents the following attribute in the schema: :showDropZones
    #[xml(attr = "showDropZones")]
    pub show_drop_zones: Option<bool>,
    /// createdVersion
    /// Represents the following attribute in the schema: :createdVersion
    #[xml(attr = "createdVersion")]
    pub created_version: Option<u8>,
    /// indent
    /// Represents the following attribute in the schema: :indent
    #[xml(attr = "indent")]
    pub indent: Option<i32>,
    /// showEmptyRow
    /// Represents the following attribute in the schema: :showEmptyRow
    #[xml(attr = "showEmptyRow")]
    pub show_empty_row: Option<bool>,
    /// showEmptyCol
    /// Represents the following attribute in the schema: :showEmptyCol
    #[xml(attr = "showEmptyCol")]
    pub show_empty_column: Option<bool>,
    /// showHeaders
    /// Represents the following attribute in the schema: :showHeaders
    #[xml(attr = "showHeaders")]
    pub show_headers: Option<bool>,
    /// compact
    /// Represents the following attribute in the schema: :compact
    #[xml(attr = "compact")]
    pub compact: Option<bool>,
    /// outline
    /// Represents the following attribute in the schema: :outline
    #[xml(attr = "outline")]
    pub outline: Option<bool>,
    /// outlineData
    /// Represents the following attribute in the schema: :outlineData
    #[xml(attr = "outlineData")]
    pub outline_data: Option<bool>,
    /// compactData
    /// Represents the following attribute in the schema: :compactData
    #[xml(attr = "compactData")]
    pub compact_data: Option<bool>,
    /// published
    /// Represents the following attribute in the schema: :published
    #[xml(attr = "published")]
    pub published: Option<bool>,
    /// gridDropZones
    /// Represents the following attribute in the schema: :gridDropZones
    #[xml(attr = "gridDropZones")]
    pub grid_drop_zones: Option<bool>,
    /// immersive
    /// Represents the following attribute in the schema: :immersive
    #[xml(attr = "immersive")]
    pub stop_immersive_ui: Option<bool>,
    /// multipleFieldFilters
    /// Represents the following attribute in the schema: :multipleFieldFilters
    #[xml(attr = "multipleFieldFilters")]
    pub multiple_field_filters: Option<bool>,
    /// chartFormat
    /// Represents the following attribute in the schema: :chartFormat
    #[xml(attr = "chartFormat")]
    pub chart_format: Option<i32>,
    /// rowHeaderCaption
    /// Represents the following attribute in the schema: :rowHeaderCaption
    #[xml(attr = "rowHeaderCaption")]
    pub row_header_caption: Option<String>,
    /// colHeaderCaption
    /// Represents the following attribute in the schema: :colHeaderCaption
    #[xml(attr = "colHeaderCaption")]
    pub column_header_caption: Option<String>,
    /// fieldListSortAscending
    /// Represents the following attribute in the schema: :fieldListSortAscending
    #[xml(attr = "fieldListSortAscending")]
    pub field_list_sort_ascending: Option<bool>,
    /// mdxSubqueries
    /// Represents the following attribute in the schema: :mdxSubqueries
    #[xml(attr = "mdxSubqueries")]
    pub mdx_subqueries: Option<bool>,
    /// customListSort
    /// Represents the following attribute in the schema: :customListSort
    #[xml(attr = "customListSort")]
    pub custom_list_sort: Option<bool>,
    /// _
    #[xml(child = "x:location")]
    pub location: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Location,
    /// _
    #[xml(child = "x:pivotFields")]
    pub pivot_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotFields,
    >,
    /// _
    #[xml(child = "x:rowFields")]
    pub row_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowFields,
    >,
    /// _
    #[xml(child = "x:rowItems")]
    pub row_items: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowItems,
    >,
    /// _
    #[xml(child = "x:colFields")]
    pub column_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnFields,
    >,
    /// _
    #[xml(child = "x:colItems")]
    pub column_items: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnItems,
    >,
    /// _
    #[xml(child = "x:pageFields")]
    pub page_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageFields,
    >,
    /// _
    #[xml(child = "x:dataFields")]
    pub data_fields: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DataFields,
    >,
    /// _
    #[xml(child = "x:formats")]
    pub formats: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Formats,
    >,
    /// _
    #[xml(child = "x:conditionalFormats")]
    pub conditional_formats: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ConditionalFormats,
    >,
    /// _
    #[xml(child = "x:chartFormats")]
    pub chart_formats: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ChartFormats,
    >,
    /// _
    #[xml(child = "x:pivotHierarchies")]
    pub pivot_hierarchies: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotHierarchies,
    >,
    /// _
    #[xml(child = "x:pivotTableStyleInfo")]
    pub pivot_table_style: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableStyle,
    >,
    /// _
    #[xml(child = "x:filters")]
    pub pivot_filters: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotFilters,
    >,
    /// _
    #[xml(child = "x:rowHierarchiesUsage")]
    pub row_hierarchies_usage: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::RowHierarchiesUsage,
    >,
    /// _
    #[xml(child = "x:colHierarchiesUsage")]
    pub column_hierarchies_usage: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ColumnHierarchiesUsage,
    >,
    /// _
    #[xml(child = "x:extLst")]
    pub pivot_table_definition_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinitionExtensionList,
    >,
}
