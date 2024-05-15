use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Candidate {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Ort")]
    pub location: String,
    #[serde(rename = "Qualif.")]
    pub qualification: Option<String>,
    #[serde(rename = "Stundenumfang")]
    pub hours: Option<String>,
    #[serde(rename = "Mobilität")]
    pub mobility: Option<String>,
    #[serde(rename = "Eingang")]
    pub received_at: Option<String>,
    #[serde(rename = "Bemerkungen")]
    pub notes: Option<String>,
    #[serde(rename = "Geplanter Beginn")]
    pub start_note: Option<String>,
    #[serde(rename = "Personalbogen")]
    pub sent_documents: Option<String>,
    #[serde(rename = "Checkliste komplett")]
    pub completed_checklist: Option<String>,
    #[serde(rename = "Masernschutz")]
    pub vaccination_stat: Option<String>,
    #[serde(rename = "Führungszeugnis")]
    pub certification_state: Option<String>,
    #[serde(rename = "Personalbogen")]
    pub personal_documentation: Option<String>,
    #[serde(rename = "Geplantes Kind")]
    pub planned_child: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChildCareRequest {
    #[serde(rename = "Einrichtung")]
    pub institution: String,
    #[serde(rename = "Ort")]
    pub location: String,
    #[serde(rename = "Klasse")]
    pub grade: String,
    #[serde(rename = "Stunden")]
    pub hours: String,
    #[serde(rename = "Diagnose")]
    pub diagnosis: Option<String>,
    #[serde(rename = "Ansprechpartner*in")]
    pub contact: String,
    #[serde(rename = "Datum")]
    pub received_at: Option<String>,
    #[serde(rename = "Bemerkung")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CandidateRequests {
    pub candidates: Vec<Candidate>,
    pub child_care_requests: Vec<ChildCareRequest>,
}

impl CandidateRequests {
    pub fn new(candidates: Vec<Candidate>, child_care_requests: Vec<ChildCareRequest>) -> Self {
        Self {
            candidates,
            child_care_requests,
        }
    }
}

pub struct ImportFiles {
    pub candidates_file: PathBuf,
}

impl ImportFiles {
    pub fn new(candidates_file: PathBuf) -> Self {
        Self { candidates_file }
    }
}
