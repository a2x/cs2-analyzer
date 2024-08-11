use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabPaneProps {
    pub file_name: String,
    pub name: String,
    pub is_active: bool,
    pub children: Children,
}

#[function_component(TabPane)]
pub fn tab_pane(props: &TabPaneProps) -> Html {
    html! {
        <div
            class={if props.is_active { "active fade show tab-pane" } else { "fade tab-pane" }}
            id={format!("{}-{}", props.name.to_lowercase(), props.file_name)}
            role="tabpanel"
        >
            <div class="list-group mt-4">
                <div class="list-group-item">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
