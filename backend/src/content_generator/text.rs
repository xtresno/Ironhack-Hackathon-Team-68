use crate::prelude::*;

use async_openai::{types::CreateCompletionRequestArgs, Client};

pub async fn text_query(client: &Client, prompt: &str) -> Result<String> {
    let num_words = prompt.chars().filter(|c| c.is_whitespace()).count() + 1;
    let max_tokens = (num_words as f32 * 1.75) as u16;

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prompt)
        .max_tokens(max_tokens)
        .build()?;

    // Call API
    let response = client
        .completions() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await?;

    Ok(response
        .choices
        .first()
        .ok_or(Error::QueryOpenAi)?
        .text
        .to_owned())
}
