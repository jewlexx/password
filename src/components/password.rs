use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct PasswordProps {
    pub passwords: Vec<String>,
}

/// Root app component
#[function_component(Password)]
pub fn password(props: &PasswordProps) -> Html {
    let passwords = props.passwords.iter().map(|password| {
        html! {
            <>
                {password}<br/>
            </>
        }
    });

    html! {
        <p class="password-container">{ for passwords }</p>
    }
}
