use crate::error_template::{AppError, ErrorTemplate};
use leptos::{ev::SubmitEvent, *};
use leptos_meta::*;
use leptos_router::*;
use leptos::logging::error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    first_name: String,
    last_name: String
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/frontend_demo.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // import the type for <input>
    use leptos::html::Input;

    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());


        // we'll use a NodeRef to store a reference to the input element
    // this will be filled when the element is created
    let first_name_input_element: NodeRef<Input> = create_node_ref();
    let last_name_input_element: NodeRef<Input> = create_node_ref();

    // fires when the form `submit` event happens
    // this will store the value of the <input> in our signal
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let first_name_value = first_name_input_element()
        //Demo 1: Expect
            .expect("<input> to exist")
            .value();
        
        let last_name_value = last_name_input_element()
            .expect("<input> to exist")
            .value();

        let new_person = Person {
            first_name: first_name_value,
            last_name: last_name_value
        };

        spawn_local(async move {
            //Demo 3: Result enum
            match add_name(new_person.clone()).await {
                Ok(_) => {
                    set_first_name(new_person.first_name);
                    set_last_name(new_person.last_name);
                },
                Err(_) => error!("Something went wrong!"),
            }
        });
    };

    view! {
        <div class="flex h-screen justify-center items-center bg-slate-900">
        <div class="flex flex-col justify-center p-5 border border-slate-300 rounded bg-slate-800">
            <form on:submit=on_submit class="flex flex-col text-white gap-5">
                <div class="flex flex-row gap-2 items-center">
                    <label class="text-center" for="first_name">First name</label>
                    <input id="first_name" type="text" value=first_name node_ref=first_name_input_element class="bg-slate-600 text-white rounded p-2" />
                </div>
                <div class="flex flex-row gap-2 items-center">
                    <label class="text-center" for="last_name">Last name</label>
                    <input id="last_name" type="text" value=last_name node_ref=last_name_input_element class="bg-slate-600 text-white rounded p-2" />
                </div>
                <input type="submit" value="Submit" class="bg-teal-500 text-white p-2 rounded cursor-pointer" />
            </form>
            <p class="p-2 text-white">"Name is: " {first_name}" "{last_name}</p>
            </div>
        </div>
        // Demo: The line below generates a Rust error when uncommented:
        // <input>
        // Why would that be?
    }
}

#[server(AddName, "/api")]
pub async fn add_name(person: Person) -> Result<(), ServerFnError> {
    Ok(println!("I am storing you name, {} {}, pinky promise!", person.first_name, person.last_name))
    // let mut conn = db().await?;

    // match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
    //     .bind(title)
    //     .execute(&mut conn)
    //     .await
    // {
    //     Ok(_row) => Ok(()),
    //     Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    // }
}
