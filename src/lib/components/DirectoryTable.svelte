<script lang="ts">
	import { get } from 'svelte/store'
	import { excelPathCommandStore, type ExcelPathCommand } from '$lib/stores/path';
	import DirectoryRow from './DirectoryRow.svelte';	

	let directories: ExcelPathCommand[] = get(excelPathCommandStore).directories

	
</script>

<div class="table-container">
	<!-- Native Table Element -->
	<table class="table table-hover">
		<thead>
			<tr>
				<th>Datei</th>
				<th>Pfad</th>
				<th>Status</th>
			</tr>
		</thead>
		<tbody>
			{#key directories}
				{#each directories as row, i}
					<DirectoryRow row={row}/>
				{/each}
			{/key}
		</tbody>
	</table>
</div>
