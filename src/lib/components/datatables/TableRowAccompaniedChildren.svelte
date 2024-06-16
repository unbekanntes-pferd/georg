<script lang="ts">
	import IconParkSolidConnection from '~icons/icon-park-solid/connection';
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import {
		type ResponseGetAccompaniedChildMatches,

		type AccompaniedChild


	} from '$lib/models/models';
	import { findAccompaniedChildrenMatches } from '$lib/data/data';

	const drawerStore = getDrawerStore();
	export let accompaniedChild: AccompaniedChild;

	let isOpenMatches = false;
	let matches: ResponseGetAccompaniedChildMatches[];

	let drawerSettings: DrawerSettings = {
		id: 'matchaccompaniedChildsToAccompaniedChildren',
		position: 'right',
		width: 'w-1/2'
	};

	async function getMatches(id: string) {
		isOpenMatches = !isOpenMatches;
		matches = await findAccompaniedChildrenMatches(id);
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		<td on:click={() => getMatches(accompaniedChild.id)}><IconParkSolidConnection /></td>
		<td>{accompaniedChild.name ? accompaniedChild.name : '-'}</td>
		<td>{accompaniedChild.qualification ? accompaniedChild.qualification : '-'}</td>
		<td>{accompaniedChild.approvalUntil ? accompaniedChild.approvalUntil : '-'}</td>
		<td>{accompaniedChild.fundingAgency ? accompaniedChild.fundingAgency : '-'}</td>
		<td>{accompaniedChild.contactPerson ? accompaniedChild.contactPerson : '-'}</td>
		<td>{accompaniedChild.contactPhone ? accompaniedChild.contactPhone : '-'}</td>
		<td>{accompaniedChild.contactEmail ? accompaniedChild.contactEmail : '-'}</td>
		<td>{accompaniedChild.notes ? accompaniedChild.notes : '-'}</td>
		
	</tr>
</tbody>
