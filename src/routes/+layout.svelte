<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar, type DrawerSettings, getDrawerStore } from '@skeletonlabs/skeleton';
	import FeBar from '~icons/fe/bar';
	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup, Toast  } from '@skeletonlabs/skeleton';
	import { initializeStores, Drawer } from '@skeletonlabs/skeleton';
	import LeftSideBar from '$lib/components/LeftSideBar.svelte';
	import CandidateDrawer from '$lib/components/drawers/CandidateDrawer.svelte';
	import logo from "$lib/assets/georg_logo.png";
	import ChildDrawer from '$lib/components/drawers/ChildDrawer.svelte';
	import SchoolAssistantDrawer from '$lib/components/drawers/SchoolAssistantDrawer.svelte';
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	initializeStores();

	let isHiddenSidebar = false;

	function toogleSidebar() {
		isHiddenSidebar = !isHiddenSidebar;
	}
	const drawerStore = getDrawerStore();

	

</script>

<Toast />

<Drawer>
	{#if $drawerStore.id === 'matchCandidatesToChildCareRequests'}
		<CandidateDrawer />
	{:else if $drawerStore.id === 'matchChildCareRequestsToCandidates'}
		<ChildDrawer />
	{:else if $drawerStore.id === 'matchSchoolAssistantsToSchoolAssistants'}
		<SchoolAssistantDrawer />
	{:else}
		<!-- No Drawer Content -->
	{/if}

</Drawer>

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
				>
				<img src={logo} alt="georg-logo" class="w-8"></svelte:fragment
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
