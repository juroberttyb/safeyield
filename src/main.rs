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

        <h1>"Welcome to Leptos"</h1>
        <h2><i>"On Github Pages"</i></h2>

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