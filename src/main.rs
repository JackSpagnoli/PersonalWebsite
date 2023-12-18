use leptos::{IntoView, Show, component, mount_to_body, view};

fn main() {
    mount_to_body(|| App)
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="h-screen flex flex-col bg-gray-400">
            <Header/>
            <Panels/>
            <Footer/>
        </div>
    }
}

mod header;
use header::*;

mod footer;
use footer::*;

#[component]
fn Panels() -> impl IntoView {
    // Row of panels
    let panels = vec![
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
        <div class="flex flex-row gap-2 justify-center h-full w-full px-2">
            <PersonalPanel/>
            {panels}
        </div>
    }
}

const PANEL_OUTER_CLASS: &str = "transition-w duration-300 outline-dashed justify-center w-1/3 hover:outline-3 hover:w-5/6 flex flex-col h-full gap-1";
const PANEL_INNER_CLASS: &str = "flex flex-col h-full grayscale hover:grayscale-0";
const PANEL_TITLE_CLASS: &str = "h-12 text-5xl indent-3";
const PANEL_SUBTITLE_CLASS: &str = "h-12 text-xl indent-5";

#[component]
fn PersonalPanel() -> impl IntoView {
    let gif_url = "https://media.giphy.com/media/2dK0W3oUksQk0Xz8OK/giphy.gif";
    let style = format!("background-image: url('{}'); background-repeat: no-repeat; background-size: cover; background-position: center;", gif_url);
    let class = format!("{PANEL_OUTER_CLASS} outline-orange-800");
    view! {
        <a class=class href="https://github.com/jackspagnoli">
            <div class=PANEL_INNER_CLASS style=style>
                <div class="h-full"></div>
                <div class=PANEL_TITLE_CLASS>"GitHub"</div>
                <div class=PANEL_SUBTITLE_CLASS>"I use Rust btw"</div>
            </div>
        </a>
    }
}

#[component]
fn Panel(title: String, subtitle: String, coming_soon: bool) -> impl IntoView {
    view! {
        <div class=PANEL_OUTER_CLASS>
            <div class=PANEL_INNER_CLASS>
                <div class="h-full"></div>
                <Show when=move || coming_soon>
                    <ComingSoonDiv/>
                </Show>
                <div class=PANEL_TITLE_CLASS>{title}</div>
                <div class=PANEL_SUBTITLE_CLASS>{subtitle}</div>
            </div>
        </div>
    }
}

#[component]
fn ComingSoonDiv() -> impl IntoView {
    view! { <div class="h-12 text-m indent-5">"(Coming Soon)"</div> }
}

