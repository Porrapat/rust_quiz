use leptos::prelude::*;
use rust_quiz::quiz::quiz_bank;
use rust_quiz::engine::QuizState;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let quizzes = quiz_bank();
    let mut state = QuizState::new();
    let first_quiz = state.current_quiz(&quizzes);

    view! {
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1>RustQuiz (Leptos)</h1>
            // {first_quiz.title}
            // {match first_quiz {
            //     Some(q) => view! {
            //         <div>
            //             <h2>{q.title}</h2>
            //             <p>{q.question}</p>
            //         </div>
            //     }.into_view(),
            //     None => view! {
            //         <div>
            //             <p>No quiz available</p>
            //         </div>
            //     }.into_view(),
            // }}
        </div>
    }
}
