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
					<Profile class="mr-2" />
					<p class="font-">{getFullName(schoolAssistantMatch.assistant)}</p>
				</div>
				<div class="flex flex-row">
					<LocationIcon class="mr-2" />
					<p>{schoolAssistantMatch.assistant.city ? schoolAssistantMatch.assistant.city : '-'} (Distanz: {schoolAssistantMatch ? schoolAssistantMatch.distance.toFixed(0) : '-'} km)</p>
				</div>
				<div class="flex flex-row">
					<Child class="mr-2" />
					<p>{schoolAssistantMatch.assistant.accompanyingChild ? schoolAssistantMatch.assistant.accompanyingChild : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Skills class="mr-2" />
					<p>{schoolAssistantMatch.assistant.level ? schoolAssistantMatch.assistant.level : '-'}</p>
				</div>
                <div class="flex flex-row">
					<Approved class="mr-2" />
					<p>{schoolAssistantMatch.assistant.approved ? schoolAssistantMatch.assistant.approved : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Certificat class="mr-2" />
					<p>{schoolAssistantMatch.assistant.certificates ? schoolAssistantMatch.assistant.certificates : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Qulification class="mr-2" />
					<p>{schoolAssistantMatch.assistant.professionalQualification ? schoolAssistantMatch.assistant.professionalQualification : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Family class="mr-2" />
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
