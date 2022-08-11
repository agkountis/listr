use yew::prelude::*;
use yew_oauth2::context::OAuth2Context;
use listr_common::{extract_user_from_token, User};

#[derive(Properties, PartialEq, Clone)]
pub struct WithUserProps {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &WithUserProps) -> Html {
    let user_state = use_state_eq(|| User::default());
    let oauth_context = use_context::<OAuth2Context>().unwrap();
    let authentication = oauth_context.authentication().unwrap();

    let user = extract_user_from_token(&authentication.access_token);
    user_state.set(user);

    html! {
        <ContextProvider<User> context={(*user_state).clone()}>
            { props.children.clone() }
        </ContextProvider<User>>
    }
}
