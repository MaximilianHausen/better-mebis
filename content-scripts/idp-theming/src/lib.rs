use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub async fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    web_sys::window().unwrap().document().unwrap().body().unwrap().class_list().add_1("dark").unwrap();

    for e in get_elements(".bg-gray-light") {
        e.class_list().add_1("dark:bg-blue-1000").unwrap();
    }
    for e in get_elements(".bg-white") {
        e.class_list().add_1("dark:bg-blue-900").unwrap();
    }

    for e in get_elements(".font-display") {
        e.class_list().add_1("dark:text-white").unwrap();
    }
    for e in get_elements("legend") {
        e.class_list().add_1("dark:text-white").unwrap();
    }

    for e in get_elements("input") {
        e.class_list().add_2("dark:bg-blue-900", "dark:text-white").unwrap();
    }

    for e in get_elements("button .bg-orange-800") {
        e.class_list().add_1("dark:ext-bg-orange-dark").unwrap();
        e.class_list().add_1("dark:ext-border-orange-dark").unwrap();
        e.class_list().add_1("dark:group-hover:ext-bg-orange-800").unwrap();
        e.class_list().add_1("dark:group-hover:ext-border-orange-800").unwrap();
    }
}

pub fn get_elements(selector: &str) -> Vec<Element> {
    let mut vec = vec![];

    let node_list = web_sys::window().unwrap().document().unwrap().query_selector_all(selector).unwrap();
    for i in 0..node_list.length() {
        vec.push(Element::from(JsValue::from(node_list.get(i))));
    }

    vec
}
