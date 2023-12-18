use leptos::{IntoView, component, view};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="transition-h duration-300 items-center justify-left h-24 hover:h-32 w-full flex flex-row">
            <div class="w-12"></div>
            <div class="flex flex-row">
                <img
                    class="w-16 h-16 rounded-full"
                    src="https://avatars.githubusercontent.com/u/114365987?v=4"
                />
                <div class="w-4"></div>
                <h1 class="text-5xl">"Jack Spagnoli"</h1>
            </div>
            <div class="w-1/6"></div>
            <div class="align-middle">
                <h2>"Impressive Personal Website"</h2>
                <h3>"(Unfortunately I'm not available for hire)"</h3>
            </div>
            <div class="w-1/6"></div>
            <div class="w-12">
                <a href="https://gitea.spagnoli.co.uk/Jack/PersonalWebsite">
                    <div
                        class="w-16 h-16 rounded-full hover:w-20 hover:h-20"
                        style="background-image: url('https://static.wikia.nocookie.net/logopedia/images/8/8f/25231.svg');background-repeat: no-repeat; background-size: cover;"
                    />
                </a>
            </div>
        </div>
    }
}
