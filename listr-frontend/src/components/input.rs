use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Default, Debug, PartialEq)]
pub struct ActionInputProps {
    pub input_type: String,
    pub action_name: String,
    pub action: Callback<String>,
}

#[function_component(ActionInput)]
pub fn action_input(props: &ActionInputProps) -> Html {

    let input_ref = use_node_ref();

    let onclick = {
        let input_ref = input_ref.clone();
        let action = props.action.clone();

        move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let item_data = input.value();
                action.emit(item_data);
                input.set_value("")
            }
        }
    };

    html! {
        <>
            <input type={props.input_type.clone()} ref={input_ref}/>
            <button {onclick}>{props.action_name.clone()}</button>
        </>
    }
}
