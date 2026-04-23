<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb/index.js';
	import { Separator } from '$lib/components/ui/separator/index.js';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { createGetTenant } from '$lib/api/generated/tenants/tenants';
	import { page } from '$app/stores';

	type Crumb = { label: string; href?: string };

	const LABELS: Record<string, string> = {
		tenants: 'Tenants',
		users: 'Users'
	};

	// Extract tenant ID if we're on /tenants/[id]
	const tenantId = $derived.by(() => {
		const segments = $page.url.pathname.split('/').filter(Boolean);
		if (segments[0] === 'tenants' && segments.length === 2) return segments[1];
		return '';
	});

	const tenantQuery = createGetTenant(() => tenantId);
	const tenantName = $derived.by(() => {
		if (!tenantId) return null;
		return tenantQuery.data?.status === 200 ? tenantQuery.data.data.name : null;
	});

	const crumbs = $derived.by((): Crumb[] => {
		const segments = $page.url.pathname.split('/').filter(Boolean);
		if (segments.length === 0) return [{ label: 'Dashboard' }];

		const result: Crumb[] = [];
		let href = '';
		for (let i = 0; i < segments.length; i++) {
			const seg = segments[i];
			href += `/${seg}`;
			const isLast = i === segments.length - 1;

			let label: string;
			if (LABELS[seg]) {
				label = LABELS[seg];
			} else if (isLast && tenantName) {
				label = tenantName;
			} else {
				label = seg;
			}

			result.push(isLast ? { label } : { label, href });
		}
		return result;
	});
</script>

<header
	class="flex h-14 shrink-0 items-center gap-2 border-b px-4 transition-[width,height] ease-linear"
>
	<div class="flex items-center gap-2">
		<Sidebar.Trigger class="-ml-1" />
		<Separator orientation="vertical" class="mr-2 h-4" />
		<Breadcrumb.Root>
			<Breadcrumb.List>
				{#each crumbs as crumb, i}
					<Breadcrumb.Item>
						{#if crumb.href}
							<Breadcrumb.Link href={crumb.href}>{crumb.label}</Breadcrumb.Link>
						{:else}
							<Breadcrumb.Page>{crumb.label}</Breadcrumb.Page>
						{/if}
					</Breadcrumb.Item>
					{#if i < crumbs.length - 1}
						<Breadcrumb.Separator />
					{/if}
				{/each}
			</Breadcrumb.List>
		</Breadcrumb.Root>
	</div>
</header>
