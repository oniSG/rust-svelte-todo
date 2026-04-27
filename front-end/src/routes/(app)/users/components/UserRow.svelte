<script lang="ts">
	import type { User } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import { ContextMenu as ContextMenuPrimitive } from 'bits-ui';
	import { ROLE_LABELS, roleBadgeVariant } from '../users.utils';
	import EditUserDialog from './EditUserDialog.svelte';
	import DeleteUserDialog from './DeleteUserDialog.svelte';

	let { user }: { user: User } = $props();

	let editOpen = $state(false);
	let deleteOpen = $state(false);
</script>

<ContextMenu.Root>
	<ContextMenuPrimitive.Trigger>
		{#snippet child({ props })}
			<Table.Row {...props} class="cursor-context-menu select-none">
				<Table.Cell class="font-medium">{user.full_name}</Table.Cell>
				<Table.Cell class="text-muted-foreground">{user.email}</Table.Cell>
				<Table.Cell>
					<Badge variant={roleBadgeVariant(user.role)}>
						{ROLE_LABELS[user.role] ?? user.role}
					</Badge>
				</Table.Cell>
			</Table.Row>
		{/snippet}
	</ContextMenuPrimitive.Trigger>

	<ContextMenu.Content>
		<ContextMenu.Label class="text-xs text-muted-foreground">{user.full_name}</ContextMenu.Label>
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

<EditUserDialog {user} bind:open={editOpen} />
<DeleteUserDialog {user} bind:open={deleteOpen} />
