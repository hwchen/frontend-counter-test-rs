use yew::{
    html,
    Component,
    ComponentLink,
    Html,
    ShouldRender,
};

struct Model {
    count: u64,
    says: String,
}

enum Msg {
    IncrementCounter,
    SetCounter(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            count: 0,
            says: "".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::IncrementCounter => {
                if self.count < 10 {
                    self.count += 1;
                } else {
                    self.says = "can't increment more than 10".to_owned();
                }
                true
            }
            Msg::SetCounter(n_str) => {
                match n_str.parse() {
                    Ok(n) => {
                        if n <= 10 {
                            self.count = n;
                            self.says = "".to_owned();
                        } else {
                            self.says = "can't set more than 10".to_owned();
                        }
                    },
                    Err(_) => {
                        self.says = "Please enter a number".to_owned();
                    }
                }
                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::IncrementCounter>{ "Click Me!" }</button>
                <p>{self.count}</p>
                <div>
                    <p>{ "Enter a number below to set counter" }</p>
                    <text::TextInput return_input=|txt| Msg::SetCounter(txt) />
                </div>
                <p>{&self.says}</p>
            </div>
        }
    }
}

mod text {
    use yew::{
        html,
        Callback,
        Component,
        ComponentLink,
        Html,
        IKeyboardEvent,
        Properties,
        ShouldRender,
    };

    pub struct TextInput {
        input: String,
        return_input: Callback<String>,
    }

    pub enum Msg {
        GetInput(String),
        ReturnInput,
        Pass,
    }

    #[derive(Properties, PartialEq)]
    pub struct Props {
        #[props(required)]
        pub return_input: Callback<String>,
    }

    impl Component for TextInput {
        type Message = Msg;
        type Properties = Props;

        fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
            Self {
                input: "".into(),
                return_input: props.return_input,
            }
        }

        fn update(&mut self, msg: Self::Message) -> ShouldRender {
            match msg {
                Msg::GetInput(input) => {
                    self.input = input;
                    false
                }
                Msg::ReturnInput => {
                    let res = self.input.clone();
                    self.input = "".to_owned();
                    self.return_input.emit(res);
                    true
                },
                Msg::Pass => false,
            }
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            self.return_input = props.return_input;
            true
        }

        fn view(&self) -> Html<Self> {
            html! {
                <input
                    value=&self.input
                    oninput=|e| Msg::GetInput(e.value)
                    onkeyup=|e| if e.key() == "Enter" { Msg::ReturnInput } else { Msg::Pass }
                />
            }
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
