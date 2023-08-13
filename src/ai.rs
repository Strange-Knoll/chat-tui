use std::error::Error;

use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, Role}};
#[derive(Clone)]
pub struct Ai{
    client: Client,
    user: String,
    system: String,
    assistant: String,

}

impl Ai{
    pub fn new(client:Client) -> Self{
        Self{
            client,
            user: String::new(),
            system: String::new(),
            assistant: String::new(),
        }
    }
    pub fn client(&mut self, client:Client) -> &mut Self{
        self.client = client;
        self
    }
    pub fn user(&mut self, user: String) -> &mut Self{
        self.user = user;
        self
    }
    pub fn system(&mut self, system: String) -> &mut Self{
        self.system = system;
        self
    }
    pub fn assistant(&mut self, assistant: String) -> &mut Self{
        self.assistant = assistant;
        self
    }

    pub async fn request(&self, query:String) -> Result<String, Box<dyn Error>>{
        let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo-0613")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(&self.system)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content(&self.assistant)
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(&query)
                .build()?,
        ]).build()?;

        let response = self.client.chat().create(request).await?;
        let response = &response.choices[0].message.content;
        Ok(response.to_string())
    }
}