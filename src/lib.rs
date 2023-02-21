use glib::Object;
use gtk::glib;
use gtk::glib::IsA;

mod imp;

glib::wrapper! {
    pub struct MarkdownView(ObjectSubclass<imp::MarkdownView>)
        @extends gtk::Widget, gtk::TextView,
        @implements gtk::Accessible, gtk::ConstraintTarget, gtk::Buildable, gtk::Scrollable;
}

impl Default for MarkdownView {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownView {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

pub trait MarkdownViewExt {

}

impl <O: IsA<MarkdownView>> MarkdownViewExt for O {

}


#[cfg(test)]
mod tests {
    use gtk::{Image, Justification, TextBuffer, TextTag};
    use gtk::gdk::Cursor;
    use gtk::prelude::*;

    use super::*;

    fn build_ui(application: &gtk::Application) {
        let window = gtk::ApplicationWindow::new(application);

        window.set_title(Some("SourceView5 + Rust"));
        window.set_default_size(500, 500);

        // let buffer = sourceview5::Buffer::new(None);
        let buffer = TextBuffer::default();
        let view = MarkdownView::new();
        view.set_buffer(Some(&buffer));

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        buffer.insert_at_cursor("Helloworld\n\n");
        let image = Image::from_file("/home/vignesh/Downloads/DSCN9286.JPG");
        image.set_width_request(500);
        image.set_height_request(500);

        // Inserting image
        let mut text_iter2 = buffer.iter_at_offset(buffer.cursor_position() - 1);
        let anchor = buffer.create_child_anchor(&mut text_iter2);
        view.add_child_at_anchor(&image, &anchor);

        // Justification: Centre
        // Centering the image in the view
        let text_iter = buffer.iter_at_offset(buffer.cursor_position() - 2);
        let text_iter_next = buffer.iter_at_offset(buffer.cursor_position() - 1);
        let tag = TextTag::builder()
            .justification(Justification::Center)
            .build();
        view.set_cursor(Some(&Cursor::builder()
            .hotspot_x(0)
            .hotspot_y(0)
            .build()));
        buffer.tag_table().add(&tag);
        buffer.apply_tag(&tag, &text_iter, &text_iter_next);

        //let view = sourceview5::View::with_buffer(&buffer);


        view.set_monospace(false);
        // view.set_background_pattern(sourceview5::BackgroundPatternType::Grid);
        // view.set_show_line_numbers(false);
        // view.set_highlight_current_line(false);
        // view.set_tab_width(4);
        view.set_hexpand(true);
        container.append(&view);
        // let map = sourceview5::Map::new();
        // map.set_view(&view);
        // container.append(&map);

        window.set_child(Some(&container));
        window.show();
    }

    #[test]
    fn preview() {
        let application = gtk::Application::new(
            Some("com.github.vigneshchennai.markdown-view-test"),
            Default::default(),
        );
        application.connect_activate(build_ui);

        application.run_with_args::<String>(&[]);
    }
}
