use leptos::prelude::*;
use rust_quiz::quiz::quiz_bank;
use rust_quiz::engine::QuizState;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let quizzes = quiz_bank();
    let state = QuizState::new();
    let first_quiz = state.current_quiz(&quizzes);

    view! {
        <div style="font-family: sans-serif; padding: 2rem;">
            <h1>RustQuiz (Leptos)</h1>
            {if let Some(q) = first_quiz {
                view! {
                    <div>
                        <h2>{q.title}</h2>
                        <p>{q.question}</p>
                        <div style="margin-top: 1rem;">
                            {q.choices.iter().enumerate().map(|(i, choice)| {
                                let choice_text = choice.to_string();
                                view! {
                                    <div style="margin: 0.5rem 0;">
                                        <label>
                                            <input type="radio" name="answer" value={i}/>
                                            " " {choice_text}
                                        </label>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div>
                        <p>"No quiz available"</p>
                    </div>
                }.into_any()
            }}
        </div>
    }
}
