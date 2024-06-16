<script lang="ts">
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { ChildCareRequest } from '$lib/models/models';
	import type { Readable } from 'svelte/store';
	import TableRow from '$lib/components/datatables/TableRowChildren.svelte';
	import Search from '$lib/components/datatables/Search.svelte';

	export let childCareRequests: ChildCareRequest[];
	console.log(childCareRequests);
	let handler: DataHandler<ChildCareRequest> = new DataHandler(childCareRequests, {
		rowsPerPage: 10
	});
	let rows: Readable<ChildCareRequest[]> = handler.getRows();
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
					<td>Einrichtung</td>
					<td>Ort</td>
					<td>Klasse.</td>
					<td>Stunden</td>
					<td>Diagnose</td>
					<td>Ansprechpartner*in</td>
					<td>Datum </td>
					<td>Bemerkung</td>
				</tr>
			</thead>
			
				{#each $rows as row}
					<TableRow childCareRequest={row} />
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
</style>
