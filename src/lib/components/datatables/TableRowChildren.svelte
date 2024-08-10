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
	import type { ColumnKeyChildCare } from '$lib/stores/columnsChildcare';

	const drawerStore = getDrawerStore();
	export let childCareRequest: ChildCareRequest;
	export let visibleColumns: ColumnKeyChildCare[];
	
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
		{#each visibleColumns as column}
			<td>
				{#if column === 'id'}
					<button on:click={() => getMatches(childCareRequest.id)}><IconParkSolidConnection /></button>
				{:else}
					{childCareRequest[column] === null ? "-" : childCareRequest[column]}
				{/if}
			</td>
		{/each}
	</tr>
</tbody>
