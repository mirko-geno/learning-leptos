use gloo_timers::future::TimeoutFuture;
use leptos::{
    html::p,
    prelude::*
};

// IT is extremely important to note that while an async function is 
// still working, LocalResource.get() won't return None, but it's last value
// that is, Some(previous_value). Its value is only None on creation
// Suspense allows access to the waiting state as a fallback



// Here we define an async function
// This could be anything: a network request, database read, etc.
// Here, we just multiply a number by 10
async fn load_data(value: i32) -> i32 {
    // fake a one-second delay
    TimeoutFuture::new(5_000).await;
    value * 10
}

fn value_or_wait(async_data: LocalResource<i32>) -> impl IntoView {
    Suspense(SuspenseProps {
        fallback: (move || p().child("Loading...")).into(),
        children: ToChildren::to_children(move || p().child(move || async_data.get())),
    })
}

#[component]
pub fn App() -> impl IntoView {
    // this count is our synchronous, local state
    let (count, set_count) = signal(0);

    // tracks `count`, and reloads by calling `load_data`
    // whenever it changes
    let async_data = LocalResource::new(move || load_data(count.get()));

    // a resource will only load once if it doesn't read any reactive data
    let stable = LocalResource::new(|| load_data(1));

    // we can access the resource values with .get()
    // this will reactively return None before the Future has resolved
    // and update to Some(T) when it has resolved
    let async_result = move || {
        async_data
            .get()
            .map(|value| format!("Server returned {value:?}"))
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading...".into())
    };

    view! {
        <button
            on:click=move |_| *set_count.write() += 1
        >
            "Click me"
        </button>
        <p>
            <code>"stable"</code>": " {move || stable.get()}
        </p>
        <p>
            <code>"count"</code>": " {count}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
        </p>
        <p>"Future State: "</p>
        <Suspense
            fallback=move || view! { <p>"Loading..."</p> }
        >
            {move || async_data.get()}
        </Suspense>

        {value_or_wait(async_data)}
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}