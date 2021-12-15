
use gtk::{
    Inhibit,
    prelude::GtkMenuItemExt,
    prelude::MenuShellExt,
    prelude::OrientableExt,
    prelude::WidgetExt,
};
use gtk::prelude::*;
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
        let application_menu = gtk::Menu::new();
        let project_menu = gtk::Menu::new();
        let task_menu = gtk::Menu::new();

        let application_item = gtk::MenuItem::with_label("Application");
        let project_item = gtk::MenuItem::with_label("Project");
        let task_item = gtk::MenuItem::with_label("Task");
        application_item.set_submenu(Some(&application_menu));
        project_item.set_submenu(Some(&project_menu));
        task_item.set_submenu(Some(&task_menu));
        
        let application_quit_item = gtk::MenuItem::with_label("Quit");
        application_menu.append(&application_quit_item);
        
        self.widgets.menubar.append(&application_item);
        self.widgets.menubar.append(&project_item);
        self.widgets.menubar.append(&task_item);
        self.widgets.menubar.show_all();

        connect!(application_quit_item, connect_activate(_), self.model.relm, Msg::Quit);
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
            title: crate::gui::TITLEBAR,
            gtk::Box {
                orientation: Vertical,
                #[name="menubar"]
                gtk::MenuBar {
                },
                #[name="panes"]
                gtk::Paned {
                    child: {
                        expand: true,
                        fill: true,
                    },
                    orientation: gtk::Orientation::Horizontal,
                    wide_handle: true,
                    #[name="nb"]
                    gtk::Notebook {
                        tab_pos: gtk::PositionType::Left,
                    },
                },
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        }
    }
}

pub fn launch() {
    MainWindow::run(()).unwrap();
}
