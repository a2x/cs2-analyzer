use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub file_name: String,
    pub tab: String,
    pub is_active: bool,
}

#[function_component(Tab)]
pub fn tab(props: &TabProps) -> Html {
    html! {
        <li class="nav-item" role="presentation">
            <button
                class={if props.is_active { "nav-link active" } else { "nav-link" }}
                id={format!("{}-tab-{}", props.tab.to_lowercase(), props.file_name)}
                type="button"
                role="tab"
                data-bs-toggle="tab"
                data-bs-target={format!("#{}-{}", props.tab.to_lowercase(), props.file_name)}
            >
                {props.tab.clone()}
            </button>
        </li>
    }
}
