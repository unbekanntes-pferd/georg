use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidateResponse {
    pub id: String,
    pub name: String,
    pub location: String,
    pub qualification: Option<String>,
    pub hours: Option<String>,
    pub mobility: Option<String>,
    pub received_at: Option<String>,
    pub notes: Option<String>,
    pub start_note: Option<String>,
    pub sent_documents: Option<String>,
    pub completed_checklist: Option<String>,
    pub vaccination_stat: Option<String>,
    pub certification_state: Option<String>,
    pub personal_documentation: Option<String>,
    pub planned_child: Option<String>,
}

impl From<Candidate> for CandidateResponse {
    fn from(cand: Candidate) -> Self {
        let unique_factors = format!("{}:{}", cand.name, cand.location);

        let id = Sha256::digest(unique_factors.as_bytes())
            .to_vec()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        Self {
            id,
            name: cand.name,
            location: cand.location,
            qualification: cand.qualification,
            hours: cand.hours,
            mobility: cand.mobility,
            received_at: cand.received_at,
            notes: cand.notes,
            start_note: cand.start_note,
            sent_documents: cand.sent_documents,
            completed_checklist: cand.completed_checklist,
            vaccination_stat: cand.vaccination_stat,
            certification_state: cand.certification_state,
            personal_documentation: cand.personal_documentation,
            planned_child: cand.planned_child,
        }
    }
}

impl CandidateResponse {
    #[cfg(test)]
    pub fn new_mock(loc: &str) -> Self {
        Self {
            id: "123".to_string(),
            name: "Max Mustermann".to_string(),
            location: loc.to_string(),
            qualification: Some("Erzieher".to_string()),
            hours: None,
            mobility: None,
            received_at: None,
            notes: None,
            start_note: None,
            sent_documents: None,
            completed_checklist: None,
            vaccination_stat: None,
            certification_state: None,
            personal_documentation: None,
            planned_child: None,
        }
    }
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildCareRequestResponse {
    pub id: String,
    pub institution: String,
    pub location: String,
    pub grade: String,
    pub hours: String,
    pub diagnosis: Option<String>,
    pub contact: String,
    pub received_at: Option<String>,
    pub notes: Option<String>,
}

impl From<ChildCareRequest> for ChildCareRequestResponse {
    fn from(req: ChildCareRequest) -> Self {
        let unique_factors = format!("{}:{}", req.institution, req.location);

        let id = Sha256::digest(unique_factors.as_bytes())
            .to_vec()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        Self {
            id,
            institution: req.institution,
            location: req.location,
            grade: req.grade,
            hours: req.hours,
            diagnosis: req.diagnosis,
            contact: req.contact,
            received_at: req.received_at,
            notes: req.notes,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandidateRequests {
    pub candidates: Vec<CandidateResponse>,
    pub child_care_requests: Vec<ChildCareRequestResponse>,
}

impl CandidateRequests {
    pub fn new(candidates: Vec<CandidateResponse>, child_care_requests: Vec<ChildCareRequestResponse>) -> Self {
        Self {
            candidates,
            child_care_requests,
        }
    }

    #[cfg(test)]
    pub fn new_mock() -> Self {
        let cand_1 = CandidateResponse::new_mock("Berlin");
        let cand_2 = CandidateResponse::new_mock("Regensburg");
        let cand_3 = CandidateResponse::new_mock("Kareth");
        let cand_4 = CandidateResponse::new_mock("Maxhütte");

        let candidates = vec![cand_1, cand_2, cand_3, cand_4];

        Self {
            candidates,
            child_care_requests: vec![],
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
