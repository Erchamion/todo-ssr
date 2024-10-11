use leptos::html::Input;
use leptos::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Todo {
    id: Uuid,
    description: String,
    is_complete: bool,
}

impl Todo {
    fn new(desc: String) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            description: desc,
            is_complete: false,
        }
    }
}

#[component]
pub fn Todos() -> impl IntoView {
    let rows: Vec<Todo> = Vec::new();
    let (todos, set_todos) = create_signal(rows.clone());
    view! {
        <div>
            <CreateTodo setter=set_todos/>
        </div>
        <div>
            <ul>{move || todos.get().into_iter().map(|todo| view! { <Todo value=todo setter=set_todos /> }).collect_view()}</ul>
        </div>
    }
}

#[component]
pub fn CreateTodo(setter: WriteSignal<Vec<Todo>>) -> impl IntoView {
    let (description, _) = create_signal("".to_string());
    let input_el: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let input_el = input_el().unwrap();
        let value = input_el.value();
        setter.update(|todos| {
            todos.push(Todo::new(value.clone()));
        });
        input_el.set_value("");
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" value=description placeholder="create a new TODO!" node_ref=input_el />
            <input type="submit" value="Add" />
        </form>
    }
}

#[component]
pub fn Todo(value: Todo, setter: WriteSignal<Vec<Todo>>) -> impl IntoView {
    let description = value.description.clone();
    let done = value.clone();
    let remove = value.clone();
    view! {
        <li>
            <div class="inline-block">
                <span>{description}</span>
                <button on:click=move |_| setter.update(|todos| todos.retain(|t| t != &done)) >done</button>
                <button on:click=move |_| setter.update(|todos| todos.retain(|t| t != &remove)) >remove</button>
            </div>
        </li>
    }
}
