mod test {

    #[tokio::test]
    pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let api_key = dotenv!("API_KEY");
        let api_base = dotenv!("API_BASE");
        eprintln!("API_KEY: {}\nAPI_BASE:{}", api_key, api_base);
        let config = async_openai::config::OpenAIConfig::new()
            .with_api_key(api_key)
            .with_api_base(api_base)
            .with_org_id("the-continental");
        let client = async_openai::Client::with_config(config);
        let request = async_openai::types::CreateChatCompletionRequestArgs::default()
            .max_tokens(40u32)
            .model("gpt-4o-mini")
            .messages([
                async_openai::types::ChatCompletionRequestUserMessageArgs::default()
                    .content("Hi!")
                    .build()?
                    .into(),
            ])
            .build()?;

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
