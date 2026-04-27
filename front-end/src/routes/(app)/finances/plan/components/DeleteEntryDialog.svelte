<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createDeleteFinancePlanEntry,
		getListFinancePlanEntriesQueryKey
	} from '$lib/api/generated/finance-plan/finance-plan';
	import type { FinancePlanEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { fmtDate } from '../plan.utils';

	let { entry, open = $bindable(false) }: { entry: FinancePlanEntry; open: boolean } = $props();

	const queryClient = useQueryClient();
	const mutation = createDeleteFinancePlanEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListFinancePlanEntriesQueryKey() });
				toast.success('Entry deleted');
				open = false;
			},
			onError: () => {
				toast.error('Failed to delete entry');
			}
		}
	}));
</script>

<Dialog.Root
	{open}
	onOpenChange={(v) => {
		if (!v) open = v;
	}}
>
	<Dialog.Content class="sm:max-w-110" showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Delete Entry</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete the entry for
				<strong>{fmtDate(entry.period_date)}</strong>? This cannot be undone.
			</Dialog.Description>
		</Dialog.Header>

		<Dialog.Footer>
			<Button type="button" variant="outline" onclick={() => (open = false)}>Cancel</Button>
			<Button
				type="button"
				variant="destructive"
				disabled={mutation.isPending}
				onclick={() => mutation.mutate({ id: entry.id })}
			>
				{mutation.isPending ? 'Deleting...' : 'Delete'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
