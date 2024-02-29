
pub mod ui{
    use std::io::Read;
    use std::ops::Add;
    use std::sync::{self, Arc, Mutex, MutexGuard};
    use std::{self, fmt::Write, thread};
    use gtk::gdk::Display;
    use gtk::glib::ControlFlow;
    use gtk4 as gtk;
    use gtk::{prelude::*, CssProvider, EntryBuffer, ScrolledWindow, Text, TextBuffer, TextView};
    use gtk:: Button;
    use crate::libhttp;

    fn but_create(label: &str) -> Button {
        let button = Button::builder()
            .label(label)
            .build();
        button
    }

    fn textview_create() -> (TextView, TextBuffer) {
        let buffer = TextBuffer::new(None);
        let text_view = gtk::TextView::builder()
            .vscroll_policy(gtk::ScrollablePolicy::Minimum)
            .overflow(gtk::Overflow::Hidden)
            .buffer(&buffer)
            .editable(false)
            .build();
        (text_view, buffer)
    }

    fn text_create(text: &str) -> (Text, EntryBuffer) {
        let buffer = EntryBuffer::new(None::<String>);
        let text = Text::builder()
            .buffer(&buffer)
            .placeholder_text(text)
            .build();
        (text, buffer)
    }
    pub fn load_css() {
        let style = gtk::gio::File::for_path("src/style.css");
        let provider = CssProvider::new();
        provider.load_from_file(&style);
        gtk::style_context_add_provider_for_display(&Display::default().expect("Could not connect to display"), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
    // Initialize the UI
    pub fn init_ui() -> gtk::Application{
        let app = gtk4::Application::builder()
            .application_id("org.zamhttp.poopmypants")
            .build();
        app.connect_activate(|app| {
            let layout = gtk::Grid::builder()
                .column_homogeneous(true)
                .column_spacing(4)
                .row_spacing(4)
                .build();
            let (text_view, mut textview_buffer) = textview_create();
            let (text_server, textbox_server_buffer) = text_create("Server");
            let (text_badge_data, textbox_badge_buffer) = text_create("Badge");
            let button = but_create("Send");
            let scrollable_window = ScrolledWindow::new();

            text_server.set_css_classes(&["text_server"]);
            text_badge_data.set_css_classes(&["text_badge"]);
            scrollable_window.set_child(Some(&text_view));
            layout.attach(&button, 0, 0, 15, 10);
            layout.attach(&text_server, 15, 2, 15, 7);
            layout.attach(&text_badge_data, 30, 2, 15, 7);
            layout.attach(&scrollable_window, 0, 50, 50, 78);
            button.connect_clicked(move |_|{
                let queue = Arc::new(Mutex::new(Vec::new()));
                let amutex = queue.clone();
                let mut data = queue.try_lock().unwrap();
                let (a, b) = (textbox_badge_buffer.text().clone().to_string(), textbox_server_buffer.text().clone().to_string());
                *&mut data.push(a);
                *&mut data.push(b);
                Mutex::unlock(data);
                    // Spawn thread to handle the HTTP request
                gtk::glib::idle_add(move ||{
                    let mut data = amutex.try_lock().unwrap();
                    let mut buf = String::new();
                    let req = format!("POST /authenticate/badge?t=2CBUBMnsrcRVnpbllN HTTP/1.0\r\nContent-Type: application/json\r\n\r\n{{\"data\": [{{\"type\": \"rawBadgeData\", \"attributes\": {{\"value\"\"{}\"}}}}]}}", *&data[0]);
                    let mut stream = libhttp::ssl_connect(*&data[1].as_str());
                    data.clear();
                    match &mut stream {
                        Ok(stream) => {
                            stream.ssl_write(req.as_bytes()).unwrap();
                            stream.read_to_string(&mut buf).unwrap();
                            data.push(buf);
                            Mutex::unlock(data);
                            ControlFlow::Break
                        }
                        Err(error) => {
                            data.push((*error).to_string());
                            ControlFlow::Break
                        }
                    }
                });
                while let Err(result) = queue.try_lock() {
                   continue;
                }
                let result = Arc::clone(&queue);
                let result = result.try_lock().unwrap();
                textview_buffer.clone().write_str(error).unwrap();
                Mutex::unlock(result);
             });
            // Create the main window
            let window = gtk4::ApplicationWindow::builder()
                .application(app)
                .default_width(500)
                .default_height(350)
                .child(&layout)
                .resizable(false)
                .build();
            window.present();
        });
        // Return the Gtk Application object to main program
        app
    }
}
