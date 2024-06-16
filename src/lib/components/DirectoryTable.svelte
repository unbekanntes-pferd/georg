<script lang="ts">
	import { Command, State, type DirectoryPathCommand } from './directoryTableModals';
	import DirectoryRow from './DirectoryRow.svelte';

	let candidates = {
		fileName: 'Bewerbungen und Anfragen_Begleitungen.xlsx',
		path: null,
		command: Command.CandidatesAndChildCareRequests,
		state: State.Tbd
	};

	let schoolAssistants = {
		fileName: 'Daten der SchulbegleiterInnen_Test Dracoon.xlsx',
		path: null,
		command: Command.SchoolAssistants,
		state: State.Tbd
	}

	let directories: DirectoryPathCommand[] = [candidates, schoolAssistants];

	
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
