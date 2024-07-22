<script lang="ts">
	import '../app.css';
	import studio from '$lib/assets/studio.png';
	import Modal from '$lib/settings.svelte';
	import Settings from 'svelte-material-icons/Cog.svelte';
	import { onMount, setContext } from 'svelte';
	import { browser } from '$app/environment';
	import { writable } from 'svelte/store';
	const FLAVOUR = 'mocha';
	let showModal = false;
	//TODO custom bg
	type gameState = {
		gameMode: string;
		timeLimit: number;
		keycodes: object;
		size: number;
	};
	const getState = (): gameState => {
		if (browser) {
			return JSON.parse(
				localStorage.getItem('state') ||
					JSON.stringify({
						gameMode: 'timer',
						timeLimit: 30,
						keycodes: {
							wU: 87,
							wD: 83,
							wL: 65,
							wR: 68,
							aU: 38,
							aD: 40,
							aL: 37,
							aR: 39,
							submit: 32,
							reset: 82
						},
						size: 4
					})
			);
		} else {
			return JSON.parse(
				JSON.stringify({
					gameMode: 'timer',
					timeLimit: 30,
					keycodes: {
						wU: 87,
						wD: 83,
						wL: 65,
						wR: 68,
						aU: 38,
						aD: 40,
						aL: 37,
						aR: 39,
						submit: 32,
						reset: 82
					},
					size: 4
				})
			);
		}
	};
	const state = writable<gameState>(getState());

	if (browser) {
		state.subscribe(($state) => {
			localStorage.setItem('state', JSON.stringify($state));
		});
	}

	setContext('state', state);
</script>

<main class={FLAVOUR}>
	<div class="flex flex-col justify-between h-full max-h-screen min-w-screen">
		<div class="flex flex-row bg-base justify-between h-fit w-full items-center">
			<div class="flex flex-row text-4xl text-rosewater p-2">
				<x class="text-blue">Double</x> <x class="text-mauve font-bold">TAPP</x>
			</div>
			<button
				on:click={() => {
					showModal = true;
				}}
			>
				<Settings color="#cdd6f4" class=" h-12 w-12 p-2" />
			</button>
		</div>
		<div class="bg-base h-screen">
			<slot></slot>
		</div>
		<div class="flex flex-row bg-base justify-between h-24 w-full items-center">
			<div class="flex flex-row items-center opacity-50">
				<a href="https://studiosquared.co.uk"> <img class=" m-4 h-10" src={studio} alt="[S]^2" /> </a>
			</div>
		</div>
	</div>

	<Modal bind:showModal />
</main>
