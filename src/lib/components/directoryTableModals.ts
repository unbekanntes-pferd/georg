export interface DirectoryPathCommand {
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
