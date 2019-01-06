use azul::{prelude::*, widgets::{label::Label, button::Button}};

struct CounterApplication {
    counter: i64,
}

fn increment_counter(app_state: &mut AppState<CounterApplication>, _event: WindowEvent<CounterApplication>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter += 1);
    UpdateScreen::Redraw
}


fn decrement_counter(app_state: &mut AppState<CounterApplication>, _event: WindowEvent<CounterApplication>) -> UpdateScreen {
    app_state.data.modify(|state| state.counter -= 1);
    UpdateScreen::Redraw
}

impl Layout for CounterApplication {
    fn layout(&self, _info: WindowInfo<Self>) -> Dom<Self> {
        let label = Label::new(format!("{}", self.counter)).dom();
        let incbutton = Button::with_label("increment Counter").dom().with_class("mybutton")
        .with_callback(On::MouseUp, Callback(increment_counter));
        let decbutton = Button::with_label("increment Counter").dom().with_class("mybutton")
        .with_callback(On::MouseUp, Callback(decrement_counter));
        let div = Dom::new(NodeType::Div).with_id("buttons")
            .with_child(incbutton)
            .with_child(decbutton);
        Dom::new(NodeType::Div)
            .with_child(label)
            .with_child(div)
    }
}

fn main() {
    macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/mycss.css")) };
    let cssfile = css::override_native(include_str!(CSS_PATH!())).unwrap();

    let app = App::new(CounterApplication { counter: 0 }, AppConfig::default());
    app.run(Window::new(WindowCreateOptions::default(), cssfile).unwrap()).unwrap();}
