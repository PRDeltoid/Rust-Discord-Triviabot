use std::fmt::{Display, Formatter, Result};
use rand::{thread_rng, Rng};


pub struct Question {
    pub prompt: String,
    pub answer: String,
    pub answer_letter: String,
    pub answer_prompt: String,
    pub incorrect_answers: Vec<String>,
    pub category: String,
    pub difficulty: String,
    pub answered: bool,
}

impl Question {
    pub fn set_answer_prompt(&mut self) {
        //Put all of our questions potential answers in a vector
        let mut answers = self.incorrect_answers.clone();
        answers.push(self.answer.clone());
        
        //Shuffle the vector
        let mut rng = thread_rng();
        rng.shuffle(&mut answers);

        //Search our shuffled vector for our correct answer.
        let mut index = 0;
        for (i, answer) in answers.iter().enumerate() {
            if answer == &self.answer {
                index = i
            }
        }
                
        //Store the correct answers letter for answer checking based on the index
        self.answer_letter = match index {
            0 => String::from("A"),
            1 => String::from("B"),
            2 => String::from("C"),
            3 => String::from("D"),
            _ => String::from("A"),
        };
        
        //Return a String of a formatted list of potential answers
        self.answer_prompt = format!("\nA. {}\nB. {}\nC. {}\nD. {}\n",
                answers[0],
                answers[1],
                answers[2],
                answers[3])
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "\tCat: {},\n\tDifficulty: {},\n\tQuestion: {},\n\tAnswer: {}\n",
            self.category, self.difficulty, self.prompt, self.answer
        )
    }
}
