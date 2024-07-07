import { persisted, } from 'svelte-persisted-store'
import { get } from 'svelte/store';

export interface ExcelPathCommandStore {
  directories: {
      purpose: string;
      path: string;
      command: Command;
      state: State;
  }[];
}

export interface ExcelPathCommand {
	purpose: string;
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
  purpose: 'Bewerbungen und Anfragen',
  path: '',
  command: Command.CandidatesAndChildCareRequests,
  state: State.Tbd
};

const schoolAssistants = {
  purpose: 'Daten der Schulbegleiter*innen und Übersicht der Schulbegleitungen',
  path: '',
  command: Command.SchoolAssistants,
  state: State.Tbd
}

export const excelPathCommandStore = persisted('excelPathCommands', {
  directories: [candidates, schoolAssistants], 
})


export function setPath(purpose: string, path: string) {
  excelPathCommandStore.update(store => {
    const directory = store.directories.find(directory => directory.purpose === purpose);
    if (directory) {
      directory.path = path;
    }
    return store;
  })
}

export function setPathState(purpose: string, state: State) {
  excelPathCommandStore.update(store => {
    const directory = store.directories.find(directory => directory.purpose === purpose);
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

