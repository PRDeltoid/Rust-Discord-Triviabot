use serenity::model::id::ChannelId;

pub struct OptionSet {
    pub number_of_questions: u32,
    pub difficulty: String,
    pub category: String,
    pub channel: ChannelId,
}
