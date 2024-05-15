use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::models::{AppState, GeorgError, GeorgState};

use self::{candidates::parse_candidate_data, models::ImportFiles};

pub use models::{CandidateRequests, ChildCareRequest, ChildCareRequestResponse, CandidateResponse};

pub(crate) mod candidates;
mod models;

const CANDIDATE_XLSX_NAME: &str = "Bewerbungen_und_Anfragen_Begleitungen";

#[tauri::command(async)]
pub fn import_candidate_data(path: String, state: tauri::State<AppState>) -> Result<(), String> {
    let path = Path::new(&path).to_owned(); // Ensure path is owned before moving into the thread

    get_import_files(&path)
        .map_err(|err| err.to_string())
        .and_then(move |import_files| {
            let data = parse_candidate_data(&import_files).map_err(|err| err.to_string())?;
            state.inner().inner().update(data);
            state.inner().inner().set_geo_codes().map_err(|err| err.to_string())?;

            //sync_candidate_data(data).map_err(|err| err.to_string())?; TODO: implement writing DB
            Ok(())
        })
}

fn get_import_files(path: &Path) -> Result<ImportFiles, GeorgError> {
    let files = std::fs::read_dir(path)?;

    // get all xlsx files
    let mut xlsx_files = files
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;

            let path = entry.path();

            match path.extension().and_then(|s| s.to_str()) {
                Some("xlsx") => Some(path),
                _ => None,
            }
        })
        .collect::<Vec<PathBuf>>();

    // sort them by updated date
    xlsx_files.sort_by_key(|file| {
        file.metadata()
            .map(|metadata| metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH))
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });

    let candidates_file = xlsx_files
        .iter()
        .find(|file| {
            file.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .starts_with(CANDIDATE_XLSX_NAME)
        })
        .ok_or(GeorgError::MissingCandidatesFile)?;

    Ok(ImportFiles::new(candidates_file.to_path_buf()))
}

#[cfg(test)]
mod tests {
    use crate::parse::get_import_files;

    #[test]
    fn test_get_import_files() {
        let path = std::path::Path::new("src/tests");
        let import_files = get_import_files(path).unwrap();
        assert_eq!(
            import_files.candidates_file,
            path.join("Bewerbungen_und_Anfragen_Begleitungen_Test.xlsx")
        );
    }

    #[test]
    fn test_get_import_files_no_files() {
        let path = std::path::Path::new("src");
        let import_files = get_import_files(path);
        assert!(import_files.is_err());
    }
}
