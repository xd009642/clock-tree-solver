use clock_solver::types::*;

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
                println!("Rendering");
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
        }
    }
}

fn main() {
    App::run(()).unwrap();
}
