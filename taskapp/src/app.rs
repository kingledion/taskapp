use crate::model::Task;
use crate::server::{create_task, get_tasks};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/taskapp.css"/>

        // sets the document title
        <Title text="Task App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=TaskListPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn TaskCard(task: Task) -> impl IntoView {
    view! {
        <div class="task-card">
            <h3 class="task-title">{task.title}</h3>
            <p class="task-description">{task.description}</p>
        </div>
    }
}

#[component]
fn NewTaskForm(on_created: Action<(String, String), Result<Task, ServerFnError>>) -> impl IntoView {
    let (show_form, set_show_form) = signal(false);
    let (title, set_title) = signal(String::new());
    let (description, set_description) = signal(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let t = title.get();
        let d = description.get();
        if !t.is_empty() {
            on_created.dispatch((t, d));
            set_title.set(String::new());
            set_description.set(String::new());
            set_show_form.set(false);
        }
    };

    view! {
        <div class="new-task-section">
            {move || {
                if show_form.get() {
                    view! {
                        <form class="new-task-form" on:submit=on_submit>
                            <input
                                type="text"
                                placeholder="Task title"
                                prop:value=move || title.get()
                                on:input=move |ev| set_title.set(event_target_value(&ev))
                            />
                            <textarea
                                placeholder="Task description"
                                prop:value=move || description.get()
                                on:input=move |ev| set_description.set(event_target_value(&ev))
                            />
                            <div class="form-buttons">
                                <button type="submit" class="submit-btn">"Create Task"</button>
                                <button
                                    type="button"
                                    class="cancel-btn"
                                    on:click=move |_| set_show_form.set(false)
                                >
                                    "Cancel"
                                </button>
                            </div>
                        </form>
                    }.into_any()
                } else {
                    view! {
                        <button
                            class="new-task-btn"
                            on:click=move |_| set_show_form.set(true)
                        >
                            "New Task"
                        </button>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[component]
fn TaskListPage() -> impl IntoView {
    let tasks_resource = Resource::new(|| (), |_| get_tasks());

    let create_action = Action::new(|(title, description): &(String, String)| {
        let title = title.clone();
        let description = description.clone();
        async move { create_task(title, description).await }
    });

    // Refetch tasks when a new task is created
    Effect::new(move || {
        if create_action.value().get().is_some() {
            tasks_resource.refetch();
        }
    });

    view! {
        <div class="task-list-page">
            <h1>"Tasks"</h1>
            <NewTaskForm on_created=create_action />
            <div class="task-list">
                <Suspense fallback=move || view! { <p>"Loading tasks..."</p> }>
                    {move || {
                        tasks_resource.get().map(|result| {
                            match result {
                                Ok(tasks) => {
                                    if tasks.is_empty() {
                                        view! { <p class="no-tasks">"No tasks yet. Create one!"</p> }.into_any()
                                    } else {
                                        view! {
                                            <For
                                                each=move || tasks.clone()
                                                key=|task| task.id
                                                children=move |task| view! { <TaskCard task=task /> }
                                            />
                                        }.into_any()
                                    }
                                }
                                Err(e) => view! { <p class="error">"Error loading tasks: " {e.to_string()}</p> }.into_any()
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
