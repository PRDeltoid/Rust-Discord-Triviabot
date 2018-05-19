use serenity::model::channel::Message;
use typemap::Key;

pub struct TriviaManager {
    pub running: bool,
    current_question: String,
}


impl Key for TriviaManager {
    type Value = TriviaManager;
}

impl TriviaManager {

    pub fn new() -> TriviaManager {
        TriviaManager {
            running: false,
            current_question: "".to_string(),
        }
    }

    pub fn start(&mut self, message: &Message) {
        let text = match self.running {
            false => {
                self.running = true;
                String::from("Trivia Running")
            },
            true => {
                String::from("Trivia is already running")
            },
        };

        let _ = message.channel_id.say(text);
    }

    pub fn stop(&mut self, message: &Message) {
        let text = match self.running {
            true => {
                self.running = false;
                String::from("Trivia Stopping")
            },
            false => {
                String::from("Trivia is not running")
            },
        };
        let _ = message.channel_id.say(text);
    }
    
    pub fn unrecognized_command(&self, message: &Message) {
        let _ = message.channel_id.say("Invalid Command");
    }

    pub fn on_message(&self, message: &Message) {
        println!("{}", message.content);
    }

}
