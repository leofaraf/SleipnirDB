use std::{collections::HashSet, hash, path::PathBuf, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};

pub mod add;
pub mod query;
pub mod contains;
pub mod update;
pub mod remove;
pub mod information;
pub mod save;

pub const EX_TABLE_EXTENSION: &str = ".exdb";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExTable<T: Eq + hash::Hash>
{
    pub label: String,
    pub save_path: PathBuf,
    pub items: HashSet<T>
}