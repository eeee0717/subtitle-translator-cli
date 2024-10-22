use async_openai::{config::OpenAIConfig, Client};

#[derive(Debug)]
pub struct OpenAI {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
    pub client: Client<OpenAIConfig>,
}
impl OpenAI {
    pub fn new() -> Self {
        let api_key = dotenv!("API_KEY").to_string();
        let api_base = dotenv!("API_BASE").to_string();
        let model = dotenv!("MODEL").to_string();
        eprintln!(
            "API Key: {}\nAPI Base: {}\nModel: {}",
            api_key, api_base, model
        );
        let config = OpenAIConfig::new()
            .with_api_key(&api_key)
            .with_api_base(&api_base);
        let client = Client::with_config(config);
        Self {
            api_key,
            api_base,
            model,
            client,
        }
    }
}
mod test {

    #[tokio::test]
    pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let openai = super::OpenAI::new();
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
