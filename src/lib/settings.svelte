<script lang="ts">
	export let showModal: boolean;
	let dialog: any;
	let defaults = JSON.stringify({
		wU: 87,
		wD: 83,
		wL: 65,
		wR: 68,
		aU: 38,
		aD: 40,
		aL: 37,
		aR: 39,
		submit: 32
	});

	$: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	class="h-screen w-screen bg-crust/80 flex flex-col items-center justify-center"
>
	<div class="w-96 h-fit flex flex-col items-center bg-surface0">
		<div class="h-16 w-16 border-2 border-black m-2" />
		<div class="flex flex-row">
			<div class="h-16 w-16 border-2 border-black m-2" />
			<div class="h-16 w-16 border-2 border-black m-2" />
			<div class="h-16 w-16 border-2 border-black m-2" />
		</div>
		<button class="text-red" on:click={() => dialog.close()}>press anywhere to close</button>
	</div>
</dialog>
