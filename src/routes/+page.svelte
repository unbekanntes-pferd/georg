<!-- YOU CAN DELETE EVERYTHING IN THIS PAGE -->
<script lang="ts">
	import { goto } from "$app/navigation";
	import logo from "$lib/assets/georg_logo.png";
	import { invokePathCommand } from "$lib/data/data";
	import { allPathsAreSet, allStatesAreOk, excelPathCommandStore, type ExcelPathCommand } from "$lib/stores/path";
	import { path } from "@tauri-apps/api";
	import { get } from "svelte/store";
	import { getToastStore, type ToastSettings } from '@skeletonlabs/skeleton';
	import { onMount } from "svelte";

	const toastStore = getToastStore();
	// Initially check if path are set in the store, if not redirect to settings
	// if path are set, send commands to the backend
	let loading = false;
	let pathSet = true;
	let statesOk = true;


	
	
	
	onMount(() => {
		if(!allPathsAreSet()) {
		const t: ToastSettings = {
			message: 'Für mindestens eine Exceldatei wurde kein Pfad gesetzt, überprüfe die Einstellungen',
			timeout: 5000
		};
		toastStore.trigger(t);
		pathSet = false;
	}

	if (!allStatesAreOk()) {
		const t: ToastSettings = {
			message: 'Eine Exceldatei konnte nicht gefunden werden, überprüfe die Pfade in den Einstellungen',
			timeout: 5000
		};
		toastStore.trigger(t);
		statesOk = false;
	}

	if (!pathSet || !statesOk) {
		goto("/settings");
	}


	
	let directories = get(excelPathCommandStore).directories;
	loading = true;
	directories.forEach(async (directory: ExcelPathCommand) => {
		await invokePathCommand(directory.command, directory.path!);
	});
	loading = false;
	
	});

</script>
<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-2 text-center flex flex-col items-center">
		<h2 class="h2 mb-8">Willkommen bei GEOrg</h2>
		<img src={logo} alt="georg-logo" width="200px">
		{#if loading}
			<p class="text-lg">Lade Daten...</p>
		{:else}
			<p class="text-lg">Daten wurden erfolgreich geladen</p>
		{/if}
		
	</div>
</div>
