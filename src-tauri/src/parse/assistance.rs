use std::{fs::File, io::BufReader};

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

use crate::models::{AppState, GeorgError};
use super::models::{Assistant, AssistantResponse, ImportFiles};

const ASSISTANTS_SHEET_NAME: &str = "SB 23_24";

#[tauri::command]
pub fn get_assistant_data(state: tauri::State<AppState>) -> Result<Vec<AssistantResponse>, String> {
    let georg_state = state.inner().inner();
    let data = georg_state.assistant_data.lock().expect("poisoned lock");
    Ok(data.clone())
}

pub (crate) fn parse_assistant_data(import_files: &ImportFiles) -> Result<Vec<AssistantResponse>, GeorgError> {
    let path = &import_files.assistants_file;
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let assistants = parse_assistants(&mut workbook)?;

    let assistants = assistants.into_iter().map(|c| c.into()).collect();

    Ok(assistants)
}

fn parse_assistants(work_book: &mut Xlsx<BufReader<File>>) -> Result<Vec<Assistant>, GeorgError> {
    let mut assistants = Vec::new();
    let sheet = work_book.worksheet_range(ASSISTANTS_SHEET_NAME)?;

    let de = RangeDeserializerBuilder::new()
        .from_range(&sheet)
        .map_err(|_| GeorgError::MissingAssistantsFile)?;

    for row in de {
        if let Ok(assistant) = row.map_err(|err| {dbg!(err); GeorgError::InvalidAssistant}) {
            assistants.push(assistant);
        }
    }

    Ok(assistants)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::parse::get_import_files;

    use super::*;

    #[test]
    fn test_parse_assistants() {
        let import_files = get_import_files(Path::new("src/tests")).unwrap();
        let assistants = parse_assistant_data(&import_files).unwrap();
  
        assert_eq!(assistants.len(), 4);
    }
}