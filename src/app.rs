use html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use logging::log;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    log!("App starting");

    view! {
        <Stylesheet id="leptos" href="/pkg/todo-ssr.css" />

        // sets the document title
        <Title text="TODO" />

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
    id: u32,
    description: String,
    is_complete: bool,
}

impl Todo {
    fn new(desc: String) -> Todo {
        Todo {
            id: 0,
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
    let todos: Vec<Todo> = vec![
        Todo::new(String::from("Clean living room")),
        Todo::new(String::from("Wash car")),
        Todo::new(String::from("Clip dogs nails")),
        Todo::new(String::from("Learn rust")),
    ];
    view! {
        <div>
            <CreateTodo />
        </div>
        <div>
            <ul>{todos.into_iter().map(|todo| view! { <TodoComponent value=todo /> }).collect_view()}</ul>
        </div>
    }
}

#[component]
fn CreateTodo() -> impl IntoView {
    let (description, _) = create_signal("".to_string());
    let input_el: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let input_el = input_el().unwrap();
        let value = input_el.value();
        input_el.set_value("");
        log!("setting desc: {value}");
    };
    view! {
        <form on:submit=on_submit>
            <input type="text" value=description placeholder="create a new TODO!" node_ref=input_el />
            <input type="submit" value="Add" />
        </form>
    }
}

#[component]
fn TodoComponent(value: Todo) -> impl IntoView {
    view! {
        <li>
            <div class="inline-block">
                <input type="checkbox" />
                <span>{value.description}</span>
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
