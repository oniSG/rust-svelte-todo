<script lang="ts">
	import type { FinancePlanEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import { ContextMenu as ContextMenuPrimitive } from 'bits-ui';
	import { fmtDate, fmtCurrency } from '../plan.utils';
	import EditEntryDialog from './EditEntryDialog.svelte';
	import DeleteEntryDialog from './DeleteEntryDialog.svelte';

	let { entry }: { entry: FinancePlanEntry } = $props();

	let editOpen = $state(false);
	let deleteOpen = $state(false);
</script>

<ContextMenu.Root>
	<ContextMenuPrimitive.Trigger>
		{#snippet child({ props })}
			<Table.Row {...props} class="cursor-context-menu select-none">
				<Table.Cell class="font-medium">{fmtDate(entry.period_date)}</Table.Cell>
				<Table.Cell class="text-right font-mono tabular-nums">
					{fmtCurrency(entry.income)}
				</Table.Cell>
				<Table.Cell class="text-muted-foreground">
					{new Date(entry.created_at).toLocaleDateString()}
				</Table.Cell>
			</Table.Row>
		{/snippet}
	</ContextMenuPrimitive.Trigger>

	<ContextMenu.Content>
		<ContextMenu.Label class="text-xs text-muted-foreground">
			{fmtDate(entry.period_date)}
		</ContextMenu.Label>
		<ContextMenu.Separator />
		<ContextMenu.Item onclick={() => (editOpen = true)}>Edit</ContextMenu.Item>
		<ContextMenu.Separator />
		<ContextMenu.Item
			class="text-destructive focus:text-destructive"
			onclick={() => (deleteOpen = true)}
		>
			Delete
		</ContextMenu.Item>
	</ContextMenu.Content>
</ContextMenu.Root>

<EditEntryDialog {entry} bind:open={editOpen} />
<DeleteEntryDialog {entry} bind:open={deleteOpen} />
