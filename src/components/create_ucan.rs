use crate::ucan::playground::UcanPlayground;
use tracing::info;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CreateUcanProps {
    pub playground: UseStateHandle<UcanPlayground>,
    pub error: UseStateHandle<Option<String>>,
    pub loading: UseStateHandle<bool>,
}

#[function_component(CreateUcan)]
pub fn create_ucan(props: &CreateUcanProps) -> Html {
    let audience_ref = use_node_ref();
    let scope_ref = use_node_ref();

    let onsubmit = {
        let playground = props.playground.clone();
        let error = props.error.clone();
        let loading = props.loading.clone();
        let audience_ref = audience_ref.clone();
        let scope_ref = scope_ref.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let playground = playground.clone();
            let error = error.clone();
            let loading = loading.clone();
            let audience = audience_ref
                .cast::<HtmlInputElement>()
                .map(|el| el.value())
                .unwrap_or_default();
            let scope = scope_ref
                .cast::<HtmlInputElement>()
                .map(|el| el.value())
                .unwrap_or_default();

            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let mut new_playground = (*playground).clone();
                match new_playground.create_root_ucan(&audience, scope).await {
                    Ok(token) => {
                        info!("Created UCAN: {}", token.jwt);
                        new_playground
                            .save_to_storage()
                            .unwrap_or_else(|e| error.set(Some(e.to_string())));
                        playground.set(new_playground);
                        error.set(None);
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
                loading.set(false);
            });
        })
    };

    html! {
        <div class="form-group">
            <h2>{ "Create Root UCAN" }</h2>
            <form onsubmit={onsubmit}>
                <div class="form-group">
                    <label for="audience">{ "Audience DID" }</label>
                    <input type="text" id="audience" ref={audience_ref} placeholder="did:key:..." required=true aria-required="true" pattern="^did:key:[a-zA-Z0-9+/=]+$" />
                </div>
                <div class="form-group">
                    <label for="scope">{ "Scope" }</label>
                    <input type="text" id="scope" ref={scope_ref} placeholder="publiish/topic1" required=true aria-required="true" />
                </div>
                <button type="submit" disabled={*props.loading} aria-busy={(*props.loading).to_string()}>
                    { if *props.loading { "Creating..." } else { "Create UCAN" } }
                </button>
            </form>
        </div>
    }
}
