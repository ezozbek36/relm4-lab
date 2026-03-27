use relm4::gtk::glib::clone;
use relm4::gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt};
use relm4::{ComponentParts, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppActions {
    Increment,
    Decrement,
}

struct AppWidgets {
    label: relm4::gtk::Label,
}

impl SimpleComponent for AppModel {
    type Input = AppActions;
    type Output = ();
    type Init = u8;
    type Root = relm4::gtk::Window;
    type Widgets = AppWidgets;

    fn init_root() -> Self::Root {
        relm4::gtk::Window::builder().title("Simple App").build()
    }

    fn init(
        counter: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { counter };

        let inc_button = {
            let it = relm4::gtk::Button::with_label("Increment");

            it.connect_clicked(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(AppActions::Increment);
                }
            ));

            it
        };

        let dec_button = {
            let it = relm4::gtk::Button::with_label("Decrement");

            it.connect_clicked(clone!(
                #[strong]
                sender,
                move |_| {
                    sender.input(AppActions::Decrement);
                }
            ));

            it
        };

        let label = {
            let it = relm4::gtk::Label::new(Some(&format!("Count: {}", model.counter)));
            it.set_margin_all(5);
            it
        };

        let vbox = {
            let it = relm4::gtk::Box::builder()
                .orientation(relm4::gtk::Orientation::Vertical)
                .spacing(5)
                .build();

            it.set_margin_all(5);
            it.append(&inc_button);
            it.append(&dec_button);
            it.append(&label);

            it
        };

        window.set_child(Some(&vbox));

        let widgets = AppWidgets { label };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppActions::Increment => self.counter = self.counter.wrapping_add(1),
            AppActions::Decrement => self.counter = self.counter.wrapping_sub(1),
        }
    }

    fn update_view(&self, widgets: &mut Self::Widgets, _sender: relm4::ComponentSender<Self>) {
        widgets
            .label
            .set_label(&format!("Counter: {}", self.counter));
    }
}

fn main() {
    let app = RelmApp::new("relm4.simple_manual");

    app.run::<AppModel>(0);
}
