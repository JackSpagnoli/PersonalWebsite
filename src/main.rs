use leptos::{component, mount_to_body, view, IntoView, Show};

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
            title: "GitHub".to_string(),
            subtitle: "I use Rust btw".to_string(),
            coming_soon: false,
            gif_url: "https://media.giphy.com/media/2dK0W3oUksQk0Xz8OK/giphy.gif".to_string(),
            outline_colour: "outline-orange-800".to_string(),
            link: "https://github.com/jackspagnoli".to_string(),
        },
        PanelProps {
            title: "Work".to_string(),
            subtitle: "I do some real work".to_string(),
            coming_soon: false,
            gif_url: "".to_string(),
            outline_colour: "outline-blue-800".to_string(),
            link: "https://www.linkedin.com/in/jackspagnoli/".to_string(),
        },
        PanelProps {
            title: "Blog".to_string(),
            subtitle: "I write things down".to_string(),
            coming_soon: true,
            gif_url: "".to_string(),
            outline_colour: "outline-green-800".to_string(),
            link: "".to_string(),
        },
    ];
    view! { <div class="flex flex-row gap-2 justify-center h-full w-full px-2">{panels}</div> }
}

const PANEL_OUTER_CLASS: &str = "transition-w duration-300 outline-dashed justify-center w-1/3 hover:outline-3 hover:w-5/6 flex flex-col h-full gap-1";
const PANEL_INNER_CLASS: &str = "flex flex-col h-full grayscale hover:grayscale-0";

#[component]
fn Panel(
    title: String,
    subtitle: String,
    coming_soon: bool,
    gif_url: String,
    outline_colour: String,
    link: String,
) -> impl IntoView {
    let panel_inner_style = format!("background-image: url('{}'); background-repeat: no-repeat; background-size: cover; background-position: center;", gif_url);
    let panel_outer_class = format!("{PANEL_OUTER_CLASS} {outline_colour}");
    let href = link.to_string();
    view! {
        <a class=panel_outer_class href=href>
            <div class=PANEL_INNER_CLASS style=panel_inner_style>
                <div class="h-full"></div>
                <TextBox coming_soon=coming_soon title=title subtitle=subtitle/>
            </div>
        </a>
    }
}

const PANEL_TEXTBOX_CLASS: &str = "flex flex-col justify-end items-end origin-bottom-right -translate-x-full sm:translate-x-0 rotate-90 sm:transform sm:rotate-0";
const PANEL_TEXTBOX_STYLE: &str = "color: white; text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5); padding: 10px; background-color: rgba(0, 0, 0, 0.5);";

const PANEL_TITLE_CLASS: &str = "h-12 sm:align-bottom text-5xl indent-3";

const PANEL_SUBTITLE_CLASS: &str = "hidden sm:block h-12 text-xl indent-5 align-bottom";

#[component]
fn TextBox(coming_soon: bool, title: String, subtitle: String) -> impl IntoView {
    view! {
        <div class=PANEL_TEXTBOX_CLASS style=PANEL_TEXTBOX_STYLE>
            <Show when=move || coming_soon>
                <ComingSoonDiv/>
            </Show>
            <div id="title" class=PANEL_TITLE_CLASS>
                {title.clone()}
            </div>
            <div id="subtitle" class=PANEL_SUBTITLE_CLASS>
                {subtitle}
            </div>
        </div>
    }
}

#[component]
fn ComingSoonDiv() -> impl IntoView {
    view! {
        <div class="hidden sm:block sm:h-12 text-m indent-5 justify-center align-middle">
            "(Coming Soon)"
        </div>
    }
}

