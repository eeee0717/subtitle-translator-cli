use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};

#[derive(Debug)]
pub struct OpenAI {
    model: String,
    client: Client<OpenAIConfig>,
}
impl OpenAI {
    pub fn new() -> Self {
        let api_key = dotenv!("API_KEY").to_string();
        let api_base = dotenv!("API_BASE").to_string();
        let model = dotenv!("MODEL").to_string();
        // eprintln!("API_KEY: {}", api_key);
        // eprintln!("API_BASE: {}", api_base);

        let config = OpenAIConfig::new()
            .with_api_key(&api_key)
            .with_api_base(&api_base);
        let client = Client::with_config(config);
        Self { model, client }
    }
    pub async fn chat(
        &self,
        prompt: String,
        user_message: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(4_000u32)
            .model(self.model.clone())
            .messages([
                ChatCompletionRequestSystemMessageArgs::default()
                    .content(prompt)
                    .build()?
                    .into(),
                ChatCompletionRequestUserMessageArgs::default()
                    .content(user_message)
                    .build()?
                    .into(),
            ])
            .build()?;
        let response = self.client.chat().create(request).await?;
        Ok(response.choices[0]
            .message
            .content
            .clone()
            .expect("No Content Found"))
    }
}
#[cfg(test)]
mod test {
    #[tokio::test]
    pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let openai = super::OpenAI::new();
        let request = async_openai::types::CreateChatCompletionRequestArgs::default()
            .max_tokens(40u32)
            .model("gpt-4o-mini")
            .messages([
                async_openai::types::ChatCompletionRequestUserMessageArgs::default()
                    .content("Hi! Are you ChatGPT?")
                    .build()?
                    .into(),
            ])
            .build()?;

        let response = openai.client.chat().create(request).await?;

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
