<script lang="ts">
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Search from '$lib/components/datatables/Search.svelte';
	import TableRow from '$lib/components/datatables/TableRowAccompaniedChildren.svelte';
	import ThSort from '$lib/components/datatables/ThSort.svelte';
	import type { AccompaniedChild } from '$lib/models/models';
	import { DataHandler } from '@vincjo/datatables';
	import type { Readable } from 'svelte/store';
	export let accompaniedChildren: AccompaniedChild[];

	let handler: DataHandler<AccompaniedChild> = new DataHandler(accompaniedChildren, {
		rowsPerPage: 10
	});
	let rows: Readable<AccompaniedChild[]> = handler.getRows();
</script>

{#if rows}
	<div class=" overflow-x-auto space-y-2">
		<header class="flex justify-between gap-4">
			<Search {handler} />
		</header>
		<table class="table table-hover table-compact table-auto w-full text-base">
			<thead>
				<tr>
					<td>Match</td>
					<ThSort {handler} orderBy="name">Name</ThSort>
					<td>Qualifikation</td>
					<td>Bescheid bis</td>
					<td>Kostenträger</td>
					<td>Ansprechpartner*in</td>
					<td>Telefon</td>
					<td>Email</td>
					<td>Bemerkung</td>
				</tr>
			</thead>

			{#each $rows as row}
				<TableRow accompaniedChild={row} />
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
