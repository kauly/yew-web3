pub mod button;
pub mod conn_button;
pub mod spinner;

use conn_button::ConnButton;
use std::future;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;
use web3::api::Web3;
use web3::futures::StreamExt;
use web3::transports::eip_1193::{Eip1193, Provider};
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct Web3Type {
    pub web3: Option<Web3<Eip1193>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MetaMaskState {
    Connected,
    Disconnected,
    NoWallet,
    Loading,
}

fn get_web3() -> Web3Type {
    match Provider::default() {
        Ok(Some(p)) => Web3Type {
            web3: Some(Web3::new(Eip1193::new(p))),
        },
        _ => Web3Type { web3: None },
    }
}

#[function_component(App)]
fn app() -> Html {
    let web3_transport = use_state(get_web3);
    let metamask_state = use_state(|| MetaMaskState::Disconnected);
    let chain_id = use_state(|| String::from(""));
    let wallet_addr = use_state(|| String::from(""));

    let btn_disabled = metamask_state.deref().clone() == MetaMaskState::Connected
        || metamask_state.deref().clone() == MetaMaskState::Loading;

    let on_conn = {
        let web3_transport = web3_transport.clone();
        let metamask_state = metamask_state.clone();
        Callback::from(move |_| {
            let wp = web3_transport.web3.as_ref().unwrap().eth();
            let metamask_state = metamask_state.clone();
            spawn_local(async move {
                metamask_state.set(MetaMaskState::Loading);
                let res = wp.request_accounts().await;
                match res {
                    Ok(_) => metamask_state.set(MetaMaskState::Connected),
                    Err(_) => metamask_state.set(MetaMaskState::Disconnected),
                };
            });
        })
    };

    {
        let chain_id = chain_id.clone();
        let wallet_addr = wallet_addr.clone();
        let metamask_state = metamask_state.clone();
        let web3_transport = web3_transport.web3.clone();
        use_effect_with_deps(
            move |_| {
                match web3_transport {
                    Some(wp) => {
                        let wp_1 = wp.clone();
                        let wallet_addr_1 = wallet_addr.clone();
                        let chain_id_1 = chain_id.clone();
                        let metamask_state_1 = metamask_state.clone();
                        spawn_local(async move {
                            let chain_res = wp_1.eth().chain_id().await;
                            let addrs_res = wp_1.eth().accounts().await;
                            if let Ok(id) = chain_res {
                                chain_id_1.set(id.to_string());
                            }
                            if let Ok(addrs) = addrs_res {
                                if !addrs.is_empty() {
                                    wallet_addr_1.set(addrs[0].to_string());
                                    metamask_state_1.set(MetaMaskState::Connected);
                                } else {
                                    wallet_addr_1.set("Unavailable".to_string());
                                    metamask_state_1.set(MetaMaskState::Disconnected);
                                }
                            }
                        });
                        let wp_2 = wp.clone();
                        let wallet_addr = wallet_addr.clone();
                        let metamask_state = metamask_state.clone();
                        spawn_local(async move {
                            wp_2.transport()
                                .accounts_changed_stream()
                                .for_each(|addrs| {
                                    if !addrs.is_empty() {
                                        wallet_addr.set(addrs[0].to_string());
                                    } else {
                                        wallet_addr.set("Unavailable".to_string());
                                        metamask_state.set(MetaMaskState::Disconnected);
                                    }
                                    future::ready(())
                                })
                                .await;
                        });

                        spawn_local(async move {
                            wp.transport()
                                .chain_changed_stream()
                                .for_each(|id| {
                                    chain_id.set(id.to_string());
                                    future::ready(())
                                })
                                .await;
                        });
                    }
                    None => metamask_state.set(MetaMaskState::NoWallet),
                };
                || ()
            },
            (),
        );
    }

    return html! {
        <div class="flex justify-center items-center h-screen">
            <div class="p-2 border-4 border-zinc-800 flex flex-col space-y-4">
                <div class="border-b-4 border-zinc-800 text-center">
                    <ConnButton metamask_state={metamask_state.deref().clone()} disabled={btn_disabled} handler={on_conn.clone()} wallet_addr={wallet_addr.deref().clone()} />
                </div>
                <div class="border-b-4 border-zinc-800">
                    <span class="bold text-zinc-800 text-lg mr-2">{"Chain ID: "}</span>
                    <span class="bold text-zinc-800 text-lg">{chain_id.deref().clone()}</span>
                </div>
                <div class="border-b-4 border-zinc-800">
                    <span class="bold text-zinc-800 text-lg mr-2">{"Wallet Address: "}</span>
                    <span class="bold text-zinc-800 text-lg">{wallet_addr.deref().clone()}</span>
                </div>
            </div>
        </div>
    };
}

fn main() {
    yew::start_app::<App>();
}
