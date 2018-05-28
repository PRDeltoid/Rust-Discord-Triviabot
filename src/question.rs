use std::fmt::{Display, Formatter, Result};

pub struct Question {
    pub prompt: String,
    pub answer: String,
    pub category: String,
    pub difficulty: String,
    pub answered: bool,
}

impl Question {
    #[allow(dead_code)]
    pub fn new(prompt: String, answer: String, category: String, difficulty: String) -> Question {
        Question {
            prompt: prompt,
            answer: answer,
            category: category,
            difficulty: difficulty,
            answered: false,
        }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "\tCat: {},\n\tDifficulty: {},\n\tQuestion: {},\n\tAnswer: {}\n",
            self.category, self.difficulty, self.prompt, self.answer
        )
    }
}
