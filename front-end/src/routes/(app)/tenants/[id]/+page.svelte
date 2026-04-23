<script lang="ts">
	import { page } from '$app/state';
	import { createGetTenant, createGetTenantStats } from '$lib/api/generated/tenants/tenants';
	import type { TenantResponse, TenantStatsResponse } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import { ChartContainer, ChartTooltip, type ChartConfig } from '$lib/components/ui/chart/index.js';
	import { AreaChart, PieChart, Tooltip } from 'layerchart';

	const id = $derived(page.params.id ?? '');
	const tenantQuery = createGetTenant(() => id);
	const tenant = $derived(
		tenantQuery.data?.status === 200 ? (tenantQuery.data.data as TenantResponse) : null
	);

	const statsQuery = createGetTenantStats(() => id);
	const stats = $derived(
		statsQuery.data?.status === 200 ? (statsQuery.data.data as TenantStatsResponse) : null
	);

	let activeTab = $state<'data' | 'analytics'>('data');

	function planVariant(plan: string | null | undefined): 'default' | 'secondary' | 'outline' {
		if (plan === 'PRO') return 'default';
		if (plan === 'BASIC') return 'secondary';
		return 'outline';
	}

	const CHART_COLORS = [
		'var(--chart-1)',
		'var(--chart-2)',
		'var(--chart-3)',
		'var(--chart-4)',
		'var(--chart-5)'
	];

	const areaConfig: ChartConfig = {
		count: { label: 'Total Fans', color: 'var(--chart-2)' }
	};

	// Fans over time with delta from previous month
	const fansWithDelta = $derived(
		(stats?.fans_over_time ?? []).map((d, i, arr) => {
			const prev = i > 0 ? arr[i - 1].count : null;
			const delta = prev !== null ? d.count - prev : null;
			const pct = prev !== null && prev > 0 ? ((d.count - prev) / prev) * 100 : null;
			return { ...d, delta, pct };
		})
	);

	const gendersData = $derived(
		(stats?.distributions.genders ?? []).map((d, i) => ({
			...d,
			_color: CHART_COLORS[i % CHART_COLORS.length]
		}))
	);

	const devicesData = $derived(
		(stats?.distributions.devices ?? []).slice(0, 5).map((d, i) => ({
			...d,
			_color: CHART_COLORS[i % CHART_COLORS.length]
		}))
	);

	const citiesData = $derived(
		(stats?.distributions.cities ?? []).slice(0, 5).map((d, i) => ({
			...d,
			_color: CHART_COLORS[i % CHART_COLORS.length]
		}))
	);

	// Dynamic configs so ChartTooltip can resolve label + color by key
	const genderConfig = $derived<ChartConfig>(
		Object.fromEntries(gendersData.map((d) => [d.label, { label: d.label, color: d._color }]))
	);
	const deviceConfig = $derived<ChartConfig>(
		Object.fromEntries(devicesData.map((d) => [d.label, { label: d.label, color: d._color }]))
	);
	const cityConfig = $derived<ChartConfig>(
		Object.fromEntries(citiesData.map((d) => [d.label, { label: d.label, color: d._color }]))
	);

	function fmtDate(ts: number): string {
		return new Date(ts).toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
	}

	function fmtDelta(delta: number | null, pct: number | null): string {
		if (delta === null || pct === null) return '';
		const sign = delta >= 0 ? '+' : '';
		return `${sign}${delta.toLocaleString()} (${sign}${pct.toFixed(1)}%)`;
	}
</script>

{#snippet field(label: string, value: unknown, mono = false)}
	{#if value !== null && value !== undefined && value !== ''}
		<div class="flex min-h-8 items-start gap-4 py-1.5">
			<span class="w-52 shrink-0 text-sm text-muted-foreground">{label}</span>
			{#if typeof value === 'boolean'}
				<Badge variant={value ? 'default' : 'secondary'}>{value ? 'Yes' : 'No'}</Badge>
			{:else}
				<span class={mono ? 'font-mono text-sm' : 'text-sm'}>{value}</span>
			{/if}
		</div>
	{/if}
{/snippet}

{#snippet boolField(label: string, value: boolean | null | undefined)}
	{#if value !== null && value !== undefined}
		<div class="flex min-h-8 items-start gap-4 py-1.5">
			<span class="w-52 shrink-0 text-sm text-muted-foreground">{label}</span>
			<Badge variant={value ? 'default' : 'outline'}>{value ? 'Yes' : 'No'}</Badge>
		</div>
	{/if}
{/snippet}

{#snippet section(title: string)}
	<div class="pt-8">
		<p class="mb-2 font-semibold tracking-widest text-muted-foreground uppercase">
			{title}
		</p>
		<hr class="mb-2 border-border" />
	</div>
{/snippet}

<div class="space-y-6">
	{#if tenantQuery.isPending}
		<div class="text-sm text-muted-foreground">Loading…</div>
	{:else if tenantQuery.isError || !tenant}
		<div class="text-sm text-destructive">Failed to load tenant.</div>
	{:else}
		<!-- Header -->
		<div class="flex items-center gap-4">
			{#if tenant.club_logo}
				<img
					src={tenant.club_logo}
					alt={tenant.name}
					class="h-16 w-16 rounded-full object-contain"
				/>
			{:else}
				<div
					class="flex h-16 w-16 items-center justify-center rounded-full bg-muted text-xl font-semibold text-muted-foreground"
				>
					{tenant.name.slice(0, 2).toUpperCase()}
				</div>
			{/if}
			<div>
				<h1 class="text-2xl font-semibold tracking-tight">{tenant.name}</h1>
				<p class="font-mono text-sm text-muted-foreground">{tenant.hostname}</p>
				<div class="mt-1.5 flex gap-2">
					{#if tenant.plan}
						<Badge variant={planVariant(tenant.plan)}>{tenant.plan}</Badge>
					{/if}
					{#if tenant.active !== null && tenant.active !== undefined}
						<Badge variant={tenant.active ? 'default' : 'secondary'}>
							{tenant.active ? 'Active' : 'Inactive'}
						</Badge>
					{/if}
					{#if tenant.content_type}
						<Badge variant="outline">{tenant.content_type}</Badge>
					{/if}
				</div>
			</div>
		</div>

		<!-- Tab switcher -->
		<div class="flex border-b border-border">
			<button
				class="px-4 py-2 text-sm font-medium transition-colors border-b-2 -mb-px {activeTab === 'analytics'
					? 'border-primary text-foreground'
					: 'border-transparent text-muted-foreground hover:text-foreground'}"
				onclick={() => (activeTab = 'analytics')}
			>
				Analytics
			</button>
			<button
				class="px-4 py-2 text-sm font-medium transition-colors border-b-2 -mb-px {activeTab === 'data'
					? 'border-primary text-foreground'
					: 'border-transparent text-muted-foreground hover:text-foreground'}"
				onclick={() => (activeTab = 'data')}
			>
				Data
			</button>
		</div>

		<!-- Analytics tab -->
		{#if activeTab === 'analytics'}
			{#if statsQuery.isPending}
				<div class="text-sm text-muted-foreground">Loading analytics…</div>
			{:else if statsQuery.isError || !stats}
				<div class="text-sm text-destructive">Failed to load analytics.</div>
			{:else}
				<div class="space-y-6">
					<!-- Stat cards -->
					<div class="grid grid-cols-2 gap-4">
						<div class="rounded-lg border border-border bg-card p-5">
							<p class="text-sm text-muted-foreground">Total fans</p>
							<p class="mt-1 text-3xl font-semibold tracking-tight">
								{stats.fans_count.toLocaleString()}
							</p>
						</div>
						<div class="rounded-lg border border-border bg-card p-5">
							<p class="text-sm text-muted-foreground">New fans (last 30 days)</p>
							<p class="mt-1 text-3xl font-semibold tracking-tight">
								{stats.new_fans_last_month.toLocaleString()}
							</p>
						</div>
					</div>

					<!-- Fans over time chart -->
					{#if fansWithDelta.length > 0}
						<div class="rounded-lg border border-border bg-card p-5">
							<p class="mb-4 text-sm font-medium">Fan growth over time</p>
							<ChartContainer config={areaConfig} class="h-64 w-full aspect-auto">
								<AreaChart
									data={fansWithDelta}
									x={(d) => new Date(d.timestamp)}
									y="count"
									series={[{ key: 'count', label: 'Total Fans', color: 'var(--color-count)' }]}
								>
									{#snippet tooltip({ context })}
										{@const d = context.tooltip.data as (typeof fansWithDelta)[0] | null}
										{#if d}
											<Tooltip.Root variant="none">
												<div class="border-border/50 bg-background grid min-w-[10rem] gap-1.5 rounded-lg border px-2.5 py-1.5 text-xs shadow-xl">
													<p class="font-medium">{fmtDate(d.timestamp)}</p>
													<div class="flex items-center justify-between gap-6">
														<div class="flex items-center gap-1.5">
															<span class="h-2.5 w-2.5 shrink-0 rounded-[2px]" style="background: var(--color-count)"></span>
															<span class="text-muted-foreground">Total fans</span>
														</div>
														<span class="font-mono font-medium tabular-nums">{d.count.toLocaleString()}</span>
													</div>
													{#if d.delta !== null && d.pct !== null}
														<div class="flex items-center justify-between gap-6">
															<span class="text-muted-foreground pl-4">vs prev month</span>
															<span class="font-mono font-medium tabular-nums {d.delta >= 0 ? 'text-emerald-600 dark:text-emerald-400' : 'text-destructive'}">
																{fmtDelta(d.delta, d.pct)}
															</span>
														</div>
													{/if}
												</div>
											</Tooltip.Root>
										{/if}
									{/snippet}
								</AreaChart>
							</ChartContainer>
						</div>
					{/if}

					<!-- Distribution pie charts -->
					<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
						<!-- Gender distribution -->
						{#if gendersData.length > 0}
							<div class="rounded-lg border border-border bg-card p-5">
								<p class="mb-4 text-sm font-medium">Gender</p>
								<ChartContainer config={genderConfig} class="h-44 w-full aspect-auto">
									<PieChart
										data={gendersData}
										key="label"
										value="count"
										label="label"
										c="_color"
										innerRadius={0.55}
										padAngle={0.025}
										cornerRadius={3}
									>
										{#snippet tooltip()}
											<ChartTooltip hideLabel />
										{/snippet}
									</PieChart>
								</ChartContainer>
								<div class="mt-4 space-y-1.5">
									{#each gendersData as d}
										<div class="flex items-center justify-between text-xs">
											<div class="flex items-center gap-2">
												<span
													class="inline-block h-2.5 w-2.5 shrink-0 rounded-[2px]"
													style="background: {d._color}"
												></span>
												<span class="text-muted-foreground">{d.label}</span>
											</div>
											<span class="font-medium">{d.percentage}%</span>
										</div>
									{/each}
								</div>
							</div>
						{/if}

						<!-- Device distribution -->
						{#if devicesData.length > 0}
							<div class="rounded-lg border border-border bg-card p-5">
								<p class="mb-4 text-sm font-medium">Devices</p>
								<ChartContainer config={deviceConfig} class="h-44 w-full aspect-auto">
									<PieChart
										data={devicesData}
										key="label"
										value="count"
										label="label"
										c="_color"
										innerRadius={0.55}
										padAngle={0.025}
										cornerRadius={3}
									>
										{#snippet tooltip()}
											<ChartTooltip hideLabel />
										{/snippet}
									</PieChart>
								</ChartContainer>
								<div class="mt-4 space-y-1.5">
									{#each devicesData as d}
										<div class="flex items-center justify-between text-xs">
											<div class="flex items-center gap-2">
												<span
													class="inline-block h-2.5 w-2.5 shrink-0 rounded-[2px]"
													style="background: {d._color}"
												></span>
												<span class="text-muted-foreground truncate max-w-24">{d.label}</span>
											</div>
											<span class="font-medium">{d.percentage}%</span>
										</div>
									{/each}
								</div>
							</div>
						{/if}

						<!-- City distribution -->
						{#if citiesData.length > 0}
							<div class="rounded-lg border border-border bg-card p-5">
								<p class="mb-4 text-sm font-medium">Top cities</p>
								<ChartContainer config={cityConfig} class="h-44 w-full aspect-auto">
									<PieChart
										data={citiesData}
										key="label"
										value="count"
										label="label"
										c="_color"
										innerRadius={0.55}
										padAngle={0.025}
										cornerRadius={3}
									>
										{#snippet tooltip()}
											<ChartTooltip hideLabel />
										{/snippet}
									</PieChart>
								</ChartContainer>
								<div class="mt-4 space-y-1.5">
									{#each citiesData as d}
										<div class="flex items-center justify-between text-xs">
											<div class="flex items-center gap-2">
												<span
													class="inline-block h-2.5 w-2.5 shrink-0 rounded-[2px]"
													style="background: {d._color}"
												></span>
												<span class="text-muted-foreground truncate max-w-24">{d.label}</span>
											</div>
											<span class="font-medium">{d.percentage}%</span>
										</div>
									{/each}
								</div>
							</div>
						{/if}
					</div>
				</div>
			{/if}

		<!-- Data tab -->
		{:else}
			<!-- Identity -->
			{@render section('Identity')}
			{@render field('ID', tenant.id, true)}
			{@render field('Company', tenant.company)}
			{@render field('Address', tenant.address)}
			{@render field('IČO', tenant.ico)}
			{@render field('DIČ', tenant.dic)}
			{@render field('Domain', tenant.domain_name, true)}
			{@render field('Language', tenant.default_language)}
			{@render field('Version', tenant.version)}
			{@render field('DB name', tenant.db_name, true)}

			<!-- Modules -->
			{@render section('Modules')}
			{@render field('Business module', tenant.business_module)}
			{@render field('Fans module', tenant.fans_module)}
			{@render field('Mobile module', tenant.mobile_module)}
			{@render boolField('AI segments', tenant.ai_segments)}
			{@render boolField('Gina', tenant.is_gina_enabled)}
			{@render boolField('Bonus guideline', tenant.bonus_guideline)}
			{@render boolField('Automation BC', tenant.automation_business_case_state)}

			<!-- Integrations -->
			{@render section('Integrations')}
			{@render boolField('Enigoo', tenant.enigoo_integration)}
			{@render boolField('OneID', tenant.oneid_integration)}
			{@render boolField('Neon', tenant.neon_integration)}
			{@render boolField('Google SSO', tenant.google)}
			{@render boolField('Microsoft SSO', tenant.microsoft)}
			{@render boolField('Futured', tenant.futured_integration)}
			{@render boolField('SpartaID', tenant.spartaid_integration)}
			{@render boolField('Ticket portal', tenant.ticket_portal_integration)}
			{@render boolField('Association CRM', tenant.association_crm)}

			<!-- Email -->
			{@render section('Email')}
			{@render field('Host', tenant.email_host, true)}
			{@render field('Port', tenant.email_port)}
			{@render field('Username', tenant.email_username, true)}

			<!-- Infrastructure -->
			{@render section('Infrastructure')}
			{@render field('API endpoint', tenant.api_endpoint, true)}
			{@render field('RabbitMQ IP', tenant.rabbitmq_ip, true)}
			{@render field('RabbitMQ vhost', tenant.rabbitmq_vhost, true)}
			{@render field('RabbitMQ user', tenant.rabbitmq_user, true)}
			{@render field('RabbitMQ consumers', tenant.rabbitmq_consumer_count)}

			<!-- Security -->
			{#if tenant.security}
				{@render section('Security')}
				{@render boolField('First login password change', tenant.security.firstLoginPasswordChange)}
				{@render boolField('2FA required', tenant.security.twoFARequired)}
			{/if}
		{/if}
	{/if}
</div>
