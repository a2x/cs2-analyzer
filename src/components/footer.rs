use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <a
                class="text-muted"
                href="https://github.com/a2x/cs2-analyzer"
                target="_blank"
                rel="noopener noreferrer"
            >
                <u>{"GitHub"}</u>
            </a>
        </footer>
    }
}
