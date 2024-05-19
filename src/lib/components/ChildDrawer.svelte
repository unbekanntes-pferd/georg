<script lang="ts">
	import type { ResponseGetChildcareMatches } from '$lib/models/models';
	import LocationIcon from '~icons/fluent/location-16-regular';
	import Time from '~icons/fluent-mdl2/date-time';
	import TimeEntry from '~icons/fluent-mdl2/time-entry';
	import Notes from '~icons/material-symbols/note-outline';
	import Qulification from '~icons/hugeicons/new-job';	
	import Profile from '~icons/iconamoon/profile-thin';
	import Transport from '~icons/material-symbols-light/transportation-outline-sharp';
	import { getDrawerStore } from '@skeletonlabs/skeleton';
	import Family from '~icons/noto/family-man-girl';

	const drawerStore = getDrawerStore();
	let childCareRequests: ResponseGetChildcareMatches[] = $drawerStore.meta.matches;

	console.log(childCareRequests);
</script>

<div class="p-4">
	{#if childCareRequests}
		{#each childCareRequests as childCareRequest}
			<div class="border rounded-md p-2 mb-2 grid grid-cols-2 gap-4 childCard">
				<div class="flex flex-row">
					<Profile class="mr-2" />
					<p class="font-">{childCareRequest.candidate.name ? childCareRequest.candidate.name : '-'}</p>
				</div>
				<div class="flex flex-row">
					<LocationIcon class="mr-2" />
					<p>{childCareRequest.candidate.location ? childCareRequest.candidate.location : '-'} (Distanz: {childCareRequest ? childCareRequest.distance.toFixed(0) : '-'} km)</p>
				</div>
				<div class="flex flex-row">
					<Qulification class="mr-2" />
					<p>{childCareRequest.candidate.qualification ? childCareRequest.candidate.qualification : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Time class="mr-2" />
					<p>{childCareRequest.candidate.hours ? childCareRequest.candidate.hours : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Transport class="mr-2" />
					<p>{childCareRequest.candidate.mobility ? childCareRequest.candidate.mobility : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Family class="mr-2" />
					<p>{childCareRequest.candidate.plannedChild ? childCareRequest.candidate.plannedChild : '-'}</p>
				</div>
				<div class="flex flex-row">
					<TimeEntry class="mr-2" />
					<p>{childCareRequest.candidate.receivedAt ? childCareRequest.candidate.receivedAt : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Notes class="mr-2" />
					<p>{childCareRequest.candidate.notes ? childCareRequest.candidate.notes : '-'}</p>
				</div>
			</div>
		{/each}
	{/if}
</div>

<style>
	.childCard p {
		@apply text-surface-400;
	}
	.childCard div {
		@apply items-center;
	}
</style>
