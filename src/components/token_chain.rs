use crate::ucan::types::UcanToken;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TokenChainProps {
    pub tokens: Vec<UcanToken>,
}

#[function_component(TokenChain)]
pub fn token_chain(props: &TokenChainProps) -> Html {
    let tokens = &props.tokens;

    html! {
        <div class="token-chain">
            <h2>{ "Token Chain" }</h2>
            { if tokens.is_empty() {
                html! { <p>{ "No tokens created yet." }</p> }
            } else {
                html! {
                    <>
                        <svg width="800" height={(tokens.len() * 100).to_string()} aria-label="Token chain visualization">
                            { for tokens.iter().enumerate().map(|(i, token)| {
                                let y = i * 100 + 50;
                                html! {
                                    <g>
                                        <rect x="10" y={(y - 20).to_string()} width="780" height="80" fill="#fafafa" stroke="#ddd" stroke-width="1"/>
                                        <text x="20" y={y.to_string()} font-size="14">{ format!("Issuer: {}", token.issuer) }</text>
                                        <text x="20" y={(y + 20).to_string()} font-size="14">{ format!("Audience: {}", token.audience) }</text>
                                        <text x="20" y={(y + 40).to_string()} font-size="14">{ format!("Expires: {}", token.expiration.to_rfc3339()) }</text>
                                        { if i > 0 {
                                            html! {
                                                <line x1="400" y1={(y - 80).to_string()} x2="400" y2={(y - 20).to_string()} stroke="#333" stroke-width="2"/>
                                            }
                                        } else {
                                            html! {}
                                        }}
                                    </g>
                                }
                            })}
                        </svg>
                        { for tokens.iter().map(|token| {
                            html! {
                                <div class="token">
                                    <p><strong>{ "JWT: " }</strong>{ &token.jwt }</p>
                                </div>
                            }
                        })}
                    </>
                }
            }}
        </div>
    }
}
