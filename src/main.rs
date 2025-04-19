use tracing::info;
use ucan_playground::components::create_ucan::CreateUcan;
use ucan_playground::components::token_chain::TokenChain;
use ucan_playground::ucan::playground::UcanPlayground;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let playground = use_state(|| {
        let mut pg = UcanPlayground::new();
        pg.load_from_storage()
            .unwrap_or_else(|e| tracing::error!("Failed to load tokens: {}", e));
        pg
    });
    let error = use_state(|| None::<String>);
    let loading = use_state(|| false);

    use_effect_with((), move |_| {
        tracing_wasm::set_as_global_default();
        info!("UCAN Playground initialized");
        || ()
    });

    html! {
        <div class="container">
            <h1>{ "UCAN Playground" }</h1>
            { (*error).as_ref().map(|e| html! { <div class="error">{ e }</div> }) }
            <CreateUcan playground={playground.clone()} error={error.clone()} loading={loading.clone()} />
            <TokenChain tokens={playground.get_tokens().clone()} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
