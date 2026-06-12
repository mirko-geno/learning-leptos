use leptos::{
    ev,
    html::{div, button, span, p},
    prelude::*,
};

pub fn counter(
    rvalue: ReadSignal<i32>, wvalue: WriteSignal<i32>,
    rlock: ReadSignal<bool>, wlock: WriteSignal<bool>,
    initial_value: i32, step: i32
) -> impl IntoView {
    wvalue.set(initial_value);

    div().child((
        button()
            .on(ev::click, move |_| {
                wvalue.set(0);
                wlock.set(false);
            })
            .child("Clear"),

        button()
            .on(ev::click, move |_| {
                let result = rvalue.get() - step;
                match rlock.get() {
                    true => if result > 5 { *wvalue.write() = result},
                    false => *wvalue.write() = result
                }
            })
            .child("-1"),

        button()
            .on(ev::click, move |_| {
                *wvalue.write() += step;
                if rvalue.get() > 5 { *wlock.write() = true };
            })
            .child("+1"),
        
        span().child(("Value: ", move || rvalue.get(), "!")),
    ))
}

pub fn algo(rvalue: ReadSignal<i32>) -> impl IntoView {
    Show(ShowProps {
        when: move || rvalue.get() > 5,
        fallback: (move || p().child("I will appear if value is 5 or lower")).into(),
        children: ToChildren::to_children(|| p().child("If value ever goes above 5, the only way to get below 5 again is by resseting")),
    })
}


pub fn app() -> impl IntoView {
    let (count, set_count) = signal(0i32);
    let (lock, set_lock) = signal(false);

    view! {
        { counter(count, set_count, lock, set_lock, 0, 1) }
        <br/>
        { algo(count) }
    }
}

pub fn main() {
    leptos::mount::mount_to_body(app);
}
