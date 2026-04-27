<script lang="ts">
	import { createListBillingEntries } from '$lib/api/generated/billing/billing';
	import * as Table from '$lib/components/ui/table/index.js';
	import BillingRow from './BillingRow.svelte';

	const billingQuery = createListBillingEntries();
	const entries = $derived(billingQuery.data?.status === 200 ? billingQuery.data.data : []);
</script>

{#if billingQuery.isPending}
	<div class="text-sm text-muted-foreground">Loading...</div>
{:else if billingQuery.isError}
	<div class="text-sm text-destructive">Failed to load billing entries.</div>
{:else}
	<div class="rounded-lg border">
		<Table.Root class="table-fixed">
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-36">Fans Count</Table.Head>
					<Table.Head class="w-32">Condition</Table.Head>
					<Table.Head class="w-32 text-right">Basic</Table.Head>
					<Table.Head class="w-32 text-right">Standard</Table.Head>
					<Table.Head class="w-32 text-right">Premium</Table.Head>
					<Table.Head class="w-28 text-center">Individual</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each entries as entry (entry.id)}
					<BillingRow {entry} />
				{:else}
					<Table.Row>
						<Table.Cell class="py-8 text-center text-muted-foreground" colspan={6}>
							No billing entries yet. Add your first rule.
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
{/if}
