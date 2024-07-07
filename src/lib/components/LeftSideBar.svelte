<script lang="ts">
	import { goto } from '$app/navigation';
	import { Command, State, excelPathCommandStore, type ExcelPathCommandStore } from '$lib/stores/path';

	

	$: disabledCandidates = disableCandidates($excelPathCommandStore);
	$: disabledSchoolAssistants = disableSchoolAssistants($excelPathCommandStore);

	function disableCandidates(store: ExcelPathCommandStore) {
		let candidatesEntry = store.directories.find((directory) => directory.command === Command.CandidatesAndChildCareRequests);
		if (candidatesEntry?.path === '' || candidatesEntry?.state !== State.Ok) {
			return true;
		} 
		return false;
	}

	function disableSchoolAssistants(store: ExcelPathCommandStore) {
		let schoolAssistantsEntry = $excelPathCommandStore.directories.find((directory) => directory.command === Command.SchoolAssistants);
		if (schoolAssistantsEntry?.path === '' || schoolAssistantsEntry?.state !== State.Ok) {
			return true;
		} 
		return false;
	}

</script>

<div class="flex flex-col w-48 h-full">
	<button disabled={disabledCandidates} on:click={() => goto('candidates')} class="btn">Bewerber</button>
	<button disabled={disabledSchoolAssistants} on:click={() => goto('school-assistance')} class="btn">Schulbegleitung</button>
	<button on:click={() => goto('settings')} class="btn mt-auto">Einstellungen</button>
</div>
