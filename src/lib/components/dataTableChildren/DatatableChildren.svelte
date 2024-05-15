<script lang="ts">
	import Search from '$lib/components/dataTableChildren/Search.svelte';
	import ThFilter from '$lib/components/dataTableChildren/ThFilter.svelte';
	import ThSort from '$lib/components/dataTableChildren/ThSort.svelte';
	import RowCount from '$lib/components/dataTableChildren/RowCount.svelte';
	import RowsPerPage from '$lib/components/dataTableChildren/RowsPerPage.svelte';
	import Pagination from '$lib/components/dataTableChildren/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { ChildCareRequest } from '$lib/models/models';
	import type { Readable } from 'svelte/store';

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
			<!-- <Search {handler} /> -->
		</header>
		<table class="table table-hover table-compact table-auto w-full text-xs">
			<thead>
				<tr>
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
			<tbody>
				{#each $rows as row}
					<tr>
						<td>{row.institution ? row.institution : '-'}</td>
						<td>{row.location ? row.location : '-'}</td>
						<td>{row.grade ? row.grade : '-'}</td>
						<td>{row.hours ? row.hours : '-'}</td>
						<td>{row.diagnosis ? row.diagnosis : '-'}</td>
						<td>{row.contact ? row.contact : '-'}</td>
						<td>{row.receivedAt ? row.receivedAt : '-'}</td>
						<td>{row.notes ? row.notes : '-'}</td>
					</tr>
				{/each}
			</tbody>
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
