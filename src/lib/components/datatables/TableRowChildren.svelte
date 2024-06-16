<script lang="ts">
	import IconParkSolidConnection from '~icons/icon-park-solid/connection';
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import {
		type Candidate,
		type ChildCareRequest,
		type ResponseGetCandidates,

		type ResponseGetChildcareMatches


	} from '$lib/models/models';
	import { findChildcareReqMatches } from '$lib/data/data';

	const drawerStore = getDrawerStore();
	export let childCareRequest: ChildCareRequest;

	let isOpenMatches = false;
	let matches: ResponseGetChildcareMatches[];

	let drawerSettings: DrawerSettings = {
		id: 'matchChildCareRequestsToCandidates',
		position: 'right',
		width: 'w-1/2'
	};

	async function getMatches(id: string) {
		isOpenMatches = !isOpenMatches;
		matches = await findChildcareReqMatches(id);
		console.log("matches", matches)
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		<td on:click={() => getMatches(childCareRequest.id)}><IconParkSolidConnection /></td>
		<td>{childCareRequest.institution ? childCareRequest.institution : '-'}</td>
						<td>{childCareRequest.location ? childCareRequest.location : '-'}</td>
						<td>{childCareRequest.grade ? childCareRequest.grade : '-'}</td>
						<td>{childCareRequest.hours ? childCareRequest.hours : '-'}</td>
						<td>{childCareRequest.diagnosis ? childCareRequest.diagnosis : '-'}</td>
						<td>{childCareRequest.contact ? childCareRequest.contact : '-'}</td>
						<td>{childCareRequest.receivedAt ? childCareRequest.receivedAt : '-'}</td>
						<td>{childCareRequest.notes ? childCareRequest.notes : '-'}</td>
	</tr>
</tbody>
