//! the manufacturer command responses

use crate::manufacturers::manufacturer_id::ManufacturerId;
use chrono::{DateTime, Utc};

/// It represents a response for manufacturers creation
#[derive(Debug, PartialEq, Eq)]
pub struct ManufacturerCreated {
    /// the manufacturer id for the new manufacturer
    pub manufacturer_id: ManufacturerId,
    /// the manufacturer creation timestamp
    pub created_at: DateTime<Utc>,
}

/// It represents a response for manufacturer updates
#[derive(Debug, PartialEq, Eq)]
pub struct ManufacturerUpdated {
    /// the manufacturer id for the updated manufacturer
    pub manufacturer_id: ManufacturerId,
    /// the manufacturer update timestamp
    pub last_modified_at: DateTime<Utc>,
}
