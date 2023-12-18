use leptos::*;

fn main() {
    mount_to_body(|| App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="h-screen flex flex-col">
            <Header/>
            <Panels/>
            <Footer/>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
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
            // Add a rounded rectangular button linking to a github page
            <div>
                <a
                    href="https://github.com/jackspagnoli"
                    class="rounded-md bg-blue-500 text-white px-4 py-2"
                >
                    GitHub
                </a>
            </div>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div class="transition-h duration-300 flex items-center justify-center h-16 hover:h-32 w-full">
            <h1>"This footer exists for pure aesthetic value"</h1>
        </div>
    }
}

#[component]
fn Panels() -> impl IntoView {
    // Row of panels
    let panels = vec![
        PanelProps {
            title: "GitHub".to_string(),
            subtitle: "I use Rust btw".to_string(),
            coming_soon: false,
        },
        PanelProps {
            title: "Work".to_string(),
            subtitle: "I do some real work".to_string(),
            coming_soon: false,
        },
        PanelProps {
            title: "Blog".to_string(),
            subtitle: "I write things down".to_string(),
            coming_soon: true,
        },
    ];
    view! {
        <div class="flex flex-row gap-2 justify-center h-full w-full">
            {panels}
        </div>
    }
}

#[component]
fn Panel(title: String, subtitle: String, coming_soon: bool) -> impl IntoView {
    view! {
        <div class="transition-w duration-300 outline-dashed justify-center w-1/3 hover:outline-2 hover:w-5/6 h-full flex flex-col gap-1">
            <div class="h-full"></div>
            <div class="h-12 text-5xl indent-3">{title}</div>
            <div class="h-12 text-xl indent-5">{subtitle}</div>
            <Show when=move || coming_soon>
                <ComingSoonDiv/>
            </Show>

        </div>
    }
}

#[component]
fn ComingSoonDiv() -> impl IntoView {
    view! { <div class="h-12 text-m indent-5">"(Coming Soon)"</div> }
}

