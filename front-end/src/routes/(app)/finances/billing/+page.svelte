<script lang="ts">
	import AddEntryDialog from './components/AddEntryDialog.svelte';
	import BillingTable from './components/BillingTable.svelte';
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold tracking-tight">Billing</h1>
			<p class="text-sm text-muted-foreground">
				Define plan pricing per fan count threshold and condition.
			</p>
		</div>
		<AddEntryDialog />
	</div>

	<BillingTable />
</div>
