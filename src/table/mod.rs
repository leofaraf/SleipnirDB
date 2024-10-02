use std::{collections::HashSet, hash, path::PathBuf, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};

use crate::database::ExDatabase;

pub mod add;
pub mod query;
pub mod contains;
pub mod update;
pub mod remove;
pub mod information;
pub mod save;

pub const EX_TABLE_EXTENSION: &str = ".exdb";

#[derive(Debug, Clone)]
pub struct ExTable<T: Eq + hash::Hash>
{
    pub database: Arc<ExDatabase>,
    pub label: String,
    pub items: Arc<Mutex<HashSet<T>>>
}