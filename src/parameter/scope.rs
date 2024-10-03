use derive_more::derive::Display;
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize)]
#[display("{_variant}")]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Address,
    Email,
    Openid,
    Phone,
    Profile,
}

#[derive(Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct ScopeSet(HashSet<Scope>);

impl<'de> serde::Deserialize<'de> for ScopeSet {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?
            .split(' ')
            .map(|s| s.into_deserializer())
            .map(Scope::deserialize)
            .collect::<Result<HashSet<Scope>, D::Error>>()
            .map(Self)
    }
}

impl ScopeSet {
    pub fn contains(&self, scope: &Scope) -> bool {
        self.0.contains(scope)
    }
}
