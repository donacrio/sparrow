use std::hash::Hash;
use std::str::FromStr;

pub trait StorageKey: FromStr + ToString + Eq + Hash {}
pub trait StorageValue: FromStr + ToString {}
