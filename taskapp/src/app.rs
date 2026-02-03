use crate::model::Task;
use crate::server::{complete_task, create_task, delete_task, get_tasks};
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
fn TaskCard(
    task: Task,
    on_complete: Action<(i32, bool), Result<Task, ServerFnError>>,
    on_delete: Action<i32, Result<(), ServerFnError>>,
) -> impl IntoView {
    let task_id = task.id;
    let is_completed = task.is_completed();

    view! {
        <div class="task-card" class:completed=is_completed>
            <div class="task-checkbox">
                <input
                    type="checkbox"
                    checked=is_completed
                    on:change=move |ev| {
                        let checked = event_target_checked(&ev);
                        on_complete.dispatch((task_id, checked));
                    }
                />
            </div>
            <div class="task-content">
                <h3 class="task-title">{task.title}</h3>
                <p class="task-description">{task.description}</p>
            </div>
            <button
                class="delete-btn"
                title="Delete task"
                on:click=move |_| { on_delete.dispatch(task_id); }
            >
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
                    <line x1="10" y1="11" x2="10" y2="17"></line>
                    <line x1="14" y1="11" x2="14" y2="17"></line>
                </svg>
            </button>
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

    let complete_action = Action::new(|(id, completed): &(i32, bool)| {
        let id = *id;
        let completed = *completed;
        async move { complete_task(id, completed).await }
    });

    let delete_action = Action::new(|id: &i32| {
        let id = *id;
        async move { delete_task(id).await }
    });

    // Refetch tasks when any action completes
    Effect::new(move || {
        if create_action.value().get().is_some() {
            tasks_resource.refetch();
        }
    });

    Effect::new(move || {
        if complete_action.value().get().is_some() {
            tasks_resource.refetch();
        }
    });

    Effect::new(move || {
        if delete_action.value().get().is_some() {
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
                                                children=move |task| view! {
                                                    <TaskCard
                                                        task=task
                                                        on_complete=complete_action
                                                        on_delete=delete_action
                                                    />
                                                }
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
