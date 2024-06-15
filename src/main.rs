use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }

            // class:red=move || count.get() % 2 == 1
            // style="position: absolute"
            // style:left=move || format!("{}px", count.get() + 100)
            // style:background-color=move || format!("rgb({},{},100)", count.get(), 100)
            style:max-width="400px"
        >
            // style=("--columns", count.get())
            "Click me: "
            {move || count.get()}
        </button>
        <br/>
        // <progress
        // max="50"
        // value= move ||count.get()
        // />
        // <progress
        // max="50"
        // value= move ||count.get() * 2
        // />
        <ProgressBar1 max=16 progress=move || count.get()/>
        <hr/>
        <ProgressBar2 progress=double_count/>
        <br/>
        <ProgressBar2 max=32 progress=count/>
        <div>
            <IterStatic/>
        </div>
        <div>
            <StaticList/>
        </div>
        <div>
            <DynamicList length=5/>
        </div>
    }
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
