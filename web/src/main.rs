use leptos::prelude::*;
use rust_quiz::quiz::quiz_bank;
use rust_quiz::engine::QuizState;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    mount_to_body(|| view! { <App/> })
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum GameMode {
    NotSelected,
    Random5,
    AllQuestions,
}

#[component]
fn App() -> impl IntoView {
    let all_quizzes = StoredValue::new(quiz_bank());
    let _state = QuizState::new();
    
    let (game_mode, set_game_mode) = signal(GameMode::NotSelected);
    let (quiz_list, set_quiz_list) = signal(Vec::new());
    let (current_index, set_current_index) = signal(0usize);
    let (selected_answer, set_selected_answer) = signal(None::<usize>);
    let (feedback, set_feedback) = signal(None::<(bool, String)>);
    let (score, set_score) = signal(0usize);

    view! {
        <div class="container py-5">
            <div class="text-center mb-4">
                <h1 class="display-4"><i class="bi bi-gear-fill"></i> " Rust Quiz"</h1>
            </div>
            
            {move || {
                match game_mode.get() {
                    GameMode::NotSelected => {
                        view! {
                            <div class="row justify-content-center">
                                <div class="col-md-6">
                                    <div class="card p-4">
                                        <h2 class="text-center mb-4">"Choose Your Quiz Mode"</h2>
                                        <div class="d-grid gap-3">
                                            <button 
                                                class="btn btn-primary btn-lg"
                                                on:click=move |_| {
                                                    let mut rng = thread_rng();
                                                    let mut selected: Vec<_> = all_quizzes.get_value().clone();
                                                    selected.shuffle(&mut rng);
                                                    selected.truncate(5);
                                                    set_quiz_list.set(selected);
                                                    set_game_mode.set(GameMode::Random5);
                                                    set_current_index.set(0);
                                                    set_selected_answer.set(None);
                                                    set_feedback.set(None);
                                                    set_score.set(0);
                                                }
                                            >
                                                <i class="bi bi-dice-5"></i> " Random 5 Questions"
                                            </button>
                                            <button 
                                                class="btn btn-outline-primary btn-lg"
                                                on:click=move |_| {
                                                    set_quiz_list.set(all_quizzes.get_value().clone());
                                                    set_game_mode.set(GameMode::AllQuestions);
                                                    set_current_index.set(0);
                                                    set_selected_answer.set(None);
                                                    set_feedback.set(None);
                                                    set_score.set(0);
                                                }
                                            >
                                                <i class="bi bi-journal-text"></i> " All " {all_quizzes.get_value().len()} " Questions"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    }
                    _ => {
                        let quizzes = quiz_list.get();
                        let index = current_index.get();
                        if index < quizzes.len() {
                            let quiz = &quizzes[index];
                            let total = quizzes.len();
                            let current = index + 1;
                            let quiz_title = quiz.title;
                            let quiz_question = quiz.question;
                            let quiz_code = quiz.code;
                            let quiz_choices = quiz.choices.clone();
                            let quiz_correct = quiz.correct;
                            let quiz_explanation = quiz.explanation;
                            
                            view! {
                                <div class="card p-4">
                                    <div class="d-flex justify-content-between align-items-center mb-3">
                                        <div>
                                            <span class="badge bg-secondary me-2">
                                                "Question " {current} " of " {total}
                                            </span>
                                            <span class="badge bg-primary">
                                                "Score: " {score.get()} "/" {index}
                                            </span>
                                        </div>
                                        <button 
                                            class="btn btn-outline-secondary btn-sm"
                                            on:click=move |_| {
                                                set_game_mode.set(GameMode::NotSelected);
                                                set_quiz_list.set(Vec::new());
                                                set_current_index.set(0);
                                                set_selected_answer.set(None);
                                                set_feedback.set(None);
                                                set_score.set(0);
                                            }
                                        >
                                            <i class="bi bi-arrow-left"></i> " Back to Menu"
                                        </button>
                                    </div>
                                    
                                    <h2 class="h4 mb-3">{quiz_question}</h2>
                                    
                                    {move || {
                                        if let Some(code) = quiz_code {
                                            view! {
                                                <pre class="mb-3"><code>{code}</code></pre>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                    
                                    <div class="mb-3">
                                        {quiz_choices.iter().enumerate().map(|(i, choice)| {
                                            let choice_text = choice.to_string();
                                            view! {
                                                <div class="form-check mb-2 p-3 border rounded">
                                                    <input 
                                                        class="form-check-input"
                                                        type="radio" 
                                                        name="answer" 
                                                        id=format!("choice-{}", i)
                                                        value={i}
                                                        disabled=move || feedback.get().is_some()
                                                        prop:checked=move || selected_answer.get() == Some(i)
                                                        on:change=move |_| set_selected_answer.set(Some(i))
                                                    />
                                                    <label class="form-check-label w-100" for=format!("choice-{}", i)>
                                                        {choice_text}
                                                    </label>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                    
                                    {move || {
                                        if let Some((is_correct, explanation)) = feedback.get() {
                                            view! {
                                                <div class=format!("alert alert-{} d-flex align-items-start", 
                                                    if is_correct { "success" } else { "danger" })
                                                >
                                                    <i class=format!("bi bi-{} me-2 fs-4", 
                                                        if is_correct { "check-circle-fill" } else { "x-circle-fill" })
                                                    ></i>
                                                    <div>
                                                        <h4 class="alert-heading">
                                                            {if is_correct { "Correct!" } else { "Incorrect" }}
                                                        </h4>
                                                        <p class="mb-0">{explanation}</p>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                    
                                    <div class="d-flex gap-2 mt-3">
                                        {move || {
                                            let feedback_value = feedback.get();
                                            if feedback_value.is_none() {
                                                view! {
                                                    <>
                                                        <button 
                                                            class="btn btn-primary"
                                                            disabled=move || selected_answer.get().is_none()
                                                            on:click=move |_| {
                                                                if let Some(answer_idx) = selected_answer.get() {
                                                                    let correct = quiz_correct == answer_idx;
                                                                    if correct {
                                                                        set_score.update(|s| *s += 1);
                                                                    }
                                                                    set_feedback.set(Some((correct, quiz_explanation.to_string())));
                                                                }
                                                            }
                                                        >
                                                            <i class="bi bi-check-lg"></i> " Submit Answer"
                                                        </button>
                                                        <button 
                                                            class="btn btn-outline-secondary"
                                                            disabled=move || selected_answer.get().is_none()
                                                            on:click=move |_| {
                                                                set_selected_answer.set(None);
                                                            }
                                                        >
                                                            <i class="bi bi-arrow-counterclockwise"></i> " Clear"
                                                        </button>
                                                    </>
                                                }.into_any()
                                            } else if current_index.get() < total - 1 {
                                                view! {
                                                    <button 
                                                        class="btn btn-primary"
                                                        on:click=move |_| {
                                                            set_current_index.update(|i| *i += 1);
                                                            set_selected_answer.set(None);
                                                            set_feedback.set(None);
                                                        }
                                                    >
                                                        "Next Question " <i class="bi bi-arrow-right"></i>
                                                    </button>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div class="w-100">
                                                        <div class="alert alert-info text-center">
                                                            <h3 class="alert-heading">
                                                                <i class="bi bi-trophy-fill"></i> " Quiz Complete!"
                                                            </h3>
                                                            <p class="fs-2 mb-0">
                                                                "Final Score: " {score.get()} "/" {total}
                                                            </p>
                                                        </div>
                                                        <div class="d-flex gap-2 justify-content-center">
                                                            <button 
                                                                class="btn btn-primary"
                                                                on:click=move |_| {
                                                                    set_game_mode.set(GameMode::NotSelected);
                                                                    set_quiz_list.set(Vec::new());
                                                                    set_current_index.set(0);
                                                                    set_selected_answer.set(None);
                                                                    set_feedback.set(None);
                                                                    set_score.set(0);
                                                                }
                                                            >
                                                                <i class="bi bi-house-fill"></i> " Back to Menu"
                                                            </button>
                                                            <button 
                                                                class="btn btn-outline-primary"
                                                                on:click=move |_| {
                                                                    let current_mode = game_mode.get();
                                                                    if current_mode == GameMode::Random5 {
                                                                        let mut rng = thread_rng();
                                                                        let mut selected: Vec<_> = all_quizzes.get_value().clone();
                                                                        selected.shuffle(&mut rng);
                                                                        selected.truncate(5);
                                                                        set_quiz_list.set(selected);
                                                                    } else {
                                                                        set_quiz_list.set(all_quizzes.get_value().clone());
                                                                    }
                                                                    set_current_index.set(0);
                                                                    set_selected_answer.set(None);
                                                                    set_feedback.set(None);
                                                                    set_score.set(0);
                                                                }
                                                            >
                                                                <i class="bi bi-arrow-repeat"></i> " Play Again"
                                                            </button>
                                                        </div>
                                                    </div>
                                                }.into_any()
                                            }
                                        }}
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="alert alert-warning">
                                    <i class="bi bi-exclamation-triangle"></i> " No quiz available"
                                </div>
                            }.into_any()
                        }
                    }
                }
            }}
        </div>
    }
}
