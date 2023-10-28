//! [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#content-models)

pub mod embedded;
pub mod flow;
pub mod heading;
pub mod interactive;
pub mod metadata;
pub mod palpable;
pub mod phrasing;
pub mod script_supporting;
pub mod sectioning;

pub use embedded::*;
pub use flow::*;
pub use heading::*;
pub use interactive::*;
pub use metadata::*;
pub use palpable::*;
pub use phrasing::*;
pub use script_supporting::*;
pub use sectioning::*;

/// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#content-models)
#[derive(Clone, Debug, PartialEq, strum_macros::Display, strum_macros::EnumIs)]
#[strum(serialize_all = "lowercase")]
pub enum HTMLContent {
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#embedded-content)
    Embedded(EmbeddedContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#flow-content)
    Flow(FlowContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#heading-content)
    Heading(HeadingContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#interactive-content)
    Interactive(InteractiveContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#metadata-content)
    Metadata(MetadataContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#palpable-content)
    Palpable(PalpableContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#phrasing-content)
    Phrasing(PhrasingContent),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#script-supporting-elements)
    ScriptSupporting(ScriptSupportingElement),
    /// [WHATWG specification](https://html.spec.whatwg.org/multipage/dom.html#sectioning-content)
    Sectioning(SectioningContent),
}
