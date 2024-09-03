<script lang="ts">
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import { type ChildCareRequest, type ResponseGetChildcareMatches } from '$lib/models/models';
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
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		{#each visibleColumns as column}
			<td on:click={() => getMatches(childCareRequest.id)} class="cursor-pointer">
				{childCareRequest[column] === null ? '-' : childCareRequest[column]}
			</td>
		{/each}
	</tr>
</tbody>
