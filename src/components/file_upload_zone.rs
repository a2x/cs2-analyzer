use gloo::file::File;

use js_sys::wasm_bindgen::JsCast;

use web_sys::{DragEvent, Event, FileList, HtmlInputElement};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FileUploadZoneProps {
    pub on_files_uploaded: Callback<Vec<File>>,
}

#[function_component(FileUploadZone)]
pub fn file_upload_zone(props: &FileUploadZoneProps) -> Html {
    let on_files_uploaded = props.on_files_uploaded.clone();

    let upload_files = move |files: Option<FileList>| {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .filter(|f| f.name().ends_with(".dll"))
                .map(File::from);

            result.extend(files);
        }

        on_files_uploaded.emit(result);
    };

    html! {
        <div class="bg-body-tertiary mb-4 p-5">
            <h1 class="display-3">{"File upload"}</h1>

            <div
                class="file-upload-zone"
                onclick={Callback::from(|event: MouseEvent| {
                    event.stop_propagation();

                    let input: HtmlInputElement = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_element_by_id("file-upload")
                        .unwrap()
                        .dyn_into()
                        .unwrap();

                    input.click();
                })}
                ondragenter={Callback::from(|event: DragEvent| {
                    event.prevent_default();
                })}
                ondragover={Callback::from(|event: DragEvent| {
                    event.prevent_default();
                })}
                ondrop={{
                    let upload_files = upload_files.clone();

                    move |event: DragEvent| {
                        event.prevent_default();

                        upload_files(event.data_transfer().unwrap().files());
                    }
                }}
            >
                <p class="lead">
                    {"Drag files here to upload, or click to select (e.g. client.dll, engine2.dll)."}
                </p>

                <p>
                    <a class="btn btn-lg btn-success" href="#" role="button">
                        {"Upload files"}
                    </a>
                </p>

                <input
                    id="file-upload"
                    type="file"
                    accept=".dll"
                    multiple=true
                    style="display: none;"
                    onchange={{
                        let upload_files = upload_files.clone();

                        move |event: Event| {
                            let input: HtmlInputElement = event.target_unchecked_into();

                            upload_files(input.files());
                        }
                    }}
                />
            </div>
        </div>
    }
}
