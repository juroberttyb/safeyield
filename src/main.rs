use leptos::*;use leptos::prelude::*;
use web_sys;

#[component]
fn App() -> impl IntoView {
    let deposit_address = "0xa0fE0fFA1234cf1f9Ee198De2d6A3BA7A23Cd874";
    let (deposit_address_getter, _) = signal(deposit_address.to_string());

    view! {
        <div class="container">

            <img
                id="logo"
                // class="rss opacity"
                loading="lazy"
                src="./images/eth.png"
                alt=""
                height="300"
                width="300"
            />

            <div class="paragraph">
                <div class="align-left">
                    <p class="upper">"從Max交易所,使用Arbitrum網路,提領ETH,USDT,USDC中任一種到這個地址"</p>
                    <p class="center-text upper">{ move || deposit_address_getter.get() }</p>
                    <div class="center upper lower">
                        <button
                            class="button"
                            on:click= move |_| {
                                let window = web_sys::window().unwrap();
                                let navigator = window.navigator();
                                let clipboard = navigator.clipboard();
                                let _ = clipboard.write_text(deposit_address);
                            }
                        >
                            複製地址
                        </button>
                    </div>
                </div>
            </div>

            <div class="paragraph">
                <div class="align-left">
                    <p>"這邊的收益 (每日更新): ETH (1.7%), USDC (5.5%), USDT (5.1%)"</p>
                    <p><a href="https://max.maicoin.com/docs/yield">"Max的收益 (每日更新): ETH (0.8%), USDC (1.0%), USDT (1.38%)"</a></p>
                </div>
            </div>

        </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}