use std::{fs::File, io::BufReader};

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

const CANDIDATE_SHEET_NAME: &str = "Bewerber_innen";
const CHILD_CARE_SHEET_NAME: &str = "Anfragen Kinder";

use crate::models::{AppState, GeorgError};

use super::models::{Candidate, CandidateRequests, ChildCareRequest, ImportFiles};

#[tauri::command]
pub fn get_candidate_data(state: tauri::State<AppState>) -> Result<CandidateRequests, String> {
    let georg_state = state.inner().inner();
    let data = georg_state.candidate_requests.lock().expect("poisoned lock");
    Ok(data.clone())
}

pub (crate) fn parse_candidate_data(import_files: &ImportFiles) -> Result<CandidateRequests, GeorgError> {
    let path = &import_files.candidates_file;
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let candidates = parse_candidates(&mut workbook)?;
    let candidates = candidates.into_iter().map(|c| c.into()).collect();
    let child_care_requests = parse_child_care_requests(&mut workbook)?;
    let child_care_requests = child_care_requests.into_iter().map(|c| c.into()).collect();
    Ok(CandidateRequests::new(candidates, child_care_requests))
}

fn parse_candidates(work_book: &mut Xlsx<BufReader<File>>) -> Result<Vec<Candidate>, GeorgError> {
    let mut candidates = Vec::new();
    let sheet = work_book.worksheet_range(CANDIDATE_SHEET_NAME)?;

    let de = RangeDeserializerBuilder::new()
        .from_range(&sheet)
        .map_err(|_| GeorgError::MissingCandidatesFile)?;

    for row in de {
        if let Ok(candidate) = row.map_err(|_| GeorgError::InvalidCandidate) {
            candidates.push(candidate);
        }
    }

    Ok(candidates)
}

fn parse_child_care_requests(
    work_book: &mut Xlsx<BufReader<File>>,
) -> Result<Vec<ChildCareRequest>, GeorgError> {
    let mut child_care_requests = Vec::new();
    let sheet = work_book.worksheet_range(CHILD_CARE_SHEET_NAME)?;

    let de = RangeDeserializerBuilder::new()
        .from_range(&sheet)
        .map_err(|_| GeorgError::MissingCandidatesFile)?;

    for row in de {
        if let Ok(child_care_request) = row.map_err(|err| {dbg!(err); GeorgError::InvalidCandidate}) {
            child_care_requests.push(child_care_request);
        }
    }

    Ok(child_care_requests)
}

pub fn sync_candidate_data(data: CandidateRequests) -> Result<(), GeorgError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::parse::get_import_files;

    use super::parse_candidate_data;

    #[test]
    fn test_parse_candidates_data() {
        let import_files = get_import_files(Path::new("src/tests")).unwrap();

        let data = parse_candidate_data(&import_files).unwrap();

        assert_eq!(data.candidates.len(), 6);
        assert_eq!(data.child_care_requests.len(), 7);
    }
}
