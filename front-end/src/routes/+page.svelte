<script lang="ts">
	import { browser } from '$app/environment';
	import { createMe } from '$lib/api/users/users';

	const token = browser ? localStorage.getItem('token') : null;

	const meQuery = createMe(() => ({
		request: {
			headers: {
				Authorization: `Bearer ${token ?? ''}`
			}
		}
	}));

	const user = $derived(meQuery.data?.status === 200 ? meQuery.data.data : null);
</script>

{#if !token}
	<p>Not signed in. <a href="/auth/signin">Sign in</a></p>
{:else if meQuery.isPending}
	<p>Loading...</p>
{:else if user}
	<h1>Hello {user.full_name}</h1>
	<p>id: {user.id}</p>
	<p>email: {user.email}</p>
{:else}
	<p>Failed to load user. <a href="/auth/signin">Sign in again</a></p>
{/if}
