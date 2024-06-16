<script lang="ts">
	import { goto } from '$app/navigation';
</script>

<div class="w-48 bg-surface-800 h-full">
	<button on:click={() => goto('candidates')} class="btn">Bewerber</button>
	<button on:click={() => goto('school-assistant')} class="btn">Schulbegleitung</button>
	<button on:click={() => goto('settings')} class="btn">Einstellungen</button>
</div>
