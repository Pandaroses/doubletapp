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
	const defaults = JSON.stringify({
		gameMode: 'timer',
		timeLimit: 30,
		keycodes: {
			wU: 'w',
			wD: 's',
			wL: 'a',
			wR: 'd',
			aU: 'ArrowUp',
			aD: 'ArrowDown',
			aL: 'ArrowLeft',
			aR: 'ArrowRight',
			submit: ' ',
			reset: 'r'
		},
		size: 4,
		das: 133,
		dasDelay: 150
	});
	const getState = (): gameState => {
		if (browser) {
			return JSON.parse(localStorage.getItem('state') || defaults);
		} else {
			return JSON.parse(defaults);
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
	<div class="flex flex-col justify-between h-full max-h-screen min-w-screen font-mono">
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
				<a href="https://studiosquared.co.uk">
					<img class=" m-4 h-10" src={studio} alt="[S]^2" />
				</a>
			</div>
		</div>
	</div>

	<Modal bind:showModal />
</main>
