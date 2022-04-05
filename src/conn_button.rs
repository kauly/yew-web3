use crate::button::Button;
use crate::MetaMaskState;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ConnButtonProps {
    pub disabled: bool,
    pub metamask_state: MetaMaskState,
    pub handler: Callback<MouseEvent>,
    pub wallet_addr: String,
}

#[function_component(ConnButton)]
pub fn conn_button(props: &ConnButtonProps) -> Html {
    html! {
        if props.metamask_state == MetaMaskState::NoWallet {
            <p>{"Please, install a ethereum compatible wallet"}</p>
        } else {
          <Button handler={props.handler.clone()} loading={props.metamask_state == MetaMaskState::Loading} disabled={props.disabled}>
           if props.metamask_state == MetaMaskState::Disconnected {
            <div class="flex items-center">
                <img src="static/metamask.svg" alt="MetaMask Icon" width="32" height="32" />
                <p class="ml-2">{"Connect to MetaMask"}</p>
            </div>
           } else {
               {props.wallet_addr.clone()}
           }
          </Button>
        }
    }
}
