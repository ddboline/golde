use dioxus::prelude::*;
use fermi::*;
use golde::*;

fn main() {
    dioxus::desktop::launch(app)
}

static RESULT: Atom<f64> = |_| 0.0;

fn app(cx: Scope) -> Element {
    init_app(&cx, |_| {});

    let (a, a_setter) = use_state(&cx, || 0.0).split();
    let (b, b_setter) = use_state(&cx, || 0.0).split();

    let res = use_read(&cx, RESULT);

    let setter = use_set(&cx, RESULT).clone();

    cx.render(rsx!(
        App {
            trigger: trigger!(
                test => move |_, v| {
                    setter(v.as_number().unwrap_or(0.0));
                }
            ),
            input {
                value: "{a}",
                onchange: move |data| a_setter.modify(|_|
                    data.value.parse::<f64>().unwrap_or(0.0)
                )
            }
            input {
                value: "{b}",
                onchange: move |data| b_setter.modify(|_|
                    data.value.parse::<f64>().unwrap_or(0.0)
                )
            }
            button {
                onclick: move |_| {
                    let code = format!("{} + {}", &a, &b);
                    println!("{:?}", code);
                    call(&cx, "test", code);
                },
                "Calc"
            }
            p { "Result: {res}" }
        }
    ))
}
