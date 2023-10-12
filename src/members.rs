use serde::{Deserialize, Serialize};

/// A league member of the Fantasy Football League
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct MemberId(pub String);
