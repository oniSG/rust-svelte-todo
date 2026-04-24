<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createListFinancePlanEntries,
		createCreateFinancePlanEntry,
		createUpdateFinancePlanEntry,
		createDeleteFinancePlanEntry,
		getListFinancePlanEntriesQueryKey
	} from '$lib/api/generated/finance-plan/finance-plan';
	import type { FinancePlanEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import { ContextMenu as ContextMenuPrimitive } from 'bits-ui';
	import { ChartContainer, type ChartConfig } from '$lib/components/ui/chart/index.js';
	import { AreaChart, Tooltip } from 'layerchart';

	const queryClient = useQueryClient();
	const planQuery = createListFinancePlanEntries();
	const entries = $derived(planQuery.data?.status === 200 ? planQuery.data.data : []);

	// ── Chart ─────────────────────────────────────────────────────────────────
	const incomeConfig: ChartConfig = {
		income: { label: 'Planned Income', color: 'var(--chart-1)' }
	};

	function fmtDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
	}

	function fmtCurrency(n: number): string {
		return n.toLocaleString('en-US');
	}

	// ── Add entry ─────────────────────────────────────────────────────────────
	let addOpen = $state(false);
	let addForm = $state({ period_date: '', income: 0 });
	let addError = $state('');

	const addMutation = createCreateFinancePlanEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListFinancePlanEntriesQueryKey() });
				toast.success('Entry created');
				addOpen = false;
				addForm = { period_date: '', income: 0 };
				addError = '';
			},
			onError: (err) => {
				addError = err.error ?? 'Failed to create entry';
				toast.error(err.error ?? 'Failed to create entry');
			}
		}
	}));

	function submitAdd(e: SubmitEvent) {
		e.preventDefault();
		addMutation.mutate({
			data: { period_date: addForm.period_date, income: Number(addForm.income) }
		});
	}

	// ── Edit entry ────────────────────────────────────────────────────────────
	let editOpen = $state(false);
	let editingEntry = $state<FinancePlanEntry | null>(null);
	let editForm = $state({ period_date: '', income: 0 });
	let editError = $state('');

	function openEdit(entry: FinancePlanEntry) {
		editingEntry = entry;
		editForm = { period_date: entry.period_date, income: entry.income };
		editError = '';
		editOpen = true;
	}

	const editMutation = createUpdateFinancePlanEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListFinancePlanEntriesQueryKey() });
				toast.success('Entry updated');
				editOpen = false;
				editingEntry = null;
				editError = '';
			},
			onError: (err) => {
				editError = err.error ?? 'Failed to update entry';
				toast.error(err.error ?? 'Failed to update entry');
			}
		}
	}));

	function submitEdit(e: SubmitEvent) {
		e.preventDefault();
		if (!editingEntry) return;
		editMutation.mutate({
			id: editingEntry.id,
			data: { period_date: editForm.period_date, income: Number(editForm.income) }
		});
	}

	// ── Delete entry ──────────────────────────────────────────────────────────
	let deleteOpen = $state(false);
	let deletingEntry = $state<FinancePlanEntry | null>(null);

	function openDelete(entry: FinancePlanEntry) {
		deletingEntry = entry;
		deleteOpen = true;
	}

	const deleteMutation = createDeleteFinancePlanEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListFinancePlanEntriesQueryKey() });
				toast.success('Entry deleted');
				deleteOpen = false;
				deletingEntry = null;
			},
			onError: () => {
				toast.error('Failed to delete entry');
			}
		}
	}));
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold tracking-tight">Finance Plan</h1>
			<p class="text-sm text-muted-foreground">Plan and track projected income over time.</p>
		</div>
		<Button onclick={() => (addOpen = true)}>+ Add Entry</Button>
	</div>

	<!-- Income Chart -->
	{#if entries.length > 0}
		<div class="w-full rounded-lg border bg-card px-5 pb-5 pt-4">
			<p class="mb-3 text-sm font-medium text-muted-foreground">Planned Income</p>
			<!-- ChartContainer is flex+justify-center internally; flex-1 makes AreaChart fill it -->
			<ChartContainer config={incomeConfig} class="h-64 w-full [&>*]:flex-1 [&>*]:min-w-0">
				<AreaChart
					data={entries}
					x={(d) => new Date(d.period_date)}
					y="income"
					series={[{ key: 'income', label: 'Planned Income', color: 'var(--color-income)' }]}
				>
					{#snippet tooltip({ context })}
						{@const d = context.tooltip.data as FinancePlanEntry | null}
						{#if d}
							<Tooltip.Root variant="none">
								<div
									class="border-border/50 bg-background grid min-w-[10rem] gap-1.5 rounded-lg border px-2.5 py-1.5 text-xs shadow-xl"
								>
									<p class="font-medium">{fmtDate(d.period_date)}</p>
									<div class="flex items-center justify-between gap-6">
										<div class="flex items-center gap-1.5">
											<span
												class="h-2.5 w-2.5 shrink-0 rounded-[2px]"
												style="background: var(--color-income)"
											></span>
											<span class="text-muted-foreground">Income</span>
										</div>
										<span class="font-mono font-medium tabular-nums"
											>{fmtCurrency(d.income)}</span
										>
									</div>
								</div>
							</Tooltip.Root>
						{/if}
					{/snippet}
				</AreaChart>
			</ChartContainer>
		</div>
	{/if}

	<!-- Table -->
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
								<ContextMenu.Item onclick={() => openEdit(entry)}>Edit</ContextMenu.Item>
								<ContextMenu.Separator />
								<ContextMenu.Item
									class="text-destructive focus:text-destructive"
									onclick={() => openDelete(entry)}
								>
									Delete
								</ContextMenu.Item>
							</ContextMenu.Content>
						</ContextMenu.Root>
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
</div>

<!-- Add Entry Dialog -->
<Dialog.Root bind:open={addOpen}>
	<Dialog.Content class="sm:max-w-[400px]">
		<Dialog.Header>
			<Dialog.Title>Add Plan Entry</Dialog.Title>
			<Dialog.Description>Set the projected income for a given period.</Dialog.Description>
		</Dialog.Header>
		<form class="space-y-4 py-2" onsubmit={submitAdd}>
			<div class="space-y-1.5">
				<Label for="add-period">Period Date</Label>
				<Input id="add-period" type="date" bind:value={addForm.period_date} required />
			</div>
			<div class="space-y-1.5">
				<Label for="add-income">Income</Label>
				<Input
					id="add-income"
					type="number"
					min="0"
					max="500000000"
					bind:value={addForm.income}
					placeholder="0"
					required
				/>
			</div>
			{#if addError}
				<p class="text-sm text-destructive">{addError}</p>
			{/if}
			<Dialog.Footer>
				<Button type="button" variant="outline" onclick={() => (addOpen = false)}>Cancel</Button>
				<Button type="submit" disabled={addMutation.isPending}>
					{addMutation.isPending ? 'Creating...' : 'Create'}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

<!-- Edit Entry Dialog -->
<Dialog.Root bind:open={editOpen}>
	<Dialog.Content class="sm:max-w-[400px]">
		<Dialog.Header>
			<Dialog.Title>Edit Plan Entry</Dialog.Title>
			<Dialog.Description>
				Update the period and income for {editingEntry ? fmtDate(editingEntry.period_date) : ''}.
			</Dialog.Description>
		</Dialog.Header>
		<form class="space-y-4 py-2" onsubmit={submitEdit}>
			<div class="space-y-1.5">
				<Label for="edit-period">Period Date</Label>
				<Input id="edit-period" type="date" bind:value={editForm.period_date} required />
			</div>
			<div class="space-y-1.5">
				<Label for="edit-income">Income</Label>
				<Input
					id="edit-income"
					type="number"
					min="0"
					max="500000000"
					bind:value={editForm.income}
					required
				/>
			</div>
			{#if editError}
				<p class="text-sm text-destructive">{editError}</p>
			{/if}
			<Dialog.Footer>
				<Button type="button" variant="outline" onclick={() => (editOpen = false)}>Cancel</Button>
				<Button type="submit" disabled={editMutation.isPending}>
					{editMutation.isPending ? 'Saving...' : 'Save'}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>

<!-- Delete Confirm Dialog -->
<Dialog.Root bind:open={deleteOpen}>
	<Dialog.Content class="sm:max-w-[400px]">
		<Dialog.Header>
			<Dialog.Title>Delete Entry</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete the entry for
				<strong>{deletingEntry ? fmtDate(deletingEntry.period_date) : ''}</strong>? This cannot be
				undone.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Button variant="outline" onclick={() => (deleteOpen = false)}>Cancel</Button>
			<Button
				variant="destructive"
				disabled={deleteMutation.isPending}
				onclick={() => {
					if (deletingEntry) deleteMutation.mutate({ id: deletingEntry.id });
				}}
			>
				{deleteMutation.isPending ? 'Deleting...' : 'Delete'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
