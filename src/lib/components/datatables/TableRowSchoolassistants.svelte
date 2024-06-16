<script lang="ts">
	import IconParkSolidConnection from '~icons/icon-park-solid/connection';
	import { getDrawerStore, type DrawerSettings } from '@skeletonlabs/skeleton';
	import {
		type ResponseGetSchoolAssistantMatches,
		type SchoolAssistant
	} from '$lib/models/models';
	import { findSAssistantMatches } from '$lib/data/data';

	const drawerStore = getDrawerStore();
	export let schoolAssistant: SchoolAssistant;

	let isOpenMatches = false;
	let matches: ResponseGetSchoolAssistantMatches[];

	let drawerSettings: DrawerSettings = {
		id: 'matchSchoolAssistantsToSchoolAssistants',
		position: 'right',
		width: 'w-1/2'
	};

	async function getMatches(id: string) {
		isOpenMatches = !isOpenMatches;
		matches = await findSAssistantMatches(id);
		drawerSettings.meta = {
			matches
		};
		drawerStore.open(drawerSettings);
	}
</script>

<tbody>
	<tr>
		<td on:click={() => getMatches(schoolAssistant.id)}><IconParkSolidConnection /></td>
		<td>{schoolAssistant.lastName ? schoolAssistant.lastName : '-'}</td>
		<td>{schoolAssistant.firstName ? schoolAssistant.firstName : '-'}</td>
		<td>{schoolAssistant.birthDate ? schoolAssistant.birthDate : '-'}</td>
		<td>{schoolAssistant.accompanyingChild ? schoolAssistant.accompanyingChild : '-'}</td>
		<td>{schoolAssistant.phone ? schoolAssistant.phone : '-'}</td>
		<td>{schoolAssistant.mobile ? schoolAssistant.mobile : '-'}</td>
		<td>{schoolAssistant.email ? schoolAssistant.email : '-'}</td>
		<td>{schoolAssistant.streetHouseNumber ? schoolAssistant.streetHouseNumber : '-'}</td>
		<td>{schoolAssistant.postalCode ? schoolAssistant.postalCode : '-'}</td>
		<td>{schoolAssistant.city ? schoolAssistant.city : '-'}</td>
		<td>{schoolAssistant.level ? schoolAssistant.level : '-'}</td>
		<td>{schoolAssistant.approved ? schoolAssistant.approved : '-'}</td>
		<td>{schoolAssistant.info ? schoolAssistant.info : '-'}</td>
		<td>{schoolAssistant.certificates ? schoolAssistant.certificates : '-'}</td>
		<td>{schoolAssistant.professionalQualification ? schoolAssistant.professionalQualification : '-'}</td>
		<td>{schoolAssistant.children ? schoolAssistant.children : '-'}</td>
		<td>{schoolAssistant.membershipDate ? schoolAssistant.membershipDate : '-'}</td>
	</tr>
</tbody>
