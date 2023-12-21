use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::{http_component, llm};

use serde::Deserialize;
use serde_json::{from_slice, json, to_string};

#[derive(Deserialize)]
struct StoryData<'a> {
    place: &'a str,
    characters: Vec<&'a str>,
    objects: Vec<&'a str>,
}

const PROMPT: &str = r#"\
[INST]
<<SYS>>
You are a bot that writes short stories based on the user input. These stories will always have a Christmas theme and should fit within the bounds of the output.

You should expect to receive input with a location, a list of characters, and a list of objects. The story should weave together all of these inputs into a coherent story. Please refrain from adding parts to the story where you talk as yourself and please refrain from saying how much you like the story. The output should simply be the story.
<</SYS>>

The location is {LOCATION} the characters are {CHARACTERS} and the objects are {OBJECTS}.
</INST>
"#;

/// A simple Spin HTTP component.
#[http_component]
fn handle_santa_capacity(req: Request) -> anyhow::Result<impl IntoResponse> {
    let story_data: StoryData = from_slice(req.body())?;
    let model = llm::InferencingModel::Llama2Chat;

    //let prompt = format!(
    //    "Write a short story from the perspective of a narrator. The story should take place in the {}, contain these characters {:?}, and makes use of these objects: {:?}.",
    //    story_data.place,
    //    story_data.characters,
    //    story_data.objects,
    //);

    let story = format!(
        "{:?}",
        llm::infer_with_options(
            model,
            &PROMPT
                .replace("{LOCATION}", story_data.place)
                .replace("{CHARACTERS}", &format!("{:?}", story_data.characters))
                .replace("{OBJECTS}", &format!("{:?}", story_data.objects)),
            llm::InferencingParams {
                max_tokens: 400,
                repeat_penalty: 1.1,
                repeat_penalty_last_n_token_count: 64,
                temperature: 0.8,
                top_k: 40,
                top_p: 0.9,
            },
        )
        .unwrap()
        .text
        .trim()
    );
    println!("{}", story);

    let json_obj = to_string(&json!({
        "story": story,
    }))
    .unwrap();

    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json_obj)?)
}
