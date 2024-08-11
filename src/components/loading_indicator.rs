use yew::prelude::*;

#[function_component(LoadingIndicator)]
pub fn loading_indicator() -> Html {
    html! {
        <div class="mb-4 mt-4 text-center">
            <p>{"File(s) being analyzed, please wait..."}</p>

            <div class="spinner-border" role="status">
                <span class="visually-hidden">{"Loading..."}</span>
            </div>
        </div>
    }
}
