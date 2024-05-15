<script lang="ts">
	import type { DataHandler } from '@vincjo/datatables';
	export let handler: DataHandler;
	const rowsPerPage = handler.getRowsPerPage();
	const options = [10, 20, 50, 100];
</script>

<aside class="flex place-items-center mr-8">
	<span class="flex flex-row whitespace-nowrap text-sm">Zeilen pro Seite:</span>
	<select class="select ml-2" bind:value={$rowsPerPage}>
		{#each options as option}
			<option value={option}>
				{option}
			</option>
		{/each}
	</select>
</aside>
