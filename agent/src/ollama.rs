use rig::{
    completion::{CompletionError, CompletionModel, CompletionResponse},
    embeddings::{Embedding, EmbeddingError, EmbeddingModel},
};

struct Response {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    done_reason: String,
    context: Vec<u32>,
    total_duration: u64,
    load_duration: u64,
    prompt_eval_count: u64,
    prompt_eval_duration: u64,
    eval_count: u64,
    eval_duration: u64,
}

#[derive(Clone, Debug)]
struct Ollama {
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
        request: rig::completion::CompletionRequest,
    ) -> Result<CompletionResponse<Response>, CompletionError> {
        Result::Err(CompletionError::ResponseError("()".to_string()))
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
        Result::Err(EmbeddingError::ResponseError("()".to_string()))
    }
}
