<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, type DrawerSettings } from '@skeletonlabs/skeleton';
	import FeBar from '~icons/fe/bar';
	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { initializeStores, Drawer } from '@skeletonlabs/skeleton';
	import LeftSideBar from '$lib/components/LeftSideBar.svelte';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	initializeStores();

	let isHiddenSidebar = false;

	function toogleSidebar() {
		isHiddenSidebar = !isHiddenSidebar;
	}
</script>

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<!-- App Bar -->
		<AppBar gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
			<svelte:fragment slot="lead">
				<button on:click={toogleSidebar} aria-label="Toggle Sidebar">
					<FeBar class={isHiddenSidebar ? 'rotate-90' : ''} />
				</button>
			</svelte:fragment>

			<svelte:fragment slot="trail"
				><strong class="text-xl uppercase">Georg</strong></svelte:fragment
			>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		<!-- Hidden below Tailwind's large breakpoint -->
		<div id="sidebar-left" class="h-full" class:hidden={isHiddenSidebar}><LeftSideBar /></div>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
