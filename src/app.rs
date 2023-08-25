use crate::views::home::Home;
use crate::views::not_found::NotFound;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/lleyton.css"/>
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.classless.min.css" />

        // content for this welcome page
        <Router>
            <Routes>
                <Route path="" view=Home/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
    }
}
