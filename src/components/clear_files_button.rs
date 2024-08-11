use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ClearFilesButtonProps {
    pub visible: bool,
    pub onclick: Callback<()>,
}

#[function_component(ClearFilesButton)]
pub fn clear_files_button(props: &ClearFilesButtonProps) -> Html {
    if props.visible {
        html! {
            <p class="text-end">
                <a
                    class="link-danger"
                    href="#"
                    onclick={props.onclick.reform(|_| ())}
                >
                    {"Clear uploaded files."}
                </a>
            </p>
        }
    } else {
        html! {}
    }
}
