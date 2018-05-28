use serenity::model::id::UserId;
use serenity::model::user::User;
use std::collections::HashMap;

pub struct Scores {
    score_list: HashMap<UserId, (String, u32)>,
}

impl Scores {
    pub fn new() -> Scores {
        Scores {
            score_list: HashMap::new(),
        }
    }

    pub fn increase_score(&mut self, user: User, points: u32) {
        let old_score = self.get_score(&user);
        self.score_list
            .insert(user.id, (user.name, old_score + points));
    }

    pub fn get_score(&self, user: &User) -> u32 {
        match self.score_list.get(&user.id) {
            Some(s) => s.1.clone(),
            None => 0,
        }
    }

    pub fn output_scores(&self) -> String {
        let mut output = String::from("Scores:\n");
        for (userid, score) in self.score_list.iter() {
            let s = format!("{} - {} - {}\n", userid, score.0, score.1);
            output.push_str(&s);
        }

        output
    }
}
