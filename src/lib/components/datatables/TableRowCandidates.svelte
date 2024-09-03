<script lang="ts">
	import IconParkSolidConnection from '~icons/icon-park-solid/connection';
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import { type Candidate, type ResponseGetCandidates } from '$lib/models/models';
	import { findCandidateMatches } from '$lib/data/data';
	import type { ColumnKeyCandidate } from '$lib/stores/columnsCandidate';

	const drawerStore = getDrawerStore();
	export let candidate: Candidate;
	export let visibleColumns: ColumnKeyCandidate[];

	let isOpenMatches = false;
	let matches: ResponseGetCandidates[];

	let drawerSettings: DrawerSettings = {
		id: 'matchCandidatesToChildCareRequests',
		position: 'right',
		width: 'w-1/2'
	};

	async function getMatches(id: string) {
		isOpenMatches = !isOpenMatches;
		matches = await findCandidateMatches(id);
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		{#each visibleColumns as column}
			<td on:click={() => getMatches(candidate.id)} class="cursor-pointer">
				{candidate[column] === null ? '-' : candidate[column]}
			</td>
		{/each}
	</tr>
</tbody>
