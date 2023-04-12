use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SessionIdProperties {
    pub on_change: Callback<String>,
    pub value: String,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    e
        .target()
        .unwrap()
        .unchecked_into::<HtmlInputElement>()
        .value()
}

#[function_component(SessionId)]
pub fn session_id(
    SessionIdProperties { on_change, value }: &SessionIdProperties,
) -> Html {
    let on_change = on_change.clone();
    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });


    html! {
                <input id="session_id" type="text" placeholder="SESSION ID"
                        oninput = { oninput }
                        value= { value.clone() }
        />
    }
}