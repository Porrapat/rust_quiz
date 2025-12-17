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
        <div>
            <h1>"Rust Quiz"</h1>
            {move || {
                match game_mode.get() {
                    GameMode::NotSelected => {
                        view! {
                            <div style="text-align: center; margin-top: 2rem;">
                                <h2 style="color: var(--rust-orange); margin-bottom: 2rem;">"Choose Your Quiz Mode"</h2>
                                <div style="display: flex; flex-direction: column; gap: 1rem; max-width: 400px; margin: 0 auto;">
                                    <button 
                                        class="btn btn-primary"
                                        style="padding: 1.5rem; font-size: 1.1rem;"
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
                                        "🎲 Random 5 Questions"
                                    </button>
                                    <button 
                                        class="btn btn-outline"
                                        style="padding: 1.5rem; font-size: 1.1rem;"
                                        on:click=move |_| {
                                            set_quiz_list.set(all_quizzes.get_value().clone());
                                            set_game_mode.set(GameMode::AllQuestions);
                                            set_current_index.set(0);
                                            set_selected_answer.set(None);
                                            set_feedback.set(None);
                                            set_score.set(0);
                                        }
                                    >
                                        {format!("📚 All {} Questions", all_quizzes.get_value().len())}
                                    </button>
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
                                <div>
                                    <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                        <p style="color: var(--rust-copper); font-weight: 600;">
                                            "Question " {current} " of " {total} " | Score: " {score.get()} "/" {index}
                                        </p>
                                        <button 
                                            class="btn btn-outline"
                                            style="padding: 0.5rem 1rem; font-size: 0.9rem;"
                                            on:click=move |_| {
                                                set_game_mode.set(GameMode::NotSelected);
                                                set_quiz_list.set(Vec::new());
                                                set_current_index.set(0);
                                                set_selected_answer.set(None);
                                                set_feedback.set(None);
                                                set_score.set(0);
                                            }
                                        >
                                            "← Back to Menu"
                                        </button>
                                    </div>
                                    <h2>{quiz_title}</h2>
                                    <p>{quiz_question}</p>
                                    
                                    {move || {
                                        if let Some(code) = quiz_code {
                                            view! {
                                                <pre style="background: var(--rust-bg); padding: 1rem; border-radius: var(--radius-md); border-left: 4px solid var(--rust-orange); overflow-x: auto; margin: 1rem 0;">
                                                    <code style="color: var(--rust-text); font-family: 'Courier New', monospace; font-size: 0.95rem;">
                                                        {code}
                                                    </code>
                                                </pre>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                    
                                    <div>
                                        {quiz_choices.iter().enumerate().map(|(i, choice)| {
                                            let choice_text = choice.to_string();
                                            view! {
                                                <div>
                                                    <label>
                                                        <input 
                                                            type="radio" 
                                                            name="answer" 
                                                            value={i}
                                                            disabled=move || feedback.get().is_some()
                                                            prop:checked=move || selected_answer.get() == Some(i)
                                                            on:change=move |_| set_selected_answer.set(Some(i))
                                                        />
                                                        " " {choice_text}
                                                    </label>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                    
                                    {move || {
                                        if let Some((is_correct, explanation)) = feedback.get() {
                                            view! {
                                                <div style=format!(
                                                    "margin-top: 1.5rem; padding: 1rem; border-radius: var(--radius-md); background: {}; border: 2px solid {};",
                                                    if is_correct { "rgba(93, 155, 93, 0.1)" } else { "rgba(194, 64, 64, 0.1)" },
                                                    if is_correct { "var(--success)" } else { "var(--error)" }
                                                )>
                                                    <h3 style=format!(
                                                        "color: {}; margin-bottom: 0.5rem;",
                                                        if is_correct { "var(--success)" } else { "var(--error)" }
                                                    )>
                                                        {if is_correct { "✓ Correct!" } else { "✗ Incorrect" }}
                                                    </h3>
                                                    <p style="color: var(--rust-steel);">{explanation}</p>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                    
                                    <div style="margin-top: 1.5rem; display: flex; gap: 1rem;">
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
                                                            "Submit Answer"
                                                        </button>
                                                        <button 
                                                            class="btn btn-outline"
                                                            disabled=move || selected_answer.get().is_none()
                                                            on:click=move |_| {
                                                                set_selected_answer.set(None);
                                                            }
                                                        >
                                                            "Clear Answer"
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
                                                        "Next Question"
                                                    </button>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <div>
                                                        <p style="font-size: 1.2rem; font-weight: 600; color: var(--rust-orange); margin-bottom: 1rem;">
                                                            "Quiz Complete! Final Score: " {score.get()} "/" {total}
                                                        </p>
                                                        <div style="display: flex; gap: 1rem;">
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
                                                                "Back to Menu"
                                                            </button>
                                                            <button 
                                                                class="btn btn-outline"
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
                                                                "Play Again"
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
                                <div>
                                    <p>"No quiz available"</p>
                                </div>
                            }.into_any()
                        }
                    }
                }
            }}
        </div>
    }
}
