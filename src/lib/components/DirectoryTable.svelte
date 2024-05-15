<script lang="ts">
	import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Command, State, type DirectoryPathCommand } from './directoryTableModals';

	let candidates = {
		fileName: 'Bewerbungen und Anfragen_Begleitungen.xlsx',
		path: null,
		command: Command.CandidatesAndChildCareRequests,
		state: State.Tbd
	};

	let directories: DirectoryPathCommand[] = [candidates];

	async function handleOpenDirectory(directory: DirectoryPathCommand) {
		const selectedDirectory = await open({
			directory: true,
			multiple: false
		});

		if (selectedDirectory === null) {
			// user cancelled the selection
		} else {
			// todo remove later
			console.log(selectedDirectory);
			if (!Array.isArray(selectedDirectory) && directories) {
				const foundDirectory = directories.find((d) => d.fileName === directory.fileName);
				if (foundDirectory) {
					foundDirectory.path = selectedDirectory;
					foundDirectory.state = State.Ok;
					directories = directories;
				}
			}

			invoke(directory.command, {
				path: selectedDirectory
			})
				.then((message) => {
					if (!Array.isArray(selectedDirectory)) {
						directory.path = selectedDirectory;
					}
					directory.state = State.Ok;
					console.log(message);
				})
				.catch((error) => {
					directory.state = State.Error;
					console.error(error);
				});
		}
	}
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
					<tr>
						<td>{row.fileName}</td>
						{#if row.path}
							<td>{row.path}</td>
						{:else}
							<td
								><button on:click={() => handleOpenDirectory(row)} class="text-yellow-500"
									>Noch kein Pfad angegeben - Auswählen</button
								></td
							>
						{/if}
						<td>
							{#if row.state === State.Ok}
								<span class="text-green-500">OK</span>
							{:else if row.state === State.Error}
								<span class="text-red-500">Fehler</span>
							{:else}
								<span class="text-yellow-500">Ausstehend</span>
							{/if}
						</td>
					</tr>
				{/each}
			{/key}
		</tbody>
	</table>
</div>
