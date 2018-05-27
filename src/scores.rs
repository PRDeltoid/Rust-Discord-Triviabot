use std::collections::HashMap;
use serenity::model::id::UserId;

pub struct Scores {
    score_list: HashMap<UserId, u32>,
}

impl Scores {
    pub fn new() -> Scores {
        Scores {
            score_list: HashMap::new(),
        }
    }

    pub fn increase_score(&mut self, userid: UserId, points: u32) {

        let old_score = self.get_score(&userid);
        self.score_list.insert(userid, old_score + points);
    }

    pub fn get_score(&self, userid: &UserId) -> u32 {
        match self.score_list.get(userid) {
            Some(s) => s.clone(),
            None => 0,
        }
    }

    pub fn output_scores(&self) -> String {
        let mut output = String::from("");
        for (userid, score) in self.score_list.iter() {
            let s = format!("{} - {}", userid, score);
            output.push_str(&s);
        }

        output
    }
}
