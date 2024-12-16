use anyhow::Result;
use lazy_static::lazy_static;
use ollama::Ollama;
use rand::{thread_rng, Rng};
use rig::{
    agent::AgentBuilder,
    completion::{Chat, Message, ToolDefinition},
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shield_circuit::{
    pool::{AnonymityPool, Note},
    Hash,
};
use std::io::{self, Write};
use std::{collections::HashMap, ops::AddAssign};
use tokio::sync::Mutex;

mod ollama;

lazy_static! {
    static ref NOTES: Mutex<HashMap<Hash, Note>> = Mutex::new(HashMap::new());
    static ref POOL: Mutex<AnonymityPool> = Mutex::new(AnonymityPool::new());
    static ref TOPIC: Mutex<u64> = Mutex::new(0);
}

#[derive(Debug, thiserror::Error)]
enum WalletError {
    #[error("Note not found: {0}")]
    NotFound(String),
}

#[derive(Deserialize, Serialize)]
struct Status;

impl Tool for Status {
    const NAME: &'static str = "show_wallet_status";

    type Error = WalletError;
    type Args = HashMap<String, String>;
    type Output = String;

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
        let mut lines = Vec::new();

        lines.push("Unshielded accounts:".to_string());
        let pool = POOL.lock().await;
        for (account, amount) in pool.balances() {
            lines.push(format!("{}: {}", account, amount));
        }
        lines.push(String::new());

        lines.push("Shielded accounts:".to_string());
        let notes = NOTES.lock().await;
        for (_, note) in notes.iter() {
            lines.push(format!("{}: {}", note.recipiant(), AnonymityPool::amount()));
        }

        let output = lines.join("\n");
        Ok(output)
    }
}

#[derive(Deserialize, Serialize)]
struct Deposit;

impl Tool for Deposit {
    const NAME: &'static str = "deposit";

    type Error = WalletError;
    type Args = HashMap<String, String>;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let desc = "Deposit some amount into the privacy/anonymity pool from the main address.";
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: desc.to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "recipient": {
                        "type": "number",
                        "description": "The recipient address. The address that will withdraw the amount from the pool."
                    },
                }
            }),
        }
    }

    async fn call(&self, arg: Self::Args) -> Result<Self::Output, Self::Error> {
        let secret = {
            let rng = &mut thread_rng();
            rng.gen::<u64>()
        };
        let sender = AnonymityPool::account();
        let recipient_value = arg.get("recipient").unwrap();
        let recipient = recipient_value.parse::<u64>().unwrap();

        let mut pool = POOL.lock().await;
        let mut topic = TOPIC.lock().await;
        let mut notes = NOTES.lock().await;
        let note = pool.deposit(sender, secret, *topic, recipient);
        let nullifier = note.nullifier();
        notes.insert(nullifier.clone(), note);

        topic.add_assign(1);
        Ok(nullifier.to_hex())
    }
}

#[derive(Deserialize, Serialize)]
struct Withdraw;

impl Tool for Withdraw {
    const NAME: &'static str = "withdraw";

    type Error = WalletError;
    type Args = HashMap<String, String>;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let desc = "Withdraw the amount from privacy/anonymity pool into the main address.";
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: desc.to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "nullifier": {
                        "type": "string",
                        "description": "The nullifier value. Unique identifier of the note related to previous deposit."
                    },
                }
            }),
        }
    }

    async fn call(&self, arg: Self::Args) -> Result<Self::Output, Self::Error> {
        let nullifier_value = arg.get("nullifier").unwrap();
        let nullifier = Hash::from_hex(nullifier_value.clone());

        let mut pool = POOL.lock().await;
        let notes = NOTES.lock().await;
        let res = match notes.get(&nullifier) {
            Some(note) => pool.withdraw(note.clone()),
            None => return Err(WalletError::NotFound(nullifier.to_hex())),
        };
        Ok(format!("Result: {}", res))
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ollama = Ollama::new("http://localhost:11434".to_string());
    // Create agent with a single context prompt and two tools
    let calculator_agent = AgentBuilder::new(ollama)
        .preamble("You are a cryptocurrency wallet, that help users preserve their privacy while transacting on-chain. There is a concept of anonymity pool, where users are depositing their coins, and later withdraw anonymously. While the coins stay in the anonymity pool, they are shielded, hence your name Shield Wallet.")
        .max_tokens(1024)
        .tool(Status)
        .tool(Deposit)
        .tool(Withdraw)
        .build();

    let mut history = Vec::new();

    println!("Recommended commands:");
    println!("Show me wallet status. Call tool: show_wallet_status");
    println!("Deposit into the pool with recipient = 456. Call tool: deposit");
    println!("Withdraw from the pool with nullifier = [nullifier]. Call tool: withdraw");

    let inp = io::stdin();
    print!("> ");
    std::io::stdout().flush()?;

    let mut line = String::new();
    while inp.read_line(&mut line)? != 0 {
        let line = std::mem::take(&mut line);
        let exit_functions = vec!["exit", "abort", "quit"];
        if exit_functions.contains(&line.trim()) {
            break;
        }
        history.push(Message {
            role: "user".to_string(),
            content: line.clone(),
        });

        let res = calculator_agent
            .chat(line.as_str(), history.clone())
            .await?;
        history.push(Message {
            role: "assistant".to_string(),
            content: res.clone(),
        });
        println!("{}", res);

        print!("> ");
        std::io::stdout().flush()?;
    }

    Ok(())
}
