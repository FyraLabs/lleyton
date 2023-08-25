use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Title text="Not found..." />
        <header>
            <hgroup>
                <h1>"Not found"</h1>
                <h2>"404"</h2>
            </hgroup>
        </header>
        <main>
            <p>
                "The page you are looking for does not exist. Perhaps you want to go back to the homepage?"
            </p>
            <A href="/">
                "Go back."
            </A>
        </main>
    }
}
