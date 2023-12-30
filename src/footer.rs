use leptos::{component, view, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div id="Footer" class="transition-h duration-300 flex flex-col items-center justify-center h-20 hover:h-32 w-full">
            <h1>"This footer exists for pure aesthetic value"</h1>
            <h2>
                "(And to tell you this website uses "
                <a href="https://leptos.dev">"Leptos"</a> " and " <a href="https://tailwindcss.com">"Tailwind CSS"</a>")"
            </h2>
        </div>
    }
}

