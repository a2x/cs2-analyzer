use std::fmt::Debug;
use std::rc::Rc;

use cs2_analyzer::Analyzer;

use yew::prelude::*;

use crate::app::FileDetails;
use crate::components::{Alert, AlertStyle, FileAccordion, Tab, TabPane};

#[derive(Properties, PartialEq)]
pub struct FileAnalysisProps {
    pub file: FileDetails,
    pub analyzer: Rc<Analyzer>,
}

#[function_component(FileAnalysis)]
pub fn file_analysis(props: &FileAnalysisProps) -> Html {
    let file_name = props.file.name.clone();

    match props.analyzer.analyze_from_bytes(&props.file.data) {
        Ok(result) => {
            let tabs: [(&str, &dyn Debug); 7] = [
                ("Buttons", &result.buttons),
                ("ConCommands", &result.concommands),
                ("ConVars", &result.convars),
                ("Interfaces", &result.interfaces),
                ("Offsets", &result.offsets),
                ("Classes", &result.classes),
                ("Enums", &result.enums),
            ];

            html! {
                <FileAccordion file={props.file.clone()}>
                    <ul class="nav nav-tabs" id={format!("tabs-{}", file_name)} role="tablist">
                        {tabs.iter().enumerate().map(|(i, (name, _items))| {
                            html! {
                                <Tab file_name={file_name.clone()} tab={*name} is_active={i == 0} />
                            }
                        }).collect::<Html>()}
                    </ul>

                    <div class="tab-content" id={format!("tab-content-{}", file_name)}>
                        {tabs.iter().enumerate().map(|(i, (name, items))| {
                            html! {
                                <TabPane
                                    file_name={file_name.clone()}
                                    name={name.to_string()}
                                    is_active={i == 0}
                                >
                                    <pre>
                                        <code>{format!("{:#X?}", items)}</code>
                                    </pre>
                                </TabPane>
                            }
                        }).collect::<Html>()}
                    </div>
                </FileAccordion>
            }
        }
        Err(_) => html! {
            <Alert
                style={AlertStyle::Danger}
                message={format!("Failed to analyze {}. Are you sure this is a valid game binary?", file_name)}
            />
        },
    }
}
