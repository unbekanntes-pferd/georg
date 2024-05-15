<script lang="ts">
	import Search from '$lib/components/dataTable/Search.svelte';
	import ThFilter from '$lib/components/dataTable/ThFilter.svelte';
	import ThSort from '$lib/components/dataTable/ThSort.svelte';
	import RowCount from '$lib/components/dataTable/RowCount.svelte';
	import RowsPerPage from '$lib/components/dataTable/RowsPerPage.svelte';
	import Pagination from '$lib/components/dataTable/Pagination.svelte';
	import { DataHandler } from '@vincjo/datatables';
	import { getCandidates, randomCandidates } from '$lib/data/data';
	import { onMount } from 'svelte';

	const handler = new DataHandler(randomCandidates(), { rowsPerPage: 5 });
	const rows = handler.getRows();

	onMount(async () => {
		const res = await getCandidates();
		console.log(res);
	});
</script>

<div class=" overflow-x-auto space-y-2">
	<header class="flex justify-between gap-4">
		<!-- <Search {handler} /> -->
	</header>
	<table class="table table-hover table-compact table-auto w-full">
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
					<td>{row.name}</td>
					<td>{row.location}</td>
					<td>{row.qualification}</td>
					<td>{row.hours}</td>
					<td>{row.mobility}</td>
					<td>{row.receivedAt}</td>
					<td>{row.notes}</td>
					<td>{row.startNote}</td>
					<td>{row.sentDocuments}</td>
					<td>{row.completedChecklist}</td>
					<td>{row.vaccinationStat}</td>
					<td>{row.certificationState}</td>
					<td>{row.personalDocumentation}</td>
					<td>{row.plannedChild}</td>
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
