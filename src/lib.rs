mod utils;

use std::rc::Rc;
use std::{cell::RefCell, mem::forget};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, Worker}; // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
                                     // allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, learn-worker!");
}

// #[wasm_bindgen(start)]
#[wasm_bindgen]
pub async fn start() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log::init_with_level(log::Level::Info).unwrap();
    log::info!("hello");

    let (sender, receiver) = crossbeam_channel::unbounded();
    let on_worker_message = Closure::wrap(Box::new(move |e: MessageEvent| {
        let val = e.data().as_f64().unwrap() as i32;
        log::info!("on_worker_message {}", e.data().as_f64().unwrap() as i32);
        sender.send(val).unwrap();
    }) as Box<dyn FnMut(MessageEvent)>);

    let worker_handle = start_worker();
    {
        let worker = &*worker_handle.borrow();
        worker.set_onmessage(Some(on_worker_message.as_ref().unchecked_ref()));
    }
    forget(on_worker_message);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if web_sys::window().is_none() {
            return;
        }
        // if i > 300 {
        //     body().set_text_content(Some("All done!"));

        //     // Drop our handle to this closure so that it will get cleaned
        //     // up once we return.
        //     let _ = f.borrow_mut().take();
        //     return;
        // }

        if let Ok(received) = receiver.try_recv() {
            log::info!("received {}", received);
        }

        let worker = &*worker_handle.borrow();
        let value = format!("{}", i);
        let _ = worker.post_message(&value.into());

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        body().set_text_content(Some(&text));

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn start_worker() -> Rc<RefCell<Worker>> {
    let mut options = web_sys::WorkerOptions::new();
    options.type_(web_sys::WorkerType::Module);
    let worker_handle = Rc::new(RefCell::new(
        Worker::new_with_options("./worker.js", &options).unwrap(),
    ));
    log::info!("Created a new worker from within WASM");

    worker_handle
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    if web_sys::window().is_none() {
        return;
    }

    log::info!("request_aniamtion_frame");
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

// worker.js 에서 부르는 함수
#[wasm_bindgen]
pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}
