<script lang="ts">
	import { createListFinancePlanEntries } from '$lib/api/generated/finance-plan/finance-plan';
	import * as Table from '$lib/components/ui/table/index.js';
	import PlanRow from './PlanRow.svelte';

	const planQuery = createListFinancePlanEntries();
	const entries = $derived(planQuery.data?.status === 200 ? planQuery.data.data : []);
</script>

{#if planQuery.isPending}
	<div class="text-sm text-muted-foreground">Loading...</div>
{:else if planQuery.isError}
	<div class="text-sm text-destructive">Failed to load plan entries.</div>
{:else}
	<div class="rounded-lg border">
		<Table.Root class="table-fixed">
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-44">Period</Table.Head>
					<Table.Head class="w-44 text-right">Income</Table.Head>
					<Table.Head>Created</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each entries as entry (entry.id)}
					<PlanRow {entry} />
				{:else}
					<Table.Row>
						<Table.Cell class="py-8 text-center text-muted-foreground" colspan={3}>
							No entries yet. Add your first plan entry.
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
{/if}
