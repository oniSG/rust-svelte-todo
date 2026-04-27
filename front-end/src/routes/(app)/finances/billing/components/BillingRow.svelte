<script lang="ts">
	import type { BillingEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import { ContextMenu as ContextMenuPrimitive } from 'bits-ui';
	import { CONDITION_LABELS, conditionVariant, fmtPrice } from '../billing.utils';
	import EditEntryDialog from './EditEntryDialog.svelte';
	import DeleteEntryDialog from './DeleteEntryDialog.svelte';

	let { entry }: { entry: BillingEntry } = $props();

	let editOpen = $state(false);
	let deleteOpen = $state(false);
</script>

<ContextMenu.Root>
	<ContextMenuPrimitive.Trigger>
		{#snippet child({ props })}
			<Table.Row {...props} class="cursor-context-menu select-none">
				<Table.Cell class="font-mono font-medium tabular-nums">
					{entry.fans_count.toLocaleString()}
				</Table.Cell>
				<Table.Cell>
					<Badge variant={conditionVariant(entry.condition)}>
						{CONDITION_LABELS[entry.condition] ?? entry.condition}
					</Badge>
				</Table.Cell>
				<Table.Cell class="text-right font-mono tabular-nums text-muted-foreground">
					{fmtPrice(entry.basic_plan_price)}
				</Table.Cell>
				<Table.Cell class="text-right font-mono tabular-nums text-muted-foreground">
					{fmtPrice(entry.standard_plan_price)}
				</Table.Cell>
				<Table.Cell class="text-right font-mono tabular-nums text-muted-foreground">
					{fmtPrice(entry.premium_plan_price)}
				</Table.Cell>
				<Table.Cell class="text-center">
					{#if entry.individual_plan_price}
						<Badge variant="outline">Yes</Badge>
					{:else}
						<span class="text-sm text-muted-foreground">No</span>
					{/if}
				</Table.Cell>
			</Table.Row>
		{/snippet}
	</ContextMenuPrimitive.Trigger>

	<ContextMenu.Content>
		<ContextMenu.Label class="text-xs text-muted-foreground">
			{CONDITION_LABELS[entry.condition]} {entry.fans_count.toLocaleString()} fans
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
