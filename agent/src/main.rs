use anyhow::Result;
use ollama::Ollama;
use rig::{
    agent::AgentBuilder,
    completion::{Prompt, ToolDefinition},
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod ollama;

lazy_static! {
    static ref NOTES: Mutex<HashMap<Hash, Note>> = Mutex::new(HashMap::new());
    static ref POOL: Mutex<AnonymityPool> = Mutex::new(AnonymityPool::new(DEFAULT_ACCOUNT));
    static ref TOPIC: Mutex<u64> = Mutex::new(0);
}

#[derive(Debug, thiserror::Error)]
#[error("Wallet Error")]
struct WalletError;

#[derive(Deserialize, Serialize, Default)]
struct AddressStatus {
    address: String,
    amount: u64,
}

#[derive(Deserialize, Serialize, Default)]
struct WalletStatus {
    shielded: Vec<AddressStatus>,
    unshielded: Vec<AddressStatus>,
}

#[derive(Deserialize, Serialize)]
struct Status;
impl Tool for Status {
    const NAME: &'static str = "status";

    type Error = WalletError;
    type Args = ();
    type Output = WalletStatus;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let desc =
            "Show wallet status. (i.e. Shielded and Unshielded addresses and their balances)";
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: desc.to_string(),
            parameters: Value::Null,
        }
    }

    async fn call(&self, _: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(WalletStatus::default())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ollama = Ollama::new("http://localhost:11434".to_string());
    // Create agent with a single context prompt and two tools
    let calculator_agent = AgentBuilder::new(ollama)
        .preamble("You are a calculator here to help the user perform arithmetic operations. Use the tools provided to answer the user's question.")
        .max_tokens(1024)
        .tool(Status)
        .build();

    // Prompt the agent and print the response
    println!("Calculate 5 - 4");
    println!(
        "Calculator Agent: {}",
        calculator_agent.prompt("Calculate 5 - 4").await?
    );

    Ok(())
}
