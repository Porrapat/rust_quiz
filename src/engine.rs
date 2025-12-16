#![allow(dead_code)]

use crate::quiz::Quiz;

#[derive(Debug)]
pub struct QuizState {
    pub current: usize,
    pub score: usize,
    pub finished: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnswerResult {
    Correct,
    Wrong,
    Finished,
}

impl QuizState {
    pub fn new() -> Self {
        Self {
            current: 0,
            score: 0,
            finished: false,
        }
    }

    pub fn current_quiz<'a>(&self, quizzes: &'a [Quiz]) -> Option<&'a Quiz> {
        if self.finished {
            None
        } else {
            quizzes.get(self.current)
        }
    }

    pub fn answer(&mut self, quizzes: &[Quiz], choice: usize) -> AnswerResult {
        if self.finished {
            return AnswerResult::Finished;
        }

        let quiz = match quizzes.get(self.current) {
            Some(q) => q,
            None => {
                self.finished = true;
                return AnswerResult::Finished;
            }
        };

        let result = if quiz.correct == choice {
            self.score += 1;
            AnswerResult::Correct
        } else {
            AnswerResult::Wrong
        };

        self.current += 1;

        if self.current >= quizzes.len() {
            self.finished = true;
        }

        result
    }

    pub fn progress(&self, total: usize) -> (usize, usize) {
        (self.current.min(total), total)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::quiz::{Quiz, Level};

    fn mock_quiz(correct: usize) -> Quiz {
        Quiz {
            id: 1,
            title: "Test",
            question: "Q?",
            code: None,
            choices: vec!["A", "B", "C"],
            correct,
            explanation: "Because.",
            tags: vec![],
            level: Level::Intro,
        }
    }

    #[test]
    fn test_correct_answer_increases_score() {
        let quizzes = vec![mock_quiz(1)];
        let mut state = QuizState::new();

        let result = state.answer(&quizzes, 1);

        assert_eq!(result, AnswerResult::Correct);
        assert_eq!(state.score, 1);
        assert!(state.finished);
    }

    #[test]
    fn test_wrong_answer_does_not_increase_score() {
        let quizzes = vec![mock_quiz(2)];
        let mut state = QuizState::new();

        let result = state.answer(&quizzes, 0);

        assert_eq!(result, AnswerResult::Wrong);
        assert_eq!(state.score, 0);
        assert!(state.finished);
    }

    #[test]
    fn test_progress_tracking() {
        let quizzes = vec![mock_quiz(0), mock_quiz(1)];
        let mut state = QuizState::new();

        state.answer(&quizzes, 0);
        let (current, total) = state.progress(quizzes.len());

        assert_eq!(current, 1);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_reset_state() {
        let quizzes = vec![mock_quiz(0)];
        let mut state = QuizState::new();

        state.answer(&quizzes, 0);
        state.reset();

        assert_eq!(state.current, 0);
        assert_eq!(state.score, 0);
        assert!(!state.finished);
    }
}
