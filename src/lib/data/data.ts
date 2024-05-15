import { generateCandidates } from '$lib/models/models';
import { invoke } from '@tauri-apps/api';
import { type ResponseGetCandidates } from '$lib/models/models';

// used to generate random data
export function randomCandidates() {
	return generateCandidates(100);
}

export async function getCandidates(): Promise<ResponseGetCandidates> {
	try {
		let res: ResponseGetCandidates = await invoke('get_candidate_data');
		return res;
	} catch (error) {
		console.error(error);
		throw error;
	}
}