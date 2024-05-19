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

    let mut distances = calculate_distances(candidate_geo_code, reqs_geo_codes);
    distances.sort_by(|(_, distance), (_, other_distance)| {
        distance
            .partial_cmp(other_distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let data_clone = data.clone();

    let matches = build_childcare_req_matches(distances, data_clone);

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

    let mut matches = calculate_distances(req_geo_code, candidates_geo_codes);

    matches.sort_by(|(_, distance), (_, other_distance)| {
        distance
            .partial_cmp(other_distance)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let data_clone = data.clone();

    let matches = build_candidate_matches(matches, data_clone);

    Ok(matches)

}

fn calculate_distances(
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
    use std::sync::Arc;

    use crate::{geo::{build_candidate_matches, build_childcare_req_matches, calculate_distances}, models::{GeoCode, GeorgState}, parse::CandidateRequests};


    #[test]
    fn test_find_matches_no_distances() {

        let candidate = GeoCode::new(52.5200, 13.4050);
        let reqs = vec![
            (String::from("1"), GeoCode::new(52.5200, 13.4050)),
            (String::from("2"), GeoCode::new(52.5200, 13.4050)),
            (String::from("3"), GeoCode::new(52.5200, 13.4050)),
        ];

        let matches = calculate_distances(candidate, reqs);

        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].0, "1");
        assert_eq!(matches[1].0, "2");
        assert_eq!(matches[2].0, "3");
        assert_eq!(matches[0].1, 0.0);
        assert_eq!(matches[1].1, 0.0);
        assert_eq!(matches[2].1, 0.0);
    }

    #[test]
    fn test_find_matches_distances() {
        let candidate = GeoCode::new(52.5200, 13.4050);
        let reqs = vec![
            (String::from("1"), GeoCode::new(52.5200, 13.8050)),
            (String::from("2"), GeoCode::new(52.3200, 14.2050)),
            (String::from("3"), GeoCode::new(51.1200, 13.2050)),
        ];

        let matches = calculate_distances(candidate, reqs);

        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].0, "1");
        assert_eq!(matches[1].0, "2");
        assert_eq!(matches[2].0, "3");
        assert_eq!(matches[0].1, 27.064119317696267);
        assert_eq!(matches[1].1, 58.632262603064405);
        assert_eq!(matches[2].1, 156.278491353312);
    }

    #[test]
    fn test_build_childcare_req_matches() {
        let state = Arc::new(GeorgState::new());
        let candidate_requests = CandidateRequests::new_mock();
        state.update(candidate_requests);
        state.set_geo_codes().unwrap();

        let matches = vec![
            (String::from("1"), 0.0),
            (String::from("2"), 2.0),
            (String::from("3"), 5.0),
        ];

        let matches = build_childcare_req_matches(matches, state.clone());

        assert_eq!(matches.len(), 3);

        let match_1 = matches[0].clone();
        assert_eq!(match_1.distance, 0.0);
        assert_eq!(match_1.candidate.id, "1");
        assert_eq!(match_1.candidate.location, "Berlin");

        let match_2 = matches[1].clone();
        assert_eq!(match_2.distance, 2.0);
        assert_eq!(match_2.candidate.id, "2");
        assert_eq!(match_2.candidate.location, "Regensburg");

        let match_3 = matches[2].clone();
        assert_eq!(match_3.distance, 5.0);
        assert_eq!(match_3.candidate.id, "3");
        assert_eq!(match_3.candidate.location, "Kareth");

    }

    #[test]
    fn test_build_candidate_matches() {
        let state = Arc::new(GeorgState::new());
        let candidate_requests = CandidateRequests::new_mock();
        state.update(candidate_requests);
        state.set_geo_codes().unwrap();

        let matches = vec![
            (String::from("1"), 0.0),
            (String::from("2"), 2.0),
            (String::from("3"), 5.0),
        ];

        let matches = build_candidate_matches(matches, state.clone());

        assert_eq!(matches.len(), 3);

        let match_1 = matches[0].clone();
        assert_eq!(match_1.distance, 0.0);
        assert_eq!(match_1.candidate.id, "1");
        assert_eq!(match_1.candidate.location, "Berlin");

        let match_2 = matches[1].clone();
        assert_eq!(match_2.distance, 2.0);
        assert_eq!(match_2.candidate.id, "2");
        assert_eq!(match_2.candidate.location, "Regensburg");

        let match_3 = matches[2].clone();
        assert_eq!(match_3.distance, 5.0);
        assert_eq!(match_3.candidate.id, "3");
        assert_eq!(match_3.candidate.location, "Kareth");
    }
}