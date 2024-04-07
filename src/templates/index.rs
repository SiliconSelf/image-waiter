//! The index page

use sailfish::TemplateOnce;

use crate::image_actor::Image;

/// A representation of the index page template
#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub(crate) struct IndexTemplate {
    pub(crate) images: Vec<Image>,
}
