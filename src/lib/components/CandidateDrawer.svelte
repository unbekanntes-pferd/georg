<script lang="ts">
	import type { ChildCareRequest } from '$lib/models/models';
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
	let childCareRequests: ChildCareRequest[] = $drawerStore.meta.matches;
</script>

<div class="p-4">
	{#if childCareRequests}
		{#each childCareRequests as ChildCareRequest}
			<div class="border rounded-md p-2 mb-2 grid grid-cols-2 gap-4 childCard">
				<div class="flex flex-row">
					<Institution class="mr-2" />
					<p class="font-">{ChildCareRequest.institution ? ChildCareRequest.institution : '-'}</p>
				</div>
				<div class="flex flex-row">
					<LocationIcon class="mr-2" />
					<p>{ChildCareRequest.location ? ChildCareRequest.location : '-'}</p>
				</div>
				<div class="flex flex-row">
					<School class="mr-2" />
					<p>{ChildCareRequest.grade ? ChildCareRequest.grade : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Time class="mr-2" />
					<p>{ChildCareRequest.hours ? ChildCareRequest.hours : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Diagnosis class="mr-2" />
					<p>{ChildCareRequest.diagnosis ? ChildCareRequest.diagnosis : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Contact class="mr-2" />
					<p>{ChildCareRequest.contact ? ChildCareRequest.contact : '-'}</p>
				</div>
				<div class="flex flex-row">
					<TimeEntry class="mr-2" />
					<p>{ChildCareRequest.receivedAt ? ChildCareRequest.receivedAt : '-'}</p>
				</div>
				<div class="flex flex-row">
					<Notes class="mr-2" />
					<p>{ChildCareRequest.notes ? ChildCareRequest.notes : '-'}</p>
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
