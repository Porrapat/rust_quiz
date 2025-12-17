use rust_quiz::quiz::quiz_bank;
use rust_quiz::engine::QuizState;

fn main() {
    let quizzes = quiz_bank();
    let state = QuizState::new();

    println!("Loaded {} quizzes", quizzes.len());

    if let Some(q) = state.current_quiz(&quizzes) {
        println!("First quiz title: {}", q.title);
    }
}