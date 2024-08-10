<script lang="ts">
    import { defaultCandidateColumns, visibleCandidateColumns } from '$lib/stores/columnsCandidate';
    import { visibleChildCareColumns } from "$lib/stores/columnsChildcare";
    import { visibleSchoolAssistantColumns } from "$lib/stores/columnsSchoolAssistant";
    import { ColumnType, type ColumnKey, type Persisted } from "$lib/stores/models";
    import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
    import { onMount } from "svelte";
    import BarsIcon from '~icons/fa6-solid/bars';
	
    // Props for the component with generic type
    export let columns: ColumnKey[];
    export let columnLabels: { [key in ColumnKey]?: string } = {};
    export let columnType: ColumnType;


    let columnStore = getStoreForType(columnType);

    // Get the store for the first column key type
    function getStoreForType(columnType: ColumnType): Persisted<{
    defaultColumns: ColumnKey[];
}> {
        switch (columnType) {
            case ColumnType.Candidate:
                return visibleCandidateColumns;
            case ColumnType.ChildCare:
                return visibleChildCareColumns;
            case ColumnType.SchoolAssistant:
                return visibleSchoolAssistantColumns;
        }
    }
   
    // Component state
    let showColumnSelection = false;
    $: selectedColumns = $columnStore.defaultColumns;
    let columnSelectionDiv: HTMLDivElement;
    

    export const columnPositions: { [key in ColumnKey]: number } = defaultCandidateColumns.reduce((acc, column, index) => {
        acc[column as ColumnKey] = index;
        return acc;
    }, {} as { [key in ColumnKey]: number });

    function toggleColumn(columnKey: ColumnKey) {
        console.log('selectedColumns', selectedColumns);
        console.log('columnKey', columnKey);

        const index = selectedColumns.indexOf(columnKey);

        console.log('index', index);
        if (index === -1) {
            // Add column back at its original position
            const position = columnPositions[columnKey];
            if (position === undefined) {
                selectedColumns.push(columnKey);
            } else {
                selectedColumns.splice(position, 0, columnKey);
            }
        } else {
            // Remove column
            selectedColumns.splice(index, 1);
        }
       
        $columnStore.defaultColumns = selectedColumns
    }

    // handle click outside column selection div to clode showColumnSelection
    function handleClickOutside(event: MouseEvent) {
    if (columnSelectionDiv && !columnSelectionDiv.contains(event.target as Node)) {
        showColumnSelection = false;
    }
}

    onMount(() => {
        document.addEventListener('click', handleClickOutside);
        return () => {
            document.removeEventListener('click', handleClickOutside);
        };
    });

    $: fakeColumns = selectedColumns;
</script>

<div bind:this={columnSelectionDiv} class="relative mr-4">
    <button class="btn btn-primary" on:click={() => showColumnSelection = !showColumnSelection}>
        <BarsIcon class="rotate-90" />
        <span>({selectedColumns.length}/{columns.length})</span>
    </button>
    {#if showColumnSelection}
        <div class="absolute right-0 z-50 text-surface-900 dark:text-surface-100 bg-surface-200 dark:bg-surface-700 p-2 rounded-xl overflow-y-visible">
            <ListBox multiple>
                {#each columns as columnKey}
                    <ListBoxItem on:click={() => toggleColumn(columnKey)} name="columns" value={columnKey} group={fakeColumns}>
                        {columnLabels[columnKey]}
                    </ListBoxItem>
                {/each}
            </ListBox>
        </div>
    {/if}
</div>