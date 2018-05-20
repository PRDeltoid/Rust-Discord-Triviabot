pub struct Question {
    pub prompt: String,
    pub answer: String,
}


impl Question {
    pub fn new(prompt: String, answer: String) -> Question {
        Question {
            prompt: prompt,
            answer: answer,
        }
    }
}

