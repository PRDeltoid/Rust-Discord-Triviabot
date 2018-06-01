use question::Question;
use std::fmt::{Display, Formatter, Result};

pub struct QuestionSet {
    questions: Vec<Question>,
    _number_of_questions: u32,
    current_question_number: usize,
}

impl QuestionSet {
    /// Generates a new QuestionSet item 
    pub fn new(questions: Vec<Question>, number_of_questions: u32) -> QuestionSet {
        QuestionSet {
            questions,
            _number_of_questions: number_of_questions,
            current_question_number: 0,
        }
    }

    /// Gets the current question as an Option
    pub fn get_current_question(&self) -> Option<&Question> {
        self.questions.get(self.current_question_number)
    }

    /// Changes the current question to the next question in the QuestionSet
    pub fn next_question(&mut self) {
        self.current_question_number += 1;
    }
}

impl Display for QuestionSet {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        println!("\n[");
        for question in &self.questions {
            println!("{}, ", question);
        }
        println!("\n]");

        Ok(())
    }
}
