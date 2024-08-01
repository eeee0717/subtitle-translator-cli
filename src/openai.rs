// use std::error::Error;

// use async_openai::{
//     config::OpenAIConfig,
//     types::{ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs},
//     Client,
// };

mod tests {

    use std::error::Error;

    use async_openai::{
        config::OpenAIConfig,
        types::{ChatCompletionRequestSystemMessageArgs, CreateChatCompletionRequestArgs},
        Client,
    };

    #[tokio::test]
    async fn test_openai() -> Result<(), Box<dyn Error>> {
        let api_key = "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";
        let api_base = "https://api.groq.com/openai/v1";

        let config = OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base);
        let client = Client::with_config(config);

        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u32)
            // .model("gpt-4o-mini")
            .model("llama-3.1-8b-instant")
            .messages([ChatCompletionRequestSystemMessageArgs::default()
                .content("Say Hello.")
                .build()?
                .into()])
            .build()?;

        println!("{}", serde_json::to_string(&request).unwrap());

        let response = client.chat().create(request).await?;

        println!("\nResponse:\n");
        for choice in response.choices {
            println!(
                "{}: Role: {}  Content: {:?}",
                choice.index, choice.message.role, choice.message.content
            );
        }

        Ok(())
    }
}
