use yew::prelude::*;

#[derive(PartialEq)]
pub enum AlertStyle {
    Success,
    Danger,
    Warning,
    Info,
}

impl AlertStyle {
    #[inline]
    pub fn class_name(&self) -> &str {
        match self {
            AlertStyle::Success => "alert-success",
            AlertStyle::Danger => "alert-danger",
            AlertStyle::Warning => "alert-warning",
            AlertStyle::Info => "alert-info",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AlertProps {
    pub style: AlertStyle,
    pub message: String,
}

#[function_component(Alert)]
pub fn alert(props: &AlertProps) -> Html {
    html! {
        <div
            class={format!("alert {} alert-dismissible fade show", props.style.class_name())}
            role="alert"
        >
            {props.message.clone()}

            <button class="btn-close" type="button" data-bs-dismiss="alert"></button>
        </div>
    }
}
