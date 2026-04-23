<script lang="ts">
	import { createSignin } from '$lib/api/generated/auth/auth';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as Field from '$lib/components/ui/field/index.js';

	const id = $props.id();

	let email = $state('');
	let password = $state('');

	const signinMutation = createSignin();

	function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		signinMutation.mutate(
			{ data: { email, password } },
			{
				onSuccess: (res: { status: number }) => {
					if (res.status === 200) {
						window.location.href = '/';
					}
				}
			}
		);
	}
</script>

<div class="flex min-h-screen w-full flex-col items-center justify-center gap-6 px-4">
	<!-- Brand -->
	<a href="/" class="flex items-center justify-center">
		<img src="/logo.png" alt="Relatoo" class="h-6 w-auto" />
	</a>

	<!-- Card -->
	<Card.Root class="w-full max-w-sm shadow-md">
		<Card.Header class="text-center">
			<Card.Title class="text-xl">Welcome back</Card.Title>
			<Card.Description>Sign in to your account to continue</Card.Description>
		</Card.Header>

		<Card.Content>
			<form onsubmit={handleSubmit}>
				<Field.Group>
					<Field.Field>
						<Field.Label for="email-{id}">Email</Field.Label>
						<Input
							id="email-{id}"
							type="email"
							placeholder="you@example.com"
							bind:value={email}
							autocomplete="email"
							required
						/>
					</Field.Field>

					<Field.Field>
						<Field.Label for="password-{id}">Password</Field.Label>
						<Input
							id="password-{id}"
							type="password"
							placeholder="••••••••"
							bind:value={password}
							autocomplete="current-password"
							required
						/>
					</Field.Field>

					{#if signinMutation.error}
						<p class="text-sm text-destructive">
							{signinMutation.error.error ?? 'Invalid email or password.'}
						</p>
					{/if}

					<Field.Field>
						<Button type="submit" class="w-full" disabled={signinMutation.isPending}>
							{#if signinMutation.isPending}
								<svg
									class="mr-2 h-4 w-4 animate-spin"
									xmlns="http://www.w3.org/2000/svg"
									fill="none"
									viewBox="0 0 24 24"
								>
									<circle
										class="opacity-25"
										cx="12"
										cy="12"
										r="10"
										stroke="currentColor"
										stroke-width="4"
									></circle>
									<path
										class="opacity-75"
										fill="currentColor"
										d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
									></path>
								</svg>
								Signing in…
							{:else}
								Sign in
							{/if}
						</Button>
					</Field.Field>
				</Field.Group>
			</form>
		</Card.Content>
	</Card.Root>
</div>
