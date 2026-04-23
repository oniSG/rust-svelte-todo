<script lang="ts">
	import { createListTenants } from '$lib/api/generated/tenants/tenants';
	import type { TenantResponse } from '$lib/api/generated/rustSvelteTodo.schemas';
	import { Badge } from '$lib/components/ui/badge/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { buttonVariants } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { ContextMenu as ContextMenuPrimitive } from 'bits-ui';
	import { goto } from '$app/navigation';

	type ColumnDef = {
		id: string;
		label: string;
		defaultVisible: boolean;
	};

	const ALL_COLUMNS: ColumnDef[] = [
		{ id: 'index', label: '#', defaultVisible: true },
		{ id: 'name', label: 'Club', defaultVisible: true },
		{ id: 'hostname', label: 'Hostname', defaultVisible: false },
		{ id: 'plan', label: 'Plan', defaultVisible: true },
		{ id: 'content_type', label: 'Content type', defaultVisible: true },
		{ id: 'active', label: 'Active', defaultVisible: true },
		{ id: 'business_module', label: 'Business module', defaultVisible: false },
		{ id: 'fans_module', label: 'Fans module', defaultVisible: false },
		{ id: 'mobile_module', label: 'Mobile module', defaultVisible: false },
		{ id: 'db_name', label: 'DB name', defaultVisible: false },
		{ id: 'version', label: 'Version', defaultVisible: false },
		{ id: 'default_language', label: 'Language', defaultVisible: false },
		{ id: 'domain_name', label: 'Domain', defaultVisible: false },
		{ id: 'address', label: 'Address', defaultVisible: false },
		{ id: 'company', label: 'Company', defaultVisible: true },
		{ id: 'api_endpoint', label: 'API endpoint', defaultVisible: false },
		{ id: 'enigoo_integration', label: 'Enigoo', defaultVisible: false },
		{ id: 'oneid_integration', label: 'OneID', defaultVisible: false },
		{ id: 'neon_integration', label: 'Neon', defaultVisible: false },
		{ id: 'ai_segments', label: 'AI segments', defaultVisible: false },
		{ id: 'is_gina_enabled', label: 'Gina', defaultVisible: false },
		{ id: 'google', label: 'Google SSO', defaultVisible: false },
		{ id: 'microsoft', label: 'Microsoft SSO', defaultVisible: false },
		{ id: 'email_host', label: 'Email host', defaultVisible: false },
		{ id: 'rabbitmq_ip', label: 'RabbitMQ IP', defaultVisible: false },
		{ id: 'ticket_portal_integration', label: 'Ticket portal', defaultVisible: false },
		{ id: 'association_crm', label: 'Association CRM', defaultVisible: false },
		{ id: 'bonus_guideline', label: 'Bonus guideline', defaultVisible: false },
		{ id: 'automation_business_case_state', label: 'Automation BC', defaultVisible: false },
		{ id: 'futured_integration', label: 'Futured', defaultVisible: false },
		{ id: 'spartaid_integration', label: 'SpartaID', defaultVisible: false },
		{ id: 'dic', label: 'DIC', defaultVisible: true },
		{ id: 'ico', label: 'ICO', defaultVisible: true }
	];

	// Track visibility per column id
	let columnVisibility = $state<Record<string, boolean>>(
		Object.fromEntries(ALL_COLUMNS.map((c) => [c.id, c.defaultVisible]))
	);

	const visibleColumns = $derived(ALL_COLUMNS.filter((c) => columnVisibility[c.id]));

	let search = $state('');

	function normalize(s: string) {
		return s
			.normalize('NFD')
			.replace(/\p{Diacritic}/gu, '')
			.toLowerCase();
	}

	type Segment = { text: string; match: boolean };

	function highlight(name: string, query: string): Segment[] {
		const q = normalize(query.trim());
		if (!q) return [{ text: name, match: false }];

		const normName = normalize(name);
		const segments: Segment[] = [];
		let i = 0;
		while (i < name.length) {
			const idx = normName.indexOf(q, i);
			if (idx === -1) {
				segments.push({ text: name.slice(i), match: false });
				break;
			}
			if (idx > i) segments.push({ text: name.slice(i, idx), match: false });
			segments.push({ text: name.slice(idx, idx + q.length), match: true });
			i = idx + q.length;
		}
		return segments;
	}

	const tenantsQuery = createListTenants();
	const allTenants = $derived(tenantsQuery.data?.status === 200 ? tenantsQuery.data.data : []);
	const tenants = $derived(
		search.trim()
			? allTenants.filter((t) => normalize(t.name).includes(normalize(search.trim())))
			: allTenants
	);

	function planVariant(plan: string | null | undefined): 'default' | 'secondary' | 'outline' {
		if (plan === 'PRO') return 'default';
		if (plan === 'BASIC') return 'secondary';
		return 'outline';
	}

	function moduleVariant(val: string | null | undefined): 'default' | 'secondary' | 'outline' {
		return val === 'ON' ? 'default' : 'outline';
	}

	function openSameTab(tenant: TenantResponse) {
		window.open(`https://${tenant.hostname}`, '_self');
	}

	function openNewTab(tenant: TenantResponse) {
		window.open(`https://${tenant.hostname}`, '_blank', 'noopener,noreferrer');
	}

	function copyHostname(tenant: TenantResponse) {
		navigator.clipboard.writeText(tenant.hostname);
	}
</script>

{#snippet boolCell(val: boolean | null | undefined)}
	{#if val}
		<Badge variant="default" class="text-xs">Yes</Badge>
	{:else if val === false}
		<span class="text-xs text-muted-foreground">No</span>
	{:else}
		<span class="text-muted-foreground">—</span>
	{/if}
{/snippet}

{#snippet textCell(val: string | number | null | undefined, mono?: boolean)}
	{#if val !== null && val !== undefined && val !== ''}
		<span class={mono ? 'font-mono text-sm' : ''}>{val}</span>
	{:else}
		<span class="text-muted-foreground">—</span>
	{/if}
{/snippet}

{#snippet cell(tenant: TenantResponse, colId: string, i: number)}
	{#if colId === 'index'}
		<Table.Cell class="w-10 text-center text-muted-foreground">{i + 1}</Table.Cell>
	{:else if colId === 'name'}
		<Table.Cell>
			<div class="flex items-center gap-3">
				{#if tenant.club_logo}
					<img
						src={tenant.club_logo}
						alt={tenant.name}
						class="h-8 w-8 rounded-full object-contain"
					/>
				{:else}
					<div
						class="flex h-8 w-8 items-center justify-center rounded-full bg-muted text-xs font-semibold text-muted-foreground"
					>
						{tenant.name.slice(0, 2).toUpperCase()}
					</div>
				{/if}
				<span class="font-medium">
					{#each highlight(tenant.name, search) as seg}
						{#if seg.match}
							<mark class="rounded bg-yellow-300/60 px-0.5 text-inherit dark:bg-yellow-500/40"
								>{seg.text}</mark
							>
						{:else}
							{seg.text}
						{/if}
					{/each}
				</span>
			</div>
		</Table.Cell>
	{:else if colId === 'hostname'}
		<Table.Cell class="font-mono text-sm text-muted-foreground">{tenant.hostname}</Table.Cell>
	{:else if colId === 'plan'}
		<Table.Cell>
			{#if tenant.plan}
				<Badge variant={planVariant(tenant.plan)}>{tenant.plan}</Badge>
			{:else}
				<span class="text-muted-foreground">—</span>
			{/if}
		</Table.Cell>
	{:else if colId === 'content_type'}
		<Table.Cell>
			{#if tenant.content_type}
				<Badge variant="outline">{tenant.content_type}</Badge>
			{:else}
				<span class="text-muted-foreground">—</span>
			{/if}
		</Table.Cell>
	{:else if colId === 'active'}
		<Table.Cell>
			{#if tenant.active}
				<Badge variant="default">Active</Badge>
			{:else if tenant.active === false}
				<Badge variant="secondary">Inactive</Badge>
			{:else}
				<span class="text-muted-foreground">—</span>
			{/if}
		</Table.Cell>
	{:else if colId === 'business_module'}
		<Table.Cell>
			<Badge variant={moduleVariant(tenant.business_module)}>{tenant.business_module}</Badge>
		</Table.Cell>
	{:else if colId === 'fans_module'}
		<Table.Cell>
			<Badge variant={moduleVariant(tenant.fans_module)}>{tenant.fans_module}</Badge>
		</Table.Cell>
	{:else if colId === 'mobile_module'}
		<Table.Cell>
			{#if tenant.mobile_module}
				<Badge variant={moduleVariant(tenant.mobile_module)}>{tenant.mobile_module}</Badge>
			{:else}
				<span class="text-muted-foreground">—</span>
			{/if}
		</Table.Cell>
	{:else if colId === 'db_name'}
		<Table.Cell>{@render textCell(tenant.db_name, true)}</Table.Cell>
	{:else if colId === 'version'}
		<Table.Cell>{@render textCell(tenant.version)}</Table.Cell>
	{:else if colId === 'default_language'}
		<Table.Cell>{@render textCell(tenant.default_language)}</Table.Cell>
	{:else if colId === 'domain_name'}
		<Table.Cell>{@render textCell(tenant.domain_name, true)}</Table.Cell>
	{:else if colId === 'address'}
		<Table.Cell>{@render textCell(tenant.address)}</Table.Cell>
	{:else if colId === 'company'}
		<Table.Cell>{@render textCell(tenant.company)}</Table.Cell>
	{:else if colId === 'api_endpoint'}
		<Table.Cell>{@render textCell(tenant.api_endpoint, true)}</Table.Cell>
	{:else if colId === 'enigoo_integration'}
		<Table.Cell>{@render boolCell(tenant.enigoo_integration)}</Table.Cell>
	{:else if colId === 'oneid_integration'}
		<Table.Cell>{@render boolCell(tenant.oneid_integration)}</Table.Cell>
	{:else if colId === 'neon_integration'}
		<Table.Cell>{@render boolCell(tenant.neon_integration)}</Table.Cell>
	{:else if colId === 'ai_segments'}
		<Table.Cell>{@render boolCell(tenant.ai_segments)}</Table.Cell>
	{:else if colId === 'is_gina_enabled'}
		<Table.Cell>{@render boolCell(tenant.is_gina_enabled)}</Table.Cell>
	{:else if colId === 'google'}
		<Table.Cell>{@render boolCell(tenant.google)}</Table.Cell>
	{:else if colId === 'microsoft'}
		<Table.Cell>{@render boolCell(tenant.microsoft)}</Table.Cell>
	{:else if colId === 'email_host'}
		<Table.Cell>{@render textCell(tenant.email_host, true)}</Table.Cell>
	{:else if colId === 'rabbitmq_ip'}
		<Table.Cell>{@render textCell(tenant.rabbitmq_ip, true)}</Table.Cell>
	{:else if colId === 'ticket_portal_integration'}
		<Table.Cell>{@render boolCell(tenant.ticket_portal_integration)}</Table.Cell>
	{:else if colId === 'association_crm'}
		<Table.Cell>{@render boolCell(tenant.association_crm)}</Table.Cell>
	{:else if colId === 'bonus_guideline'}
		<Table.Cell>{@render boolCell(tenant.bonus_guideline)}</Table.Cell>
	{:else if colId === 'automation_business_case_state'}
		<Table.Cell>{@render boolCell(tenant.automation_business_case_state)}</Table.Cell>
	{:else if colId === 'futured_integration'}
		<Table.Cell>{@render boolCell(tenant.futured_integration)}</Table.Cell>
	{:else if colId === 'spartaid_integration'}
		<Table.Cell>{@render boolCell(tenant.spartaid_integration)}</Table.Cell>
	{:else if colId === 'dic'}
		<Table.Cell>{@render textCell(tenant.dic)}</Table.Cell>
	{:else if colId === 'ico'}
		<Table.Cell>{@render textCell(tenant.ico)}</Table.Cell>
	{/if}
{/snippet}

<div class="space-y-4">
	<!-- Header -->
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-2xl font-semibold tracking-tight">Tenants</h1>
			<p class="text-sm text-muted-foreground">All organisations registered on the platform.</p>
		</div>

		<div class="flex items-center gap-2">
			<Input bind:value={search} placeholder="Search by name…" class="w-64" />

			<!-- Columns visibility dropdown -->
			<DropdownMenu.Root>
				<DropdownMenu.Trigger class={buttonVariants({ variant: 'outline', size: 'sm' })}>
					Columns
				</DropdownMenu.Trigger>
				<DropdownMenu.Content align="end" class="max-h-96 w-52 overflow-y-auto">
					<DropdownMenu.Label>Toggle columns</DropdownMenu.Label>
					<DropdownMenu.Separator />
					{#each ALL_COLUMNS as col (col.id)}
						{#if col.id !== 'index'}
							<DropdownMenu.CheckboxItem
								bind:checked={columnVisibility[col.id]}
								closeOnSelect={false}
							>
								{col.label}
							</DropdownMenu.CheckboxItem>
						{/if}
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</div>

	<!-- Table -->
	{#if tenantsQuery.isPending}
		<div class="text-sm text-muted-foreground">Loading...</div>
	{:else if tenantsQuery.isError}
		<div class="text-sm text-destructive">Failed to load tenants.</div>
	{:else}
		<div class="overflow-x-auto rounded-lg border">
			<Table.Root>
				<Table.Header>
					<Table.Row>
						{#each visibleColumns as col (col.id)}
							{#if col.id === 'index'}
								<Table.Head class="w-10 text-center">#</Table.Head>
							{:else}
								<Table.Head>{col.label}</Table.Head>
							{/if}
						{/each}
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each tenants as tenant, i (tenant.id)}
						<ContextMenu.Root>
							<ContextMenuPrimitive.Trigger>
								{#snippet child({ props })}
									<Table.Row
									{...props}
									class="cursor-pointer select-none"
									onclick={() => goto(`/tenants/${tenant.id}`)}
								>
										{#each visibleColumns as col (col.id)}
											{@render cell(tenant, col.id, i)}
										{/each}
									</Table.Row>
								{/snippet}
							</ContextMenuPrimitive.Trigger>

							<ContextMenu.Content>
								<ContextMenu.Label class="text-xs text-muted-foreground">
									{tenant.hostname}
								</ContextMenu.Label>
								<ContextMenu.Separator />
								<ContextMenu.Item onclick={() => openSameTab(tenant)}>Open</ContextMenu.Item>
								<ContextMenu.Item onclick={() => openNewTab(tenant)}>
									Open in new tab
								</ContextMenu.Item>
								<ContextMenu.Separator />
								<ContextMenu.Item onclick={() => copyHostname(tenant)}>Copy host</ContextMenu.Item>
							</ContextMenu.Content>
						</ContextMenu.Root>
					{:else}
						<Table.Row>
							<Table.Cell
								class="py-8 text-center text-muted-foreground"
								colspan={visibleColumns.length}
							>
								No tenants found.
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		</div>
	{/if}
</div>
