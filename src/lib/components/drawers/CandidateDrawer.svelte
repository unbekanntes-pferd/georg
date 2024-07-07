<script lang="ts">
	import type { ChildCareRequest, ResponseGetCandidateMatches } from '$lib/models/models';
	import LocationIcon from '~icons/fluent/location-16-regular';
	import School from '~icons/material-symbols/school';
	import Time from '~icons/fluent-mdl2/date-time';
	import Diagnosis from '~icons/material-symbols/diagnosis-outline-sharp';
	import Contact from '~icons/mdi/contact-outline';
	import TimeEntry from '~icons/fluent-mdl2/time-entry';
	import Notes from '~icons/material-symbols/note-outline';
	import Institution from '~icons/tdesign/institution';
	import { getDrawerStore } from '@skeletonlabs/skeleton';

	const drawerStore = getDrawerStore();
	let childCareRequests: ResponseGetCandidateMatches[] = $drawerStore.meta.matches;
</script>

<div class="p-4">
	{#if childCareRequests}
		{#each childCareRequests as childCareRequest}
			<div class="border rounded-md p-2 mb-2 grid grid-cols-2 gap-4 childCard">
				<div class="flex flex-row">
					<span class="font-bold">Einrichtung </span>
					<Institution class="mx-2" />
					<p class="font-">{childCareRequest.candidate.institution ? childCareRequest.candidate.institution : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Wohnort </span>
					<LocationIcon class="mx-2" />
					<p>{childCareRequest.candidate.location ? childCareRequest.candidate.location : '-'} (Distanz: {childCareRequest ? childCareRequest.distance.toFixed(0) : '-'} km)</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Klasse </span>
					<School class="mx-2" />
					<p>{childCareRequest.candidate.grade ? childCareRequest.candidate.grade : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Stunden </span>
					<Time class="mx-2" />
					<p>{childCareRequest.candidate.hours ? childCareRequest.candidate.hours : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Diagnose </span>
					<Diagnosis class="mx-2" />
					<p>{childCareRequest.candidate.diagnosis ? childCareRequest.candidate.diagnosis : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Ansprechpartner*in </span>
					<Contact class="mx-2" />
					<p>{childCareRequest.candidate.contact ? childCareRequest.candidate.contact : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Datum </span>
					<TimeEntry class="mx-2" />
					<p>{childCareRequest.candidate.receivedAt ? childCareRequest.candidate.receivedAt : '-'}</p>
				</div>
				<div class="flex flex-row">
					<span class="font-bold">Bemerkung </span>
					<Notes class="mx-2" />
					<p>{childCareRequest.candidate.notes ? childCareRequest.candidate.notes : '-'}</p>
				</div>
			</div>
		{/each}
	{/if}
</div>

<style>
	
	.childCard div {
		@apply items-center;
	}
</style>
