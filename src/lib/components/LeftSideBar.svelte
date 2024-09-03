<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import {
		Command,
		State,
		excelPathCommandStore,
		type ExcelPathCommandStore
	} from '$lib/stores/path';

	$: disabledCandidates = disableCandidates($excelPathCommandStore);
	$: disabledSchoolAssistants = disableSchoolAssistants($excelPathCommandStore);

	function disableCandidates(store: ExcelPathCommandStore) {
		let candidatesEntry = store.directories.find(
			(directory) => directory.command === Command.CandidatesAndChildCareRequests
		);
		if (candidatesEntry?.path === '' || candidatesEntry?.state !== State.Ok) {
			return true;
		}
		return false;
	}

	function disableSchoolAssistants(store: ExcelPathCommandStore) {
		let schoolAssistantsEntry = $excelPathCommandStore.directories.find(
			(directory) => directory.command === Command.SchoolAssistants
		);
		if (schoolAssistantsEntry?.path === '' || schoolAssistantsEntry?.state !== State.Ok) {
			return true;
		}
		return false;
	}

	$: isCandidatesRoute = $page.url.pathname === '/candidates';
	$: isSchoolAssistantsRoute = $page.url.pathname === '/school-assistance';
	$: isSettingsRoute = $page.url.pathname === '/settings';
</script>

<div class="flex flex-col w-48 h-full">
	<button
		disabled={disabledCandidates}
		on:click={() => goto('candidates')}
		class="btn {isCandidatesRoute ? 'active' : ''}"
	>
		Bewerber
	</button>
	<button
		disabled={disabledSchoolAssistants}
		on:click={() => goto('school-assistance')}
		class="btn {isSchoolAssistantsRoute ? 'active' : ''}"
	>
		Schulbegleitung
	</button>
	<button on:click={() => goto('settings')} class="btn mt-auto {isSettingsRoute ? 'active' : ''}">
		Einstellungen
	</button>
</div>

<style>
	.active {
		text-decoration: underline;
	}
</style>
