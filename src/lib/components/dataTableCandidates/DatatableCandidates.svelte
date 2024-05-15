<script lang="ts">
	import Search from '$lib/components/dataTableCandidates/Search.svelte';
	import ThFilter from '$lib/components/dataTableCandidates/ThFilter.svelte';
	import ThSort from '$lib/components/dataTableCandidates/ThSort.svelte';
	import RowCount from '$lib/components/dataTableCandidates/RowCount.svelte';
	import RowsPerPage from '$lib/components/dataTableCandidates/RowsPerPage.svelte';
	import Pagination from '$lib/components/dataTableCandidates/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import type { Candidate, ResponseGetCandidates } from '$lib/models/models';
	import type { Readable } from 'svelte/store';

	export let candidates: Candidate[];

	let handler: DataHandler<Candidate> = new DataHandler(candidates, {
		rowsPerPage: 10
	});
	let rows: Readable<Candidate[]> = handler.getRows();
</script>

{#if rows}
	<div class=" overflow-x-auto space-y-2">
		<header class="flex justify-between gap-4">
			<!-- <Search {handler} /> -->
		</header>
		<table class="table table-hover table-compact table-auto w-full text-xs">
			<thead>
				<tr>
					<td>Name</td>
					<td>Ort</td>
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
			<tbody>
				{#each $rows as row}
					<tr>
						<td>{row.name ? row.name : '-'}</td>
						<td>{row.location ? row.location : '-'}</td>
						<td>{row.qualification ? row.qualification : '-'}</td>
						<td>{row.hours ? row.hours : '-'}</td>
						<td>{row.mobility ? row.mobility : '-'}</td>
						<td>{row.receivedAt ? row.receivedAt : '-'}</td>
						<td>{row.notes ? row.notes : '-'}</td>
						<td>{row.startNote ? row.startNote : '-'}</td>
						<td>{row.sentDocuments ? row.sentDocuments : '-'}</td>
						<td>{row.completedChecklist ? row.completedChecklist : '-'}</td>
						<td>{row.vaccinationStat ? row.vaccinationStat : '-'}</td>
						<td>{row.certificationState ? row.certificationState : '-'}</td>
						<td>{row.personalDocumentation ? row.personalDocumentation : '-'}</td>
						<td>{row.plannedChild ? row.plannedChild : '-'}</td>
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
