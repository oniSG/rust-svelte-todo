<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createDeleteBillingEntry,
		getListBillingEntriesQueryKey
	} from '$lib/api/generated/billing/billing';
	import type { BillingEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { CONDITION_LABELS } from '../billing.utils';

	let { entry, open = $bindable(false) }: { entry: BillingEntry; open: boolean } = $props();

	const queryClient = useQueryClient();
	const mutation = createDeleteBillingEntry();

	async function handleDelete() {
		try {
			await mutation.mutateAsync({ id: entry.id });
			queryClient.invalidateQueries({ queryKey: getListBillingEntriesQueryKey() });
			toast.success('Billing entry deleted');
			open = false;
		} catch {
			toast.error('Failed to delete billing entry');
		}
	}
</script>

<Dialog.Root {open} onOpenChange={(v) => { if (!v) open = v; }}>
	<Dialog.Content class="sm:max-w-110" showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Delete Billing Entry</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete the rule for
				<strong>
					{CONDITION_LABELS[entry.condition]}
					{entry.fans_count.toLocaleString()} fans
				</strong>? This cannot be undone.
			</Dialog.Description>
		</Dialog.Header>

		<Dialog.Footer>
			<Button type="button" variant="outline" onclick={() => (open = false)}>Cancel</Button>
			<Button type="button" variant="destructive" disabled={mutation.isPending} onclick={handleDelete}>
				{mutation.isPending ? 'Deleting...' : 'Delete'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
