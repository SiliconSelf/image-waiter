//! The index page

use sailfish::TemplateOnce;

use crate::image_actor::Image;

/// A representation of the index page template
#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub(crate) struct IndexTemplate {
    /// Images to display on the page
    pub(crate) images: Vec<Image>,
}
