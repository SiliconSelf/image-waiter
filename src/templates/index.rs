//! The index page

use sailfish::TemplateOnce;

/// A representation of the index page template
#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub(crate) struct IndexTemplate;
