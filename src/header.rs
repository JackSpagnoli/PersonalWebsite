use leptos::{IntoView, component, view};

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="transition-h duration-300 items-center justify-center h-24 hover:h-32 w-full flex flex-row">
            <div class="w-10"></div>
            <div class="flex flex-row w-1/3">
                <img
                    class="w-16 h-16 rounded-full"
                    src="https://avatars.githubusercontent.com/u/114365987?v=4"
                />
                <div class="w-4"></div>
                <h1 class="text-5xl">"Jack Spagnoli"</h1>
            </div>
            <div class="w-1/6"></div>
            <div class="align-middle w-1/2 justify-center flex items-center">
                <h2>"Impressive Personal Website"</h2>
            </div>
            <div class="w-1/6"></div>
            <div class="flex flex-row w-1/3 justify-end">
                <a href="https://gitea.spagnoli.co.uk/Jack/PersonalWebsite">
                    <div
                        class="w-16 h-16 rounded-full hover:w-20 hover:h-20 hover:ml-20 transition-h duration-300"
                        style="background-image: url('https://static.wikia.nocookie.net/logopedia/images/8/8f/25231.svg');background-repeat: no-repeat; background-size: cover;"
                    />
                </a>
            </div>
            <div class="w-10"></div>
        </div>
    }
}
