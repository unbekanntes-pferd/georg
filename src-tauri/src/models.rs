use std::{collections::HashMap, sync::{Arc, Mutex}};

use haversine::Location;
use photon_geocoding::{error, filter::ForwardFilter, PhotonApiClient};

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
        Self {
            lat,
            lon,
        }
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
    pub candidate_requests: Mutex<CandidateRequests>,
    photon_api_client: PhotonApiClient,
    pub candidates_geo_codes: Mutex<HashMap<String, GeoCode>>,
    pub req_geo_codes: Mutex<HashMap<usize, GeoCode>>,
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
            candidate_requests: Mutex::new(candidate_requests),
            candidates_geo_codes: Mutex::new(HashMap::new()),
            req_geo_codes: Mutex::new(HashMap::new()),
            photon_api_client: client,
        }
    }

    pub fn update(&self, candidate_requests: CandidateRequests) {
        *self.candidate_requests.lock().expect("poisoned mutex") = candidate_requests;
    }

    pub fn set_geo_codes(&self) -> Result<(), GeorgError> {
        let mut geo_codes = self.candidates_geo_codes.lock().expect("poisoned mutex");
        let candidate_reqs = self.candidate_requests.lock().expect("poisoned mutex");

        for (index, candidate) in candidate_reqs.candidates.iter().enumerate() {
            let geo_code = self.get_geo_code(&candidate.location)?;
            geo_codes.insert(candidate.id.clone(), geo_code);
        }

        let mut req_geo_codes = self.req_geo_codes.lock().expect("poisoned mutex");

        for (index, req) in candidate_reqs.child_care_requests.iter().enumerate() {
            let geo_code = self.get_geo_code(&req.location)?;
            req_geo_codes.insert(index, geo_code);
        }
  
        Ok(())
    }

    fn get_geo_code(&self, search: &str) -> Result<GeoCode, GeorgError> {

        let filter = ForwardFilter::new()
            .limit(1)
            .language("de");
        

        let geo_code_results = self.photon_api_client.forward_search(search, Some(filter)).map_err(|err| {
            dbg!(err);
            GeorgError::Unknown
        })?;

        if geo_code_results.is_empty() {
            return Err(GeorgError::Unknown);
        }

        let geo_code = geo_code_results.first().expect("no geo code found - checked before");

        Ok(GeoCode::new(geo_code.coords.lat, geo_code.coords.lon))

    }


}

#[cfg(test)]
mod tests {
    use crate::parse::CandidateRequests;

    use super::GeorgState;

    #[test]
    fn test_set_geo_codes() {
        let state = GeorgState::new();
        let candidate_requests = CandidateRequests::new_mock();
        state.update(candidate_requests);
        state.set_geo_codes().unwrap();

        let candidates_geo_codes = state.candidates_geo_codes.lock().expect("poisoned mutex");

        assert_eq!(candidates_geo_codes.len(), 4);

        assert!(candidates_geo_codes.values().all(|geo_code| geo_code.lat != 0.0 && geo_code.lon != 0.0));
    }
}