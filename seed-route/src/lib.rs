use seed::{button, div, p};
use seed::prelude::*;

#[derive(Clone)]
enum Page {
    One,
    Two,
}

struct Model {
    page: Page,
}

impl Model {
    fn new() -> Self {
        Self {
            page: Page::One,
        }
    }
}

#[derive(Clone)]
enum Msg {
    GoToPageTwo,
    Route(String),
    Change(Page),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::GoToPageTwo => {
            orders.skip().send_msg(Msg::Route("two".into()));
        },
        Msg::Route(path) => {
            seed::push_route(vec![path.clone()]);
            match path.as_str() {
                "one" => orders.skip().send_msg(Msg::Change(Page::One)),
                "two" => orders.skip().send_msg(Msg::Change(Page::Two)),
                _ => orders.skip().send_msg(Msg::Change(Page::One)),
            };
        },
        Msg::Change(page) => {
            model.page = page;
        },
    }
}

fn view(model: &Model) -> impl View<Msg> {
    match model.page {
        Page::One => {
            div![
                p!["one"],
                button![simple_ev(Ev::Click, Msg::GoToPageTwo), "go to page two"],
            ]
        },
        Page::Two => {
            p!["two"]
        },
    }
}

fn routes(url: seed::Url) -> Option<Msg> {
    url.path.get(0)
        .map(|path| {
            match path.as_str() {
                "one" => Msg::Change(Page::One),
                "two" => Msg::Change(Page::Two),
                _ => Msg::Change(Page::One),
            }
        })
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(
        |_, _| Init::new(Model::new()),
        update,
        view,
    )
    .routes(routes)
    .build_and_start();
}
