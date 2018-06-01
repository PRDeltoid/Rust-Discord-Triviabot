use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use typemap::Key;
use std::fmt::Display;

use db;
use optionset::OptionSet;
use questionset::QuestionSet;
use scores::Scores;

/// The TriviaManager holds the current gamestate
pub struct TriviaManager {
    pub running: bool,
    question_set: Option<QuestionSet>,
    channel: Option<ChannelId>,
    scores: Option<Scores>,
    skips: u32,
}

impl Key for TriviaManager {
    type Value = TriviaManager;
}

/// This object keeps track of the current gamestate
impl TriviaManager {
    /// Generates a new trivia manager. Channel and running are unset by default.
    pub fn new() -> TriviaManager {
        TriviaManager {
            running: false,
            question_set: None,
            channel: None,
            scores: None,
            skips: 0,
        }
    }

    /// Starts the trivia bot
    ///
    /// This function sets the `channel` member, which is used for sending messages during a game
    pub fn start(&mut self, optionset: &OptionSet) {
        if !self.running {
            //Configure the trivia manager
            self.running = true;
            self.question_set = Some(db::get_question_set(&optionset));
            self.channel = Some(optionset.channel);
            self.scores = Some(Scores::new());

            //Tell the user we've started and ask a question
            self.say("Trivia Starting");
            self.ask_question();
        } else {
            self.say("Trivia is already running");
        }
    }

    /// Stops the trivia bot
    pub fn stop(&mut self) {
        if self.running {
            self.print_scores();
            self.say("Trivia Stopping");
            self.running = false;
            self.question_set = None;
            self.channel = None;
            self.scores = None;
        } else {
            self.say("Trivia is not running");
        }
    }

    /// Skips the current question on the triviabot
    pub fn skip(&mut self) {
        if self.running {
            self.skips += 1;
            if self.skips >= 3 {
                self.say("Skipping question.");

                self.question_set
                    .as_mut()
                    .expect("Error getting questionset")
                    .next_question();

                self.ask_question();
                self.say(format!("Votes Needed: {}/3", self.skips).as_str());
            }
        } else {
            self.say("Can't skip because trivia is not running");
        }
    }

    /// Method which runs whenever a new message is recieved.
    ///
    /// If the triviabot is running, the text is checked to see if it is an answer
    pub fn on_message(&mut self, message: Message) {
        if self.running {
            let correct = self.check_answer(message.content.as_str());
            if correct {
                self.say("Correct");

                self.scores
                    .as_mut()
                    .expect("Error getting scores in on_message()")
                    .increase_score(message.author, 1);

                self.question_set
                    .as_mut()
                    .expect("Error getting questionset in on_message()")
                    .next_question();

                self.ask_question();
            }
        }
    }

    /// Sets the channel where the triviabot will send it's messages
    pub fn set_channel(&mut self, message: &Message) {
        self.channel = Some(message.channel_id);
    }

    // Prints out the scorelist to the current channel
    fn print_scores(&self) {
        let scores = self.scores
                        .as_ref()
                        .expect("Error getting scores")
                        .output_scores();
        self.say(scores);
    }

    // Sends a message to the active trivia channel with the current question
    // When no more questions are available, this method calls the stop() method
    fn ask_question(&mut self) {
        // If question is false, there was no question to ask
        let question = match self.question_set
                                .as_ref()
                                .expect("Error getting questionset in ask_question()")
                                .get_current_question() 
        {
            Some(q) => {
                let prompt = format!("Question: {}", q.prompt);
                self.say(&prompt);
                println!("Answer: {}", q.answer);
                true
            }
            None => {
                self.say("Out of questions");
                false
            }
        };

        // Stop if we don't have any more questions to ask
        if !question {
            self.stop();
        }
    }

    // Checks if a given string matches the current question's answer
    fn check_answer(&mut self, message: &str) -> bool {
        let question = self.question_set
                            .as_ref()
                            .expect("Error getting question set")
                            .get_current_question();

        match question {
            Some(q) => {
                //Check if the message is the same as the answer
                message.to_lowercase() == q.answer.to_lowercase()
            }
            None => false,
        }
    }

    // Sends a message to the currently set channel
    // Accepts a String object
    // If no channel exists, outputs an error message to the console
    //
    fn say<T: Display>(&self, message: T) {
        let _ = self.channel
            .expect("Tried using say without a channel set")
            .say(format!("{}", message));
    }
}
