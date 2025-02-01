use leptos::*;use leptos::prelude::*;

#[component]
fn App(increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
    <div class="container">

        <img
            id="logo"
            // class="rss opacity"
            loading="lazy"
            src="/images/eth.png"
            alt=""
            height="300"
            width="300"
        />

        <h1>"0xa0fE0fFA1234cf1f9Ee198De2d6A3BA7A23Cd874"</h1>
        <h2><i>"ETH, USDT, USDC"</i></h2>

        <button
            on:click= move |_| {
                set_count.set(count.get() + increment)
            }
        >
            "Click me: "
            {count}
        </button>


    </div>
    }
}

fn main() {
    mount_to_body(|| {
        view! {
            <App increment=5 />
        }
    })
}