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
    diagram: DrawHandler<DrawingArea>
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
            Message::Render => {
                if self.has_changed {
                    let context = self.widgets.diagram.get_context();
                    context.set_source_rgb(1.0, 1.0, 1.0);
                    context.paint();
                    self.model.render(&context, 0.0, 0.0, 400.0, 400.0);
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
        let mut da = DrawingArea::new();
        da.set_size_request(400, 400);
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

        connect!(relm, 
                 window, 
                 connect_delete_event(_,_), 
                 return (Some(Message::Quit), Inhibit(false)));

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
                diagram: handler,
            },
            has_changed: true,
        }
    }
}

fn main() {
    App::run(()).unwrap();
}
