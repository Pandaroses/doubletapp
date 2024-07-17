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
		submit: 32,
		restart: 82
	});

	$: if (dialog && showModal) dialog.showModal();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	class="h-screen w-screen bg-crust/80 flex items-center justify-center {showModal ? '' : 'hidden'}"
>
	<div class="flex flex-col bg-surface0 w-fit h-fit">
		<div class="flex flex-row">
			<div class="flex flex-col items-center">
				<div
					class="bg-text rounded-md w-16 h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 m-1 select-none"
				>
					a
				</div>
				<div class="flex flex-row">
					<div
						class="bg-text rounded-md w-16 h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 m-1 select-none"
					>
						a
					</div>
					<div
						class="bg-text rounded-md w-16 h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 m-1 select-none"
					>
						a
					</div>
					<div
						class="bg-text rounded-md w-16 h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 m-1 select-none"
					>
						a
					</div>
				</div>
			</div>
			<div>meow2</div>
		</div>

		<button class="text-red" on:click={() => dialog.close()}>press anywhere to close</button>
	</div>
</dialog>
