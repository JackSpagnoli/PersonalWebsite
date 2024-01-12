use leptos::{IntoView, component, view};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div id="Header" class="transition-h duration-300 items-center justify-center h-32 hover:h-36 w-full flex flex-row">
            <div class="w-10"/>
            <div class="flex flex-row w-1/3 items-center">
                <img
                    class="w-16 h-16 rounded-full"
                    src="https://avatars.githubusercontent.com/u/114365987?v=4"
                />
                <div class="w-4"/>
                <h1 class="text-4xl align-middle">"Jack Spagnoli"</h1>
            </div>
            <div class="w-1/6"/>
            <div class="align-middle w-1/6 text-center">
                <h2>"Impressive Personal Website"</h2>
            </div>
            <div class="w-1/6"/>
            <div class="flex flex-row w-1/3 justify-end">
                <a href="https://github.com/JackSpagnoli/PersonalWebsite">
                    <div
                        class="w-16 h-16 rounded-full hover:w-20 hover:h-20 hover:ml-20 transition-h duration-300"
                        style="background-image: url('https://static.wikia.nocookie.net/logopedia/images/8/8f/25231.svg');background-repeat: no-repeat; background-size: cover;"
                    />
                </a>
            </div>
            <div class="w-10"/>
        </div>
    }
}
