<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import { createDeleteUser, getListUsersQueryKey } from '$lib/api/generated/users/users';
	import type { User } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';

	let { user, open = $bindable(false) }: { user: User; open: boolean } = $props();

	const queryClient = useQueryClient();
	const mutation = createDeleteUser();

	async function handleDelete() {
		try {
			await mutation.mutateAsync({ id: user.id });
			queryClient.invalidateQueries({ queryKey: getListUsersQueryKey() });
			toast.success('User deleted');
			open = false;
		} catch {
			toast.error('Failed to delete user');
		}
	}
</script>

<Dialog.Root {open} onOpenChange={(v) => { if (!v) open = v; }}>
	<Dialog.Content class="sm:max-w-110" showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Delete User</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete <strong>{user.full_name}</strong>? This cannot be undone.
			</Dialog.Description>
		</Dialog.Header>

		<Dialog.Footer>
			<Button type="button" variant="outline" onclick={() => (open = false)}>Cancel</Button>
			<Button
				type="button"
				variant="destructive"
				disabled={mutation.isPending}
				onclick={handleDelete}
			>
				{mutation.isPending ? 'Deleting...' : 'Delete'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
