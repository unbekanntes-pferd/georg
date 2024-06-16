<script lang="ts">
	import DatatableSchoolAssistants from '$lib/components/datatables/DatatableSchoolAssistants.svelte';
	import DatatableAccompaniedChildren from '$lib/components/datatables/DatatableAccompaniedChildren.svelte';
	import { getAssistantData } from '$lib/data/data';
	import { Tab, TabGroup } from '@skeletonlabs/skeleton';

	let tabSet: number = 0;
</script>

<div class="p-4">
	{#await getAssistantData()}
		<p>Loading...</p>
	{:then res}
		<TabGroup>
			<Tab bind:group={tabSet} name="tab1" value={0}>
				<span>Schulbegleiter*innen</span>
			</Tab>
			<!-- not needed for now 
			<!-- 
			<Tab bind:group={tabSet} name="tab2" value={1}><span>Begleitede Kinder</span></Tab> --> 
			<!-- Tab Panels --->
			<svelte:fragment slot="panel">
				{#if tabSet === 0}
					<DatatableSchoolAssistants schoolAssistants={res} />
				
				<!-- not needed for now  --> 
				<!--
					{:else if tabSet === 1}
					<DatatableAccompaniedChildren accompaniedChildren={res.children} />
				-->
					{/if}
			</svelte:fragment>
		</TabGroup>
	{:catch error}
		<p>{error.message}</p>
	{/await}
</div>
