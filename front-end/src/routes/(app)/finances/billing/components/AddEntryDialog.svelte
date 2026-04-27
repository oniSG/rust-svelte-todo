<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { useQueryClient } from '@tanstack/svelte-query';
	import {
		createCreateBillingEntry,
		getListBillingEntriesQueryKey
	} from '$lib/api/generated/billing/billing';
	import { BillingCondition } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Checkbox } from '$lib/components/ui/checkbox/index.js';
	import * as Field from '$lib/components/ui/field/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Select from '$lib/components/ui/select/index.js';
	import { CONDITION_LABELS, parsePrice } from '../billing.utils';

	const queryClient = useQueryClient();
	const mutation = createCreateBillingEntry();

	const schema = yup.object({
		fans_count: yup.number().typeError('Must be a number').min(0).required('Required'),
		condition: yup.string().oneOf(Object.values(BillingCondition)).required('Required'),
		basic_plan_price: yup.string().optional(),
		standard_plan_price: yup.string().optional(),
		premium_plan_price: yup.string().optional(),
		individual_plan_price: yup.boolean()
	});

	let open = $state(false);
	let submitAttempted = $state(false);

	const fe = (errs: string[] | null | undefined) =>
		submitAttempted ? (errs ?? []).map((message) => ({ message })) : [];

	const { form, data, errors, isSubmitting, reset, setFields } = createForm({
		initialValues: {
			fans_count: 0,
			condition: BillingCondition.less_than as string,
			basic_plan_price: '',
			standard_plan_price: '',
			premium_plan_price: '',
			individual_plan_price: false
		},
		extend: validator({ schema }),
		onSubmit: async (values) => {
			await mutation.mutateAsync({
				data: {
					fans_count: Number(values.fans_count),
					condition: values.condition as (typeof BillingCondition)[keyof typeof BillingCondition],
					basic_plan_price: parsePrice(values.basic_plan_price),
					standard_plan_price: parsePrice(values.standard_plan_price),
					premium_plan_price: parsePrice(values.premium_plan_price),
					individual_plan_price: Boolean(values.individual_plan_price)
				}
			});
			queryClient.invalidateQueries({ queryKey: getListBillingEntriesQueryKey() });
			toast.success('Billing entry created');
			open = false;
		},
		onError: (err: unknown) => {
			console.error(err);
			toast.error('Failed to create billing entry');
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

	<Dialog.Content class="sm:max-w-[480px]" showCloseButton={false}>
		<Dialog.Header>
			<Dialog.Title>Add Billing Entry</Dialog.Title>
			<Dialog.Description>Define plan prices for a fan count threshold.</Dialog.Description>
		</Dialog.Header>

		<form use:form onsubmit={() => (submitAttempted = true)}>
			<Field.Group>
				<div class="grid grid-cols-2 gap-4">
					<Field.Field>
						<Field.Label for="add-fans-count">Fans Count</Field.Label>
						<Input id="add-fans-count" name="fans_count" type="number" min="0" placeholder="1000" />
						<Field.Error errors={fe($errors.fans_count)} />
					</Field.Field>

					<Field.Field>
						<Field.Label>Condition</Field.Label>
						<Select.Root
							type="single"
							value={$data.condition}
							onValueChange={(v) => setFields('condition', v, true)}
						>
							<Select.Trigger class="w-full">
								{CONDITION_LABELS[$data.condition] ?? 'Select'}
							</Select.Trigger>
							<Select.Content>
								{#each Object.values(BillingCondition) as c (c)}
									<Select.Item value={c}>{CONDITION_LABELS[c]}</Select.Item>
								{/each}
							</Select.Content>
						</Select.Root>
						<Field.Error errors={fe($errors.condition)} />
					</Field.Field>
				</div>

				<div class="grid grid-cols-3 gap-4">
					<Field.Field>
						<Field.Label for="add-basic">Basic Plan</Field.Label>
						<Input id="add-basic" name="basic_plan_price" type="number" min="0" placeholder="—" />
					</Field.Field>

					<Field.Field>
						<Field.Label for="add-standard">Standard Plan</Field.Label>
						<Input
							id="add-standard"
							name="standard_plan_price"
							type="number"
							min="0"
							placeholder="—"
						/>
					</Field.Field>

					<Field.Field>
						<Field.Label for="add-premium">Premium Plan</Field.Label>
						<Input
							id="add-premium"
							name="premium_plan_price"
							type="number"
							min="0"
							placeholder="—"
						/>
					</Field.Field>
				</div>

				<div class="flex items-center gap-2.5">
					<Checkbox
						id="add-individual"
						checked={$data.individual_plan_price}
						onCheckedChange={(v) => setFields('individual_plan_price', !!v, true)}
					/>
					<Field.Label for="add-individual">Individual plan price applies</Field.Label>
				</div>
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
