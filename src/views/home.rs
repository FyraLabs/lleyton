use futures_util::TryFutureExt;
use leptos::*;
use leptos_meta::*;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum DiscordStatus {
    Online,
    Idle,
    Dnd,
    Offline,
}

#[derive(Deserialize, Serialize)]
struct LanyardData {
    discord_status: DiscordStatus,
}

#[derive(Deserialize, Serialize)]
struct LanyardResponse {
    data: LanyardData,
}

#[component]
pub fn DiscordStatusBadge(cx: Scope) -> impl IntoView {
    let lanyard = create_resource(
        cx,
        || (),
        |_| async move {
            reqwest::get("https://lanyard.fyralabs.com/v1/users/228736069091196928")
                .and_then(|res| res.json::<LanyardResponse>())
                .await
                .ok()
        },
    );

    let status = move || {
        lanyard
            .with(cx, |status| status.as_ref().map(|s| s.data.discord_status))
            .flatten()
            .unwrap_or(DiscordStatus::Online)
    };

    view! {
        cx,
        <span
            data-text=move || format!("Discord Status: {}", match status() {
                DiscordStatus::Online => "Online",
                DiscordStatus::Idle => "Idle",
                DiscordStatus::Dnd => "Do Not Disturb",
                DiscordStatus::Offline => "Offline",
            })
            class=move || format!("tooltip w-6 h-6 rounded-full border-4 absolute right-0 bottom-0.5 {}", match status() {
                DiscordStatus::Online => "bg-emerald-500 border-emerald-600 dark:bg-emerald-400 dark:border-emerald-500",
                DiscordStatus::Idle => "bg-yellow-500 border-yellow-600 dark:bg-amber-300 dark:border-amber-400",
                DiscordStatus::Dnd => "bg-red-500 border-red-600 dark:bg-red-400 dark:border-red-500",
                DiscordStatus::Offline => "bg-gray-500 border-gray-600 dark:bg-gray-300 dark:border-gray-400",
            })
        />
    }
}

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Title text="Lea @ Fyra" />
        <header>
            <div class="relative inline-block">
                <img src="/assets/avatar.webp" class="rounded-full w-20 h-20 bg-zinc-700" />
                <DiscordStatusBadge />
            </div>
            <hgroup>
                <h1>"Lea @ Fyra"</h1>
                <h2>"Student by day." <br/> "CEO by night." <br/> "Catgirl all the time."</h2>
            </hgroup>
            // ignore my inlined SVGs, it's 2 am and I'm tired
            <div class="flex gap-4 items-center">
                <a href="mailto:lleyton@fyralabs.com" aria-label="Open your email client with my address" class="w-6">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        <path d="M48 64C21.5 64 0 85.5 0 112c0 15.1 7.1 29.3 19.2 38.4L236.8 313.6c11.4 8.5 27 8.5 38.4 0L492.8 150.4c12.1-9.1 19.2-23.3 19.2-38.4c0-26.5-21.5-48-48-48H48zM0 176V384c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V176L294.4 339.2c-22.8 17.1-54 17.1-76.8 0L0 176z"/>
                    </svg>
                </a>
                <a href="https://github.com/lleyton" aria-label="Visit my GitHub page" class="w-6">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 496 512">
                        <path d="M165.9 397.4c0 2-2.3 3.6-5.2 3.6-3.3.3-5.6-1.3-5.6-3.6 0-2 2.3-3.6 5.2-3.6 3-.3 5.6 1.3 5.6 3.6zm-31.1-4.5c-.7 2 1.3 4.3 4.3 4.9 2.6 1 5.6 0 6.2-2s-1.3-4.3-4.3-5.2c-2.6-.7-5.5.3-6.2 2.3zm44.2-1.7c-2.9.7-4.9 2.6-4.6 4.9.3 2 2.9 3.3 5.9 2.6 2.9-.7 4.9-2.6 4.6-4.6-.3-1.9-3-3.2-5.9-2.9zM244.8 8C106.1 8 0 113.3 0 252c0 110.9 69.8 205.8 169.5 239.2 12.8 2.3 17.3-5.6 17.3-12.1 0-6.2-.3-40.4-.3-61.4 0 0-70 15-84.7-29.8 0 0-11.4-29.1-27.8-36.6 0 0-22.9-15.7 1.6-15.4 0 0 24.9 2 38.6 25.8 21.9 38.6 58.6 27.5 72.9 20.9 2.3-16 8.8-27.1 16-33.7-55.9-6.2-112.3-14.3-112.3-110.5 0-27.5 7.6-41.3 23.6-58.9-2.6-6.5-11.1-33.3 2.6-67.9 20.9-6.5 69 27 69 27 20-5.6 41.5-8.5 62.8-8.5s42.8 2.9 62.8 8.5c0 0 48.1-33.6 69-27 13.7 34.7 5.2 61.4 2.6 67.9 16 17.7 25.8 31.5 25.8 58.9 0 96.5-58.9 104.2-114.8 110.5 9.2 7.9 17 22.9 17 46.4 0 33.7-.3 75.4-.3 83.6 0 6.5 4.6 14.4 17.3 12.1C428.2 457.8 496 362.9 496 252 496 113.3 383.5 8 244.8 8zM97.2 352.9c-1.3 1-1 3.3.7 5.2 1.6 1.6 3.9 2.3 5.2 1 1.3-1 1-3.3-.7-5.2-1.6-1.6-3.9-2.3-5.2-1zm-10.8-8.1c-.7 1.3.3 2.9 2.3 3.9 1.6 1 3.6.7 4.3-.7.7-1.3-.3-2.9-2.3-3.9-2-.6-3.6-.3-4.3.7zm32.4 35.6c-1.6 1.3-1 4.3 1.3 6.2 2.3 2.3 5.2 2.6 6.5 1 1.3-1.3.7-4.3-1.3-6.2-2.2-2.3-5.2-2.6-6.5-1zm-11.4-14.7c-1.6 1-1.6 3.6 0 5.9 1.6 2.3 4.3 3.3 5.6 2.3 1.6-1.3 1.6-3.9 0-6.2-1.4-2.3-4-3.3-5.6-2z"/>
                    </svg>
                </a>
                <a href="https://ordinary.cafe/@lea" aria-label="Visit my Mastodon page" rel="me" class="w-6">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                        <path d="M433 179.11c0-97.2-63.71-125.7-63.71-125.7-62.52-28.7-228.56-28.4-290.48 0 0 0-63.72 28.5-63.72 125.7 0 115.7-6.6 259.4 105.63 289.1 40.51 10.7 75.32 13 103.33 11.4 50.81-2.8 79.32-18.1 79.32-18.1l-1.7-36.9s-36.31 11.4-77.12 10.1c-40.41-1.4-83-4.4-89.63-54a102.54 102.54 0 0 1-.9-13.9c85.63 20.9 158.65 9.1 178.75 6.7 56.12-6.7 105-41.3 111.23-72.9 9.8-49.8 9-121.5 9-121.5zm-75.12 125.2h-46.63v-114.2c0-49.7-64-51.6-64 6.9v62.5h-46.33V197c0-58.5-64-56.6-64-6.9v114.2H90.19c0-122.1-5.2-147.9 18.41-175 25.9-28.9 79.82-30.8 103.83 6.1l11.6 19.5 11.6-19.5c24.11-37.1 78.12-34.8 103.83-6.1 23.71 27.3 18.4 53 18.4 175z"/>
                    </svg>
                </a>
                <a href="https://x.com/lleyton__" aria-label="Visit my X (Twitter) page" class="w-6">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        <path d="M389.2 48h70.6L305.6 224.2 487 464H345L233.7 318.6 106.5 464H35.8L200.7 275.5 26.8 48H172.4L272.9 180.9 389.2 48zM364.4 421.8h39.1L151.1 88h-42L364.4 421.8z"/>
                    </svg>
                </a>
            </div>
        </header>
        <main>
            <section>
                <hgroup>
                    <h1>"About myself"</h1>
                    <h2>"Oh no, introductions are scary."</h2>
                </hgroup>
                <p>
                    "Dear wanderer,"
                </p>
                <p>
                    "I'm Lleyton, although I prefer to go by Lea. I'm okay with any pronouns, but she/her is pretty cool.
                    You probably know of me from my position at Fyra Labs.. after all, you're on my Fyra subdomain :).
                    While my title at Fyra Labs is \"Founder & CEO\", I do a lot more than that— everything that isn't assigned to another person.
                    This means " <em>"a lot"</em> " of writing code & devops, which are both joys for me."
                </p>
                <p>
                    "I consider myself to be a staunch advocate of free software, privacy, and liberty.
                    Maybe I'll write about my philosophy on this site.. one day.
                    Part of my job is upholding these principles within every decision we make at Fyra Labs.
                    I also consider myself to be a cypherpunk, albeit I don't do as much cryptography related things nowadays, at least compared to the past."
                </p>
                <p>
                    "This section isn't complete, so there's a lot missing about myself.
                    There'll probably be more in my writings, so keep a look out for those.
                    If you're feeling brave, feel free to reach out to me; I'd love to talk!
                    I'll try to get back to you as soon as possible, but I'm a busy person, so it might take a while."
                </p>
                <p>
                    "I'm also a catgirl, so there's that :3"
                </p>
                <p>
                    "Let's build a better future."
                    <br/>
                    "With love,"
                    <br/>
                    <b>
                        "Lea~"
                    </b>
                </p>
            </section>
            <section>
                <hgroup>
                    <h1>"Writings"</h1>
                    <h2>"Various things I've written. Feeling brave?"</h2>
                </hgroup>

                <p>
                    "Nothing here... yet"
                </p>
            </section>
        </main>
    }
}
