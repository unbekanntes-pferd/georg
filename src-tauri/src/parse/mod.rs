use std::{
    path::{Path, PathBuf},
    time::SystemTime,
};

use crate::models::{AppState, GeorgError};

use self::{candidates::parse_candidate_data, models::ImportFiles};

use assistance::parse_assistant_data;
pub use models::{CandidateRequests, CandidateResponse, ChildCareRequestResponse, AssistantResponse};

pub(crate) mod assistance;
pub(crate) mod candidates;
mod models;

const CANDIDATE_XLSX_NAME: &str = "Bewerbungen_und_Anfragen_Begleitungen";
const ASSISTANTS_XLSX_NAME: &str = "Daten_der_SchulbegleiterInnen";
const ASSISTANCE_OVERVIEW_XLSX_NAME: &str = "Ubersichtsdaten_gesamte_Schulbegleitung";

#[tauri::command(async)]
pub fn import_candidate_data(path: String, state: tauri::State<AppState>) -> Result<(), String> {
    let path = Path::new(&path).to_owned(); // Ensure path is owned before moving into the thread

    get_import_files(&path)
        .map_err(|err| err.to_string())
        .and_then(move |import_files| {
            let candidate_data = parse_candidate_data(&import_files).map_err(|err| err.to_string())?;
            state.inner().inner().update_candidate_reqs(candidate_data);
            state
                .inner()
                .inner()
                .set_geo_codes()
                .map_err(|err| err.to_string())?;

            //sync_candidate_data(data).map_err(|err| err.to_string())?; TODO: implement writing DB
            Ok(())
        })
}

#[tauri::command(async)]
pub fn import_assistant_data(path: String, state: tauri::State<AppState>) -> Result<(), String> {
    let path = Path::new(&path).to_owned(); // Ensure path is owned before moving into the thread

    get_import_files(&path)
        .map_err(|err| err.to_string())
        .and_then(move |import_files| {
            let assitant_data = parse_assistant_data(&import_files).map_err(|err| err.to_string())?;
            state.inner().inner().update_assistant_data(assitant_data);
            state
                .inner()
                .inner()
                .set_geo_codes()
                .map_err(|err| err.to_string())?;

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

    let candidates_file = find_file(&xlsx_files, CANDIDATE_XLSX_NAME)
        .ok_or(GeorgError::MissingCandidatesFile)?;

    let assistants_file = find_file(&xlsx_files, ASSISTANTS_XLSX_NAME)
        .ok_or(GeorgError::MissingAssistantsFile)?;

    let assistance_overview_file = find_file(&xlsx_files, ASSISTANCE_OVERVIEW_XLSX_NAME)
        .ok_or(GeorgError::MissingAssistanceOverviewFile)?;

    Ok(ImportFiles::new(candidates_file, assistants_file, assistance_overview_file))
}

fn find_file(files: &[PathBuf], name: &str) -> Option<PathBuf> {
    files
        .iter()
        .find(|file| {
            file.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .starts_with(name)
        })
        .cloned()
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

        assert_eq!(
            import_files.assistants_file,
            path.join("Daten_der_SchulbegleiterInnen_Test.xlsx")
        );

        assert_eq!(
            import_files.assistance_overview_file,
            path.join("Ubersichtsdaten_gesamte_Schulbegleitung_Test.xlsx")
        );
    }

    #[test]
    fn test_get_import_files_no_files() {
        let path = std::path::Path::new("src");
        let import_files = get_import_files(path);
        assert!(import_files.is_err());
    }
}
