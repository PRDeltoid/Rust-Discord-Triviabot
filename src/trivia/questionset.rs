use trivia::question::Question;
use std::fmt::{Formatter, Display, Result};

pub struct QuestionSet {
    questions: Vec<Question>,
    _number_of_questions: u32,
    _current_question_number: u32,
}

impl QuestionSet {
    pub fn new(number_of_questions: u32) -> QuestionSet {
        QuestionSet {
            questions: Vec::new(),
            _number_of_questions: number_of_questions,
            _current_question_number: 0,
        }
    }

    pub fn get_current_question(&mut self) -> Question {
        match self.questions.pop() {
            Some(question) => question,
            None => Question::new("".to_string(), "".to_string(), "".to_string(), "".to_string()),
        }
    }

    pub fn generate_questions(&mut self) {

    }
}

impl Display for QuestionSet {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        print!("\n[\n");
        for question in self.questions.iter() {
            print!("{}, ", question);
        }
        print!("\n]");

        Ok(())
    }
}
