import { type ResponseGetAccompaniedChildMatches, type ResponseGetChildcareMatches, type ResponseGetSchoolAssistantMatches, type Assistant } from '$lib/models/models';
import { invoke } from '@tauri-apps/api';
import { type ResponseGetCandidates } from '$lib/models/models';
import type { Command } from '$lib/stores/path';

export async function getCandidates(): Promise<ResponseGetCandidates> {
	try {
		const res: ResponseGetCandidates = await invoke('get_candidate_data');
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function findCandidateMatches(id: string): Promise<ResponseGetCandidates[]> {
	try {
		const res: ResponseGetCandidates[] = await invoke('find_candidate_matches', { id });
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}


export async function findChildcareReqMatches(id: string): Promise<ResponseGetChildcareMatches[]> {
	try {
		const res: ResponseGetChildcareMatches[] = await invoke('find_childcare_req_matches', { id });
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function getAssistantData() {
	try {
	 	const res: Assistant[] = await invoke('get_assistant_data');
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}



export async function findSAssistantMatches(id: string) {
	try {
		const res: ResponseGetSchoolAssistantMatches[] = await invoke('find_assistant_matches', { id });
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function findAccompaniedChildrenMatches(id: string) {
	try {
		const res: ResponseGetAccompaniedChildMatches[] = await invoke('find_accompanied_children_matches', { id });
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function invokePathCommand(command: Command, path: string) {
	try {
		await invoke(command, { path });
	} catch (error) {
		console.error(error);
		throw error;
	}
}

