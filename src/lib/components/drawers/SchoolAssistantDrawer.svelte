<script lang="ts">
	import { getFullName, type ResponseGetSchoolAssistantMatches } from '$lib/models/models';
	import LocationIcon from '~icons/fluent/location-16-regular';
	import Approved from '~icons/material-symbols-light/order-approve-outline-sharp';
	import Certificat from '~icons/ph/certificate-light';
	import Qulification from '~icons/hugeicons/new-job';	
	import Profile from '~icons/iconamoon/profile-thin';
    import Child from '~icons/healthicons/child-program';
    import Skills from '~icons/game-icons/skills';	import Transport from '~icons/material-symbols-light/transportation-outline-sharp';
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import Family from '~icons/noto/family-man-girl';

	const drawerStore = getDrawerStore();
	let schoolAssistantMatches: ResponseGetSchoolAssistantMatches[] = $drawerStore.meta.matches;
</script>

<div class="p-4">
	{#if schoolAssistantMatches}
		{#each schoolAssistantMatches as schoolAssistantMatch}
			<div class="border rounded-md p-2 mb-2 grid grid-cols-2 gap-4 assistantCard">
				<div class="flex flex-row">
					<span class="font-bold">Name </span>
					<Profile class="mx-2" />
					<p>{getFullName(schoolAssistantMatch.assistant)}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Wohnort </span>
					<LocationIcon class="mx-2" />
					<p>{schoolAssistantMatch.assistant.city ? schoolAssistantMatch.assistant.city : '-'} (Distanz: {schoolAssistantMatch ? schoolAssistantMatch.distance.toFixed(0) : '-'} km)</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Begleitetes Kind </span>
					<Child class="mx-2" />
					<p>{schoolAssistantMatch.assistant.assignedChild ? schoolAssistantMatch.assistant.assignedChild : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Eingr. </span>
					<Skills class="mx-2" />
					<p>{schoolAssistantMatch.assistant.level ? schoolAssistantMatch.assistant.level : '-'}</p>
				</div>
                <div class="flex flex-row">
					<span class="font-bold">genehmigt </span>
					<Approved class="mx-2" />
					<p>{schoolAssistantMatch.assistant.approved ? schoolAssistantMatch.assistant.approved : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Info </span>
					<Certificat class="mx-2" />
					<p>{schoolAssistantMatch.assistant.info ? schoolAssistantMatch.assistant.info : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Abschlusszeugnisse </span>
					<Qulification class="mx-2" />
					<p>{schoolAssistantMatch.assistant.certifications ? schoolAssistantMatch.assistant.certifications : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Kinder </span>
					<Family class="mx-2" />
					<p>{schoolAssistantMatch.assistant.children ? schoolAssistantMatch.assistant.children : '-'}</p>
				</div>
			</div>
		{/each}
	{/if}
</div>

<style>
	.assistantCard div {
		@apply items-center;
	}
</style>
