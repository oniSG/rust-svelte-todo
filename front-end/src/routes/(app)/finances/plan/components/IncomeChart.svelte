<script lang="ts">
	import { createListFinancePlanEntries } from '$lib/api/generated/finance-plan/finance-plan';
	import type { FinancePlanEntry } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { ChartContainer, type ChartConfig } from '$lib/components/ui/chart/index.js';
	import { AreaChart, Tooltip } from 'layerchart';
	import { fmtDate, fmtCurrency } from '../plan.utils';

	const planQuery = createListFinancePlanEntries();
	const entries = $derived(planQuery.data?.status === 200 ? planQuery.data.data : []);

	const incomeConfig: ChartConfig = {
		income: { label: 'Planned Income', color: 'var(--chart-1)' }
	};
</script>

{#if entries.length > 0}
	<div class="w-full rounded-lg border bg-card px-5 pt-4 pb-5">
		<p class="mb-3 text-sm font-medium text-muted-foreground">Planned Income</p>
		<ChartContainer config={incomeConfig} class="h-64 w-full pl-10.5 *:min-w-0 *:flex-1">
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
								class="grid min-w-40 gap-1.5 rounded-lg border border-border/50 bg-background px-2.5 py-1.5 text-xs shadow-xl"
							>
								<p class="font-medium">{fmtDate(d.period_date)}</p>
								<div class="flex items-center justify-between gap-6">
									<div class="flex items-center gap-1.5">
										<span
											class="h-2.5 w-2.5 shrink-0 rounded-xs"
											style="background: var(--color-income)"
										></span>
										<span class="text-muted-foreground">Income</span>
									</div>
									<span class="font-mono font-medium tabular-nums">{fmtCurrency(d.income)}</span>
								</div>
							</div>
						</Tooltip.Root>
					{/if}
				{/snippet}
			</AreaChart>
		</ChartContainer>
	</div>
{/if}
