<script lang="ts">
	import type { DataHandler } from '@vincjo/datatables';
	export let handler: DataHandler;
	const rowCount = handler.getRowCount();
</script>

<aside class="flex text-sm leading-8 mr-8 place-items-center">
	<span>
		{#if $rowCount.total > 0}
			<b>{$rowCount.start}</b>
			- <b>{$rowCount.end}</b>
			/ <b>{$rowCount.total}</b>
		{:else}
			Keine Einträge gefunden
		{/if}
	</span>
</aside>
