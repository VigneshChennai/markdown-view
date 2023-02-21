use std::path::Path;
use gtk::glib;
use gtk::subclass::prelude::*;

pub struct MarkdownView;


#[glib::object_subclass]
impl ObjectSubclass for MarkdownView {
    const NAME: &'static str = "VCMarkdownView";
    type Type = super::MarkdownView;
    type ParentType = gtk::TextView;
}

// Trait shared by all GObjects
impl ObjectImpl for MarkdownView {}

// Trait shared by all widgets
impl WidgetImpl for MarkdownView {}

// Trait shared by all TextView
impl TextViewImpl for MarkdownView {}

pub trait MarkdownViewImpl: TextViewImpl {


}

impl MarkdownView {
    pub fn new() -> Self {
        MarkdownViewBuilder::new().build()
    }

    pub fn from_file(path: impl AsRef<Path>) {
        let path_ref = path.as_ref();


    }
}

impl Default for MarkdownView {
    fn default() -> Self {
        Self::new()
    }
}

/// A [builder-pattern] type to construct [`MarkdownView`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct MarkdownViewBuilder {
    builder: glib::object::ObjectBuilder<'static, MarkdownView>,
}

impl MarkdownViewBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    /// Build the [`MarkdownView`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> MarkdownView {
        self.builder.build()
    }
}