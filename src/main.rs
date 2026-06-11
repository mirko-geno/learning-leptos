use leptos::{
    ev::SubmitEvent,
    prelude::*
};
use reactive_stores::{Store, StoreFieldIterator};

#[component]
fn MyDynamicList(
    #[prop(default=5)]
    amount: usize
) -> impl IntoView {
    let counters = (0..amount)
        .map( |id| (id, ArcRwSignal::new(0)) )
        .collect::<Vec<_>>();

    let counters_signal = RwSignal::new(counters);

    let mut next_id = amount;

    let add_counter = move |_| {
        counters_signal.update(
            |counters| counters.push((next_id, ArcRwSignal::new(0)))
        );
        next_id += 1;
    };
    
    view! {
        <div>
            <button on:click=add_counter>
                "Add counter"
            </button>
            <ul>
                <For
                    each = move || counters_signal.get()
                    key = |(id, _)| *id
                    children = move |(id, counter)| {
                        let counter = RwSignal::from(counter);
                        view! {
                            <li>
                                <button on:click = move |_| { counter.update( |value| *value += 1 ) }>
                                    {counter}
                                </button>
                                <button on:click = move |_| {
                                    counters_signal.update(
                                        |counters| counters.retain(
                                            |(counter_id, _)| *counter_id != id 
                                        )
                                    )
                                }>
                                "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }

    
}


#[component]
fn DynamicList(
    #[prop(default=5)]
    initial_length: usize,
) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(0)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        let sig = ArcRwSignal::new(0);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                <For
                    each=move || counters.get()
                    key=|(id, _)| *id
                    children=move |(id, count)| {
                        // we can convert our ArcRwSignal to a Copy-able RwSignal 
                        // for nicer DX when moving it into the view
                        let count = RwSignal::from(count);
                        view! {
                            <li>
                                <button
                                    on:click=move |_| *count.write() += 1
                                >
                                    {count}
                                </button>
                                <button
                                    on:click=move |_| {
                                        set_counters
                                            .write()
                                            .retain(|(counter_id, _)| {
                                                counter_id != &id
                                            });
                                    }
                                >
                                    "Remove"
                                </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[component]
fn CustomButton(counter: RwSignal<i32>) -> impl IntoView {
    view! {
        <button
            on: click = move |_| counter.update( |value| *value += 1 )
        > {counter}
        </button>
    }
}


#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    #[prop(into)]
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <br/>
        <progress
            max = max
            value = progress
        />
        <br/>
    }

}

#[derive(Debug, Clone, Store)]
struct Data {
    #[store(key: String = |content| content.key.clone())]
    content: Vec<DatabaseEntryV2>
}

#[derive(Store, Debug, Clone)]
struct DatabaseEntryV2 {
    key: String,
    value: i32,
}

#[component]
fn ModifyDbV2() -> impl IntoView {
    let data = Store::new(
        Data { content: vec![
            DatabaseEntryV2 { key: "manzana".into(), value: 3 },
            DatabaseEntryV2 { key: "banana".into(), value: 23 },
            DatabaseEntryV2 { key: "durazno".into(), value: 41 },
            ]
        }
    );

    let double_value = move |_| {
        data.content().iter_unkeyed().for_each(
            |entry| entry.value().update(
                |value| *value *= 2
            )
        );
    };

    view! {
        <button on: click = double_value >
        "Duplicar valores"
        </button>
        <For
            each = move || data.content().iter_unkeyed()
            key = |entry| entry.key().get()
            children = |db| {
                view! { <p>{move || db.value().get()}</p> }
            }
            >
        </For> 
    }
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}



#[component]
fn ModifyDb() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry { key: "manzana".into(), value: 3 },
        DatabaseEntry { key: "banana".into(), value: 23 },
        DatabaseEntry { key: "durazno".into(), value: 41 },
    ]);

    let double_value = move |_| {
        set_data.update(
            |database| database.iter_mut().map(
                |entry| entry.value *= 2
            ).collect()
        );
        leptos::logging::log!("{:?}", data.get());
    };

    view! {
        <button on: click = double_value >
        "Duplicar valores"
        </button>
        <ForEnumerate
            each = move || data.get()
            key = |entry| entry.key.clone()
            children = move |index, _| {
                let value = Memo::new(move |_| {
                    data.with(
                        |db| db.get(index.get())
                        .map(|entry| entry.value)
                        .unwrap_or(0))
                    }
                );
                view! {
                    <p>{value}</p>
                }
            }
            >
        </ForEnumerate> 
    }
}

#[component]
fn CoolForms() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input type = "text"
            bind:value = (name, set_name)
        />
        <input type = "email"
            bind:value = email
        />
        <label>
            "Please send me lots of spam email."
            <input type="checkbox"
                bind:checked=spam_me
            />
        </label>
        <fieldset>
            <legend> "Favorite color" </legend>
            <label>
                "Red"
                <input
                    type="radio"
                    name="color"
                    value="red"
                    bind:group=favorite_color
                />  
            </label>
            <label>
                "Green"
                <input
                    type="radio"
                    name="color"
                    value="green"
                    bind:group=favorite_color
                />  
            </label>
            <label>
                "Blue"
                <input
                    type="radio"
                    name="color"
                    value="blue"
                    bind:group=favorite_color
                />  
            </label>
        </fieldset>
        <p>"Your favorite color is " {favorite_color} "."</p>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You’ll receive cool bonus content!"</p>
        </Show>
    }

}

#[component]
fn UncontrolledInput() -> impl IntoView {
    let (name, set_name) = signal("Uncontrolled".to_string());
    let input_element: NodeRef<leptos::html::Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();

        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit> // on_submit defined below
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn Paragraph() -> impl IntoView {
    let some_value = RwSignal::new("Write paragraph here!".to_string());

    view! {
        <textarea
            prop:value=move || some_value.get()
            on:input:target=move |ev| some_value.set(ev.target().value())
        >
            {some_value}
        </textarea>
    }
}

#[component]
fn Unique() -> impl IntoView {
    let (value, set_value) = signal(0i32);

    view! {
        <select
            on:change:target=move |ev| {
                set_value.set(ev.target().value().parse().unwrap());
            }
            prop:value=move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
        // a button that will cycle through the options
        <button on:click=move |_| set_value.update(|n| {
                if *n == 2 {
                    *n = 0;
                } else {
                    *n += 1;
                }
            }
        )>
            "Next Option"
        </button>
    }
}

#[component]
fn Selecter() -> impl IntoView {
    let (value, set_value) = signal(0i32);

    view! {
        <p>
            "Select an option"
        </p>
        <select
            on: change: target = move |ev| {
                set_value.set(ev.target().value().parse().unwrap());
            }
            prop: value = move || value.get().to_string()
        >
            <option value="0">"0"</option>
            <option value="1">"1"</option>
            <option value="2">"2"</option>
        </select>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.with( |counter| *counter * 2);

    let values = vec![0, 1, 2];

    let len = 5;
    let counters = (0..len).map(|_| RwSignal::new(0));
    let counters_buttons = { 
        counters.map( |counter| view! { <li> <CustomButton counter /> </li> })
        .collect_view()
    };

    let (name, set_name) = signal(String::new());
    
    view! {
        <UncontrolledInput/>
        <input type = "text"
        on: input: target = move |ev| {
            set_name.set(ev.target().value());
        }
        prop: value = name
        />
        <p>"Name is: " {name}</p>
        
        <CoolForms/>
        <Paragraph/>
        <br/>
        <Unique/>
        <br/>
        <Selecter/>
        <br/>

        <button 
            on:click = move |_| set_count.update( |counter| *counter += 1 ) 
            class:red = move || count.with( |counter| counter % 2 == 1) 
            style:background-color = move || format!("rgb({}, {}, 100)", count.get(), 100)
        > 
            "Click me: "
            {count}
        </button>
        <p>
            "Double count: "
            { double_count }
        </p>
        <button
            on:click = move |_| set_count.update( |counter| *counter += 10 )
            class:red = true
        >
            "Click for + 10"
        </button> 
        <ProgressBar progress = count />
        <ProgressBar progress = Signal::derive(double_count) />
        <p>{values.clone()}</p>
        <ul>
            { values.into_iter()
                .map( |n| view!{ <li>{n}</li> })
                .collect_view()
            }
        </ul>
        <ul>
            {counters_buttons}
        </ul>
        <DynamicList/>
        <br/>
        <MyDynamicList/>
        <br/>
        <ModifyDbV2/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}