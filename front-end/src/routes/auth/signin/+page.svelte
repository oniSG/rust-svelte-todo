<script lang="ts">
	import { createSignin } from '$lib/api/users/users';
	import { browser } from '$app/environment';

	let email = '';
	let password = '';

	const signinMutation = createSignin();

	function handleSubmit() {
		signinMutation.mutate(
			{ data: { email, password } },
			{
				onSuccess: (res) => {
					if (res.status === 200 && browser) {
						localStorage.setItem('token', res.data.token);
						window.location.href = '/';
					}
				}
			}
		);
	}
</script>

<h1>Sign in</h1>

<div>
	<label for="email">Email</label>
	<input id="email" type="email" bind:value={email} placeholder="you@example.com" />
</div>

<div>
	<label for="password">Password</label>
	<input id="password" type="password" bind:value={password} placeholder="••••••••" />
</div>

{#if signinMutation.error}
	<span>{signinMutation.error.error}</span>
{/if}

<button onclick={handleSubmit} disabled={signinMutation.isPending}>
	{signinMutation.isPending ? 'Signing in…' : 'Sign in'}
</button>
