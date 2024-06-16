<script lang="ts">
	import DatatableCandidates from '$lib/components/datatables/DatatableCandidates.svelte';
	import DatatableChildren from '$lib/components/datatables/DatatableChildren.svelte';
	import { getCandidates } from '$lib/data/data';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';

	let tabSet: number = 0;
</script>

<div class="p-4">
	{#await getCandidates()}
		<p>Loading...</p>
	{:then res}
		<TabGroup>
			<Tab bind:group={tabSet} name="tab1" value={0}>
				<span>Bewerber</span>
			</Tab>
			<Tab bind:group={tabSet} name="tab2" value={1}><span>Anfragen Kinder</span></Tab>
			<!-- Tab Panels --->
			<svelte:fragment slot="panel">
				{#if tabSet === 0}
					<DatatableCandidates candidates={res.candidates} />
				{:else if tabSet === 1}
					<DatatableChildren childCareRequests={res.childCareRequests} />
				{/if}
			</svelte:fragment>
		</TabGroup>
	{:catch error}
		<p>{error.message}</p>
	{/await}
</div>
