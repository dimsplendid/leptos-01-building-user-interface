use ev::MouseEvent;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    // leptos::mount_to_body(|| view! { <App/> })
    leptos::mount_to_body(App)
}



// 1. Basic Component

/// Shows progress toward a goal.
// using trait to generic
#[component]
fn ProgressBar1(
    /// The maximum value of the progress bar
    #[prop[default = 100]]
    max: u16,
    /// How much progress should be displayed.
    // progress: ReadSignal<i32>
    progress: impl Fn() -> i32 + 'static,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

/// Shows progress toward a goal.
// Using into Props
// this method can use count instead of count.get in stable rust
#[component]
fn ProgressBar2(
    /// The maximum value of the progress bar
    #[prop[default = 100]]
    max: u16,
    /// How much progress should be displayed.
    // progress: ReadSignal<i32>
    #[prop(into)]
    progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

// 2. iteration

// Static Views  with Vec<_>

#[component]
fn IterStatic() -> impl IntoView {
    let values = vec![1, 2, 3];
    view! {
        <p>{values.clone()}</p>
        // or wrap it them into <li>
        <ul>

            {values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}

        </ul>
    }
}

#[component]
fn StaticList() -> impl IntoView {
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));

    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

// Dynamic Rendering with the <For/> Component

/// A list of counters that allows you to add or
/// remove counters
#[component]
fn DynamicList(
    /// The number of counters to begin with
    length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by 1
    // each time we create a counter
    let mut next_counter_id = length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    let initial_counters = (0..length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,\
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        let sig = create_signal(next_counter_id + 1);
        // add thiscounter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list redering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or drived signal
                    // if it's not reative, just render a Vec<_> instead of <For/>
                    each=move || counters.get()
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all render
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| {
                                    set_count.update(|n| *n += 1)
                                }>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .update(|counters| {
                                            counters.retain(|(counter_id, _)| counter_id != &id)
                                        });
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

/// test data base entry
#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

/// need modified to truly reactive
#[component]
fn ComplexList() -> impl IntoView {
    // start with as set of three rows
    let (data, set_data) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 10,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 20,
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: 30,
        },
    ]);
    view! {
        <button on:click=move |_| {
            set_data
                .update(|data| {
                    for row in data {
                        row.value *= 2;
                    }
                });
            logging::log!("{:?}", data.get());
        }>"Update Values"</button>
        // iterate over the rows and display each value
        <For each=move || data.get() key=|state| state.key.clone() let:child>
            <p>{child.value}</p>
        </For>
    }
}

// Forms and Inputs

// Controlled Inputs
// Framework controls the state of the input element

#[component]
fn ControlledInput() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input
            type="text"
            on:input=move |ev| {
                set_name.set(event_target_value(&ev));
            }
        />

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        // prop:value=name
        // value=name // this would only initial value
        <p>"Name is: " {name}</p>
    }
}

// Uncontrolled Input
#[component]
fn UncontrolledInput() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    // NodeRef is a kind of reactive smart pointer: we can use it to access the
    // the underlying DOM node.
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = input_element
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implement `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call `HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name.set(value);
    };

    view! {
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}

// Special Cases: <textarea> and <select>

// <textarea> does not support a `value` attribute. Instead,
// it receives its value as a plain text node in its HTML children.
// But it does have value property:D
#[component]
fn TextArea() -> impl IntoView {
    let (article, set_article) = create_signal("Type Something!".to_string());
    view! {
        <textarea
            on:input=move |ev| {
                set_article.set(event_target_value(&ev));
            }

            prop:value=article
        ></textarea>
        <p>"The article is: " {article}</p>
    }
}

// <select> also does not have a `value` attribute, nor a `value` property.
// Instead, its value is determined by the `selected` attribute of its
// <option> field.

#[component]
fn Select() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());
    view! {
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value.set(new_value);
        }>

            <option value="A" selected=move || value.get() == "A">
                "A"
            </option>
            <option value="B" selected=move || value.get() == "B">
                "B"
            </option>
        </select>
        <p>"The selected is: " {value}</p>
    }
}

// if there are so many option, we can refactored this
#[component]
fn SelectRefactored() -> impl IntoView {
    let (value, set_value) = create_signal("B".to_string());
    view! {
        <select on:change=move |ev| {
            let new_value = event_target_value(&ev);
            set_value.set(new_value);
        }>
            // for now, the single value would expand to value=value
            // which is not consist with html,
            // in html, the single value would refer to value=true
            // this may change in the future.
            <SelectOption value is="A"/>
            <SelectOption value is="B"/>
            <SelectOption value is="C"/>
        </select>
        <p>"The selected is: " {value}</p>
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get() == is>
            {is}
        </option>
    }
}

// Control Flow

#[component]
fn ControlFlow() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;
    let message1 = move || {
        if is_odd() {
            Some("Ding Ding Ding")
        } else {
            None
        }
    };
    // the above can make is shorter
    let message2 = move || is_odd().then(|| "Ding Ding Ding");
    let message3 = move || match value.get() {
        0 => "Zero",
        1 => "One",
        _ if is_odd() => "Odd",
        _ => "Even",
    };
    view! {
        <button on:click=move |_| {
            set_value.update(|v| *v += 1)
        }>

            "Add 1"
        </button>
        <p>
            "because `&str` implements IntoView, so" <pre>"Fn() -> &str"</pre> "would reactive!"
            <br/> "It's just work www"
        </p>
        <p>

        {move || if is_odd() { "Odd" } else { "Even" }}
        
        </p>
        <p>{message1}</p>
        <p>{message2}</p>
        <p>{message3}</p>
        // to prevent expensive over re-rendering, we can use
        // <Show/> Instead of move || if
        <Show when=is_odd
            fallback=||view! { <p>"Even Steven"</p> }
        >
            <p>"Oddment"</p>
        </Show>
    }
}

#[component]
fn TypeConversions() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    let is_odd = move || value.get() & 1 == 1;
    view! {
        <button on:click=move |_| {
            set_value.update(|v| *v += 1)
        }>

            "Add 1"
        </button>
        {move || match is_odd() {
            true if value.get() == 1 => view! { <pre>"One"</pre> }.into_any(),
            false if value.get() == 2 => view! { <p>"Two"</p> }.into_any(),
            _ => view! { <textarea>{value.get()}</textarea> }.into_any(),
        }}
    }
}

// Error Handling

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    // without error handling, this would show nothing if the parser error happen.
    view! {
        <label>
            "Type a number (or not!)" <input on:input=on_input/>
            <p>"You entered " <strong>{value}</strong></p>
        </label>
    }
}

// handle error using <ErrorBoundary/>

#[component]
fn ErrorHandleNumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| set_value.set(event_target_value(&ev).parse::<i32>());

    // without error handling, this would show nothing if the parser error happen.
    view! {
        <label>
            "Type a number (or not!)" <input on:input=on_input/>
            // the fallback receives a signal containing current errors
            <ErrorBoundary fallback=|errs| {
                view! {
                    <div class="error">
                        <p>"Not a number! Errors: "</p>
                        // we can render a list of errors as strings, if we'd like
                        <ul>
                            {move || {
                                errs.get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}

                        </ul>
                    </div>
                }
            }>

                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

// Parent-Child Communication
// Parent -> Child is easy
// But how about Child -> Parent?

// 1. Pass a `WriteSignal`
// simply pass a `WriteSignal` from the parent down to the child, and update
// it in the child.
#[component]
fn Parent1() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view! {
        <p>"Toggled?" {toggled}</p>
        <Child1 setter=set_toggled/>
    }
}

#[component]
fn Child1(setter: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button
            on:click=move |_| setter.update(|v| *v = !*v)
        >
            "Toggle"
        </button>
    }
}

// 2. Use a Callback
#[component]
fn Parent2() -> impl IntoView {
    let (toggled, set_toggled) = create_signal(false);
    view!{
        <p>"Toggled?" {toggled}</p>
        <Child2 on_click=move |_| set_toggled.update(|value| *value = !*value)/>
    }
}

#[component]
fn Child2(
    #[prop(into)]
    on_click: Callable<MouseEvent>
) -> impl IntoView {
    view! {
        <button on:click=on_click>
            "Toggle"
        </button>
    }
}


/// Test Showing Componet
#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <Parent1/>
        </div>
    }
}