<script lang="ts">
	import IconParkSolidConnection from '~icons/icon-park-solid/connection';
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import {
		generateChildCareRequests,
		type Candidate,
		type ChildCareRequest
	} from '$lib/models/models';

	const drawerStore = getDrawerStore();
	export let candidate: Candidate;

	let isOpenMatches = false;
	let matches: ChildCareRequest[];

	let drawerSettings: DrawerSettings = {
		id: 'matchCandidatesToChildCareRequests',
		position: 'right',
		width: 'w-1/2'
	};

	function getMatches() {
		isOpenMatches = !isOpenMatches;
		matches = generateChildCareRequests(5);
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		<td on:click={getMatches}><IconParkSolidConnection /></td>
		<td>{candidate.name ? candidate.name : '-'}</td>
		<td>{candidate.location ? candidate.location : '-'}</td>
		<td>{candidate.qualification ? candidate.qualification : '-'}</td>
		<td>{candidate.hours ? candidate.hours : '-'}</td>
		<td>{candidate.mobility ? candidate.mobility : '-'}</td>
		<td>{candidate.receivedAt ? candidate.receivedAt : '-'}</td>
		<td>{candidate.notes ? candidate.notes : '-'}</td>
		<td>{candidate.startNote ? candidate.startNote : '-'}</td>
		<td>{candidate.sentDocuments ? candidate.sentDocuments : '-'}</td>
		<td>{candidate.completedChecklist ? candidate.completedChecklist : '-'}</td>
		<td>{candidate.vaccinationStat ? candidate.vaccinationStat : '-'}</td>
		<td>{candidate.certificationState ? candidate.certificationState : '-'}</td>
		<td>{candidate.personalDocumentation ? candidate.personalDocumentation : '-'}</td>
		<td>{candidate.plannedChild ? candidate.plannedChild : '-'}</td>
	</tr>
</tbody>
