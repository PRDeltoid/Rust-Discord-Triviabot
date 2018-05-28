use htmlescape::{ decode_html };
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use typemap::Key;

use optionset::OptionSet;
use questionset::QuestionSet;
use question::Question;
use scores::Scores;
use db;

pub struct TriviaManager {
    pub running: bool,
    question_set: QuestionSet,
    channel: Option<ChannelId>,
    scores: Scores,
}


impl Key for TriviaManager {
    type Value = TriviaManager;
}

/// This object keeps track of the current gamestate
impl TriviaManager {

    /// Generates a new trivia manager. Channel and running are unset by default.
    pub fn new() -> TriviaManager {
        let questions: Vec<Question> = Vec::new();
        TriviaManager {
            running: false,
            question_set: QuestionSet::new(questions, 1),
            channel: None,
            scores: Scores::new(),
        }
    }

    /// Starts the trivia bot
    ///
    /// This function sets the `channel` member, which is used for sending messages during a game 
    pub fn start(&mut self, optionset: OptionSet) {
        match self.running {
            false => {
                //Configure the trivia manager
                self.running = true;

                self.question_set = db::get_question_set(optionset);

                //Tell the user we've started and ask a question
                self.say_raw("Trivia Running");
                self.ask_question();
            },
            true => {
                self.say_raw("Trivia is already running");
            },
        };
    }

    /// Stops the trivia bot
    pub fn stop(&mut self) {
        let text = match self.running {
            true => {
                self.running = false;
                self.print_scores();
                String::from("Trivia Stopping")
            },
            false => {
                String::from("Trivia is not running")
            },
        };
        self.say(text);
    }

    pub fn skip(&mut self) {
        match self.running {
            true => {
                self.say_raw("Skipping question.");
                self.question_set.next_question();
                self.ask_question();
            },
            false => {
                self.say_raw("Can't skip because trivia is not running");
            },
        };
    }
    
    /// Method which runs whenever a new message is recieved.
    ///
    /// If the triviabot is running, the text is checked to see if it is an answer
    pub fn on_message(&mut self, message: Message) {
        if self.running {
            let correct = self.check_answer(message.content);
            if correct {
                self.say_raw("Correct");
                self.scores.increase_score(message.author, 1);

                self.question_set.next_question();
                self.ask_question();
            }
        }
    }

    pub fn set_channel(&mut self, message: &Message) {
        self.channel = Some(message.channel_id);
    }

    fn print_scores(&self) {
        self.say(self.scores.output_scores());

    }


    /// Sends a message to the active trivia channel with the current question
    /// When no more questions are available, this method calls the stop() method
    fn ask_question(&mut self) {
        //If question is false, there was no question to ask
        let question = match self.question_set.get_current_question() {
            Some(q) => {
                let decoded = match decode_html(&q.prompt) {
                    Err(reason) => panic!("Error {:?} at character {}", reason.kind, reason.position),
                    Ok(s) => s
                };
                let question = format!("Question: {}", decoded);
                self.say_raw(&question);
                println!("Answer: {}", q.answer);
                true
            },
            None => {
                self.say_raw("Out of questions");
                false
            }
        };

        //Stop if we don't have any more questions to ask
        if question == false {
            self.stop();
        }

    }

    /// Checks if a given string matches the current question's answer
    fn check_answer(&mut self, message: String) -> bool {
        let question = self.question_set.get_current_question();

        match question {
            Some(q) => {
                if message.to_lowercase() == q.answer.to_lowercase() {
                    true
                } else {
                    false 
                }
            },
            None => false
        }
    }

    /// Sends a message to the currently set channel
    /// Accepts a raw Str object
    /// If no channel exists, outputs an error message to the console 
    fn say_raw(&self, message: &str) {
        match self.channel {
            Some(channel) => { let _ = channel.say(message); },
            None => { println!("Error. Tried to use say without a channel set"); },
        }
    }

    /// Sends a message to the currently set channel
    /// Accepts a String object
    /// If no channel exists, outputs an error message to the console 
    ///
    fn say(&self, message: String) {
        match self.channel {
            Some(channel) => { let _ = channel.say(message.as_str()); },
            None => { println!("Error. Tried to use say without a channel set"); },
        }
    }

}
