use std::sync::Mutex;

use serde_json::error;

use crate::parse::CandidateRequests;

#[derive(thiserror::Error, Debug)]
pub enum GeorgError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Xlsx error: {0}")]
    Xlsx(#[from] calamine::XlsxError),
    #[error("Missing candidates file")]
    MissingCandidatesFile,
    #[error("Invalid candidate data")]
    InvalidCandidate,
    #[error("Unknown error")]
    Unknown
}

pub struct GeorgState {
    pub candidate_requests: Mutex<CandidateRequests>,
}

impl GeorgState {
    pub fn new() -> Self {
        let candidate_requests = CandidateRequests::new(vec![], vec![]);
        Self {
            candidate_requests: Mutex::new(candidate_requests),
        }
    }

    pub fn update(&self, candidate_requests: CandidateRequests) {
        *self.candidate_requests.lock().expect("poisoned mutex") = candidate_requests;
    }
}