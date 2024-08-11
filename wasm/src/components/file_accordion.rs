use yew::prelude::*;

use crate::app::FileDetails;

#[derive(Properties, PartialEq)]
pub struct FileAccordionProps {
    pub file: FileDetails,
    pub children: Children,
}

#[function_component(FileAccordion)]
pub fn file_accordion(props: &FileAccordionProps) -> Html {
    let file_name = props.file.name.clone();

    html! {
        <div class="accordion mb-3">
            <div class="accordion-item">
                <h2 class="accordion-header" id={format!("heading-{}", file_name)}>
                    <button
                        class="accordion-button collapsed"
                        type="button"
                        data-bs-target={format!("#collapse-{}", file_name)}
                        data-bs-toggle="collapse"
                    >
                        {file_name.clone()}
                    </button>
                </h2>

                <div
                    class="accordion-collapse collapse"
                    id={format!("collapse-{}", file_name)}
                >
                    <div class="accordion-body">
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        </div>
    }
}
