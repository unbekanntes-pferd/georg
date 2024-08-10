<script lang="ts">
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { Candidate } from '$lib/models/models';
	import type { Readable } from 'svelte/store';
	import TableRow from '$lib/components/datatables/TableRowCandidates.svelte';
	import Search from '$lib/components/datatables/Search.svelte';
	import { visibleCandidateColumns as visibleColumns, columnLabelsCandidate as columnLabels, defaultCandidateColumns, type ColumnKeyCandidate, ColumnKeyCandidateEnum } from '$lib/stores/columnsCandidate';
	import ThSort from './ThSort.svelte';
	import ColumnToggle from './ColumnToggle.svelte';
	import { ColumnType } from '$lib/stores/models';

	export let candidates: Candidate[];
	let handler: DataHandler<Candidate> = new DataHandler(candidates, {
		rowsPerPage: 10
	});
	let rows: Readable<Candidate[]> = handler.getRows();

	$: selectedColumns = $visibleColumns.defaultColumns;

	function sortable(column: ColumnKeyCandidate) {
		let sortables: ColumnKeyCandidate[] = [ColumnKeyCandidateEnum.name, ColumnKeyCandidateEnum.location];
		return sortables.includes(column);
	}

</script>

{#if rows}
	<div class="overflow-x-auto overflow-y-visible space-y-2 h-full">
		<header class="flex justify-between gap-4">
			<Search {handler} />
			<ColumnToggle columns={defaultCandidateColumns} {columnLabels} columnType={ColumnType.Candidate} />
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
				<TableRow candidate={row} visibleColumns={selectedColumns} />
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
	th, td {
		@apply p-4;
	}
</style>
