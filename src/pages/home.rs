use leptos::*;

#[derive(Clone)]
struct Thing {
    pub id: usize,
    pub value: ReadSignal<String>,
}

impl Thing {
    pub fn new(id: usize, value: ReadSignal<String>) -> Self {
        Self { id, value }
    }
}

enum DisplayKind {
    Iter,
    For,
}

#[derive(Clone, Copy, derive_more::Deref)]
struct DynamicValue(ReadSignal<bool>);

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (display_kind, set_display_kind) = create_signal(DisplayKind::Iter);
    let (number, set_number) = create_signal(0);
    let number = leptos_use::signal_debounced(number, 100.0);
    let (value, set_value) = create_signal("Hi".to_string());
    let (dynamic_value, set_dynamic_value) = create_signal(true);
    provide_context(DynamicValue(dynamic_value));
    let (things, set_things) = create_signal(vec![]);

    create_effect(move |_| {
        number.with(|n| {
            let len = things.with(|things| things.len());
            if *n < len {
                set_things.update(|things| things.truncate(*n));
            } else if *n > len {
                let new_things = (len..*n).map(|id| Thing::new(id, value));
                set_things.update(|things| things.extend(new_things));
            }
        });
    });

    view! {
        <div style="position: sticky; top: 0; display: flex; flex-direction: column; gap: 5px; padding: 1rem; border-bottom: 2px solid white; background-color: rgb(33, 37, 41);">
            <div style="display: flex; gap: 15px;">
                <div style="display: flex; gap: 15px;">
                    <label>
                        <input
                            type="radio"
                            name="kind"
                            on:click=move|_| set_display_kind(DisplayKind::Iter)
                            checked=true
                        />
                        " Iter"
                    </label>
                    <label>
                        <input
                            type="radio"
                            name="kind"
                            on:click=move|_| set_display_kind(DisplayKind::For)
                        />
                        " For"
                    </label>
                </div>
                <div>
                    <label>
                        <input
                            type="checkbox"
                            on:input=move |e| set_dynamic_value(event_target_checked(&e))
                            prop:value=dynamic_value
                            checked=true
                        />
                        " Dynamic value"
                    </label>
                </div>
            </div>
            <div style="display: flex; gap: 15px;">
                <div>
                    <input
                        type="number"
                        on:input=move |e| {
                            let value = event_target_value(&e);
                            if let Ok(value) = value.parse::<usize>() {
                            set_number(value);
                            }
                        }
                        prop:value=number
                    />
                </div>
                <div>
                    <input
                        on:input=move |e| set_value(event_target_value(&e))
                        prop:value=value
                    />
                </div>
            </div>
        </div>
        <div
            style="display: flex; gap: 10px; flex-wrap: wrap; padding: 1rem 1rem 0 1rem;"
        >
            {move || display_kind.with(|display| match display {
                DisplayKind::Iter => view!{ <DisplayIter things />},
                DisplayKind::For => view!{ <DisplayFor things /> }
            })}
        </div>
    }
}

#[component]
fn DisplayIter(things: ReadSignal<Vec<Thing>>) -> impl IntoView {
    move || {
        things.with(|things| {
            things
                .iter()
                .map(|thing| {
                    view! { <Widget thing=thing.clone()/> }
                })
                .collect::<Vec<_>>()
        })
    }
}

#[component]
fn DisplayFor(things: ReadSignal<Vec<Thing>>) -> impl IntoView {
    view! {
        <For
            each=things
            key=|thing| thing.id
            let:thing
        >
            <Widget thing />
        </For>
    }
}

#[component]
fn Widget(thing: Thing) -> impl IntoView {
    let dynamic_value = expect_context::<DynamicValue>();
    view! {
        <div style="width: 5rem;">
            {move || dynamic_value.with(|dynamic| if *dynamic {
                thing.value.get()
            } else {
                "(static)".to_string()
            })}
        </div>
    }
}
