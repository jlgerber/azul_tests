extern crate azul;

use azul::prelude::*;
use std::time::Duration;

struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: WindowInfo<Self>) -> Dom<Self> {
        Dom::new(NodeType::Div)
        .with_id("mydom")
    }
}

fn main() {
     macro_rules! CSS_PATH { () => (concat!(env!("CARGO_MANIFEST_DIR"), "/src/my_css.css")) };
    let app = App::new(MyDataModel { }, AppConfig::default());
    let cssfile = css::from_str(include_str!(CSS_PATH!())).unwrap();
    let window = Window::new(WindowCreateOptions::default(), cssfile).unwrap();
    app.run(window).unwrap();
}