use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <nav class="bg-primary navbar navbar-expand-lg">
            <a class="navbar-brand" href="#">{"cs2-analyzer"}</a>
        </nav>
    }
}
