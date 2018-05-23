use question::Question;
use std::fmt::{Formatter, Display, Result};

pub struct QuestionSet {
    questions: Vec<Question>,
    number_of_questions: u32,
    current_question_number: usize,
}

impl QuestionSet {
    pub fn new(questions: Vec<Question>, number_of_questions: u32) -> QuestionSet {
        QuestionSet {
            questions: questions,
            number_of_questions: number_of_questions,
            current_question_number: 0,
        }
    }

    pub fn get_current_question(&self) -> &Question {
        &self.questions[self.current_question_number]
    }

    pub fn next_question(&mut self) {
        self.current_question_number += 1; //self.current_question_number + 1;
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
