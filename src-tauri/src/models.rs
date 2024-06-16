use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use haversine::Location;
use photon_geocoding::{filter::ForwardFilter, PhotonApiClient};

use crate::parse::{AssistantResponse, CandidateRequests};

#[derive(thiserror::Error, Debug)]
pub enum GeorgError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Xlsx error: {0}")]
    Xlsx(#[from] calamine::XlsxError),
    #[error("Missing candidates file")]
    MissingCandidatesFile,
    #[error("Missing assistants file")]
    MissingAssistantsFile,
    #[error("Missing assistance overview file")]
    MissingAssistanceOverviewFile,
    #[error("Invalid candidate data")]
    InvalidCandidate,
    #[error("Invalid assistant data")]
    InvalidAssistant,
    #[error("Unknown error")]
    Unknown,
    #[error("Missing geo code")]
    MissingGeoCode,
}

#[derive(Debug, Clone)]
pub struct GeoCode {
    pub lat: f64,
    pub lon: f64,
}

impl GeoCode {
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

impl From<GeoCode> for Location {
    fn from(geo_code: GeoCode) -> Self {
        Self {
            latitude: geo_code.lat,
            longitude: geo_code.lon,
        }
    }
}

pub struct GeorgState {
    photon_api_client: PhotonApiClient,
    pub candidate_requests: Mutex<CandidateRequests>,
    pub assistant_data: Mutex<Vec<AssistantResponse>>,
    pub candidates_geo_codes: Mutex<HashMap<String, GeoCode>>,
    pub req_geo_codes: Mutex<HashMap<String, GeoCode>>,
    pub assistant_geo_codes: Mutex<HashMap<String, GeoCode>>,
}

pub struct AppState(Arc<GeorgState>);

impl AppState {
    pub fn new() -> Self {
        Self(Arc::new(GeorgState::new()))
    }

    pub fn inner(&self) -> Arc<GeorgState> {
        self.0.clone()
    }
}

impl GeorgState {
    pub fn new() -> Self {
        let candidate_requests = CandidateRequests::new(vec![], vec![]);
        let client = PhotonApiClient::default();
        Self {
            photon_api_client: client,
            candidate_requests: Mutex::new(candidate_requests),
            assistant_data: Mutex::new(Vec::new()),
            candidates_geo_codes: Mutex::new(HashMap::new()),
            req_geo_codes: Mutex::new(HashMap::new()),
            assistant_geo_codes: Mutex::new(HashMap::new()),
        }
    }

    pub fn update_candidate_reqs(&self, candidate_requests: CandidateRequests) {
        *self.candidate_requests.lock().expect("poisoned mutex") = candidate_requests;
    }

    pub fn update_assistant_data(&self, assistants_data: Vec<AssistantResponse>) {
        *self.assistant_data.lock().expect("poisoned mutex") = assistants_data;
    }

    pub fn set_geo_codes(&self) -> Result<(), GeorgError> {
        let mut geo_codes = self.candidates_geo_codes.lock().expect("poisoned mutex");
        let candidate_reqs = self.candidate_requests.lock().expect("poisoned mutex");
        let assistant_data = self.assistant_data.lock().expect("poisoned mutex");

        for candidate in candidate_reqs.candidates.iter() {
            let geo_code = self.get_geo_code(&candidate.location)?;
            geo_codes.insert(candidate.id.clone(), geo_code);
        }

        let mut req_geo_codes = self.req_geo_codes.lock().expect("poisoned mutex");

        for req in candidate_reqs.child_care_requests.iter() {
            let geo_code = self.get_geo_code(&req.location)?;
            req_geo_codes.insert(req.id.clone(), geo_code);
        }

        let mut assistant_geo_codes = self.assistant_geo_codes.lock().expect("poisoned mutex");

        for assistant in assistant_data.iter() {
            let geo_code = self.get_geo_code(&assistant.get_address())?;
            assistant_geo_codes.insert(assistant.id.clone(), geo_code);
        }

        Ok(())
    }

    fn get_geo_code(&self, search: &str) -> Result<GeoCode, GeorgError> {
        let filter = ForwardFilter::new().limit(1).language("de");

        let geo_code_results = self
            .photon_api_client
            .forward_search(search, Some(filter))
            .map_err(|err| {
                dbg!(err);
                GeorgError::Unknown
            })?;

        if geo_code_results.is_empty() {
            return Err(GeorgError::Unknown);
        }

        let geo_code = geo_code_results
            .first()
            .expect("no geo code found - checked before");

        Ok(GeoCode::new(geo_code.coords.lat, geo_code.coords.lon))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::CandidateRequests;

    use super::GeorgState;

    #[test]
    fn test_set_candidates_geo_codes() {
        let state = GeorgState::new();
        let candidate_requests = CandidateRequests::new_mock();
        state.update_candidate_reqs(candidate_requests);
        state.set_geo_codes().unwrap();

        let candidates_geo_codes = state.candidates_geo_codes.lock().expect("poisoned mutex");

        assert_eq!(candidates_geo_codes.len(), 4);

        assert!(candidates_geo_codes
            .values()
            .all(|geo_code| geo_code.lat != 0.0 && geo_code.lon != 0.0));
    }
}
