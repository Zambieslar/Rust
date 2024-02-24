use std::{self, fmt::Write, io::Read, thread};
use gtk4 as gtk;
use gtk::{prelude::*, TextBuffer};
use gtk:: Button;
mod libhttp;

fn but_create(label: &str) -> Button {
    let button = Button::builder()
        .label(label)
        .build();
    return button;
}

fn main() {
    let app = gtk4::Application::builder()
        .application_id("org.zamhttp.poopmypants")
        .build();
    app.connect_activate(|app| {
        let buffer = TextBuffer::new(None);
        let layout = gtk::Grid::builder()
            .column_homogeneous(true)
            .column_spacing(4)
            .row_spacing(4)
            .build();
        let button = but_create("Send");
        let test_output = gtk::TextView::builder()
            .vscroll_policy(gtk::ScrollablePolicy::Minimum)
            .overflow(gtk::Overflow::Hidden)
            .vexpand(false)
            .buffer(&buffer)
            .build();
        layout.attach(&button, 0, 0, 50, 10);
        layout.attach(&test_output, 0, 50, 50, 78);
        button.connect_clicked(move |_|{
            let result = thread::spawn(||{
                let mut buf = String::new();
                let mut stream = libhttp::ssl_connect();
                stream.ssl_write(b"GET / HTTP/1.0\r\n\r\n").unwrap();
                stream.read_to_string(&mut buf).unwrap();
                buf
            }).join();
            buffer.clone().write_str(result.unwrap().as_str()).unwrap();
        });
        let window = gtk4::ApplicationWindow::builder()
            .application(app)
            .default_width(500)
            .default_height(350)
            .child(&layout)
            .resizable(false)
            .build();
        window.present();
    });
    app.run();
}
