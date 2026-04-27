<script lang="ts">
	import AddEntryDialog from './components/AddEntryDialog.svelte';
	import IncomeChart from './components/IncomeChart.svelte';
	import PlanTable from './components/PlanTable.svelte';
</script>

<div class="space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold tracking-tight">Finance Plan</h1>
			<p class="text-sm text-muted-foreground">Plan and track projected income over time.</p>
		</div>
		<AddEntryDialog />
	</div>

	<IncomeChart />
	<PlanTable />
</div>
