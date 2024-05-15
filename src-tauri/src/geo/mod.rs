use haversine::Location;

mod models;

use crate::models::{AppState, GeoCode, GeorgError};

use self::models::ChildCareRequestMatch;

#[tauri::command(async)]
pub fn find_candidate_matches(
    id: usize,
    state: tauri::State<AppState>,
) -> Result<Vec<ChildCareRequestMatch>, GeorgError> {
    let data = state.inner().inner();

    let reqs_geo_codes: Vec<_> = data
        .candidates_geo_codes
        .lock()
        .expect("poisoned mutex")
        .iter()
        .map(|(id, geo_code)| (*id, geo_code.clone()))
        .collect();

    let candidate_geo_code = data
        .candidates_geo_codes
        .lock()
        .expect("poisoned mutex")
        .get(&id)
        .cloned()
        .ok_or(GeorgError::MissingGeoCode)?;

    let matches = find_matches(candidate_geo_code, reqs_geo_codes);

    let data_clone = data.clone();

    let matches = matches
        .iter()
        .filter_map(|(id, distance)| {
            let data = data_clone
                .candidate_requests
                .lock()
                .expect("poisoned mutex");
            let Ok(req) = data.child_care_requests.get(*id).ok_or(GeorgError::Unknown) else {
                return None;
            };
            Some(ChildCareRequestMatch::new(req.clone(), *distance))
        })
        .collect();

    Ok(matches)
}

fn find_matches(
    candidate: GeoCode,
    child_care_requests: Vec<(usize, GeoCode)>,
) -> Vec<(usize, f64)> {
    child_care_requests
        .iter()
        .map(|(id, geo_code)| {
            let loc_candidate: Location = candidate.clone().into();
            let loc_req: Location = geo_code.clone().into();
            let distance =
                haversine::distance(loc_candidate, loc_req, haversine::Units::Kilometers);
            (*id, distance)
        })
        .collect()
}
