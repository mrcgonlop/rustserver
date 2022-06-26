use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use crate::lib::BinauralGenerator;

mod lib;

fn main() {
    //launch_audio();
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let binaural_gen = use_ref(&cx, || BinauralGenerator::new().ok().unwrap());

    cx.render(rsx!{
        style { [include_str!("./style.css")] }
        div {
            class: "container px-4 mx-auto",
            h2 {
                class: "text-2xl font-bold", "Binaural generator"
            }
            div{
                class: "ear-control px-4 mx-auto",
                div {
                    class: "ear px-4 mx-auto",
                    h3 {
                        class: "text-xl font-bold", "left ear"
                    },
                    input{
                        name: "left ear frequency",
                        r#type: "number",
                        value: "107",
                        min:"1",
                        max:"20000",
                        onchange: move |evnt| binaural_gen.read().set_frequency(evnt.value.clone(),"left"),
                        oninput: move |evnt| binaural_gen.read().set_frequency(evnt.value.clone(),"left")

                    }
                }
                div {
                    class: "ear px-4 mx-auto",
                    h3 {
                        class: "text-xl font-bold", "right ear"
                    },
                    input{
                        name: "right ear frequency",
                        r#type: "number",
                        value: "100",
                        min:"1",
                        max:"20000",
                        onchange: move |evnt| binaural_gen.read().set_frequency(evnt.value.clone(),"right"),
                        oninput: move |evnt| binaural_gen.read().set_frequency(evnt.value.clone(),"right")
                    }
                }
            }
            div{
                class: "container px-4 mx-auto",
                button {
                    class: "inline-block w-full md:w-auto px-6 py-3 font-medium text-white bg-indigo-500 hover:bg-indigo-600 rounded transition duration-200",
                    onclick: move |_| binaural_gen.read().resume_suspend(),
                    "▶/⯀"
                },
                input {
                    name: "vol-control",
                    r#type: "range",
                    min:"0",
                    max:"1",
                    step:"0.01",
                    value:"0",
                    onchange: move |evnt| binaural_gen.read().set_gain(evnt.value.clone().parse::<f32>().unwrap()),
                    oninput: move |evnt| binaural_gen.read().set_gain(evnt.value.clone().parse::<f32>().unwrap())
                }
            }
        }
    })
}