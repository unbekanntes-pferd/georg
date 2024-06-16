import { persisted, } from 'svelte-persisted-store'
import { get } from 'svelte/store';

export interface ExcelPathCommandStore {
  directories: {
      fileName: string;
      path: string;
      command: Command;
      state: State;
  }[];
}

export interface ExcelPathCommand {
	fileName: string;
	path: string | null;
	command: Command;
	state: State;
}

export enum Command {
	CandidatesAndChildCareRequests = 'import_candidate_data',
	SchoolAssistants ='import_assistant_data',
}

export enum State {
	Ok = 'ok',
	Error = 'error',
	Tbd = 'tbd'
}

const candidates = {
  fileName: 'Bewerbungen und Anfragen_Begleitungen.xlsx',
  path: '',
  command: Command.CandidatesAndChildCareRequests,
  state: State.Tbd
};

const schoolAssistants = {
  fileName: 'Daten_der_SchulbegleiterInnen_Test',
  path: '',
  command: Command.SchoolAssistants,
  state: State.Tbd
}

export const excelPathCommandStore = persisted('excelPathCommands', {
  directories: [candidates, schoolAssistants], 
})


export function setPath(fileName: string, path: string) {
  excelPathCommandStore.update(store => {
    const directory = store.directories.find(directory => directory.fileName === fileName);
    if (directory) {
      directory.path = path;
    }
    return store;
  })
}

export function setPathState(fileName: string, state: State) {
  excelPathCommandStore.update(store => {
    const directory = store.directories.find(directory => directory.fileName === fileName);
    if (directory) {
      directory.state = state;
    }
    return store;
  })
}

export function allPathsAreSet() {
  return get(excelPathCommandStore).directories.some(directory => {
    return directory.path !== '';
});
}

export function allStatesAreOk() {
  return get(excelPathCommandStore).directories.every(directory => {
    return directory.state === State.Ok;
});
}

