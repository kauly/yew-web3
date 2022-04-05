use crate::spinner::Spinner;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub disabled: bool,
    pub handler: Callback<MouseEvent>,
    pub children: Children,
}

fn get_btn_classes(disabled: bool) -> Classes {
    match disabled {
        false => classes!("btn-filled"),
        true => {
            classes!("btn-filled", "btn-disabled")
        }
    }
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
      <button onclick={props.handler.clone()} class={get_btn_classes(props.disabled)}>
        if props.loading {
            <div class="flex ml-2 items-center">
                <Spinner />
                <p>{"Processing..."}</p>
            </div>
        } else {
            {for props.children.iter()}
        }
     </button>
    }
}
