#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum EndPointRoleValues {
    #[default]
    Source,
    Sink,
    Replyto,
    Router,
}
crate::__string_enum! {
    EndPointRoleValues { Source = "source", Sink = "sink", Replyto = "replyTo", Router =
    "router", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MediumValues {
    #[default]
    Acoustic,
    Tactile,
    Visual,
}
crate::__string_enum! {
    MediumValues { Acoustic = "acoustic", Tactile = "tactile", Visual = "visual", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum AnchorPointValues {
    #[default]
    Start,
    End,
}
crate::__string_enum! {
    AnchorPointValues { Start = "start", End = "end", }
}
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DisjunctionTypeValues {
    #[default]
    Recognition,
    Understanding,
    Multidevice,
    Multiprocess,
}
crate::__string_enum! {
    DisjunctionTypeValues { Recognition = "recognition", Understanding = "understanding",
    Multidevice = "multiDevice", Multiprocess = "multiProcess", }
}
/// Defines the DerivedFrom Class.
/// When the object is serialized out as xml, it's qualified name is emma:derived-from.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:derived-from")]
pub struct DerivedFrom {
    /// resource
    /// Represents the following attribute in the schema: :resource
    #[xml(attr = "resource")]
    pub resource: String,
    /// composite
    /// Represents the following attribute in the schema: :composite
    #[xml(attr = "composite")]
    pub composite: Option<bool>,
}
/// Defines the Info Class.
/// When the object is serialized out as xml, it's qualified name is emma:info.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:info")]
pub struct Info {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: Option<String>,
}
/// Defines the Lattice Class.
/// When the object is serialized out as xml, it's qualified name is emma:lattice.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:lattice")]
pub struct Lattice {
    /// initial
    /// Represents the following attribute in the schema: :initial
    #[xml(attr = "initial")]
    pub initial: i32,
    /// final
    /// Represents the following attribute in the schema: :final
    #[xml(attr = "final")]
    pub r#final: String,
    /// time-ref-uri
    /// Represents the following attribute in the schema: emma:time-ref-uri
    #[xml(attr = "emma:time-ref-uri")]
    pub time_reference: Option<String>,
    /// time-ref-anchor-point
    /// Represents the following attribute in the schema: emma:time-ref-anchor-point
    #[xml(attr = "emma:time-ref-anchor-point")]
    pub time_reference_anchor_point: Option<AnchorPointValues>,
    #[xml(child = "emma:arc", child = "emma:node")]
    pub children: Vec<LatticeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum LatticeChildChoice {
    #[xml(tag = "emma:arc")]
    EmmaArc(Arc),
    #[xml(tag = "emma:node")]
    EmmaNode(Node),
}
/// Defines the Literal Class.
/// When the object is serialized out as xml, it's qualified name is emma:literal.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:literal")]
pub struct Literal {
    #[xml(text)]
    pub child: String,
}
/// Defines the Interpretation Class.
/// When the object is serialized out as xml, it's qualified name is emma:interpretation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:interpretation")]
pub struct Interpretation {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// tokens
    /// Represents the following attribute in the schema: emma:tokens
    #[xml(attr = "emma:tokens")]
    pub tokens: Option<String>,
    /// process
    /// Represents the following attribute in the schema: emma:process
    #[xml(attr = "emma:process")]
    pub process: Option<String>,
    /// lang
    /// Represents the following attribute in the schema: emma:lang
    #[xml(attr = "emma:lang")]
    pub language: Option<String>,
    /// signal
    /// Represents the following attribute in the schema: emma:signal
    #[xml(attr = "emma:signal")]
    pub signal: Option<String>,
    /// signal-size
    /// Represents the following attribute in the schema: emma:signal-size
    #[xml(attr = "emma:signal-size")]
    pub signal_size: Option<i32>,
    /// media-type
    /// Represents the following attribute in the schema: emma:media-type
    #[xml(attr = "emma:media-type")]
    pub media_type: Option<String>,
    /// confidence
    /// Represents the following attribute in the schema: emma:confidence
    #[xml(attr = "emma:confidence")]
    pub confidence: Option<String>,
    /// source
    /// Represents the following attribute in the schema: emma:source
    #[xml(attr = "emma:source")]
    pub source: Option<String>,
    /// start
    /// Represents the following attribute in the schema: emma:start
    #[xml(attr = "emma:start")]
    pub start: Option<i64>,
    /// end
    /// Represents the following attribute in the schema: emma:end
    #[xml(attr = "emma:end")]
    pub end: Option<i64>,
    /// time-ref-uri
    /// Represents the following attribute in the schema: emma:time-ref-uri
    #[xml(attr = "emma:time-ref-uri")]
    pub time_reference: Option<String>,
    /// time-ref-anchor-point
    /// Represents the following attribute in the schema: emma:time-ref-anchor-point
    #[xml(attr = "emma:time-ref-anchor-point")]
    pub time_reference_anchor_point: Option<AnchorPointValues>,
    /// offset-to-start
    /// Represents the following attribute in the schema: emma:offset-to-start
    #[xml(attr = "emma:offset-to-start")]
    pub offset_to_start: Option<i32>,
    /// duration
    /// Represents the following attribute in the schema: emma:duration
    #[xml(attr = "emma:duration")]
    pub duration: Option<i32>,
    /// medium
    /// Represents the following attribute in the schema: emma:medium
    #[xml(attr = "emma:medium")]
    pub medium: Option<String>,
    /// mode
    /// Represents the following attribute in the schema: emma:mode
    #[xml(attr = "emma:mode")]
    pub mode: Option<String>,
    /// function
    /// Represents the following attribute in the schema: emma:function
    #[xml(attr = "emma:function")]
    pub function: Option<String>,
    /// verbal
    /// Represents the following attribute in the schema: emma:verbal
    #[xml(attr = "emma:verbal")]
    pub verbal: Option<bool>,
    /// cost
    /// Represents the following attribute in the schema: emma:cost
    #[xml(attr = "emma:cost")]
    pub cost: Option<String>,
    /// grammar-ref
    /// Represents the following attribute in the schema: emma:grammar-ref
    #[xml(attr = "emma:grammar-ref")]
    pub grammar_ref: Option<String>,
    /// endpoint-info-ref
    /// Represents the following attribute in the schema: emma:endpoint-info-ref
    #[xml(attr = "emma:endpoint-info-ref")]
    pub endpoint_info_ref: Option<String>,
    /// model-ref
    /// Represents the following attribute in the schema: emma:model-ref
    #[xml(attr = "emma:model-ref")]
    pub model_ref: Option<String>,
    /// dialog-turn
    /// Represents the following attribute in the schema: emma:dialog-turn
    #[xml(attr = "emma:dialog-turn")]
    pub dialog_turn: Option<String>,
    /// no-input
    /// Represents the following attribute in the schema: emma:no-input
    #[xml(attr = "emma:no-input")]
    pub no_input: Option<bool>,
    /// uninterpreted
    /// Represents the following attribute in the schema: emma:uninterpreted
    #[xml(attr = "emma:uninterpreted")]
    pub uninterpreted: Option<bool>,
    #[xml(
        child = "emma:derived-from",
        child = "emma:info",
        child = "emma:lattice",
        child = "emma:literal",
        child = "msink:context",
    )]
    pub children: Vec<InterpretationChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum InterpretationChildChoice {
    #[xml(tag = "emma:derived-from")]
    EmmaDerivedFrom(DerivedFrom),
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
    #[xml(tag = "emma:lattice")]
    EmmaLattice(Lattice),
    #[xml(tag = "emma:literal")]
    EmmaLiteral(Literal),
    #[xml(tag = "msink:context")]
    MsinkContext(crate::schemas::schemas_microsoft_com_ink_2010_main::ContextNode),
}
/// Defines the OneOf Class.
/// When the object is serialized out as xml, it's qualified name is emma:one-of.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:one-of")]
pub struct OneOf {
    /// disjunction-type
    /// Represents the following attribute in the schema: :disjunction-type
    #[xml(attr = "disjunction-type")]
    pub disjunction_type: Option<DisjunctionTypeValues>,
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// tokens
    /// Represents the following attribute in the schema: emma:tokens
    #[xml(attr = "emma:tokens")]
    pub tokens: Option<String>,
    /// process
    /// Represents the following attribute in the schema: emma:process
    #[xml(attr = "emma:process")]
    pub process: Option<String>,
    /// lang
    /// Represents the following attribute in the schema: emma:lang
    #[xml(attr = "emma:lang")]
    pub language: Option<String>,
    /// signal
    /// Represents the following attribute in the schema: emma:signal
    #[xml(attr = "emma:signal")]
    pub signal: Option<String>,
    /// signal-size
    /// Represents the following attribute in the schema: emma:signal-size
    #[xml(attr = "emma:signal-size")]
    pub signal_size: Option<i32>,
    /// media-type
    /// Represents the following attribute in the schema: emma:media-type
    #[xml(attr = "emma:media-type")]
    pub media_type: Option<String>,
    /// confidence
    /// Represents the following attribute in the schema: emma:confidence
    #[xml(attr = "emma:confidence")]
    pub confidence: Option<String>,
    /// source
    /// Represents the following attribute in the schema: emma:source
    #[xml(attr = "emma:source")]
    pub source: Option<String>,
    /// start
    /// Represents the following attribute in the schema: emma:start
    #[xml(attr = "emma:start")]
    pub start: Option<i64>,
    /// end
    /// Represents the following attribute in the schema: emma:end
    #[xml(attr = "emma:end")]
    pub end: Option<i64>,
    /// time-ref-uri
    /// Represents the following attribute in the schema: emma:time-ref-uri
    #[xml(attr = "emma:time-ref-uri")]
    pub time_reference: Option<String>,
    /// time-ref-anchor-point
    /// Represents the following attribute in the schema: emma:time-ref-anchor-point
    #[xml(attr = "emma:time-ref-anchor-point")]
    pub time_reference_anchor_point: Option<AnchorPointValues>,
    /// offset-to-start
    /// Represents the following attribute in the schema: emma:offset-to-start
    #[xml(attr = "emma:offset-to-start")]
    pub offset_to_start: Option<i32>,
    /// duration
    /// Represents the following attribute in the schema: emma:duration
    #[xml(attr = "emma:duration")]
    pub duration: Option<i32>,
    /// medium
    /// Represents the following attribute in the schema: emma:medium
    #[xml(attr = "emma:medium")]
    pub medium: Option<String>,
    /// mode
    /// Represents the following attribute in the schema: emma:mode
    #[xml(attr = "emma:mode")]
    pub mode: Option<String>,
    /// function
    /// Represents the following attribute in the schema: emma:function
    #[xml(attr = "emma:function")]
    pub function: Option<String>,
    /// verbal
    /// Represents the following attribute in the schema: emma:verbal
    #[xml(attr = "emma:verbal")]
    pub verbal: Option<bool>,
    /// cost
    /// Represents the following attribute in the schema: emma:cost
    #[xml(attr = "emma:cost")]
    pub cost: Option<String>,
    /// grammar-ref
    /// Represents the following attribute in the schema: emma:grammar-ref
    #[xml(attr = "emma:grammar-ref")]
    pub grammar_ref: Option<String>,
    /// endpoint-info-ref
    /// Represents the following attribute in the schema: emma:endpoint-info-ref
    #[xml(attr = "emma:endpoint-info-ref")]
    pub endpoint_info_ref: Option<String>,
    /// model-ref
    /// Represents the following attribute in the schema: emma:model-ref
    #[xml(attr = "emma:model-ref")]
    pub model_ref: Option<String>,
    /// dialog-turn
    /// Represents the following attribute in the schema: emma:dialog-turn
    #[xml(attr = "emma:dialog-turn")]
    pub dialog_turn: Option<String>,
    #[xml(
        child = "emma:derived-from",
        child = "emma:info",
        child = "emma:interpretation",
        child = "emma:one-of",
        child = "emma:group",
        child = "emma:sequence",
    )]
    pub children: Vec<OneOfChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum OneOfChildChoice {
    #[xml(tag = "emma:derived-from")]
    EmmaDerivedFrom(DerivedFrom),
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
    #[xml(tag = "emma:interpretation")]
    EmmaInterpretation(Interpretation),
    #[xml(tag = "emma:one-of")]
    EmmaOneOf(OneOf),
    #[xml(tag = "emma:group")]
    EmmaGroup(Group),
    #[xml(tag = "emma:sequence")]
    EmmaSequence(Sequence),
}
/// Defines the Group Class.
/// When the object is serialized out as xml, it's qualified name is emma:group.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:group")]
pub struct Group {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// tokens
    /// Represents the following attribute in the schema: emma:tokens
    #[xml(attr = "emma:tokens")]
    pub tokens: Option<String>,
    /// process
    /// Represents the following attribute in the schema: emma:process
    #[xml(attr = "emma:process")]
    pub process: Option<String>,
    /// lang
    /// Represents the following attribute in the schema: emma:lang
    #[xml(attr = "emma:lang")]
    pub language: Option<String>,
    /// signal
    /// Represents the following attribute in the schema: emma:signal
    #[xml(attr = "emma:signal")]
    pub signal: Option<String>,
    /// signal-size
    /// Represents the following attribute in the schema: emma:signal-size
    #[xml(attr = "emma:signal-size")]
    pub signal_size: Option<i32>,
    /// media-type
    /// Represents the following attribute in the schema: emma:media-type
    #[xml(attr = "emma:media-type")]
    pub media_type: Option<String>,
    /// confidence
    /// Represents the following attribute in the schema: emma:confidence
    #[xml(attr = "emma:confidence")]
    pub confidence: Option<String>,
    /// source
    /// Represents the following attribute in the schema: emma:source
    #[xml(attr = "emma:source")]
    pub source: Option<String>,
    /// start
    /// Represents the following attribute in the schema: emma:start
    #[xml(attr = "emma:start")]
    pub start: Option<i64>,
    /// end
    /// Represents the following attribute in the schema: emma:end
    #[xml(attr = "emma:end")]
    pub end: Option<i64>,
    /// time-ref-uri
    /// Represents the following attribute in the schema: emma:time-ref-uri
    #[xml(attr = "emma:time-ref-uri")]
    pub time_reference: Option<String>,
    /// time-ref-anchor-point
    /// Represents the following attribute in the schema: emma:time-ref-anchor-point
    #[xml(attr = "emma:time-ref-anchor-point")]
    pub time_reference_anchor_point: Option<AnchorPointValues>,
    /// offset-to-start
    /// Represents the following attribute in the schema: emma:offset-to-start
    #[xml(attr = "emma:offset-to-start")]
    pub offset_to_start: Option<i32>,
    /// duration
    /// Represents the following attribute in the schema: emma:duration
    #[xml(attr = "emma:duration")]
    pub duration: Option<i32>,
    /// medium
    /// Represents the following attribute in the schema: emma:medium
    #[xml(attr = "emma:medium")]
    pub medium: Option<String>,
    /// mode
    /// Represents the following attribute in the schema: emma:mode
    #[xml(attr = "emma:mode")]
    pub mode: Option<String>,
    /// function
    /// Represents the following attribute in the schema: emma:function
    #[xml(attr = "emma:function")]
    pub function: Option<String>,
    /// verbal
    /// Represents the following attribute in the schema: emma:verbal
    #[xml(attr = "emma:verbal")]
    pub verbal: Option<bool>,
    /// cost
    /// Represents the following attribute in the schema: emma:cost
    #[xml(attr = "emma:cost")]
    pub cost: Option<String>,
    /// grammar-ref
    /// Represents the following attribute in the schema: emma:grammar-ref
    #[xml(attr = "emma:grammar-ref")]
    pub grammar_ref: Option<String>,
    /// endpoint-info-ref
    /// Represents the following attribute in the schema: emma:endpoint-info-ref
    #[xml(attr = "emma:endpoint-info-ref")]
    pub endpoint_info_ref: Option<String>,
    /// model-ref
    /// Represents the following attribute in the schema: emma:model-ref
    #[xml(attr = "emma:model-ref")]
    pub model_ref: Option<String>,
    /// dialog-turn
    /// Represents the following attribute in the schema: emma:dialog-turn
    #[xml(attr = "emma:dialog-turn")]
    pub dialog_turn: Option<String>,
    #[xml(
        child = "emma:derived-from",
        child = "emma:group-info",
        child = "emma:info",
        child = "emma:interpretation",
        child = "emma:one-of",
        child = "emma:group",
        child = "emma:sequence",
    )]
    pub children: Vec<GroupChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum GroupChildChoice {
    #[xml(tag = "emma:derived-from")]
    EmmaDerivedFrom(DerivedFrom),
    #[xml(tag = "emma:group-info")]
    EmmaGroupInfo(GroupInfo),
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
    #[xml(tag = "emma:interpretation")]
    EmmaInterpretation(Interpretation),
    #[xml(tag = "emma:one-of")]
    EmmaOneOf(OneOf),
    #[xml(tag = "emma:group")]
    EmmaGroup(Group),
    #[xml(tag = "emma:sequence")]
    EmmaSequence(Sequence),
}
/// Defines the Sequence Class.
/// When the object is serialized out as xml, it's qualified name is emma:sequence.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:sequence")]
pub struct Sequence {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// tokens
    /// Represents the following attribute in the schema: emma:tokens
    #[xml(attr = "emma:tokens")]
    pub tokens: Option<String>,
    /// process
    /// Represents the following attribute in the schema: emma:process
    #[xml(attr = "emma:process")]
    pub process: Option<String>,
    /// lang
    /// Represents the following attribute in the schema: emma:lang
    #[xml(attr = "emma:lang")]
    pub language: Option<String>,
    /// signal
    /// Represents the following attribute in the schema: emma:signal
    #[xml(attr = "emma:signal")]
    pub signal: Option<String>,
    /// signal-size
    /// Represents the following attribute in the schema: emma:signal-size
    #[xml(attr = "emma:signal-size")]
    pub signal_size: Option<i32>,
    /// media-type
    /// Represents the following attribute in the schema: emma:media-type
    #[xml(attr = "emma:media-type")]
    pub media_type: Option<String>,
    /// confidence
    /// Represents the following attribute in the schema: emma:confidence
    #[xml(attr = "emma:confidence")]
    pub confidence: Option<String>,
    /// source
    /// Represents the following attribute in the schema: emma:source
    #[xml(attr = "emma:source")]
    pub source: Option<String>,
    /// start
    /// Represents the following attribute in the schema: emma:start
    #[xml(attr = "emma:start")]
    pub start: Option<i64>,
    /// end
    /// Represents the following attribute in the schema: emma:end
    #[xml(attr = "emma:end")]
    pub end: Option<i64>,
    /// time-ref-uri
    /// Represents the following attribute in the schema: emma:time-ref-uri
    #[xml(attr = "emma:time-ref-uri")]
    pub time_reference: Option<String>,
    /// time-ref-anchor-point
    /// Represents the following attribute in the schema: emma:time-ref-anchor-point
    #[xml(attr = "emma:time-ref-anchor-point")]
    pub time_reference_anchor_point: Option<AnchorPointValues>,
    /// offset-to-start
    /// Represents the following attribute in the schema: emma:offset-to-start
    #[xml(attr = "emma:offset-to-start")]
    pub offset_to_start: Option<i32>,
    /// duration
    /// Represents the following attribute in the schema: emma:duration
    #[xml(attr = "emma:duration")]
    pub duration: Option<i32>,
    /// medium
    /// Represents the following attribute in the schema: emma:medium
    #[xml(attr = "emma:medium")]
    pub medium: Option<String>,
    /// mode
    /// Represents the following attribute in the schema: emma:mode
    #[xml(attr = "emma:mode")]
    pub mode: Option<String>,
    /// function
    /// Represents the following attribute in the schema: emma:function
    #[xml(attr = "emma:function")]
    pub function: Option<String>,
    /// verbal
    /// Represents the following attribute in the schema: emma:verbal
    #[xml(attr = "emma:verbal")]
    pub verbal: Option<bool>,
    /// cost
    /// Represents the following attribute in the schema: emma:cost
    #[xml(attr = "emma:cost")]
    pub cost: Option<String>,
    /// grammar-ref
    /// Represents the following attribute in the schema: emma:grammar-ref
    #[xml(attr = "emma:grammar-ref")]
    pub grammar_ref: Option<String>,
    /// endpoint-info-ref
    /// Represents the following attribute in the schema: emma:endpoint-info-ref
    #[xml(attr = "emma:endpoint-info-ref")]
    pub endpoint_info_ref: Option<String>,
    /// model-ref
    /// Represents the following attribute in the schema: emma:model-ref
    #[xml(attr = "emma:model-ref")]
    pub model_ref: Option<String>,
    /// dialog-turn
    /// Represents the following attribute in the schema: emma:dialog-turn
    #[xml(attr = "emma:dialog-turn")]
    pub dialog_turn: Option<String>,
    #[xml(
        child = "emma:derived-from",
        child = "emma:info",
        child = "emma:interpretation",
        child = "emma:one-of",
        child = "emma:group",
        child = "emma:sequence",
    )]
    pub children: Vec<SequenceChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum SequenceChildChoice {
    #[xml(tag = "emma:derived-from")]
    EmmaDerivedFrom(DerivedFrom),
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
    #[xml(tag = "emma:interpretation")]
    EmmaInterpretation(Interpretation),
    #[xml(tag = "emma:one-of")]
    EmmaOneOf(OneOf),
    #[xml(tag = "emma:group")]
    EmmaGroup(Group),
    #[xml(tag = "emma:sequence")]
    EmmaSequence(Sequence),
}
/// Defines the GroupInfo Class.
/// When the object is serialized out as xml, it's qualified name is emma:group-info.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:group-info")]
pub struct GroupInfo {
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
/// Defines the Derivation Class.
/// When the object is serialized out as xml, it's qualified name is emma:derivation.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:derivation")]
pub struct Derivation {
    #[xml(
        child = "emma:interpretation",
        child = "emma:one-of",
        child = "emma:sequence",
        child = "emma:group",
    )]
    pub children: Vec<DerivationChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum DerivationChildChoice {
    #[xml(tag = "emma:interpretation")]
    EmmaInterpretation(Interpretation),
    #[xml(tag = "emma:one-of")]
    EmmaOneOf(OneOf),
    #[xml(tag = "emma:sequence")]
    EmmaSequence(Sequence),
    #[xml(tag = "emma:group")]
    EmmaGroup(Group),
}
/// Defines the Grammar Class.
/// When the object is serialized out as xml, it's qualified name is emma:grammar.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:grammar")]
pub struct Grammar {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: String,
}
/// Defines the Model Class.
/// When the object is serialized out as xml, it's qualified name is emma:model.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:model")]
pub struct Model {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// ref
    /// Represents the following attribute in the schema: :ref
    #[xml(attr = "ref")]
    pub reference: Option<String>,
}
/// Defines the EndPointInfo Class.
/// When the object is serialized out as xml, it's qualified name is emma:endpoint-info.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:endpoint-info")]
pub struct EndPointInfo {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    #[xml(child = "emma:endpoint")]
    pub children: Vec<EndPointInfoChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EndPointInfoChildChoice {
    #[xml(tag = "emma:endpoint")]
    EmmaEndpoint(EndPoint),
}
/// Defines the EndPoint Class.
/// When the object is serialized out as xml, it's qualified name is emma:endpoint.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:endpoint")]
pub struct EndPoint {
    /// id
    /// Represents the following attribute in the schema: :id
    #[xml(attr = "id")]
    pub id: String,
    /// endpoint-role
    /// Represents the following attribute in the schema: emma:endpoint-role
    #[xml(attr = "emma:endpoint-role")]
    pub endpoint_role: Option<EndPointRoleValues>,
    /// endpoint-address
    /// Represents the following attribute in the schema: emma:endpoint-address
    #[xml(attr = "emma:endpoint-address")]
    pub end_point_address: Option<String>,
    /// message-id
    /// Represents the following attribute in the schema: emma:message-id
    #[xml(attr = "emma:message-id")]
    pub message_id: Option<String>,
    /// port-num
    /// Represents the following attribute in the schema: emma:port-num
    #[xml(attr = "emma:port-num")]
    pub port_number: Option<i32>,
    /// port-type
    /// Represents the following attribute in the schema: emma:port-type
    #[xml(attr = "emma:port-type")]
    pub port_type: Option<String>,
    /// endpoint-pair-ref
    /// Represents the following attribute in the schema: emma:endpoint-pair-ref
    #[xml(attr = "emma:endpoint-pair-ref")]
    pub endpoint_pair_ref: Option<String>,
    /// service-name
    /// Represents the following attribute in the schema: emma:service-name
    #[xml(attr = "emma:service-name")]
    pub service_name: Option<String>,
    /// media-type
    /// Represents the following attribute in the schema: emma:media-type
    #[xml(attr = "emma:media-type")]
    pub media_type: Option<String>,
    /// medium
    /// Represents the following attribute in the schema: emma:medium
    #[xml(attr = "emma:medium")]
    pub medium: Option<String>,
    /// mode
    /// Represents the following attribute in the schema: emma:mode
    #[xml(attr = "emma:mode")]
    pub mode: Option<String>,
}
/// Defines the Node Class.
/// When the object is serialized out as xml, it's qualified name is emma:node.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:node")]
pub struct Node {
    /// node-number
    /// Represents the following attribute in the schema: :node-number
    #[xml(attr = "node-number")]
    pub node_number: i32,
    /// confidence
    /// Represents the following attribute in the schema: emma:confidence
    #[xml(attr = "emma:confidence")]
    pub confidence: Option<String>,
    /// cost
    /// Represents the following attribute in the schema: emma:cost
    #[xml(attr = "emma:cost")]
    pub cost: Option<String>,
    #[xml(child = "emma:info")]
    pub children: Vec<NodeChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum NodeChildChoice {
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
}
/// Defines the Arc Class.
/// When the object is serialized out as xml, it's qualified name is emma:arc.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:arc")]
pub struct Arc {
    /// from
    /// Represents the following attribute in the schema: :from
    #[xml(attr = "from")]
    pub from: i32,
    /// to
    /// Represents the following attribute in the schema: :to
    #[xml(attr = "to")]
    pub to: i32,
    /// start
    /// Represents the following attribute in the schema: emma:start
    #[xml(attr = "emma:start")]
    pub start: Option<i64>,
    /// end
    /// Represents the following attribute in the schema: emma:end
    #[xml(attr = "emma:end")]
    pub end: Option<i64>,
    /// offset-to-start
    /// Represents the following attribute in the schema: emma:offset-to-start
    #[xml(attr = "emma:offset-to-start")]
    pub offset_to_start: Option<i32>,
    /// duration
    /// Represents the following attribute in the schema: emma:duration
    #[xml(attr = "emma:duration")]
    pub duration: Option<i32>,
    /// confidence
    /// Represents the following attribute in the schema: emma:confidence
    #[xml(attr = "emma:confidence")]
    pub confidence: Option<String>,
    /// cost
    /// Represents the following attribute in the schema: emma:cost
    #[xml(attr = "emma:cost")]
    pub cost: Option<String>,
    /// lang
    /// Represents the following attribute in the schema: emma:lang
    #[xml(attr = "emma:lang")]
    pub language: Option<String>,
    /// medium
    /// Represents the following attribute in the schema: emma:medium
    #[xml(attr = "emma:medium")]
    pub medium: Option<String>,
    /// mode
    /// Represents the following attribute in the schema: emma:mode
    #[xml(attr = "emma:mode")]
    pub mode: Option<String>,
    /// source
    /// Represents the following attribute in the schema: emma:source
    #[xml(attr = "emma:source")]
    pub source: Option<String>,
    #[xml(child = "emma:info")]
    pub children: Vec<ArcChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum ArcChildChoice {
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
}
/// Defines the Emma Class.
/// When the object is serialized out as xml, it's qualified name is emma:emma.
#[derive(Clone, Debug, Default, hard_xml::XmlWrite, hard_xml::XmlRead)]
#[xml(tag = "emma:emma")]
pub struct Emma {
    /// version
    /// Represents the following attribute in the schema: :version
    #[xml(attr = "version")]
    pub version: String,
    #[xml(
        child = "emma:derivation",
        child = "emma:grammar",
        child = "emma:model",
        child = "emma:endpoint-info",
        child = "emma:info",
        child = "emma:interpretation",
        child = "emma:one-of",
        child = "emma:group",
        child = "emma:sequence",
    )]
    pub children: Vec<EmmaChildChoice>,
}
#[derive(Clone, Debug, hard_xml::XmlWrite, hard_xml::XmlRead)]
pub enum EmmaChildChoice {
    #[xml(tag = "emma:derivation")]
    EmmaDerivation(Derivation),
    #[xml(tag = "emma:grammar")]
    EmmaGrammar(Grammar),
    #[xml(tag = "emma:model")]
    EmmaModel(Model),
    #[xml(tag = "emma:endpoint-info")]
    EmmaEndpointInfo(EndPointInfo),
    #[xml(tag = "emma:info")]
    EmmaInfo(Info),
    #[xml(tag = "emma:interpretation")]
    EmmaInterpretation(Interpretation),
    #[xml(tag = "emma:one-of")]
    EmmaOneOf(OneOf),
    #[xml(tag = "emma:group")]
    EmmaGroup(Group),
    #[xml(tag = "emma:sequence")]
    EmmaSequence(Sequence),
}
