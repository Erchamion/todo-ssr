use html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use uuid::Uuid;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/todo-ssr.css" />

        // sets the document title
        <Title text="Lepto<Do>s" />

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

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

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Lepto<Do>s!"</h1>
        <div>
            <TodosComponent />
        </div>
    }
}

#[component]
fn TodosComponent() -> impl IntoView {
    let rows: Vec<Todo> = Vec::new();
    let (todos, set_todos) = create_signal(rows.clone());
    view! {
        <div>
            <CreateTodo setter=set_todos/>
        </div>
        <div>
            <ul>{move || todos.get().into_iter().map(|todo| view! { <TodoComponent value=todo setter=set_todos /> }).collect_view()}</ul>
        </div>
    }
}

#[component]
fn CreateTodo(setter: WriteSignal<Vec<Todo>>) -> impl IntoView {
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
fn TodoComponent(value: Todo, setter: WriteSignal<Vec<Todo>>) -> impl IntoView {
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
