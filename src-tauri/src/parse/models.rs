use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Candidate {
    pub name: String,
    pub location: String,
    pub qualification: String,
    pub hours: Option<String>,
    pub mobility: Option<String>,
    pub received_at: DateTime<Utc>,
    pub notes: Option<String>,
    pub start_note: Option<String>,
    pub sent_documents: Option<String>,
    pub completed_checklist: Option<String>,
    pub vaccination_stat: Option<String>,
    pub certification_state: Option<String>,
    pub personal_documentation: Option<String>,
    pub planned_child: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChildCareRequest {
    pub institution: String,
    pub location: String,
    pub grade: String,
    pub hours: String,
    pub diagnosis: Option<String>,
    pub contact: String,
    pub received_at: NaiveDate,
    pub notes: Option<String>,
}