use crate::utils::{api_utils, navigation_utils};
use crate::view::asset::events::AssetEvents;
use crate::view::asset::{
    accessories::AssetAccessories, blueprint::AssetBlueprint, d1sk::AssetD1sk, event::AssetEvent,
    illuvitar::AssetIlluvitar, land::AssetLand,
};
use crate::view::loading::LoadingSpinnerGray;
use log::error;
use model::model::asset::AssetData;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub token_address: String,
    pub token_id: i32,
}

#[function_component(Asset)]
pub fn asset_function_component(props: &Props) -> Html {
    let asset = use_state(|| None);
    {
        let token_address = props.token_address.clone();
        let token_id = props.token_id;
        let asset = asset.clone();
        use_effect_with(
            (props.token_id.clone(), props.token_address.clone()),
            move |_| {
                asset.set(None);
                navigation_utils::scroll_to_top();
                wasm_bindgen_futures::spawn_local(async move {
                    match api_utils::fetch_single_api_response::<AssetData>(
                        format!(
                            "/asset/asset?token_address={}&token_id={}",
                            token_address, token_id
                        )
                        .as_str(),
                    )
                    .await
                    {
                        Ok(fetched_asset) => {
                            asset.set(Some(fetched_asset));
                        }
                        Err(e) => {
                            error!("{e}")
                        }
                    }
                });
            },
        );
    }

    return match (*asset).as_ref() {
        Some(asset) => {
            let mut asset_html = html!();
            if asset.land.is_some() {
                asset_html = html! {<AssetLand land={asset.land.clone().unwrap()} />};
            } else if asset.d1sk.is_some() {
                asset_html = html! {<AssetD1sk d1sk={asset.d1sk.clone().unwrap()} />};
            } else if asset.accessories.is_some() {
                asset_html =
                    html! {<AssetAccessories accessories={asset.accessories.clone().unwrap()} />};
            } else if asset.illuvitar.is_some() {
                asset_html =
                    html! {<AssetIlluvitar illuvitar={asset.illuvitar.clone().unwrap()} />};
            } else if asset.blueprint.is_some() {
                asset_html =
                    html! {<AssetBlueprint blueprint={asset.blueprint.clone().unwrap()} />};
            } else if asset.event.is_some() {
                asset_html = html! {<AssetEvent event={asset.event.clone().unwrap()} />};
            }

            html! {
                <selection>
                    { asset_html }
                    { html! {<AssetEvents token_id={props.token_id} token_address={props.token_address.clone()} />} }
                </selection>
            }
        }
        None => {
            html! {
                <LoadingSpinnerGray />
            }
        }
    };
}
