use clock_solver::types::*;
use clock_solver::graphics::*;

use relm::{connect, connect_stream, DrawHandler, Relm, Update, Widget};
use gtk::prelude::*;
use gtk::{Button, DrawingArea, Inhibit, Orientation, Window, WindowType};

use relm_derive::Msg;

struct Widgets {
    add: Button, 
    remove: Button,
    calculate: Button,
    connect: Button,
    set_params: Button,
    area: DrawingArea,
    handler: DrawHandler<DrawingArea>,
}

struct App {
    /// The clock tree the user's working with
    model: ClockTree,
    /// The main window
    window: Window,
    /// Widgets contained in the window
    widgets: Widgets,
    /// has changed 
    has_changed: bool,
}

#[derive(Msg)]
enum Message {
    Add,
    Connect,
    Set,
    Remove,
    Calculate,
    Render,
    Quit,
}

impl Update for App {
    type Model = ClockTree;
    type ModelParam = ();
    type Msg = Message;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        ClockTree::new()
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Message::Add => {
                let _ = self.model.add_node(Node::Input(Endpoint{
                    name: "Input".to_string(),
                    value: Value::DontCare,
                    is_internal: false,
                }));
                self.has_changed = true;
            },
            Message::Render => {
                if self.has_changed {
                    let alloc = self.widgets.area.get_allocation();
                    let context = self.widgets.handler.get_context();
                    context.set_source_rgb(1.0, 1.0, 1.0);
                    context.paint();
                    self.model.render(&context, 0.0, 0.0, alloc.width as f64, alloc.height as f64);
                    self.has_changed = false;
                }
            },
            Message::Quit => gtk::main_quit(),
            _ => {},
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: ClockTree) -> Self {
        let window = Window::new(WindowType::Toplevel);
        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        let hbox = gtk::Box::new(Orientation::Horizontal, 0);
        let add = Button::new_with_label("+");
        let remove = Button::new_with_label("-");
        let set = Button::new_with_label("Parameters");
        let calc = Button::new_with_label("Calculate");
        let conn = Button::new_with_label("Connect");
        let da = DrawingArea::new();
        da.set_size_request(400, 400);
        da.set_hexpand(true);
        da.set_vexpand(true);
        let mut handler = DrawHandler::<DrawingArea>::new().unwrap();
        handler.init(&da);

        vbox.add(&add);
        vbox.add(&remove);
        vbox.add(&set);
        vbox.add(&calc);
        vbox.add(&conn);
      
        hbox.add(&da);
        hbox.add(&vbox);

        window.add(&hbox);

        connect!(relm, window, 
                 connect_delete_event(_,_), 
                 return (Some(Message::Quit), Inhibit(false)));

        connect!(relm, add, connect_clicked(_), Message::Add);
        connect!(relm, add, connect_clicked(_), Message::Render);

        connect!(relm, 
                 da, 
                 connect_draw(_,_), 
                 return (Some(Message::Render), Inhibit(false)));

        window.show_all();

        App {
            model: model,
            window: window,
            widgets: Widgets {
                add: add,
                remove:remove,
                set_params: set,
                connect: conn,
                calculate: calc,
                area: da,
                handler: handler,
            },
            has_changed: true,
        }
    }
}

fn main() {
    App::run(()).unwrap();
}
