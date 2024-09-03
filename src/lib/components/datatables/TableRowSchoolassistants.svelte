<script lang="ts">
	import IconParkSolidConnection from '~icons/icon-park-solid/connection';
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import { type ResponseGetSchoolAssistantMatches, type Assistant } from '$lib/models/models';
	import { findSAssistantMatches } from '$lib/data/data';
	import type { ColumnKeySchoolAssistant } from '$lib/stores/columnsSchoolAssistant';

	const drawerStore = getDrawerStore();
	export let schoolAssistant: Assistant;
	export let visibleColumns: ColumnKeySchoolAssistant[];

	let isOpenMatches = false;
	let matches: ResponseGetSchoolAssistantMatches[];

	let drawerSettings: DrawerSettings = {
		id: 'matchSchoolAssistantsToSchoolAssistants',
		position: 'right',
		width: 'w-1/2'
	};

	async function getMatches(id: string) {
		isOpenMatches = !isOpenMatches;
		matches = await findSAssistantMatches(id);
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		{#each visibleColumns as column}
			<td on:click={() => getMatches(schoolAssistant.id)} class="cursor-pointer">
				{schoolAssistant[column] === null ? '-' : schoolAssistant[column]}
			</td>
		{/each}
	</tr>
</tbody>
