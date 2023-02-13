use gtk::glib;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct MarkdownView;


#[glib::object_subclass]
impl ObjectSubclass for MarkdownView {
    const NAME: &'static str = "CholaMarkdownView";
    type Type = super::MarkdownView;
    type ParentType = gtk::TextView;
}

// Trait shared by all GObjects
impl ObjectImpl for MarkdownView {}

// Trait shared by all widgets
impl WidgetImpl for MarkdownView {}

// Trait shared by all TextView
impl TextViewImpl for MarkdownView {}

