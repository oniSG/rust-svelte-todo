<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createUpdateFinancePlanEntry,
		getListFinancePlanEntriesQueryKey
	} from '$lib/api/generated/finance-plan/finance-plan';
	import type { FinancePlanEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Field from '$lib/components/ui/field/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { fmtDate } from '../plan.utils';

	let { entry, open = $bindable(false) }: { entry: FinancePlanEntry; open: boolean } = $props();

	const queryClient = useQueryClient();
	const mutation = createUpdateFinancePlanEntry();

	const schema = yup.object({
		period_date: yup.string().required('Period date is required'),
		income: yup
			.number()
			.typeError('Must be a number')
			.min(0, 'Must be non-negative')
			.max(500_000_000, 'Too large')
			.required('Income is required')
	});

	let submitAttempted = $state(false);

	const fe = (errs: string[] | null | undefined) =>
		submitAttempted ? (errs ?? []).map((message) => ({ message })) : [];

	const { form, data, errors, isSubmitting, reset, setFields } = createForm({
		initialValues: { period_date: '', income: 0 },
		extend: validator({ schema }),
		onSubmit: async (values) => {
			await mutation.mutateAsync({
				id: entry.id,
				data: { period_date: values.period_date, income: Number(values.income) }
			});
			queryClient.invalidateQueries({ queryKey: getListFinancePlanEntriesQueryKey() });
			toast.success('Entry updated');
			handleClose();
		},
		onError: (err: unknown) => {
			console.error(err);
			toast.error('Failed to update entry');
		}
	});

	function handleClose() {
		submitAttempted = false;
		reset();
		open = false;
	}

	$effect(() => {
		if (open) {
			reset();
			setFields({ period_date: entry.period_date, income: entry.income });
		}
	});
</script>

<Dialog.Root
	{open}
	onOpenChange={(v) => {
		if (!v) handleClose();
	}}
>
	<Dialog.Content class="sm:max-w-110" showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Edit Plan Entry</Dialog.Title>
			<Dialog.Description>
				Update the period and income for {fmtDate(entry.period_date)}.
			</Dialog.Description>
		</Dialog.Header>

		<form use:form onsubmit={() => (submitAttempted = true)}>
			<Field.Group>
				<Field.Field>
					<Field.Label for="edit-period">Period Date</Field.Label>
					<Input id="edit-period" name="period_date" type="date" value={$data.period_date} />
					<Field.Error errors={fe($errors.period_date)} />
				</Field.Field>

				<Field.Field>
					<Field.Label for="edit-income">Income</Field.Label>
					<Input
						id="edit-income"
						name="income"
						type="number"
						min="0"
						max="500000000"
						value={$data.income}
					/>
					<Field.Error errors={fe($errors.income)} />
				</Field.Field>
			</Field.Group>

			<Dialog.Footer class="mt-6">
				<Button type="button" variant="outline" onclick={handleClose}>Cancel</Button>
				<Button type="submit" disabled={$isSubmitting}>
					{$isSubmitting ? 'Saving...' : 'Save'}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
