use std::fmt::{Display, Formatter, Result};
use rand::{thread_rng, Rng};
use htmlescape::decode_html;


pub struct Question {
    pub prompt: String,
    pub answer: String,
    pub answer_letter: String,
    pub answer_prompt: String,
    pub category: String,
    pub difficulty: String,
    pub answered: bool,
}

impl Question {

    pub fn new(prompt: String, 
               answer: String,
               incorrect_answers: Vec<String>,
               category: String,
               difficulty: String,
               answered: bool) -> Question {
        
        let mut question = Question {
            prompt: decode_html(&prompt).expect("Error decoding a question prompt"),
            answer: decode_html(&answer).expect("Error decoding a question answer"),
            answer_letter: String::from("A"),
            answer_prompt: String::from(""), 
            category,
            difficulty,
            answered,
        };
        // Randomize the answer set and set our answer letter to the corrisponding answer
        question.set_answer_prompt(decode_html_vector(incorrect_answers));

        question
    }

    // Sets answer_letter and answer_prompt
    // This function should ONLY be used after the question information has been generated
    fn set_answer_prompt(&mut self, incorrect_answers: Vec<String>) {
        //Put all of our question's potential answers in a vector
        let mut answers = incorrect_answers;
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
                
        //Store the correct answer's letter for answer checking based on the index
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

// HTML Decode a vector of strings in-place
fn decode_html_vector(mut answers: Vec<String>) -> Vec<String> {
    for answer in &mut answers {
        *answer = decode_html(answer).expect("Error decoding an incorrect answer");
    }

    answers
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
