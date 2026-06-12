use leptos::{
    ev,
    html::{div, button, span},
    prelude::*,
};

pub fn counter(initial_value: i32, step: i32) -> impl IntoView {
    let (count, set_count) = signal(initial_value);
    div().child((
        button()
            .on(ev::click, move |_| set_count.set(0))
            .child("Clear"),

        button()
            .on(ev::click, move |_| *set_count.write() -= step)
            .child("-1"),

        button()
            .on(ev::click, move |_| *set_count.write() += step)
            .child("+1"),
        
        span().child(("Value: ", move || count.get(), "!")),
    ))
}

pub fn app() -> impl IntoView {
    counter(0, 1)
}

pub fn main() {
    leptos::mount::mount_to_body(app);
}
