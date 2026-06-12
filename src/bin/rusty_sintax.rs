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

impl ControlledCounter {
    pub fn clear(&mut self) {
        self.value = 0;
        self.lock = false;
    }
    pub fn add(&mut self, step: i32) {
        self.value += step;
        if self.value > 5 { self.lock = true }
    }
    pub fn try_sub(&mut self, step: i32) {
        let result = self.value - step;

        if !self.lock || result > 5 {
            self.value = result
        }
    }
}


pub fn counter(store_counter: Store<ControlledCounter>, step: i32) -> impl IntoView {
    div().child((
        button()
            .on(ev::click, move |_| store_counter.write().clear())
            .child("Clear"),

        button()
            .on(ev::click, move |_| store_counter.write().try_sub(step))
            .child("-1"),

        button()
            .on(ev::click, move |_| store_counter.write().add(step))
            .child("+1"),
        
        span().child(("Value: ", move || store_counter.value().get(), "!")),
    ))
}

pub fn algo(store_counter: Store<ControlledCounter>) -> impl IntoView {
    Show(ShowProps {
        when: move || store_counter.lock().get(),
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
