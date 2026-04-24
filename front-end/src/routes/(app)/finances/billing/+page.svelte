<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createListBillingEntries,
		createCreateBillingEntry,
		createUpdateBillingEntry,
		createDeleteBillingEntry,
		getListBillingEntriesQueryKey
	} from '$lib/api/generated/billing/billing';
	import { BillingCondition } from '$lib/api/generated/rustSvelteTodo.schemas';
	import type { BillingEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import { ContextMenu as ContextMenuPrimitive } from 'bits-ui';

	const queryClient = useQueryClient();
	const billingQuery = createListBillingEntries();
	const entries = $derived(billingQuery.data?.status === 200 ? billingQuery.data.data : []);

	const CONDITION_LABELS: Record<string, string> = {
		[BillingCondition.less_than]: 'Less than',
		[BillingCondition.more_than]: 'More than'
	};

	function conditionVariant(c: string): 'secondary' | 'default' {
		return c === BillingCondition.less_than ? 'secondary' : 'default';
	}

	function fmtPrice(n: number | null | undefined): string {
		if (n == null) return '—';
		return n.toLocaleString('en-US');
	}

	// ── Shared blank form ─────────────────────────────────────────────────────
	function blankForm() {
		return {
			fans_count: 0,
			condition: BillingCondition.less_than as string,
			basic_plan_price: '' as string,
			standard_plan_price: '' as string,
			premium_plan_price: '' as string,
			individual_plan_price: false
		};
	}

	function parsePrice(v: string): number | null {
		return v.trim() === '' ? null : Number(v);
	}

	// ── Add entry ─────────────────────────────────────────────────────────────
	let addOpen = $state(false);
	let addForm = $state(blankForm());
	let addError = $state('');

	const addMutation = createCreateBillingEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListBillingEntriesQueryKey() });
				toast.success('Billing entry created');
				addOpen = false;
				addForm = blankForm();
				addError = '';
			},
			onError: (err) => {
				addError = err.error ?? 'Failed to create billing entry';
				toast.error(err.error ?? 'Failed to create billing entry');
			}
		}
	}));

	function submitAdd(e: SubmitEvent) {
		e.preventDefault();
		addMutation.mutate({
			data: {
				fans_count: Number(addForm.fans_count),
				condition: addForm.condition as (typeof BillingCondition)[keyof typeof BillingCondition],
				basic_plan_price: parsePrice(addForm.basic_plan_price),
				standard_plan_price: parsePrice(addForm.standard_plan_price),
				premium_plan_price: parsePrice(addForm.premium_plan_price),
				individual_plan_price: addForm.individual_plan_price
			}
		});
	}

	// ── Edit entry ────────────────────────────────────────────────────────────
	let editOpen = $state(false);
	let editingEntry = $state<BillingEntry | null>(null);
	let editForm = $state(blankForm());
	let editError = $state('');

	function openEdit(entry: BillingEntry) {
		editingEntry = entry;
		editForm = {
			fans_count: entry.fans_count,
			condition: entry.condition,
			basic_plan_price: entry.basic_plan_price != null ? String(entry.basic_plan_price) : '',
			standard_plan_price:
				entry.standard_plan_price != null ? String(entry.standard_plan_price) : '',
			premium_plan_price: entry.premium_plan_price != null ? String(entry.premium_plan_price) : '',
			individual_plan_price: entry.individual_plan_price
		};
		editError = '';
		editOpen = true;
	}

	const editMutation = createUpdateBillingEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListBillingEntriesQueryKey() });
				toast.success('Billing entry updated');
				editOpen = false;
				editingEntry = null;
				editError = '';
			},
			onError: (err) => {
				editError = err.error ?? 'Failed to update billing entry';
				toast.error(err.error ?? 'Failed to update billing entry');
			}
		}
	}));

	function submitEdit(e: SubmitEvent) {
		e.preventDefault();
		if (!editingEntry) return;
		editMutation.mutate({
			id: editingEntry.id,
			data: {
				fans_count: Number(editForm.fans_count),
				condition: editForm.condition as (typeof BillingCondition)[keyof typeof BillingCondition],
				basic_plan_price: parsePrice(editForm.basic_plan_price),
				standard_plan_price: parsePrice(editForm.standard_plan_price),
				premium_plan_price: parsePrice(editForm.premium_plan_price),
				individual_plan_price: editForm.individual_plan_price
			}
		});
	}

	// ── Delete entry ──────────────────────────────────────────────────────────
	let deleteOpen = $state(false);
	let deletingEntry = $state<BillingEntry | null>(null);

	function openDelete(entry: BillingEntry) {
		deletingEntry = entry;
		deleteOpen = true;
	}

	const deleteMutation = createDeleteBillingEntry(() => ({
		mutation: {
			onSuccess: () => {
				queryClient.invalidateQueries({ queryKey: getListBillingEntriesQueryKey() });
				toast.success('Billing entry deleted');
				deleteOpen = false;
				deletingEntry = null;
			},
			onError: () => {
				toast.error('Failed to delete billing entry');
			}
		}
	}));
</script>

<div class="space-y-6">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold tracking-tight">Billing</h1>
			<p class="text-sm text-muted-foreground">
				Define plan pricing per fan count threshold and condition.
			</p>
		</div>
		<Button onclick={() => (addOpen = true)}>+ Add Entry</Button>
	</div>

	<!-- Table -->
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
							<Table.Cell class="py-8 text-center text-muted-foreground" colspan={6}>
								No billing entries yet. Add your first rule.
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/if}
</div>

<!-- Shared form snippet -->
{#snippet billingForm(form: ReturnType<typeof blankForm>)}
	<div class="grid grid-cols-2 gap-4">
		<div class="space-y-1.5">
			<Label for="fans-count">Fans Count</Label>
			<Input
				id="fans-count"
				type="number"
				min="0"
				bind:value={form.fans_count}
				placeholder="1000"
				required
			/>
		</div>
		<div class="space-y-1.5">
			<Label>Condition</Label>
			<Select.Root type="single" bind:value={form.condition}>
				<Select.Trigger class="w-full">
					{CONDITION_LABELS[form.condition] ?? 'Select'}
				</Select.Trigger>
				<Select.Content>
					{#each Object.values(BillingCondition) as c (c)}
						<Select.Item value={c}>{CONDITION_LABELS[c]}</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		</div>
	</div>
	<div class="grid grid-cols-3 gap-4">
		<div class="space-y-1.5">
			<Label for="basic-price">Basic Plan</Label>
			<Input
				id="basic-price"
				type="number"
				min="0"
				bind:value={form.basic_plan_price}
				placeholder="—"
			/>
		</div>
		<div class="space-y-1.5">
			<Label for="standard-price">Standard Plan</Label>
			<Input
				id="standard-price"
				type="number"
				min="0"
				bind:value={form.standard_plan_price}
				placeholder="—"
			/>
		</div>
		<div class="space-y-1.5">
			<Label for="premium-price">Premium Plan</Label>
			<Input
				id="premium-price"
				type="number"
				min="0"
				bind:value={form.premium_plan_price}
				placeholder="—"
			/>
		</div>
	</div>
	<label class="flex cursor-pointer items-center gap-2.5">
		<input
			type="checkbox"
			bind:checked={form.individual_plan_price}
			class="h-4 w-4 rounded border border-input accent-primary"
		/>
		<span class="text-sm">Individual plan price applies</span>
	</label>
{/snippet}

<!-- Add Entry Dialog -->
<Dialog.Root bind:open={addOpen}>
	<Dialog.Content class="sm:max-w-[480px]">
		<Dialog.Header>
			<Dialog.Title>Add Billing Entry</Dialog.Title>
			<Dialog.Description>Define plan prices for a fan count threshold.</Dialog.Description>
		</Dialog.Header>
		<form class="space-y-4 py-2" onsubmit={submitAdd}>
			{@render billingForm(addForm)}
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
	<Dialog.Content class="sm:max-w-[480px]">
		<Dialog.Header>
			<Dialog.Title>Edit Billing Entry</Dialog.Title>
			<Dialog.Description>
				Update the pricing rule for {editingEntry
					? `${CONDITION_LABELS[editingEntry.condition]} ${editingEntry.fans_count.toLocaleString()} fans`
					: ''}.
			</Dialog.Description>
		</Dialog.Header>
		<form class="space-y-4 py-2" onsubmit={submitEdit}>
			{@render billingForm(editForm)}
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
			<Dialog.Title>Delete Billing Entry</Dialog.Title>
			<Dialog.Description>
				Are you sure you want to delete the rule for
				<strong>
					{deletingEntry
						? `${CONDITION_LABELS[deletingEntry.condition]} ${deletingEntry.fans_count.toLocaleString()} fans`
						: ''}
				</strong>? This cannot be undone.
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
