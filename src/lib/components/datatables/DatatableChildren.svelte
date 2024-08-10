<script lang="ts">
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { ChildCareRequest } from '$lib/models/models';
	import type { Readable } from 'svelte/store';
	import TableRow from '$lib/components/datatables/TableRowChildren.svelte';
	import Search from '$lib/components/datatables/Search.svelte';
	import { visibleChildCareColumns as visibleColumns, columnLabelsChildCare as columnLabels, defaultChildCareColumns, type ColumnKeyChildCare, columnLabelsChildCare, columnPositions, ColumnKeyChildCareEnum } from '$lib/stores/columnsChildcare';
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { onMount } from 'svelte';
	import ThSort from './ThSort.svelte';
	import ColumnToggle from './ColumnToggle.svelte';
	import { ColumnType } from '$lib/stores/models';

	export let childCareRequests: ChildCareRequest[];
	let handler: DataHandler<ChildCareRequest> = new DataHandler(childCareRequests, {
		rowsPerPage: 10
	});
	let rows: Readable<ChildCareRequest[]> = handler.getRows();

	$: selectedColumns = $visibleColumns.defaultColumns;
	
	function sortable(column: ColumnKeyChildCare) {
		let sortables: ColumnKeyChildCare[] = [ColumnKeyChildCareEnum.contact, ColumnKeyChildCareEnum.institution, ColumnKeyChildCareEnum.location];
		return sortables.includes(column);
	}
</script>

{#if rows}
	<div class="overflow-x-auto space-y-2 h-full">
		<header class="flex justify-between gap-4">
			<Search {handler} />
			<ColumnToggle columns={defaultChildCareColumns} {columnLabels}  columnType={ColumnType.ChildCare}/>
		</header>
		<table class="table table-hover table-compact table-auto w-full text-base">
			<thead>
				<tr>
					{#each selectedColumns as column}
					<th class="relative group">
						{#if sortable(column)}
							<ThSort {handler} orderBy={column}>{columnLabels[column]}</ThSort>
						{:else}
							{columnLabels[column]}
						{/if}
						
					</th>
					{/each}
				</tr>
			</thead>
			
			{#each $rows as row}
				<TableRow childCareRequest={row} visibleColumns={selectedColumns} />
			{/each}
		</table>
		<footer class="flex justify-end">
			<RowsPerPage {handler} />
			<RowCount {handler} />
			<Pagination {handler} />
		</footer>
	</div>
{/if}
<!-- svelte-ignore css-unused-selector -->
<style lang="scss">
	
	td {
		@apply p-4;
	}
</style>
