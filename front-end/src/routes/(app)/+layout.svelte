<script lang="ts">
	import { createMe } from '$lib/api/generated/auth/auth';
	import AppSidebar from '$lib/components/AppSidebar.svelte';
	import SiteHeader from '$lib/components/SiteHeader.svelte';
	import * as Sidebar from '$lib/components/ui/sidebar/index.js';
	import { goto } from '$app/navigation';

	let { children } = $props();

	const meQuery = createMe();

	$effect(() => {
		if (!meQuery.isPending && meQuery.data?.status !== 200) {
			goto('/auth/signin');
		}
	});
</script>

{#if meQuery.data?.status === 200}
	<Sidebar.Provider>
		<AppSidebar />
		<Sidebar.Inset class="min-w-0">
			<SiteHeader />
			<main class="min-w-0 p-6">
				{@render children()}
			</main>
		</Sidebar.Inset>
	</Sidebar.Provider>
{/if}
