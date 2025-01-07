<script lang="ts">
	import '../app.css';
	import studio from '$lib/assets/studio.png';
	import Modal from '$lib/settings.svelte';
	import Settings from 'svelte-material-icons/Cog.svelte';
	import { onMount, setContext } from 'svelte';
	import { browser } from '$app/environment';
	import { writable } from 'svelte/store';
	import Information from 'svelte-material-icons/Information.svelte';
	const FLAVOUR = 'mocha';
	let showModal = false;
	let showWelcome = false;
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

	onMount(() => {
		if (browser) {
			const hasSeenWelcome = document.cookie.includes('seenWelcome=true');
			if (!hasSeenWelcome) {
				showWelcome = true;
			}
		}
	});

	function closeWelcome(permanent = true) {
		showWelcome = false;
		if (permanent) {
			document.cookie = 'seenWelcome=true; max-age=31536000; path=/';
		}
	}
</script>


<main class={FLAVOUR}>
	<div class="flex flex-col justify-between h-full max-h-screen min-w-screen font-mono">
		<div class="flex flex-row bg-base justify-between h-fit w-full items-center">
			<div class="flex flex-row text-4xl text-rosewater p-2">
				<x class="text-blue">Double</x> <x class="text-mauve font-bold">TAPP</x>
			</div>
			<div class="flex flex-row">
				<button on:click={() => showWelcome = true}>
					<Information color="#cdd6f4" class="h-12 w-12 p-2" />
				</button>
				<button on:click={() => showModal = true}>
					<Settings color="#cdd6f4" class="h-12 w-12 p-2" />
				</button>
			</div>
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

	{#if showWelcome}
		<div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
			<div class="bg-base p-6 rounded-lg max-w-md">
				<h2 class="text-2xl text-rosewater mb-4">Welcome to DoubleTAPP</h2>
				<p class="text-text mb-4">
					In DoubleTAPP, your aim is to move both your cursors onto different active tiles to score points. (WASD and arrow keys as default controls)
				</p>
				<p class="text-text mb-4">
					You get a point for each correct move, and lose all your points if you place your cursors incorrectly, good luck!
				</p>
				<p class="text-text mb-4">
					you can customize your controls and other settings in the settings menu. 
				</p>
				<button 
					class="bg-blue text-base px-4 py-2 rounded"
					on:click={() => closeWelcome()}
				>
					Got it!
				</button>
			</div>
		</div>
	{/if}

	<Modal bind:showModal />
</main>
