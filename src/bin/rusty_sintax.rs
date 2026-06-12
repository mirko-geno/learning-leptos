use leptos::{
    ev,
    html::{div, button, span, p},
    prelude::*,
};

pub fn counter(rsignal: ReadSignal<i32>, wsignal: WriteSignal<i32>, initial_value: i32, step: i32) -> impl IntoView {
    wsignal.set(initial_value);

    div().child((
        button()
            .on(ev::click, move |_| wsignal.set(0))
            .child("Clear"),

        button()
            .on(ev::click, move |_| *wsignal.write() -= step)
            .child("-1"),

        button()
            .on(ev::click, move |_| *wsignal.write() += step)
            .child("+1"),
        
        span().child(("Value: ", move || rsignal.get(), "!")),
    ))
}

pub fn algo(rvalue: ReadSignal<i32>, wvalue: WriteSignal<i32>) -> impl IntoView {
    Show(ShowProps {
        when: move || rvalue.get() > 5,
        fallback: (
            move || {
                leptos::logging::log!("fallback");
                wvalue.update(|value| if *value != 0 { *value += 2 });
                p().child("I will appear if value is 5 or lower")
            }
        ).into(),
        children: ToChildren::to_children(|| p().child("If value ever goes above 5, the only way to get below 5 again is by resseting")),
    })
}


pub fn app() -> impl IntoView {
    let (count, set_count) = signal(0i32);

    view! {
        { counter(count, set_count, 0, 1) }
        <br/>
        { algo(count, set_count) }
    }
}

pub fn main() {
    leptos::mount::mount_to_body(app);
}
