import { generateCandidates, type ResponseGetAssistantsAndAccompaniedChildren, type ResponseGetChildcareMatches, type ResponseGetSchoolAssistantMatches } from '$lib/models/models';
import { invoke } from '@tauri-apps/api';
import { type ResponseGetCandidates } from '$lib/models/models';

// used to generate random data
export function randomCandidates() {
	return generateCandidates(100);
}

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

export async function getSchoolAssistantsAndAccompaniedChildren() {
	try {
		const res: ResponseGetAssistantsAndAccompaniedChildren = await invoke('get_school_assistants_and_accompanied_children');
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}



export async function findSchoolAssistantMatches(id: string) {
	try {
		const res: ResponseGetSchoolAssistantMatches[] = await invoke('find_school_assistant_matches', { id });
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}

export async function findAccompaniedChildrenMatches(id: string) {
	try {
		const res: ResponseGetChildcareMatches[] = await invoke('find_accompanied_children_matches', { id });
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}