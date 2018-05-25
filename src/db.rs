extern crate serde_json;
extern crate reqwest;
extern crate url;

use question::Question;
use questionset::QuestionSet;
use optionset::OptionSet;
use url::Url;

#[derive(Serialize, Deserialize)]
struct EntrySet {
    response_code: i32,
    results: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
struct Entry {
    category:   String,
    difficulty: String,
    question:   String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

///Produces a QuestionSet based on the given OptionSet.
pub fn get_question_set(options: OptionSet) -> QuestionSet {
    //Grab the number of questions before we consume the optionset
    let number_of_questions = options.number_of_questions.clone();

    //Pull our trivia data as JSON
    let url = compose_url(options);
    let json =  get_json(url);
    //println!("JSON: {}\n", json);

    //Create our raw dataset from the JSON
    let res: EntrySet = serde_json::from_str(&json).unwrap();
    //println!("Serde: {}", v.results[0].category);

    //Create an empty questionset
    let mut questions: Vec<Question> = Vec::new();

    //For each result in our raw dataset, create a question and add it to 'questions'
    for result in res.results.iter() {
        let question = Question {
            prompt: result.question.clone(),
            answer: result.correct_answer.clone(),
            category: result.category.clone(),
            difficulty: result.difficulty.clone(),
            answered: false,
        };
        questions.push(question);
    }

    //Return the new questionset
    QuestionSet::new(questions, number_of_questions)

}

///Requests JSON from the given URL and returns it as a String
fn get_json(url: Url) -> String {
    let json = reqwest::get(url).unwrap()
        .text().unwrap();
    json 
}

///Composes a trivia request URL based on parameters.
fn compose_url(options: OptionSet) -> Url {

    let num = options.number_of_questions.to_string();
    let url = Url::parse_with_params("https://opentdb.com/api.php",
                                      &[("amount",      num), 
                                        ("type",        "multiple".to_string()),
                                        ("difficulty",  options.difficulty),
                                        ("category",    options.category)])
                                    .unwrap();

    //println!("URL: {}", url);
    url
}
