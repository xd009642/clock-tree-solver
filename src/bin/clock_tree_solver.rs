use clock_solver::types::*;

use relm::{connect, connect_stream, Relm, Update, Widget};
use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};

use relm_derive::Msg;

#[derive(Clone)]
struct App {
    model: ClockTree,
    window: Window,
}

#[derive(Msg)]
enum Message {
    Add,
    Connect,
    Set,
    Remove,
    Calculate,
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
            _ => {},
            Message::Quit => gtk::main_quit(),
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

        connect!(relm, window, connect_delete_event(_,_), return (Some(Message::Quit), Inhibit(false)));

        window.show_all();

        App {
            model: model,
            window: window,
        }
    }
}

fn main() {
    App::run(()).unwrap();
}
