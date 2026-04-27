<script lang="ts">
	import { createListUsers } from '$lib/api/generated/users/users';
	import * as Table from '$lib/components/ui/table/index.js';
	import UserRow from './UserRow.svelte';

	const usersQuery = createListUsers();
	const users = $derived(usersQuery.data?.status === 200 ? usersQuery.data.data : []);
</script>

{#if usersQuery.isPending}
	<div class="text-sm text-muted-foreground">Loading...</div>
{:else if usersQuery.isError}
	<div class="text-sm text-destructive">Failed to load users.</div>
{:else}
	<div class="rounded-lg border">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>Name</Table.Head>
					<Table.Head>Email</Table.Head>
					<Table.Head>Role</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each users as user (user.id)}
					<UserRow {user} />
				{:else}
					<Table.Row>
						<Table.Cell class="py-8 text-center text-muted-foreground" colspan={3}>
							No users found.
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
{/if}
