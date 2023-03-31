use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use serde_json::Value;

#[wasm_bindgen(start)]
pub fn run() {
    set_panic_hook();
}

#[wasm_bindgen]
pub fn update_html_preview(new_html: &str, preview_id: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let preview: web_sys::HtmlIFrameElement = document
        .get_element_by_id(preview_id)
        .expect("could not find an element by id")
        .dyn_into()
        .expect("element is not an iframe");

    let preview_document = preview.content_document().expect("iframe should have a document");
    let body = preview_document.body().expect("iframe document should have a body");
    body.set_inner_html(new_html);
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}


#[wasm_bindgen]
pub fn fetch_gpt4_content(prompt: String, api_key: String) {
    spawn_local(async move {
        let content = fetch_gpt4_content_async(&prompt, &api_key).await;
        match content {
            Ok(content) => {
                let html_input = get_html_input_element();
                html_input.set_value(&content);
                update_html_preview(&content, "preview");
            }
            Err(e) => {
                web_sys::console::error_1(&format!("Error fetching GPT-4 content: {:?}", e).into());
            }
        }
    });
}

async fn fetch_gpt4_content_async(prompt: &str, api_key: &str) -> Result<String, JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let fetch = window.fetch_with_request(&create_request(prompt, api_key)?);

    let resp_value = JsFuture::from(fetch).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    let json_str = JsFuture::from(resp.text()?).await?;
    let json: serde_json::Value = serde_json::from_str(&json_str.as_string().unwrap()).unwrap();

    let content = json
        .get("choices")
        .and_then(|choices| choices[0].get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .unwrap_or("")
        .to_string();

    Ok(content)
}

fn create_request(prompt: &str, api_key: &str) -> Result<web_sys::Request, JsValue> {
    let url = "https://api.openai.com/v1/chat/completions";
    let mut request_init = web_sys::RequestInit::new();
    request_init.method("POST");
    request_init.headers(create_headers(api_key)?.as_ref());

    let body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0.7
    });
    request_init.body(Some(&JsValue::from_str(&body.to_string())));

    web_sys::Request::new_with_str_and_init(url, &request_init)
}

fn create_headers(api_key: &str) -> Result<web_sys::Headers, JsValue> {
    let headers = web_sys::Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    headers.set("Authorization", &format!("Bearer {}", api_key))?;
    Ok(headers)
}

fn get_html_input_element() -> web_sys::HtmlTextAreaElement {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let html_input = document
        .get_element_by_id("html-input")
        .expect("could not find an element by id")
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .expect("element is not a textarea");
    html_input
}
