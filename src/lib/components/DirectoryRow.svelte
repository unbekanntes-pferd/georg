<script lang="ts">
    import { open } from '@tauri-apps/api/dialog';
	import { invoke } from '@tauri-apps/api/tauri';
	import ClarityDirectorySolid from '~icons/clarity/directory-solid';
	import MaterialSymbolsDirectorySyncRounded from '~icons/material-symbols/directory-sync-rounded';
	import { onMount } from 'svelte';
	import { State, setPath, setPathState, type ExcelPathCommand } from '$lib/stores/path';
	import { invokePathCommand } from '$lib/data/data';
    
    export let row: ExcelPathCommand;
    
    let loading = false;

    async function handleOpenDirectory(directory: ExcelPathCommand) {
		const selectedDirectory = await open({
			directory: true,
			multiple: false
		});

		if (selectedDirectory === null) {
			// user cancelled the selection
        } else {
            try {
                loading = true;
                await invokePathCommand(directory.command, selectedDirectory as string)
                
                if (!Array.isArray(selectedDirectory)) {
                    setPath(row.fileName, selectedDirectory)
                    setPathState(row.fileName, State.Ok)
                }
            } catch (error) {
                setPathState(row.fileName, State.Error)
                console.error(error);
            } finally {
                loading = false;
            }
        }
	}
    // check on mounting if path was saved in backend
    onMount(() => {
        console.log(row);
    });
</script>


<tr>
    {#key loading}
    <td>{row.fileName}</td>
    {#if row.path && !loading}
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
    {/key}
    <td>
        {#if row.state === State.Ok && !loading}
            <span class="text-green-500">OK</span>
        {:else if row.state === State.Error}
            <span class="text-red-500">Fehler</span>
        {:else if loading }
            <span class="text-yellow-500">Ausstehend</span>
        {/if}
    </td>
</tr>