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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assistant {
    #[serde(rename(serialize = "lastName", deserialize = "Name"))]
    pub last_name: String,
    #[serde(rename(serialize = "firstName", deserialize = "Vorname"))]
    pub first_name: String,
    #[serde(rename(serialize = "birthDate", deserialize = "Geb.datum"))]
    pub birth_date: String,
    #[serde(rename(serialize = "assignedChild", deserialize = "Begl. Kind"))]
    pub assigned_child: String,
    #[serde(rename(serialize = "telNumber", deserialize = "Telefon"))]
    pub tel_number: Option<String>,
    #[serde(rename(serialize = "mobileNumber", deserialize = "Mobil"))]
    pub mobile_number: Option<String>,
    #[serde(rename(serialize = "email", deserialize = "Email"))]
    pub email: Option<String>,
    #[serde(rename(serialize = "address", deserialize = "Straße/Hausnummer"))]
    pub address: String,
    #[serde(rename(serialize = "zipCode", deserialize = "PLZ"))]
    pub zip_code: f32,
    #[serde(rename(serialize = "city", deserialize = "Wohnort"))]
    pub city: String,
    #[serde(rename(serialize = "level", deserialize = "Eingr."))]
    pub level: String,
    #[serde(rename(serialize = "approved", deserialize = "genehmigt"))]
    pub approved: String,
    #[serde(rename(serialize = "info", deserialize = "Info"))]
    pub info: Option<String>,
    #[serde(rename(serialize = "certifications", deserialize = "Abschlusszeugnisse"))]
    pub certifications: String,
    #[serde(rename(
        serialize = "title",
        deserialize = "Berufsbezeichnung/Ausbildung/Abschluss"
    ))]
    pub title: Option<String>,
    #[serde(rename(serialize = "children", deserialize = "Kinder"))]
    pub children: Option<String>,
    #[serde(rename(serialize = "assitantSince", deserialize = "Zugehörigkeit"))]
    pub assitant_since: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantResponse {
    pub id: String,
    pub last_name: String,
    pub first_name: String,
    pub birth_date: String,
    pub assigned_child: String,
    pub tel_number: Option<String>,
    pub mobile_number: Option<String>,
    pub email: Option<String>,
    pub address: String,
    pub zip_code: f32,
    pub city: String,
    pub level: String,
    pub approved: String,
    pub info: Option<String>,
    pub certifications: String,
    pub title: Option<String>,
    pub children: Option<String>,
    pub assitant_since: Option<String>,
}

impl AssistantResponse {
    pub fn get_address(&self) -> String {
        format!("{} {} {}", self.address, self.zip_code, self.city)
    }
}

impl From<Assistant> for AssistantResponse {
    fn from(assistant: Assistant) -> Self {
        let unique_factors = format!(
            "{}:{}:{}",
            assistant.last_name, assistant.first_name, assistant.zip_code
        );

        let id = generate_unique_id(&unique_factors);

        Self {
            id,
            last_name: assistant.last_name,
            first_name: assistant.first_name,
            birth_date: assistant.birth_date,
            assigned_child: assistant.assigned_child,
            tel_number: assistant.tel_number,
            mobile_number: assistant.mobile_number,
            email: assistant.email,
            address: assistant.address,
            zip_code: assistant.zip_code,
            city: assistant.city,
            level: assistant.level,
            approved: assistant.approved,
            info: assistant.info,
            certifications: assistant.certifications,
            title: assistant.title,
            children: assistant.children,
            assitant_since: assistant.assitant_since,
        }
    }
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

        let id = generate_unique_id(&unique_factors);

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
    pub fn new_mock(id: &str, loc: &str) -> Self {
        Self {
            id: id.to_string(),
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

impl ChildCareRequestResponse {
    #[cfg(test)]
    pub fn new_mock(id: &str, loc: &str) -> Self {
        Self {
            id: id.to_string(),
            institution: "Kita Mockinghausen".to_string(),
            location: loc.to_string(),
            grade: "Klasse 1".to_string(),
            hours: "8".to_string(),
            diagnosis: Some("ADHS".to_string()),
            contact: "Frau Mock".to_string(),
            received_at: None,
            notes: None,
        }
    }
}

impl From<ChildCareRequest> for ChildCareRequestResponse {
    fn from(req: ChildCareRequest) -> Self {
        let unique_factors = format!("{}:{}", req.institution, req.location);

        let id = generate_unique_id(&unique_factors);

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
    pub fn new(
        candidates: Vec<CandidateResponse>,
        child_care_requests: Vec<ChildCareRequestResponse>,
    ) -> Self {
        Self {
            candidates,
            child_care_requests,
        }
    }

    #[cfg(test)]
    pub fn new_mock() -> Self {
        let cand_1 = CandidateResponse::new_mock("1", "Berlin");
        let cand_2 = CandidateResponse::new_mock("2", "Regensburg");
        let cand_3 = CandidateResponse::new_mock("3", "Kareth");
        let cand_4 = CandidateResponse::new_mock("4", "Maxhütte");

        let req_1 = ChildCareRequestResponse::new_mock("1", "Berlin");
        let req_2 = ChildCareRequestResponse::new_mock("2", "Regensburg");
        let req_3 = ChildCareRequestResponse::new_mock("3", "Kareth");
        let req_4 = ChildCareRequestResponse::new_mock("4", "Maxhütte");

        let candidates = vec![cand_1, cand_2, cand_3, cand_4];
        let child_care_requests = vec![req_1, req_2, req_3, req_4];

        Self {
            candidates,
            child_care_requests,
        }
    }
}

pub struct ImportFiles {
    pub candidates_file: PathBuf,
    pub assistants_file: PathBuf,
    pub assistance_overview_file: PathBuf,
}

impl ImportFiles {
    pub fn new(
        candidates_file: PathBuf,
        assistants_file: PathBuf,
        assistance_overview_file: PathBuf,
    ) -> Self {
        Self {
            candidates_file,
            assistants_file,
            assistance_overview_file,
        }
    }
}

pub fn generate_unique_id(unique_factors: &str) -> String {
    let hash = Sha256::digest(unique_factors.as_bytes());
    hash.iter().fold(String::new(), |mut acc, byte| {
        acc.push_str(&format!("{:02x}", byte));
        acc
    })
}
