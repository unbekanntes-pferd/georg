use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use haversine::Location;
use photon_geocoding::{filter::ForwardFilter, PhotonApiClient};

use crate::{
    db::GeoCodeRepository,
    parse::{AssistantResponse, CandidateRequests},
};

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
    pub candidate_requests: Arc<Mutex<CandidateRequests>>,
    pub assistant_data: Arc<Mutex<Vec<AssistantResponse>>>,
    pub candidates_geo_codes: Arc<Mutex<HashMap<String, GeoCode>>>,
    pub req_geo_codes: Arc<Mutex<HashMap<String, GeoCode>>>,
    pub assistant_geo_codes: Arc<Mutex<HashMap<String, GeoCode>>>,
    geo_repository: Arc<dyn GeoCodeRepository>,
}

pub struct AppState(Arc<GeorgState>);

impl AppState {
    pub fn new(geo_repository: Arc<dyn GeoCodeRepository>) -> Self {
        Self(Arc::new(GeorgState::new(geo_repository)))
    }

    pub fn inner(&self) -> Arc<GeorgState> {
        self.0.clone()
    }
}

pub enum GeoCodeType {
    Candidate,
    Request,
    Assistant,
}

impl GeorgState {
    pub fn new(geo_repository: Arc<dyn GeoCodeRepository>) -> Self {
        let candidate_requests = CandidateRequests::new(vec![], vec![]);
        let client = PhotonApiClient::default();

        Self {
            photon_api_client: client,
            candidate_requests: Arc::new(Mutex::new(candidate_requests)),
            assistant_data: Arc::new(Mutex::new(Vec::new())),
            candidates_geo_codes: Arc::new(Mutex::new(HashMap::new())),
            req_geo_codes: Arc::new(Mutex::new(HashMap::new())),
            assistant_geo_codes: Arc::new(Mutex::new(HashMap::new())),
            geo_repository,
        }
    }

    pub fn update_candidate_reqs(&self, candidate_requests: CandidateRequests) {
        *self.candidate_requests.lock().expect("poisoned mutex") = candidate_requests;
    }

    pub fn update_assistant_data(&self, assistants_data: Vec<AssistantResponse>) {
        *self.assistant_data.lock().expect("poisoned mutex") = assistants_data;
    }

    /// set geocodes for all candidates, requests and assistants
    pub async fn set_geo_codes(&self) -> Result<(), GeorgError> {
        // process candidates and quickly release lock
        {
            let candidate_ids_and_locations: Vec<(String, String)> = {
                let candidate_reqs = self.candidate_requests.lock().expect("poisoned mutex");
                candidate_reqs
                    .candidates
                    .iter()
                    .map(|c| (c.id.clone(), c.location.clone()))
                    .collect()
            };
            for (id, location) in candidate_ids_and_locations {
                self.process_geo_code(&id, &location, GeoCodeType::Candidate)
                    .await?
            }
        }

        // process requests and quickly release lock
        {
            let request_ids_and_locations: Vec<(String, String)> = {
                let candidate_reqs = self.candidate_requests.lock().expect("poisoned mutex");
                candidate_reqs
                    .child_care_requests
                    .iter()
                    .map(|r| (r.id.clone(), r.location.clone()))
                    .collect()
            };

            for (id, location) in request_ids_and_locations {
                self.process_geo_code(&id, &location, GeoCodeType::Request)
                    .await?
            }
        }

        // process assistants and quickly release lock
        {
            let assistant_ids_and_addresses: Vec<(String, String)> = {
                let assistant_data = self.assistant_data.lock().expect("poisoned mutex");
                assistant_data
                    .iter()
                    .map(|a| (a.id.clone(), a.get_address()))
                    .collect()
            };
            for (id, location) in assistant_ids_and_addresses {
                self.process_geo_code(&id, &location, GeoCodeType::Assistant)
                    .await?
            }
        }

        Ok(())
    }

    /// insert geocode into memory hashmap 
    /// either fetch from DB or photon API
    pub async fn process_geo_code(
        &self,
        id: &str,
        location: &str,
        geo_code_type: GeoCodeType,
    ) -> Result<(), GeorgError> {
        // check if geocode in DB
        if let Some(geo_code) = self.geo_repository.get_geo_location(id).await? {
            self.insert_geo_code(id, geo_code, geo_code_type).await;
            return Ok(());
        } else {
            // otherwise fetch from photon and insert into DB
            let geo_code = self.get_geo_code(location)?;
            self.geo_repository
                .insert_geo_location(id, &geo_code)
                .await?;
            self.insert_geo_code(id, geo_code, geo_code_type).await;
            return Ok(());
        }
    }

    /// insert geocode into memory hashmap
    pub async fn insert_geo_code(&self, id: &str, geo_code: GeoCode, geo_code_type: GeoCodeType) {
        let mut geo_codes = match geo_code_type {
            GeoCodeType::Candidate => self.candidates_geo_codes.lock().expect("poisoned mutex"),
            GeoCodeType::Request => self.req_geo_codes.lock().expect("poisoned mutex"),
            GeoCodeType::Assistant => self.assistant_geo_codes.lock().expect("poisoned mutex"),
        };

        geo_codes.insert(id.to_string(), geo_code);
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
    use std::sync::Arc;

    use crate::{db::MockGeoCodeRepository, parse::CandidateRequests};

    use super::GeorgState;

    #[tokio::test]
    async fn test_set_candidates_geo_codes() {
        let mock_repository = Arc::new(MockGeoCodeRepository::new());

        let state = GeorgState::new(mock_repository);
        let candidate_requests = CandidateRequests::new_mock();
        state.update_candidate_reqs(candidate_requests);
        state.set_geo_codes().await.unwrap();

        let candidates_geo_codes = state.candidates_geo_codes.lock().expect("poisoned mutex");

        assert_eq!(candidates_geo_codes.len(), 4);

        assert!(candidates_geo_codes
            .values()
            .all(|geo_code| geo_code.lat != 0.0 && geo_code.lon != 0.0));
    }
}
