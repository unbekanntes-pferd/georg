use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Candidate {
    #[serde(rename(serialize = "name", deserialize = "Name"))]
    pub name: String,
    #[serde(rename(serialize = "location", deserialize = "Ort"))]
    pub location: String,
    #[serde(rename(serialize = "qualification", deserialize = "Qualif."))]
    pub qualification: Option<String>,
    #[serde(rename(serialize = "hours", deserialize = "Stundenumfang"))]
    pub hours: Option<String>,
    #[serde(rename(serialize = "mobility", deserialize = "Mobilität"))]
    pub mobility: Option<String>,
    #[serde(rename(serialize = "receivedAt", deserialize = "Eingang"))]
    pub received_at: Option<String>,
    #[serde(rename(serialize = "notes", deserialize = "Bemerkungen"))]
    pub notes: Option<String>,
    #[serde(rename(serialize = "startNote", deserialize = "Geplanter Beginn"))]
    pub start_note: Option<String>,
    #[serde(rename(serialize = "sentDocuments", deserialize = "Personalbogen"))]
    pub sent_documents: Option<String>,
    #[serde(rename(serialize = "completedChecklist", deserialize = "Checkliste komplett"))]
    pub completed_checklist: Option<String>,
    #[serde(rename(serialize = "vaccinationStat", deserialize = "Masernschutz"))]
    pub vaccination_stat: Option<String>,
    #[serde(rename(serialize = "certificationState", deserialize = "Führungszeugnis"))]
    pub certification_state: Option<String>,
    #[serde(rename(serialize = "personalDocumentation", deserialize = "Personalbogen"))]
    pub personal_documentation: Option<String>,
    #[serde(rename(serialize = "plannedChild", deserialize = "Geplantes Kind"))]
    pub planned_child: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChildCareRequest {
    #[serde(rename(serialize = "institution", deserialize = "Einrichtung"))]
    pub institution: String,
    #[serde(rename(serialize = "location", deserialize = "Ort"))]
    pub location: String,
    #[serde(rename(serialize = "grade", deserialize = "Klasse"))]
    pub grade: String,
    #[serde(rename(serialize = "hours", deserialize = "Stunden"))]
    pub hours: String,
    #[serde(rename(serialize = "diagnosis", deserialize = "Diagnose"))]
    pub diagnosis: Option<String>,
    #[serde(rename(serialize = "contact", deserialize = "Ansprechpartner*in"))]
    pub contact: String,
    #[serde(rename(serialize = "receivedAt", deserialize = "Datum"))]
    pub received_at: Option<String>,
    #[serde(rename(serialize = "notes", deserialize = "Bemerkung"))]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
