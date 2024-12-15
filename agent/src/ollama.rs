use reqwest::Body;
use rig::{
    completion::{
        CompletionError, CompletionModel, CompletionRequest, CompletionResponse, Message,
        ModelChoice, ToolDefinition,
    },
    embeddings::{Embedding, EmbeddingError, EmbeddingModel},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum ToolCall {
    #[serde(rename = "function")]
    Function {
        name: String,
        arguments: serde_json::Value,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseMessage {
    role: Role,
    content: String,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum ToolKind {
    Function,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Tool {
    #[serde(rename = "type")]
    kind: ToolKind,
    tool: ToolDefinition,
}

impl Tool {
    pub fn new(kind: ToolKind, tool: ToolDefinition) -> Self {
        Self { kind, tool }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Request {
    model: String,
    messages: Vec<Message>,
    tools: Vec<Tool>,
    format: Option<String>,
    options: Option<serde_json::Value>,
    stream: Option<bool>,
    keep_alive: Option<String>,
}

impl Request {
    pub fn new(model: String, messages: Vec<Message>, tools: Vec<Tool>) -> Self {
        Self {
            model,
            messages,
            tools,
            format: None,
            options: None,
            stream: Some(false),
            keep_alive: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    model: String,
    created_at: String,
    message: ResponseMessage,
    done: bool,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64,
}

#[derive(Clone, Debug)]
pub struct Ollama {
    url: String,
}

impl Ollama {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl CompletionModel for Ollama {
    type Response = Response;

    async fn completion(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse<Response>, CompletionError> {
        let mut messages = Vec::new();
        messages.extend(request.chat_history);
        messages.push(Message {
            role: "user".to_string(),
            content: request.prompt,
        });
        let tools = request
            .tools
            .into_iter()
            .map(|x| Tool::new(ToolKind::Function, x))
            .collect();
        let req = Request::new("llama3.2".to_string(), messages, tools);
        let client = reqwest::Client::new();
        let res: reqwest::Response = client
            .post(format!("{}/api/chat", self.url))
            .body(Body::from(serde_json::to_string(&json!(req)).unwrap()))
            .send()
            .await
            .unwrap();
        let res: Response = res.json().await.unwrap();
        let completion_res = if let Some(tool_calls) = res.message.tool_calls.clone() {
            let tool_call = tool_calls[0].clone();
            let model_choice = match tool_call {
                ToolCall::Function { name, arguments } => ModelChoice::ToolCall(name, arguments),
            };
            CompletionResponse {
                choice: model_choice,
                raw_response: res,
            }
        } else {
            CompletionResponse {
                choice: ModelChoice::Message(res.message.content.clone()),
                raw_response: res,
            }
        };
        Result::Ok(completion_res)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct EmbedResponse {
    model: String,
    embeddings: Vec<Vec<f64>>,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
}

impl EmbedRequest {
    pub fn new(model: String, input: Vec<String>) -> Self {
        Self { model, input }
    }
}

impl EmbeddingModel for Ollama {
    const MAX_DOCUMENTS: usize = 34;

    fn ndims(&self) -> usize {
        345
    }

    async fn embed_texts(
        &self,
        texts: impl IntoIterator<Item = String> + Send,
    ) -> Result<Vec<Embedding>, EmbeddingError> {
        let req = EmbedRequest::new("llama3.2".to_string(), texts.into_iter().collect());
        let client = reqwest::Client::new();
        let res: reqwest::Response = client
            .post(format!("{}/api/embed", self.url))
            .body(Body::from(serde_json::to_string(&json!(req)).unwrap()))
            .send()
            .await
            .unwrap();
        let res: EmbedResponse = res.json().await.unwrap();
        let mut embedding = Vec::new();
        for embed in res.embeddings {
            let emb = Embedding {
                document: res.model.clone(),
                vec: embed,
            };
            embedding.push(emb);
        }
        Result::Ok(embedding)
    }
}
