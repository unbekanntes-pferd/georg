<script lang="ts">
	import Search from '$lib/components/datatables/Search.svelte';
	import ThFilter from '$lib/components/datatables/ThFilter.svelte';
	import ThSort from '$lib/components/datatables/ThSort.svelte';
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { Candidate } from '$lib/models/models';
	import type { Readable } from 'svelte/store';
	import TableRow from '$lib/components/datatables/TableRowCandidates.svelte';
	export let candidates: Candidate[];

	let handler: DataHandler<Candidate> = new DataHandler(candidates, {
		rowsPerPage: 10
	});
	let rows: Readable<Candidate[]> = handler.getRows();
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
					<ThSort {handler} orderBy="location">Ort</ThSort>
					<td>Qualif.</td>
					<td>Stundenumfang</td>
					<td>Mobilität</td>
					<td>Eingang</td>
					<td>Bemerkungen </td>
					<td>Geplanter Start</td>
					<td>Unterlagen versendet</td>
					<td>Checkliste komplett</td>
					<td>Massernschutz</td>
					<td>Führungszeugnis</td>
					<td>Personalbogen</td>
					<td>Geplantes Kind</td>
				</tr>
			</thead>

			{#each $rows as row}
				<TableRow candidate={row} />
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
