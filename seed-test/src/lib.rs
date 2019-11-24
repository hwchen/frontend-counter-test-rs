use seed::{div, input, p, button};
use seed::events::Event;
use seed::prelude::*;

const ENTER_KEY: u32 = 13;

struct Model {
    count: u32,
    response: String,
}

impl Model {
    fn new() -> Self {
        Self {
            count: 0,
            response: "".into(),
        }
    }

    fn increment(&mut self) {
        self.set(self.count + 1);
    }

    fn set(&mut self, n: u32) {
        if n > 10 {
            self.response = "Count above 10 not allowed".into();
        } else {
            self.count = n;
            self.response = "".into();
        }
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
    InputSet(Event),
}

// handle dom/io type things here.
// handle state with Model methods
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.increment(),
        Msg::InputSet(ev) => {
            let code = seed::to_kbevent(&ev).key_code();
            if code == ENTER_KEY {
                let target = ev.target().unwrap();
                let input_el = seed::to_input(&target);
                let s = input_el.value().trim().to_string();

                let n = s.parse();
                match n {
                    Ok(n) => model.set(n),
                    Err(_) => { model.response = format!("{:?} is not a number", s); },
                };

                // get access to the input element here and clear
                input_el.set_value("");
            }
        },
    }
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        button![simple_ev(Ev::Click, Msg::Increment), "increment"],
        input![raw_ev(Ev::KeyDown, Msg::InputSet)],
        p![model.count.to_string()],
        p![model.response],
    ]
}


#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(
        |_, _| Init::new(Model::new()),
        update,
        view,
    )
    .build_and_start();
}
