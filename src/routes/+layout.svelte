<script>
	import '../app.css';
	import studio from '$lib/assets/studio.png';
	import Modal from '$lib/settings.svelte';
	import Settings from 'svelte-material-icons/Cog.svelte';
	const FLAVOUR = 'mocha';
	let showModal = false;
	//TODO custom bg
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
				<img class=" m-4 h-10" src={studio} alt="[S]^2" />
			</div>
		</div>
	</div>
	<Modal bind:showModal />
</main>
