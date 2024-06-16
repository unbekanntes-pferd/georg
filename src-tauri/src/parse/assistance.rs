use std::{fs::File, io::BufReader};

use calamine::{open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

use crate::models::GeorgError;
use super::models::{Assistant, AssistantResponse, ImportFiles};

const ASSISTANTS_SHEET_NAME: &str = "SB 23_24";

pub (crate) fn parse_assistant_data(import_files: &ImportFiles) -> Result<Vec<AssistantResponse>, GeorgError> {
    let path = &import_files.candidates_file;
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
        .map_err(|_| GeorgError::MissingCandidatesFile)?;

    for row in de {
        if let Ok(assistant) = row.map_err(|_| GeorgError::InvalidAssistant) {
            assistants.push(assistant);
        }
    }

    Ok(assistants)
}