use std::sync::Arc;

use haversine::Location;
use tauri::State;

mod models;

use crate::models::{AppState, GeoCode, GeorgError, GeorgState};

use self::models::{CandidateMatch, ChildCareRequestMatch};

#[tauri::command(async)]
pub fn find_candidate_matches(
    id: String,
    state: State<AppState>,
) -> Result<Vec<ChildCareRequestMatch>, String> {
    let data = state.inner().inner();

    let reqs_geo_codes: Vec<_> = data
        .req_geo_codes
        .lock()
        .expect("poisoned mutex")
        .iter()
        .map(|(id, geo_code)| (id.clone(), geo_code.clone()))
        .collect();

    let candidate_geo_code = data
        .candidates_geo_codes
        .lock()
        .expect("poisoned mutex")
        .get(&id)
        .cloned()
        .ok_or(GeorgError::MissingGeoCode)
        .map_err(|err| err.to_string())?;

    let mut matches = find_matches(candidate_geo_code, reqs_geo_codes);
    matches.sort_by(|(_, distance), (_, other_distance)| {
        distance
            .partial_cmp(other_distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let data_clone = data.clone();

    let matches = build_childcare_req_matches(matches, data_clone);

    Ok(matches)
}

#[tauri::command(async)]
pub fn find_childcare_req_matches(
    id: String,
    state: State<AppState>
) -> Result<Vec<CandidateMatch>, String> {
    let data = state.inner().inner();

    let candidates_geo_codes: Vec<_> = data
        .candidates_geo_codes
        .lock()
        .expect("poisoned mutex")
        .iter()
        .map(|(id, geo_code)| (id.clone(), geo_code.clone()))
        .collect();

    let req_geo_code = data
        .req_geo_codes
        .lock()
        .expect("poisoned mutex")
        .get(&id)
        .cloned()
        .ok_or(GeorgError::MissingGeoCode)
        .map_err(|err| err.to_string())?;

    let mut matches = find_matches(req_geo_code, candidates_geo_codes);

    matches.sort_by(|(_, distance), (_, other_distance)| {
        distance
            .partial_cmp(other_distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let data_clone = data.clone();

    let matches = build_candidate_matches(matches, data_clone);

    Ok(matches)

}

fn find_matches(
    candidate: GeoCode,
    child_care_requests: Vec<(String, GeoCode)>,
) -> Vec<(String, f64)> {
    child_care_requests
        .iter()
        .map(|(id, geo_code)| {
            let loc_candidate: Location = candidate.clone().into();
            let loc_req: Location = geo_code.clone().into();
            let distance =
                haversine::distance(loc_candidate, loc_req, haversine::Units::Kilometers);
            (id.clone(), distance)
        })
        .collect()
}

fn build_childcare_req_matches(
    matches: Vec<(String, f64)>,
    data: Arc<GeorgState>
) -> Vec<ChildCareRequestMatch> {
    matches
        .iter()
        .filter_map(|(id, distance)| {
            let data = data
                .candidate_requests
                .lock()
                .expect("poisoned mutex");
            let Some(req) = data.child_care_requests.iter().find(|req| req.id == *id) else {
                return None;
            };
            Some(ChildCareRequestMatch::new(req.clone(), *distance))
        })
        .collect()
}

fn build_candidate_matches(
    matches: Vec<(String, f64)>,
    data: Arc<GeorgState>
) -> Vec<CandidateMatch> {
    matches
        .iter()
        .filter_map(|(id, distance)| {
            let data = data
                .candidate_requests
                .lock()
                .expect("poisoned mutex");
            let Some(candidate) = data.candidates.iter().find(|candidate| candidate.id == *id) else {
                return None;
            };
            Some(CandidateMatch::new(candidate.clone(), *distance))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{models::GeorgState, parse::CandidateRequests};


    #[test]
    fn test_build_matches() {

        let georg_state = GeorgState::new();
        let candidate_reqs = CandidateRequests::new_mock();

        georg_state.update(candidate_reqs);
        georg_state.set_geo_codes().unwrap();


    }
}