use leptos::{
    ev,
    html::{div, button, span, p},
    prelude::*,
};
use reactive_stores::Store;


#[derive(Store)]
pub struct ControlledCounter {
    value: i32,
    lock: bool
}
impl Default for ControlledCounter {
    fn default() -> Self {
        Self { value: 0, lock: false }
    }
}

pub fn counter(store_counter: Store<ControlledCounter>, step: i32) -> impl IntoView {
    div().child((
        button()
            .on(ev::click, move |_| {
                store_counter.value().set(0);
                store_counter.lock().set(false);
            })
            .child("Clear"),

        button()
            .on(ev::click, move |_| {
                let result = store_counter.value().get() - step;
                match store_counter.lock().get() {
                    true => if result > 5 { store_counter.value().set(result)},
                    false => store_counter.value().set(result)
                }
            })
            .child("-1"),

        button()
            .on(ev::click, move |_| {
                *store_counter.value().write() += step;
                if store_counter.value().get() > 5 { store_counter.lock().set(true) };
            })
            .child("+1"),
        
        span().child(("Value: ", move || store_counter.value().get(), "!")),
    ))
}

pub fn algo(store_counter: Store<ControlledCounter>) -> impl IntoView {
    Show(ShowProps {
        when: move || store_counter.value().get() > 5,
        fallback: (move || p().child("I will appear if value is 5 or lower")).into(),
        children: ToChildren::to_children(|| p().child("If value ever goes above 5, the only way to get below 5 again is by resseting")),
    })
}


pub fn app() -> impl IntoView {
    let store_counter = Store::new(ControlledCounter::default());

    view! {
        { counter(store_counter, 1) }
        <br/>
        { algo(store_counter) }
    }
}

pub fn main() {
    leptos::mount::mount_to_body(app);
}
