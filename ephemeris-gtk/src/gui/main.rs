
use gtk::{
    Inhibit,
    prelude::GtkMenuItemExt,
    prelude::MenuShellExt,
    prelude::OrientableExt,
    prelude::WidgetExt,
};
use gtk::Orientation::Vertical;
use relm::{Relm, Widget, connect};
use relm_derive::{Msg, widget};

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {
    relm: Relm<MainWindow>,
}

#[widget]
impl Widget for MainWindow {
    fn init_view(&mut self) {
        let file_menu = gtk::Menu::new();
        let file_item = gtk::MenuItem::with_label("File");
        file_item.set_submenu(Some(&file_menu));
        let quit_item = gtk::MenuItem::with_label("Quit");
        self.widgets.menubar.append(&file_item);
        file_menu.append(&quit_item);
        self.widgets.menubar.show_all();

        connect!(quit_item, connect_activate(_), self.model.relm, Msg::Quit);
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                #[name="menubar"]
                gtk::MenuBar {
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

pub fn launch() {
    MainWindow::run(()).unwrap();
}
