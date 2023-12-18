use leptos::{view,component, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="transition-h duration-300 flex items-center justify-center h-16 hover:h-32 w-full">
            <h1>"This footer exists for pure aesthetic value"</h1>
        </div>
    }
}
