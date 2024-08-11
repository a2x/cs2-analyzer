use std::collections::HashMap;
use std::rc::Rc;

use cs2_analyzer::Analyzer;

use gloo::file::callbacks::FileReader;
use gloo::file::File;

use yew::prelude::*;

use crate::components::*;

pub enum Msg {
    Loaded(String, Vec<u8>),
    Files(Vec<File>),
    ClearFiles,
}

#[derive(Clone, PartialEq)]
pub struct FileDetails {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct App {
    analyzer: Analyzer,
    files: Vec<FileDetails>,
    loading: bool,
    readers: HashMap<String, FileReader>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            analyzer: Analyzer::new(),
            files: Vec::default(),
            loading: false,
            readers: HashMap::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, data) => {
                if !self.readers.contains_key(&file_name) {
                    return false;
                }

                self.files.push(FileDetails {
                    data,
                    name: file_name.clone(),
                });

                self.readers.remove(&file_name);

                // Stop the loading animation if there are no more files to read.
                self.loading = !self.readers.is_empty();

                true
            }
            Msg::Files(files) => {
                for file in &files {
                    let file_name = file.name();

                    // Skip files that have already been processed.
                    if self.files.iter().any(|f| f.name == file_name) {
                        continue;
                    }

                    self.loading = true;

                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo::file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };

                    self.readers.insert(file_name, task);
                }

                true
            }
            Msg::ClearFiles => {
                self.files.clear();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container">
                <Header />

                <FileUploadZone on_files_uploaded={ctx.link().callback(Msg::Files)} />

                {if self.loading {
                    html! {
                        <LoadingIndicator />
                    }
                } else {
                    html! {
                        <div class="mb-4 mt-4">
                            <ClearFilesButton
                                visible={!self.files.is_empty()}
                                onclick={ctx.link().callback(|_| Msg::ClearFiles)}
                            />

                            {for self.files.iter().map(|file| html! {
                                <FileAnalysis
                                    file={file.clone()}
                                    analyzer={Rc::new(self.analyzer.clone())}
                                />
                            })}
                        </div>
                    }
                }}

                <Footer />
            </div>
        }
    }
}
