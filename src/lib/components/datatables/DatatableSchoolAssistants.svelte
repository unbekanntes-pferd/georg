<script lang="ts">
	import Search from '$lib/components/datatables/Search.svelte';
	import ThFilter from '$lib/components/datatables/ThFilter.svelte';
	import ThSort from '$lib/components/datatables/ThSort.svelte';
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { SchoolAssistant } from '$lib/models/models';
	import type { Readable } from 'svelte/store';
	import TableRow from '$lib/components/datatables/TableRowSchoolassistants.svelte';
	export let schoolAssistants: SchoolAssistant[];

	let handler: DataHandler<SchoolAssistant> = new DataHandler(schoolAssistants, {
		rowsPerPage: 10
	});
	let rows: Readable<SchoolAssistant[]> = handler.getRows();
</script>

{#if rows}
	<div class=" overflow-x-auto space-y-2">
		<header class="flex justify-between gap-4">
			<Search {handler} />
		</header>
		<table class="table table-hover table-compact table-auto w-full text-xs">
			<thead>
				<tr>
					<td>Match</td>
					<ThSort {handler} orderBy="name">Name</ThSort>
					<ThSort {handler} orderBy="location">Vorname</ThSort>
					<td>Geb. Datum</td>
					<td>Begl. Kind</td>
					<td>Telefon</td>
					<td>Mobil</td>
					<td>Email </td>
					<td>Straße</td>
					<td>PLZ</td>
					<td>Wohnort</td>
					<td>Eigr.</td>
					<td>genehm</td>
					<td>Info</td>
					<td>Abschlusszeugnisse</td>
                    <td>Berufsbezeichnung / Ausbildung</td>
                    <td>Kinder</td>
                    <td>Zugehörigkeit</td>
				</tr>
			</thead>

			{#each $rows as row}
				<TableRow schoolAssistant={row} schoolAssistants={schoolAssistants}/>
			{/each}
		</table>
		<footer class="flex justify-end">
			<RowsPerPage {handler} />
			<RowCount {handler} />
			<Pagination {handler} />
		</footer>
	</div>
{/if}

<style lang="scss">
	td {
		@apply p-4;
	}

	table :global(thead) {
		position: sticky;
		inset-block-start: 0;
		z-index: 1;
	}
</style>
