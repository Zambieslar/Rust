use gtk4 as gtk;
use gtk::prelude::*;
mod ui;
mod libhttp;

fn main() {
    let app = ui::ui::init_ui();
    app.connect_startup(|_|{ui::ui::load_css()});
    app.run();
}
