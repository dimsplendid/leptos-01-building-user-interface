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
            // style=("--columns", count.get())
        >
            "Click me: " {move || count.get()}
        </button>
        <br/>
        // <progress
        //     max="50"
        //     value= move ||count.get()
        // />
        // <progress
        //     max="50"
        //     value= move ||count.get() * 2
        // />
        <ProgressBar1 max=16 progress=move || count.get() />
        <hr/>
        <ProgressBar2 progress=double_count/>
        <br/>
        <ProgressBar2 max=32 progress=count/>
        <div>
            <IterStatic/>
        </div>
        <div>
            <IterStaticDynamicItem/>
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
    progress: impl Fn() -> i32 + 'static
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
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
    progress: Signal<i32>
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

// 2. iteration

// Static Views  with Vec<_>

#[component]
fn IterStatic() -> impl IntoView {
    let values = vec![1,2,3];
    view! {
        // this would render 012
        <p>{values.clone()}</p>
        // or wrap it them into <li>
        <ul>
            {
                values.into_iter()
                    .map(|n| view! { <li>{n}</li>})
                    .collect::<Vec<_>>()
            }
        </ul>
    }
}

#[component]
fn IterStaticDynamicItem() -> impl IntoView {
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| create_signal(idx));
    
    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

// Dynamic Rendering with the <For/> Component

