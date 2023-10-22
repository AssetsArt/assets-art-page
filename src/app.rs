use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
            <main>
                <img class="logo" src="/assets/image/assets-logo.png" alt="AssetsArt logo" />
                <div style="text-align: justify;">
                    <span class="span-link"> {{"📦 Get in touch"}}</span>
                </div>
                <div align="left">
                    <a href="https://github.com/AssetsArt" title="Github AssetsArt" style="margin-right: 10px;">
                        <img src="https://img.shields.io/badge/github - AssetsArt-blue?logo=github&logoColor=white" alt="Github - AssetsArt" />
                    </a>
                    <a href="mailto:meanstack20+assets_art@gmail.com" title="Email Me">
                        <img src="https://img.shields.io/badge/Email - meanstack20+assets_art@gmail.com-blue?logo=gmail&logoColor=white" alt=":meanstack20+assets_art@gmail.com" />
                    </a>
                </div>
            </main>
        }
}
