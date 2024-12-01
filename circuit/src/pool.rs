use std::collections::{HashMap, HashSet};

use crate::{
    hash_leaf, hash_two,
    merkle::{self, DenseIncrementalMerkleTree, Path},
    Hash,
};
use sha3::Keccak256;

const COMPANY_ACCOUNT: u64 = 349058;
const CONTRACT_ADDRESS: u64 = 123;
const DEFAULT_AMOUNT: u64 = 1000;

struct Note {
    secret: u64,
    topic: u64,
    amount: u64,
    recipiant: u64,
    merkle_path: merkle::Path,
}

struct AnonymityPool {
    tree: DenseIncrementalMerkleTree<Keccak256>,
    nullifiers: HashSet<Hash>,
    balances: HashMap<u64, u64>,
}

impl AnonymityPool {
    pub fn new(default_account: u64) -> Self {
        let tree = DenseIncrementalMerkleTree::<Keccak256>::new();
        let mut balances = HashMap::new();
        balances.insert(default_account, DEFAULT_AMOUNT);
        Self {
            tree,
            nullifiers: HashSet::new(),
            balances,
        }
    }

    pub fn deposit(
        &mut self,
        sender: u64,
        secret: u64,
        topic: u64,
        amount: u64,
        recipiant: u64,
    ) -> Note {
        let secret_hash = hash_leaf::<Keccak256>(secret.to_be_bytes().to_vec());
        let topic_hash = hash_leaf::<Keccak256>(topic.to_be_bytes().to_vec());
        let _amount = hash_leaf::<Keccak256>(amount.to_be_bytes().to_vec());

        let nullifier = hash_two::<Keccak256>(secret_hash.clone(), topic_hash);
        let commitment = hash_two::<Keccak256>(secret_hash.clone(), secret_hash);

        assert!(*self.balances.get(&sender).unwrap_or(&0) > amount);

        self.tree.insert_leaf(commitment);
        self.nullifiers.insert(nullifier);

        // Transfer fee to company account
        self.balances.entry(sender).and_modify(|x| *x -= 1);
        self.balances.entry(COMPANY_ACCOUNT).and_modify(|x| *x += 1);

        // Deposit amount to contract
        self.balances.entry(sender).and_modify(|x| *x -= amount);
        self.balances
            .entry(CONTRACT_ADDRESS)
            .and_modify(|x| *x += amount);

        Note {
            secret,
            topic,
            amount,
            recipiant,
            merkle_path: Path::default(),
        }
    }

    pub fn withdraw(&mut self, note: Note) {
        let secret_hash = hash_leaf::<Keccak256>(note.secret.to_be_bytes().to_vec());
        let topic_hash = hash_leaf::<Keccak256>(note.topic.to_be_bytes().to_vec());
        let nullifier = hash_two::<Keccak256>(secret_hash.clone(), topic_hash);
        assert!(!self.nullifiers.contains(&nullifier));
        assert!(note.merkle_path.verify_against(self.tree.root().unwrap()));

        self.balances
            .entry(CONTRACT_ADDRESS)
            .and_modify(|x| *x -= note.amount);
        self.balances
            .entry(note.recipiant)
            .and_modify(|x| *x += note.amount);

        self.nullifiers.insert(nullifier);
    }
}
