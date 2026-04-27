<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createCreateFinancePlanEntry,
		getListFinancePlanEntriesQueryKey
	} from '$lib/api/generated/finance-plan/finance-plan';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Field from '$lib/components/ui/field/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';

	const queryClient = useQueryClient();
	const mutation = createCreateFinancePlanEntry();

	const schema = yup.object({
		period_date: yup.string().required('Period date is required'),
		income: yup
			.number()
			.typeError('Must be a number')
			.min(0, 'Must be non-negative')
			.max(500_000_000, 'Too large')
			.required('Income is required')
	});

	let open = $state(false);
	let submitAttempted = $state(false);

	const fe = (errs: string[] | null | undefined) =>
		submitAttempted ? (errs ?? []).map((message) => ({ message })) : [];

	const { form, errors, isSubmitting, reset } = createForm({
		initialValues: { period_date: '', income: 0 },
		extend: validator({ schema }),
		onSubmit: async (values) => {
			await mutation.mutateAsync({
				data: { period_date: values.period_date, income: Number(values.income) }
			});
			queryClient.invalidateQueries({ queryKey: getListFinancePlanEntriesQueryKey() });
			toast.success('Entry created');
			open = false;
		},
		onError: (err: unknown) => {
			console.error(err);
			toast.error('Failed to create entry');
		}
	});

	$effect(() => {
		if (!open) {
			submitAttempted = false;
			reset();
		}
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Trigger type="button" class={buttonVariants()}>+ Add Entry</Dialog.Trigger>

	<Dialog.Content class="sm:max-w-110" showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Add Plan Entry</Dialog.Title>
			<Dialog.Description>Set the projected income for a given period.</Dialog.Description>
		</Dialog.Header>

		<form use:form onsubmit={() => (submitAttempted = true)}>
			<Field.Group>
				<Field.Field>
					<Field.Label for="add-period">Period Date</Field.Label>
					<Input id="add-period" name="period_date" type="date" />
					<Field.Error errors={fe($errors.period_date)} />
				</Field.Field>

				<Field.Field>
					<Field.Label for="add-income">Income</Field.Label>
					<Input id="add-income" name="income" type="number" min="0" max="500000000" placeholder="0" />
					<Field.Error errors={fe($errors.income)} />
				</Field.Field>
			</Field.Group>

			<Dialog.Footer class="mt-6">
				<Dialog.Close type="button" class={buttonVariants({ variant: 'outline' })}>
					Cancel
				</Dialog.Close>
				<button type="submit" class={buttonVariants()} disabled={$isSubmitting}>
					{$isSubmitting ? 'Creating...' : 'Create'}
				</button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
