#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ResourceLinkage {
    #[default]
    Embed,
    Link,
    LinkAndEmbed,
}
crate::__string_enum! {
    ResourceLinkage { Embed = "embed", Link = "link", LinkAndEmbed = "linkAndEmbed", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DetachConnection {
    #[default]
    Start,
    End,
    Both,
}
crate::__string_enum! {
    DetachConnection { Start = "start", End = "end", Both = "both", }
}
/// Defines the ShapeMoniker Class.
/// When the object is serialized out as xml, it's qualified name is oac:spMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:spMk")]
pub struct ShapeMoniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// creationId
    /// Represents the following attribute in the schema: :creationId
    #[xml(attr = "creationId")]
    pub creation_id: Option<String>,
}
/// Defines the GroupShapeMoniker Class.
/// When the object is serialized out as xml, it's qualified name is oac:grpSpMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:grpSpMk")]
pub struct GroupShapeMoniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// creationId
    /// Represents the following attribute in the schema: :creationId
    #[xml(attr = "creationId")]
    pub creation_id: Option<String>,
}
/// Defines the GraphicFrameMoniker Class.
/// When the object is serialized out as xml, it's qualified name is oac:graphicFrameMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:graphicFrameMk")]
pub struct GraphicFrameMoniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// creationId
    /// Represents the following attribute in the schema: :creationId
    #[xml(attr = "creationId")]
    pub creation_id: Option<String>,
}
/// Defines the ConnectorMoniker Class.
/// When the object is serialized out as xml, it's qualified name is oac:cxnSpMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cxnSpMk")]
pub struct ConnectorMoniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// creationId
    /// Represents the following attribute in the schema: :creationId
    #[xml(attr = "creationId")]
    pub creation_id: Option<String>,
}
/// Defines the PictureMoniker Class.
/// When the object is serialized out as xml, it's qualified name is oac:picMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:picMk")]
pub struct PictureMoniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// creationId
    /// Represents the following attribute in the schema: :creationId
    #[xml(attr = "creationId")]
    pub creation_id: Option<String>,
}
/// Defines the InkMoniker Class.
/// When the object is serialized out as xml, it's qualified name is oac:inkMk.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:inkMk")]
pub struct InkMoniker {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// creationId
    /// Represents the following attribute in the schema: :creationId
    #[xml(attr = "creationId")]
    pub creation_id: Option<String>,
}
/// Defines the DrawingMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:dgMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:dgMkLst")]
pub struct DrawingMonikerList {}
/// Defines the Transform2D Class.
/// When the object is serialized out as xml, it's qualified name is oac:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:xfrm")]
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
/// Defines the GroupShapeMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:grpSpMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:grpSpMkLst")]
pub struct GroupShapeMonikerList {}
/// Defines the DrawingElementPackage Class.
/// When the object is serialized out as xml, it's qualified name is oac:dePkg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:dePkg")]
pub struct DrawingElementPackage {}
/// Defines the DeMkLstDrawingElementMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:deMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:deMkLst")]
pub struct DeMkLstDrawingElementMonikerList {}
/// Defines the DeMasterMkLstDrawingElementMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:deMasterMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:deMasterMkLst")]
pub struct DeMasterMkLstDrawingElementMonikerList {}
/// Defines the DeSrcMkLstDrawingElementMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:deSrcMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:deSrcMkLst")]
pub struct DeSrcMkLstDrawingElementMonikerList {}
/// Defines the DeTgtMkLstDrawingElementMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:deTgtMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:deTgtMkLst")]
pub struct DeTgtMkLstDrawingElementMonikerList {}
/// Defines the OpenXmlDrawingElementMonikerListElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlDrawingElementMonikerListElement {}
/// Defines the ImgDataImgData Class.
/// When the object is serialized out as xml, it's qualified name is oac:imgData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:imgData")]
pub struct ImgDataImgData {
    #[xml(text)]
    pub child: String,
}
/// Defines the OrigImgDataImgData Class.
/// When the object is serialized out as xml, it's qualified name is oac:origImgData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:origImgData")]
pub struct OrigImgDataImgData {
    #[xml(text)]
    pub child: String,
}
/// Defines the SndDataImgData Class.
/// When the object is serialized out as xml, it's qualified name is oac:sndData.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:sndData")]
pub struct SndDataImgData {
    #[xml(text)]
    pub child: String,
}
/// Defines the OpenXmlImgDataElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlImgDataElement {
    #[xml(text)]
    pub child: String,
}
/// Defines the ResourceUrl Class.
/// When the object is serialized out as xml, it's qualified name is oac:imgUrl.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:imgUrl")]
pub struct ResourceUrl {
    /// src
    /// Represents the following attribute in the schema: :src
    #[xml(attr = "src")]
    pub src: Option<String>,
    /// linkage
    /// Represents the following attribute in the schema: :linkage
    #[xml(attr = "linkage")]
    pub linkage: Option<ResourceLinkage>,
}
/// Defines the TextBodyPackage Class.
/// When the object is serialized out as xml, it's qualified name is oac:txBodyPkg.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:txBodyPkg")]
pub struct TextBodyPackage {}
/// Defines the GroupCommand Class.
/// When the object is serialized out as xml, it's qualified name is oac:grpCmd.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:grpCmd")]
pub struct GroupCommand {
    /// verId
    /// Represents the following attribute in the schema: :verId
    #[xml(attr = "verId")]
    pub ver_id: Option<i32>,
    /// preventRegroup
    /// Represents the following attribute in the schema: :preventRegroup
    #[xml(attr = "preventRegroup")]
    pub prevent_regroup: Option<bool>,
    /// grpId
    /// Represents the following attribute in the schema: :grpId
    #[xml(attr = "grpId")]
    pub grp_id: Option<i32>,
    #[xml(
        child = "oac:dgMkLst",
        child = "oac:spMk",
        child = "oac:grpSpMk",
        child = "oac:graphicFrameMk",
        child = "oac:cxnSpMk",
        child = "oac:picMk",
        child = "oac:inkMk",
        child = "oac:grpSpPr",
        child = "oac:cNvPr",
        child = "oac:cNvGrpSpPr",
    )]
    pub children: Vec<GroupCommandChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupCommandChildChoice {
    #[xml(tag = "oac:dgMkLst")]
    OacDgMkLst(DrawingMonikerList),
    #[xml(tag = "oac:spMk")]
    OacSpMk(ShapeMoniker),
    #[xml(tag = "oac:grpSpMk")]
    OacGrpSpMk(GroupShapeMoniker),
    #[xml(tag = "oac:graphicFrameMk")]
    OacGraphicFrameMk(GraphicFrameMoniker),
    #[xml(tag = "oac:cxnSpMk")]
    OacCxnSpMk(ConnectorMoniker),
    #[xml(tag = "oac:picMk")]
    OacPicMk(PictureMoniker),
    #[xml(tag = "oac:inkMk")]
    OacInkMk(InkMoniker),
    #[xml(tag = "oac:grpSpPr")]
    OacGrpSpPr(GroupShapeProperties),
    #[xml(tag = "oac:cNvPr")]
    OacCNvPr(NonVisualDrawingProps),
    #[xml(tag = "oac:cNvGrpSpPr")]
    OacCNvGrpSpPr(NonVisualGroupDrawingShapeProps),
}
/// Defines the ImgLink Class.
/// When the object is serialized out as xml, it's qualified name is oac:imgLink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:imgLink")]
pub struct ImgLink {
    /// tgt
    /// Represents the following attribute in the schema: :tgt
    #[xml(attr = "tgt")]
    pub tgt: String,
}
/// Defines the DocumentContextMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:dcMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:dcMkLst")]
pub struct DocumentContextMonikerList {}
/// Defines the GraphicParentMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:graphicParentMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:graphicParentMkLst")]
pub struct GraphicParentMonikerList {}
/// Defines the ShapeMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:spMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:spMkLst")]
pub struct ShapeMonikerList {}
/// Defines the GraphicFrameMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:graphicFrameMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:graphicFrameMkLst")]
pub struct GraphicFrameMonikerList {}
/// Defines the ConnectorMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:cxnSpMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cxnSpMkLst")]
pub struct ConnectorMonikerList {}
/// Defines the PictureMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:picMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:picMkLst")]
pub struct PictureMonikerList {}
/// Defines the InkMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:inkMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:inkMkLst")]
pub struct InkMonikerList {}
/// Defines the TextBodyMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:txBodyMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:txBodyMkLst")]
pub struct TextBodyMonikerList {}
/// Defines the TextCharRangeMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:txMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:txMkLst")]
pub struct TextCharRangeMonikerList {}
/// Defines the HyperlinkMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlinkMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlinkMkLst")]
pub struct HyperlinkMonikerList {}
/// Defines the Model3DMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:model3DMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:model3DMkLst")]
pub struct Model3DMonikerList {}
/// Defines the ViewSelectionStgList Class.
/// When the object is serialized out as xml, it's qualified name is oac:viewSelLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:viewSelLst")]
pub struct ViewSelectionStgList {}
/// Defines the EditorSelectionStgList Class.
/// When the object is serialized out as xml, it's qualified name is oac:editorSelLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:editorSelLst")]
pub struct EditorSelectionStgList {}
/// Defines the DrawingSelectionStgList Class.
/// When the object is serialized out as xml, it's qualified name is oac:drSelLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:drSelLst")]
pub struct DrawingSelectionStgList {}
/// Defines the TableMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:tblMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:tblMkLst")]
pub struct TableMonikerList {}
/// Defines the TableCellMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:tcMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:tcMkLst")]
pub struct TableCellMonikerList {}
/// Defines the TableRowMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:trMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:trMkLst")]
pub struct TableRowMonikerList {}
/// Defines the TableColumnMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:gridColMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:gridColMkLst")]
pub struct TableColumnMonikerList {}
/// Defines the ModifyNonVisualDrawingProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvPr")]
pub struct ModifyNonVisualDrawingProps {
    /// name
    /// Represents the following attribute in the schema: :name
    #[xml(attr = "name")]
    pub name: Option<String>,
    /// descr
    /// Represents the following attribute in the schema: :descr
    #[xml(attr = "descr")]
    pub descr: Option<String>,
    /// hidden
    /// Represents the following attribute in the schema: :hidden
    #[xml(attr = "hidden")]
    pub hidden: Option<bool>,
    /// title
    /// Represents the following attribute in the schema: :title
    #[xml(attr = "title")]
    pub title: Option<String>,
    /// decor
    /// Represents the following attribute in the schema: :decor
    #[xml(attr = "decor")]
    pub decor: Option<bool>,
    /// scriptLink
    /// Represents the following attribute in the schema: :scriptLink
    #[xml(attr = "scriptLink")]
    pub script_link: Option<String>,
}
/// Defines the ModifyTransformProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:xfrm")]
pub struct ModifyTransformProps {
    /// x
    /// Represents the following attribute in the schema: :x
    #[xml(attr = "x")]
    pub x: Option<i64>,
    /// y
    /// Represents the following attribute in the schema: :y
    #[xml(attr = "y")]
    pub y: Option<i64>,
    /// cx
    /// Represents the following attribute in the schema: :cx
    #[xml(attr = "cx")]
    pub cx: Option<i64>,
    /// cy
    /// Represents the following attribute in the schema: :cy
    #[xml(attr = "cy")]
    pub cy: Option<i64>,
    /// rot
    /// Represents the following attribute in the schema: :rot
    #[xml(attr = "rot")]
    pub rot: Option<i32>,
    /// flipH
    /// Represents the following attribute in the schema: :flipH
    #[xml(attr = "flipH")]
    pub flip_h: Option<bool>,
    /// flipV
    /// Represents the following attribute in the schema: :flipV
    #[xml(attr = "flipV")]
    pub flip_v: Option<bool>,
}
/// Defines the Point2DType Class.
/// When the object is serialized out as xml, it's qualified name is oac:off.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:off")]
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
/// Defines the TextParagraphPropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is oac:pPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:pPr")]
pub struct TextParagraphPropertiesType {
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
    pub alignment: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAlignmentTypeValues,
    >,
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
    pub font_alignment: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextFontAlignmentValues,
    >,
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
    pub children: Vec<TextParagraphPropertiesTypeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextParagraphPropertiesTypeChildChoice {
    #[xml(tag = "a:lnSpc")]
    ALnSpc(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineSpacing),
    #[xml(tag = "a:spcBef")]
    ASpcBef(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SpaceBefore),
    #[xml(tag = "a:spcAft")]
    ASpcAft(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SpaceAfter),
    #[xml(tag = "a:buClrTx")]
    ABuClrTx(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletColorText,
    ),
    #[xml(tag = "a:buClr")]
    ABuClr(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletColor),
    #[xml(tag = "a:buSzTx")]
    ABuSzTx(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizeText,
    ),
    #[xml(tag = "a:buSzPct")]
    ABuSzPct(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizePercentage,
    ),
    #[xml(tag = "a:buSzPts")]
    ABuSzPts(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletSizePoints,
    ),
    #[xml(tag = "a:buFontTx")]
    ABuFontTx(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletFontText,
    ),
    #[xml(tag = "a:buFont")]
    ABuFont(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BulletFont),
    #[xml(tag = "a:buNone")]
    ABuNone(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoBullet),
    #[xml(tag = "a:buAutoNum")]
    ABuAutoNum(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AutoNumberedBullet,
    ),
    #[xml(tag = "a:buChar")]
    ABuChar(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CharacterBullet,
    ),
    #[xml(tag = "a:buBlip")]
    ABuBlip(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PictureBullet,
    ),
    #[xml(tag = "a:tabLst")]
    ATabLst(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TabStopList),
    #[xml(tag = "a:defRPr")]
    ADefRPr(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::DefaultRunProperties,
    ),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Defines the TextBodyProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:bodyPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:bodyPr")]
pub struct TextBodyProperties {
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
    pub vertical_overflow: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalOverflowValues,
    >,
    /// Text Horizontal Overflow
    /// Represents the following attribute in the schema: :horzOverflow
    #[xml(attr = "horzOverflow")]
    pub horizontal_overflow: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextHorizontalOverflowValues,
    >,
    /// Vertical Text
    /// Represents the following attribute in the schema: :vert
    #[xml(attr = "vert")]
    pub vertical: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextVerticalValues,
    >,
    /// Text Wrapping Type
    /// Represents the following attribute in the schema: :wrap
    #[xml(attr = "wrap")]
    pub wrap: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextWrappingValues,
    >,
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
    pub anchor: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TextAnchoringTypeValues,
    >,
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
    pub children: Vec<TextBodyPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum TextBodyPropertiesChildChoice {
    #[xml(tag = "a:prstTxWarp")]
    APrstTxWarp(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetTextWarp,
    ),
    #[xml(tag = "a:noAutofit")]
    ANoAutofit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoAutoFit,
    ),
    #[xml(tag = "a:normAutofit")]
    ANormAutofit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NormalAutoFit,
    ),
    #[xml(tag = "a:spAutoFit")]
    ASpAutoFit(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ShapeAutoFit,
    ),
    #[xml(tag = "a:scene3d")]
    AScene3d(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Scene3DType,
    ),
    #[xml(tag = "a:sp3d")]
    ASp3d(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Shape3DType),
    #[xml(tag = "a:flatTx")]
    AFlatTx(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FlatText),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ExtensionList,
    ),
}
/// Defines the ModifyNonVisualDrawingShapeProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvSpPr")]
pub struct ModifyNonVisualDrawingShapeProps {
    /// noGrp
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grp: Option<bool>,
    /// noSelect
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_select: Option<bool>,
    /// noRot
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rot: Option<bool>,
    /// noChangeAspect
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// noMove
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// noResize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// noEditPoints
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// noAdjustHandles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// noChangeArrowheads
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// noChangeShapeType
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// noTextEdit
    /// Represents the following attribute in the schema: :noTextEdit
    #[xml(attr = "noTextEdit")]
    pub no_text_edit: Option<bool>,
    /// txBox
    /// Represents the following attribute in the schema: :txBox
    #[xml(attr = "txBox")]
    pub tx_box: Option<bool>,
}
/// Defines the ShapePropsMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:spMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:spMkLst")]
pub struct ShapePropsMonikerList {}
/// Defines the ShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:spPr")]
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
/// Defines the XfrmEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:xfrm.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:xfrm")]
pub struct XfrmEmpty {}
/// Defines the GeomEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:geom.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:geom")]
pub struct GeomEmpty {}
/// Defines the FillEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:fill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:fill")]
pub struct FillEmpty {}
/// Defines the LnEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:ln.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:ln")]
pub struct LnEmpty {}
/// Defines the EffectEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:effect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:effect")]
pub struct EffectEmpty {}
/// Defines the Scene3dEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:scene3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:scene3d")]
pub struct Scene3dEmpty {}
/// Defines the Sp3dEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:sp3d.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:sp3d")]
pub struct Sp3dEmpty {}
/// Defines the ExtLstEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:extLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:extLst")]
pub struct ExtLstEmpty {}
/// Defines the BwModeEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:bwMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:bwMode")]
pub struct BwModeEmpty {}
/// Defines the SrcRectEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:srcRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:srcRect")]
pub struct SrcRectEmpty {}
/// Defines the FillModeEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:fillMode.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:fillMode")]
pub struct FillModeEmpty {}
/// Defines the DpiEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:dpi.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:dpi")]
pub struct DpiEmpty {}
/// Defines the RotWithShapeEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:rotWithShape.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:rotWithShape")]
pub struct RotWithShapeEmpty {}
/// Defines the StCxnEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:stCxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:stCxn")]
pub struct StCxnEmpty {}
/// Defines the EndCxnEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:endCxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:endCxn")]
pub struct EndCxnEmpty {}
/// Defines the NoGrpEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noGrp.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noGrp")]
pub struct NoGrpEmpty {}
/// Defines the NoSelectEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noSelect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noSelect")]
pub struct NoSelectEmpty {}
/// Defines the NoRotEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noRot.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noRot")]
pub struct NoRotEmpty {}
/// Defines the NoChangeAspectEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noChangeAspect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noChangeAspect")]
pub struct NoChangeAspectEmpty {}
/// Defines the NoMoveEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noMove.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noMove")]
pub struct NoMoveEmpty {}
/// Defines the NoResizeEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noResize.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noResize")]
pub struct NoResizeEmpty {}
/// Defines the NoEditPointsEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noEditPoints.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noEditPoints")]
pub struct NoEditPointsEmpty {}
/// Defines the NoAdjustHandlesEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noAdjustHandles.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noAdjustHandles")]
pub struct NoAdjustHandlesEmpty {}
/// Defines the NoChangeArrowheadsEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noChangeArrowheads.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noChangeArrowheads")]
pub struct NoChangeArrowheadsEmpty {}
/// Defines the NoChangeShapeTypeEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:noChangeShapeType.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:noChangeShapeType")]
pub struct NoChangeShapeTypeEmpty {}
/// Defines the LfPrEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:lfPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:lfPr")]
pub struct LfPrEmpty {}
/// Defines the HlinkClickEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlinkClick.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlinkClick")]
pub struct HlinkClickEmpty {}
/// Defines the HlinkHoverEmpty Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlinkHover.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlinkHover")]
pub struct HlinkHoverEmpty {}
/// Defines the OpenXmlEmptyElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlEmptyElement {}
/// Defines the ResetShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:spPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:spPr")]
pub struct ResetShapeProperties {
    /// _
    #[xml(child = "oac:xfrm")]
    pub xfrm_empty: Option<XfrmEmpty>,
    /// _
    #[xml(child = "oac:geom")]
    pub geom_empty: Option<GeomEmpty>,
    /// _
    #[xml(child = "oac:fill")]
    pub fill_empty: Option<FillEmpty>,
    /// _
    #[xml(child = "oac:ln")]
    pub ln_empty: Option<LnEmpty>,
    /// _
    #[xml(child = "oac:effect")]
    pub effect_empty: Option<EffectEmpty>,
    /// _
    #[xml(child = "oac:scene3d")]
    pub scene3d_empty: Option<Scene3dEmpty>,
    /// _
    #[xml(child = "oac:sp3d")]
    pub sp3d_empty: Option<Sp3dEmpty>,
    /// _
    #[xml(child = "oac:extLst")]
    pub ext_lst_empty: Option<ExtLstEmpty>,
    /// _
    #[xml(child = "oac:bwMode")]
    pub bw_mode_empty: Option<BwModeEmpty>,
}
/// Defines the LnRefStyleMatrixReference Class.
/// When the object is serialized out as xml, it's qualified name is oac:lnRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:lnRef")]
pub struct LnRefStyleMatrixReference {
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
    pub children: Vec<LnRefStyleMatrixReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LnRefStyleMatrixReferenceChildChoice {
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
/// Defines the FillRefStyleMatrixReference Class.
/// When the object is serialized out as xml, it's qualified name is oac:fillRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:fillRef")]
pub struct FillRefStyleMatrixReference {
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
    pub children: Vec<FillRefStyleMatrixReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum FillRefStyleMatrixReferenceChildChoice {
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
/// Defines the EffectRefStyleMatrixReference Class.
/// When the object is serialized out as xml, it's qualified name is oac:effectRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:effectRef")]
pub struct EffectRefStyleMatrixReference {
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
    pub children: Vec<EffectRefStyleMatrixReferenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EffectRefStyleMatrixReferenceChildChoice {
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
/// Defines the StyleMatrixReferenceType Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct StyleMatrixReferenceType {
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
/// Defines the FontReference Class.
/// When the object is serialized out as xml, it's qualified name is oac:fontRef.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:fontRef")]
pub struct FontReference {
    /// Identifier
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FontCollectionIndexValues,
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
/// Defines the ModifyShapeStyleProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:style.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:style")]
pub struct ModifyShapeStyleProps {
    /// _
    #[xml(child = "oac:lnRef")]
    pub ln_ref_style_matrix_reference: Option<LnRefStyleMatrixReference>,
    /// _
    #[xml(child = "oac:fillRef")]
    pub fill_ref_style_matrix_reference: Option<FillRefStyleMatrixReference>,
    /// _
    #[xml(child = "oac:effectRef")]
    pub effect_ref_style_matrix_reference: Option<EffectRefStyleMatrixReference>,
    /// _
    #[xml(child = "oac:fontRef")]
    pub font_reference: Option<FontReference>,
}
/// Defines the ResetXsdboolean Class.
/// When the object is serialized out as xml, it's qualified name is oac:reset.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:reset")]
pub struct ResetXsdboolean {
    #[xml(text)]
    pub child: bool,
}
/// Defines the UseBoundsXsdboolean Class.
/// When the object is serialized out as xml, it's qualified name is oac:useBounds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:useBounds")]
pub struct UseBoundsXsdboolean {
    #[xml(text)]
    pub child: bool,
}
/// Defines the BlipFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:blipFill")]
pub struct BlipFillProperties {
    /// DPI Setting
    /// Represents the following attribute in the schema: :dpi
    #[xml(attr = "dpi")]
    pub dpi: Option<i32>,
    /// Rotate With Shape
    /// Represents the following attribute in the schema: :rotWithShape
    #[xml(attr = "rotWithShape")]
    pub rotate_with_shape: Option<bool>,
    #[xml(child = "a:blip", child = "a:srcRect", child = "a:tile", child = "a:stretch")]
    pub children: Vec<BlipFillPropertiesChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum BlipFillPropertiesChildChoice {
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
/// Defines the FillRectRelativeRectProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:fillRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:fillRect")]
pub struct FillRectRelativeRectProps {
    /// l
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub l: Option<i32>,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<i32>,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub b: Option<i32>,
}
/// Defines the SrcRectRelativeRectProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:srcRect.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:srcRect")]
pub struct SrcRectRelativeRectProps {
    /// l
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub l: Option<i32>,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<i32>,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub b: Option<i32>,
}
/// Defines the OpenXmlRelativeRectPropsElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlRelativeRectPropsElement {
    /// l
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub l: Option<i32>,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: Option<i32>,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: Option<i32>,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub b: Option<i32>,
}
/// Defines the ResetBlipFillProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:blipFill.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:blipFill")]
pub struct ResetBlipFillProperties {
    /// _
    #[xml(child = "oac:srcRect")]
    pub src_rect_empty: Option<SrcRectEmpty>,
    /// _
    #[xml(child = "oac:fillMode")]
    pub fill_mode_empty: Option<FillModeEmpty>,
    /// _
    #[xml(child = "oac:dpi")]
    pub dpi_empty: Option<DpiEmpty>,
    /// _
    #[xml(child = "oac:rotWithShape")]
    pub rot_with_shape_empty: Option<RotWithShapeEmpty>,
}
/// Defines the ModifyNonVisualGroupDrawingShapeProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvGrpSpPr")]
pub struct ModifyNonVisualGroupDrawingShapeProps {
    /// noGrp
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grp: Option<bool>,
    /// noUngrp
    /// Represents the following attribute in the schema: :noUngrp
    #[xml(attr = "noUngrp")]
    pub no_ungrp: Option<bool>,
    /// noSelect
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_select: Option<bool>,
    /// noRot
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rot: Option<bool>,
    /// noChangeAspect
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// noMove
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// noResize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
}
/// Defines the GroupShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:grpSpPr")]
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
/// Defines the ResetGroupShapeProperties Class.
/// When the object is serialized out as xml, it's qualified name is oac:grpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:grpSpPr")]
pub struct ResetGroupShapeProperties {
    /// _
    #[xml(child = "oac:xfrm")]
    pub xfrm_empty: Option<XfrmEmpty>,
    /// _
    #[xml(child = "oac:fill")]
    pub fill_empty: Option<FillEmpty>,
    /// _
    #[xml(child = "oac:effect")]
    pub effect_empty: Option<EffectEmpty>,
    /// _
    #[xml(child = "oac:scene3d")]
    pub scene3d_empty: Option<Scene3dEmpty>,
    /// _
    #[xml(child = "oac:extLst")]
    pub ext_lst_empty: Option<ExtLstEmpty>,
    /// _
    #[xml(child = "oac:bwMode")]
    pub bw_mode_empty: Option<BwModeEmpty>,
}
/// Defines the NonVisualDrawingProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvPr")]
pub struct NonVisualDrawingProps {
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
/// Defines the NonVisualGroupDrawingShapeProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvGrpSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvGrpSpPr")]
pub struct NonVisualGroupDrawingShapeProps {
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
/// Defines the ModifyNonVisualGraphicFrameProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvGraphicFramePr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvGraphicFramePr")]
pub struct ModifyNonVisualGraphicFrameProps {
    /// noGrp
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grp: Option<bool>,
    /// noDrilldown
    /// Represents the following attribute in the schema: :noDrilldown
    #[xml(attr = "noDrilldown")]
    pub no_drilldown: Option<bool>,
    /// noSelect
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_select: Option<bool>,
    /// noChangeAspect
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// noMove
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// noResize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
}
/// Defines the StCxnConnection Class.
/// When the object is serialized out as xml, it's qualified name is oac:stCxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:stCxn")]
pub struct StCxnConnection {
    /// Identifier
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: i32,
    /// Index
    /// Represents the following attribute in the schema: :idx
    #[xml(attr = "idx")]
    pub index: i32,
}
/// Defines the EndCxnConnection Class.
/// When the object is serialized out as xml, it's qualified name is oac:endCxn.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:endCxn")]
pub struct EndCxnConnection {
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
/// Defines the ModifyNonVisualConnectorProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvCxnSpPr")]
pub struct ModifyNonVisualConnectorProps {
    /// noGrp
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grp: Option<bool>,
    /// noSelect
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_select: Option<bool>,
    /// noRot
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rot: Option<bool>,
    /// noChangeAspect
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// noMove
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// noResize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// noEditPoints
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// noAdjustHandles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// noChangeArrowheads
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// noChangeShapeType
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// _
    #[xml(child = "oac:stCxn")]
    pub st_cxn_connection: Option<StCxnConnection>,
    /// _
    #[xml(child = "oac:endCxn")]
    pub end_cxn_connection: Option<EndCxnConnection>,
}
/// Defines the ResetNonVisualConnectorProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvCxnSpPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvCxnSpPr")]
pub struct ResetNonVisualConnectorProps {
    /// _
    #[xml(child = "oac:stCxn")]
    pub st_cxn_empty: Option<StCxnEmpty>,
    /// _
    #[xml(child = "oac:endCxn")]
    pub end_cxn_empty: Option<EndCxnEmpty>,
    /// _
    #[xml(child = "oac:noGrp")]
    pub no_grp_empty: Option<NoGrpEmpty>,
    /// _
    #[xml(child = "oac:noSelect")]
    pub no_select_empty: Option<NoSelectEmpty>,
    /// _
    #[xml(child = "oac:noRot")]
    pub no_rot_empty: Option<NoRotEmpty>,
    /// _
    #[xml(child = "oac:noChangeAspect")]
    pub no_change_aspect_empty: Option<NoChangeAspectEmpty>,
    /// _
    #[xml(child = "oac:noMove")]
    pub no_move_empty: Option<NoMoveEmpty>,
    /// _
    #[xml(child = "oac:noResize")]
    pub no_resize_empty: Option<NoResizeEmpty>,
    /// _
    #[xml(child = "oac:noEditPoints")]
    pub no_edit_points_empty: Option<NoEditPointsEmpty>,
    /// _
    #[xml(child = "oac:noAdjustHandles")]
    pub no_adjust_handles_empty: Option<NoAdjustHandlesEmpty>,
    /// _
    #[xml(child = "oac:noChangeArrowheads")]
    pub no_change_arrowheads_empty: Option<NoChangeArrowheadsEmpty>,
    /// _
    #[xml(child = "oac:noChangeShapeType")]
    pub no_change_shape_type_empty: Option<NoChangeShapeTypeEmpty>,
}
/// Defines the CompressPictureProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:compressPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:compressPicPr")]
pub struct CompressPictureProps {
    /// removeCrop
    /// Represents the following attribute in the schema: :removeCrop
    #[xml(attr = "removeCrop")]
    pub remove_crop: Option<bool>,
    /// useLocalDpi
    /// Represents the following attribute in the schema: :useLocalDpi
    #[xml(attr = "useLocalDpi")]
    pub use_local_dpi: Option<bool>,
    /// cstate
    /// Represents the following attribute in the schema: :cstate
    #[xml(attr = "cstate")]
    pub cstate: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues,
    >,
}
/// Defines the ModifyNonVisualPictureProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvPicPr")]
pub struct ModifyNonVisualPictureProps {
    /// noGrp
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grp: Option<bool>,
    /// noSelect
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_select: Option<bool>,
    /// noRot
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rot: Option<bool>,
    /// noChangeAspect
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// noMove
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// noResize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// noEditPoints
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// noAdjustHandles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// noChangeArrowheads
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// noChangeShapeType
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// noCrop
    /// Represents the following attribute in the schema: :noCrop
    #[xml(attr = "noCrop")]
    pub no_crop: Option<bool>,
    /// preferRelativeResize
    /// Represents the following attribute in the schema: :preferRelativeResize
    #[xml(attr = "preferRelativeResize")]
    pub prefer_relative_resize: Option<bool>,
}
/// Defines the ResetNonVisualPictureProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvPicPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvPicPr")]
pub struct ResetNonVisualPictureProps {
    /// _
    #[xml(child = "oac:lfPr")]
    pub lf_pr_empty: Option<LfPrEmpty>,
}
/// Defines the BoundRect Class.
/// When the object is serialized out as xml, it's qualified name is oac:bounds.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:bounds")]
pub struct BoundRect {
    /// l
    /// Represents the following attribute in the schema: :l
    #[xml(attr = "l")]
    pub l: i64,
    /// t
    /// Represents the following attribute in the schema: :t
    #[xml(attr = "t")]
    pub t: i64,
    /// r
    /// Represents the following attribute in the schema: :r
    #[xml(attr = "r")]
    pub r: i64,
    /// b
    /// Represents the following attribute in the schema: :b
    #[xml(attr = "b")]
    pub b: i64,
}
/// Defines the SVGBlipMonikerList Class.
/// When the object is serialized out as xml, it's qualified name is oac:svgBlipMkLst.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:svgBlipMkLst")]
pub struct SvgBlipMonikerList {}
/// Defines the LinePropertiesType Class.
/// When the object is serialized out as xml, it's qualified name is oac:lineProps.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:lineProps")]
pub struct LinePropertiesType {
    /// line width
    /// Represents the following attribute in the schema: :w
    #[xml(attr = "w")]
    pub width: Option<i32>,
    /// line cap
    /// Represents the following attribute in the schema: :cap
    #[xml(attr = "cap")]
    pub cap_type: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineCapValues,
    >,
    /// compound line type
    /// Represents the following attribute in the schema: :cmpd
    #[xml(attr = "cmpd")]
    pub compound_line_type: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CompoundLineValues,
    >,
    /// pen alignment
    /// Represents the following attribute in the schema: :algn
    #[xml(attr = "algn")]
    pub alignment: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PenAlignmentValues,
    >,
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
    ANoFill(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NoFill),
    #[xml(tag = "a:solidFill")]
    ASolidFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SolidFill,
    ),
    #[xml(tag = "a:gradFill")]
    AGradFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::GradientFill,
    ),
    #[xml(tag = "a:pattFill")]
    APattFill(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PatternFill,
    ),
    #[xml(tag = "a:prstDash")]
    APrstDash(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetDash,
    ),
    #[xml(tag = "a:custDash")]
    ACustDash(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::CustomDash,
    ),
    #[xml(tag = "a:round")]
    ARound(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Round),
    #[xml(tag = "a:bevel")]
    ABevel(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LineJoinBevel,
    ),
    #[xml(tag = "a:miter")]
    AMiter(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Miter),
    #[xml(tag = "a:headEnd")]
    AHeadEnd(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HeadEnd),
    #[xml(tag = "a:tailEnd")]
    ATailEnd(crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TailEnd),
    #[xml(tag = "a:extLst")]
    AExtLst(
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LinePropertiesExtensionList,
    ),
}
/// Defines the ModifyNonVisualInkProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:cNvInkPr.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:cNvInkPr")]
pub struct ModifyNonVisualInkProps {
    /// noGrp
    /// Represents the following attribute in the schema: :noGrp
    #[xml(attr = "noGrp")]
    pub no_grp: Option<bool>,
    /// noSelect
    /// Represents the following attribute in the schema: :noSelect
    #[xml(attr = "noSelect")]
    pub no_select: Option<bool>,
    /// noRot
    /// Represents the following attribute in the schema: :noRot
    #[xml(attr = "noRot")]
    pub no_rot: Option<bool>,
    /// noChangeAspect
    /// Represents the following attribute in the schema: :noChangeAspect
    #[xml(attr = "noChangeAspect")]
    pub no_change_aspect: Option<bool>,
    /// noMove
    /// Represents the following attribute in the schema: :noMove
    #[xml(attr = "noMove")]
    pub no_move: Option<bool>,
    /// noResize
    /// Represents the following attribute in the schema: :noResize
    #[xml(attr = "noResize")]
    pub no_resize: Option<bool>,
    /// noEditPoints
    /// Represents the following attribute in the schema: :noEditPoints
    #[xml(attr = "noEditPoints")]
    pub no_edit_points: Option<bool>,
    /// noAdjustHandles
    /// Represents the following attribute in the schema: :noAdjustHandles
    #[xml(attr = "noAdjustHandles")]
    pub no_adjust_handles: Option<bool>,
    /// noChangeArrowheads
    /// Represents the following attribute in the schema: :noChangeArrowheads
    #[xml(attr = "noChangeArrowheads")]
    pub no_change_arrowheads: Option<bool>,
    /// noChangeShapeType
    /// Represents the following attribute in the schema: :noChangeShapeType
    #[xml(attr = "noChangeShapeType")]
    pub no_change_shape_type: Option<bool>,
    /// isComment
    /// Represents the following attribute in the schema: :isComment
    #[xml(attr = "isComment")]
    pub is_comment: Option<bool>,
}
/// Defines the HlinkClickHyperlinkProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlinkClick.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlinkClick")]
pub struct HlinkClickHyperlinkProps {
    /// source
    /// Represents the following attribute in the schema: :source
    #[xml(attr = "source")]
    pub source: Option<String>,
    /// action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// tgtFrame
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub tgt_frame: Option<String>,
    /// tooltip
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// highlightClick
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// endSnd
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_snd: Option<bool>,
    /// sndName
    /// Represents the following attribute in the schema: :sndName
    #[xml(attr = "sndName")]
    pub snd_name: Option<String>,
    /// _
    #[xml(child = "oac:sndData")]
    pub snd_data_img_data: Option<SndDataImgData>,
}
/// Defines the HlinkHoverHyperlinkProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlinkHover.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlinkHover")]
pub struct HlinkHoverHyperlinkProps {
    /// source
    /// Represents the following attribute in the schema: :source
    #[xml(attr = "source")]
    pub source: Option<String>,
    /// action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// tgtFrame
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub tgt_frame: Option<String>,
    /// tooltip
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// highlightClick
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// endSnd
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_snd: Option<bool>,
    /// sndName
    /// Represents the following attribute in the schema: :sndName
    #[xml(attr = "sndName")]
    pub snd_name: Option<String>,
    /// _
    #[xml(child = "oac:sndData")]
    pub snd_data_img_data: Option<SndDataImgData>,
}
/// Defines the OpenXmlHyperlinkPropsElement Class.
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = ".")]
pub struct OpenXmlHyperlinkPropsElement {
    /// source
    /// Represents the following attribute in the schema: :source
    #[xml(attr = "source")]
    pub source: Option<String>,
    /// action
    /// Represents the following attribute in the schema: :action
    #[xml(attr = "action")]
    pub action: Option<String>,
    /// tgtFrame
    /// Represents the following attribute in the schema: :tgtFrame
    #[xml(attr = "tgtFrame")]
    pub tgt_frame: Option<String>,
    /// tooltip
    /// Represents the following attribute in the schema: :tooltip
    #[xml(attr = "tooltip")]
    pub tooltip: Option<String>,
    /// highlightClick
    /// Represents the following attribute in the schema: :highlightClick
    #[xml(attr = "highlightClick")]
    pub highlight_click: Option<bool>,
    /// endSnd
    /// Represents the following attribute in the schema: :endSnd
    #[xml(attr = "endSnd")]
    pub end_snd: Option<bool>,
    /// sndName
    /// Represents the following attribute in the schema: :sndName
    #[xml(attr = "sndName")]
    pub snd_name: Option<String>,
}
/// Defines the ModifyHyperlinkProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlink")]
pub struct ModifyHyperlinkProps {
    /// _
    #[xml(child = "oac:hlinkClick")]
    pub hlink_click_hyperlink_props: Option<HlinkClickHyperlinkProps>,
    /// _
    #[xml(child = "oac:hlinkHover")]
    pub hlink_hover_hyperlink_props: Option<HlinkHoverHyperlinkProps>,
}
/// Defines the ResetHyperlinkProps Class.
/// When the object is serialized out as xml, it's qualified name is oac:hlink.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:hlink")]
pub struct ResetHyperlinkProps {
    /// _
    #[xml(child = "oac:hlinkClick")]
    pub hlink_click_empty: Option<HlinkClickEmpty>,
    /// _
    #[xml(child = "oac:hlinkHover")]
    pub hlink_hover_empty: Option<HlinkHoverEmpty>,
}
/// Defines the TextCharRangeContext Class.
/// When the object is serialized out as xml, it's qualified name is oac:context.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "oac:context")]
pub struct TextCharRangeContext {
    /// len
    /// Represents the following attribute in the schema: :len
    #[xml(attr = "len")]
    pub len: Option<i32>,
    /// hash
    /// Represents the following attribute in the schema: :hash
    #[xml(attr = "hash")]
    pub hash: i32,
}
