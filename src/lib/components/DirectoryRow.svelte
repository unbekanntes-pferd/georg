<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Command, State, type DirectoryPathCommand } from './directoryTableModals';
	import ClarityDirectorySolid from '~icons/clarity/directory-solid';
	import MaterialSymbolsDirectorySyncRounded from '~icons/material-symbols/directory-sync-rounded';
	import { onMount } from 'svelte';
    
    export let row: DirectoryPathCommand;
    
    let loading = false;

    // todo save path on backend and check if path was given prior to opening directory 
    async function handleOpenDirectory(directory: DirectoryPathCommand) {
		const selectedDirectory = await open({
			directory: true,
			multiple: false
		});

		if (selectedDirectory === null) {
			// user cancelled the selection
		} else {
            loading = true;
			invoke(directory.command, {
				path: selectedDirectory
			})
				.then((_message) => {
					if (!Array.isArray(selectedDirectory)) {
                        row.path = selectedDirectory;
                        row.state = State.Ok;
					}
				})
				.catch((error) => {
					row.state = State.Error;
					console.error(error);
				}).finally(() => {
                    loading = false;
                })
		}
	}
    // check on mounting if path was saved in backend
    onMount(() => {
        console.log(row);
    });
</script>


<tr>
    <td>{row.fileName}</td>
    {#if row.path}
        <td class="flex flew-row"
            >{row.path}
            <button
                on:click={() => handleOpenDirectory(row)}
                class="text-green-500 flex flex-row"
                ><MaterialSymbolsDirectorySyncRounded class="ml-2" /></button
            ></td
        >
    {:else if loading}
        <td class="flex flew-row"
            >
            <div
                class="text-yellow-500 flex flex-row"
                ><MaterialSymbolsDirectorySyncRounded class="ml-2 animate-spin" /></div
            ></td
        >
    {:else}
        <td
            ><button
                on:click={() => handleOpenDirectory(row)}
                class="text-yellow-500 flex flex-row"
                ><span>Noch kein Pfad angegeben</span><ClarityDirectorySolid
                    class="ml-2"
                /></button
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