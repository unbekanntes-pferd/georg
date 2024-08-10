<script lang="ts">
	import Search from '$lib/components/datatables/Search.svelte';
	import RowCount from '$lib/components/datatables/RowCount.svelte';
	import RowsPerPage from '$lib/components/datatables/RowsPerPage.svelte';
	import Pagination from '$lib/components/datatables/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { Assistant } from '$lib/models/models';
	import type { Readable } from 'svelte/store';
	import TableRow from '$lib/components/datatables/TableRowSchoolassistants.svelte';
	import { visibleSchoolAssistantColumns as visibleColumns, columnLabelsSchoolAssistant as columnLabels, defaultSchoolAssistantColumns } from '$lib/stores/columnsSchoolAssistant';

	import ColumnToggle from './ColumnToggle.svelte';
	import { ColumnType } from '$lib/stores/models';

	export let schoolAssistants: Assistant[];

	let handler: DataHandler<Assistant> = new DataHandler(schoolAssistants, {
		rowsPerPage: 10
	});
	let rows: Readable<Assistant[]> = handler.getRows();

	$: selectedColumns = $visibleColumns.defaultColumns;

	</script>

{#if rows}
	<div class=" overflow-x-auto space-y-2 h-full" >
		<header class="flex justify-between gap-4">
			<Search {handler} />
			<ColumnToggle columns={defaultSchoolAssistantColumns} {columnLabels} columnType={ColumnType.SchoolAssistant}/>
		</header>
		<table class="table table-hover table-compact table-auto w-full text-base">
			<thead>
				<tr>
					{#each selectedColumns as column}
						<td class="relative group">
							{columnLabels[column]}
						</td>
					{/each}
				</tr>
			</thead>
			
			{#each $rows as row}
				<TableRow  schoolAssistant={row} visibleColumns={selectedColumns} />
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
