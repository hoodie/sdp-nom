use sdp_nom::{attributes::candidate::CandidateProtocol, Session};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(msg: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn debug(msg: &str);
}

#[wasm_bindgen]
pub struct SdpSession {
    inner: sdp_nom::Session<'static>,
}

#[wasm_bindgen]
impl SdpSession {
    pub fn remove_direct_tcp_candidates(&mut self) {
        self.inner.media = self
            .inner
            .media
            .drain(..)
            .map(|mut media| {
                media.candidates = media
                    .candidates
                    .into_iter()
                    .filter(|c| c.protocol == CandidateProtocol::Tcp)
                    .collect();
                media
            })
            .collect();
    }

    pub fn to_object(&self) -> JsValue {
        JsValue::from_serde(&self.inner).unwrap()
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }

    #[cfg(feature = "json")]
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.inner).unwrap()
    }

    pub fn into_string(&self) -> String {
        self.inner.to_string()
    }
}

impl From<&str> for SdpSession {
    fn from(source: &str) -> Self {
        Self::from(Session::read_str(&source).into_owned())
    }
}

impl From<Session<'static>> for SdpSession {
    fn from(inner: sdp_nom::Session<'static>) -> Self {
        Self { inner }
    }
}

#[wasm_bindgen]
pub fn parse_sdp(content: &str) -> SdpSession {
    SdpSession::from(content)
}

#[cfg(feature = "auto_init")]
#[wasm_bindgen(start)]
pub fn main() {
    use js_sys::Function;
    use wasm_bindgen::JsCast;

    log("main function");
    if let Some(window) = web_sys::window() {
        debug("got the window");
        let my_closure =
            Closure::wrap(Box::new(move |src: String| parse_sdp(&src))
                as Box<dyn FnMut(String) -> SdpSession>);
        // let x: js_sys::Function = my_closure.as_ref().unchecked_ref().to_owned();
        // js_sys::Object::define_property(&window, &JsValue::from("wasmSdp"), js_sys::Function::from(x).into());
        let my_function: Function = my_closure.as_ref().unchecked_ref::<Function>().to_owned();
        js_sys::Object::define_property(
            &window,
            &JsValue::from("wasmSdp"),
            &js_sys::Object::create(&my_function),
        );
    }
}
