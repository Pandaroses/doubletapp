<script lang="ts">
	import { getContext } from 'svelte';
	let meow = 0;
	export let showModal: boolean;
	let dialog: any;
	let idx: any;
	let state: any = getContext('state');
	let keycodes: any;

	$: keycodes = $state.keycodes;
	const reset = () => {
		$state = JSON.parse(
			JSON.stringify({
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
			})
		);
		meow += 1;
	};
	const getChar = (i: any) => {
		let char: any;
		switch (i) {
			case '0':
				char = keycodes.wU;
				break;
			case '1':
				char = keycodes.aU;
				break;
			case '00':
				char = keycodes.wL;
				break;
			case '01':
				char = keycodes.wD;
				break;
			case '02':
				char = keycodes.wR;
				break;
			case '10':
				char = keycodes.aL;
				break;
			case '11':
				char = keycodes.aD;
				break;
			case '12':
				char = keycodes.aR;
				break;
			case '20':
				char = keycodes.submit;
				break;
			case '21':
				char = keycodes.reset;
				break;
		}
		switch (char) {
			case 'ArrowUp':
				char = '↑';
				break;
			case 'ArrowDown':
				char = '↓';
				break;
			case 'ArrowLeft':
				char = '←';
				break;
			case 'ArrowRight':
				char = '→';
				break;
		}
		return char;
	};

	const keyClick = (i: any) => {
		idx = i;
		setTimeout(() => {
			window.addEventListener('keydown', setChar, { once: true });
		}, 0);
	};
	const setChar = (e: any) => {
		switch (idx) {
			case '0':
				$state.keycodes.wU = e.key;
				break;
			case '1':
				$state.keycodes.aU = e.key;
				break;
			case '00':
				$state.keycodes.wL = e.key;
				break;
			case '01':
				$state.keycodes.wD = e.key;
				break;
			case '02':
				$state.keycodes.wR = e.key;
				break;
			case '10':
				$state.keycodes.aL = e.key;
				break;
			case '11':
				$state.keycodes.aD = e.key;
				break;
			case '12':
				$state.keycodes.aR = e.key;
				break;
			case '20':
				$state.keycodes.submit = e.key;
				break;
			case '21':
				$state.keycodes.reset = e.key;
				break;
		}
		let doc: any = document.getElementById(idx);
		let char = e.key;
		switch (char) {
			case 'ArrowUp':
				char = '↑';
				break;
			case 'ArrowDown':
				char = '↓';
				break;
			case 'ArrowLeft':
				char = '←';
				break;
			case 'ArrowRight':
				char = '→';
				break;
		}
		doc.textContent = char;
		idx = 69420;
	};

	$: if (dialog && showModal) dialog.showModal();
</script>

<dialog
	bind:this={dialog}
	on:close={() => (showModal = false)}
	class="h-screen w-screen bg-crust/0 flex items-center justify-center {showModal ? '' : 'hidden'}"
>
	{#key meow}
		<div class="flex flex-col bg-surface0 w-fit h-fit rounded-md">
			<div class="text-text text-3xl m-4 mb-0">settings</div>
			<div class="text-xl text-text mb-0 m-4">movement:</div>
			<div class="flex flex-row m-4">
				{#each Array(2) as _, x}
					<div class="flex flex-col items-center mx-4">
						<!-- svelte-ignore a11y-click-events-have-key-events -->
						<!-- svelte-ignore a11y-no-static-element-interactions -->
						<div
							id={x.toString()}
							class=" rounded-md w-16 h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 m-1 select-none cursor-pointer {idx ==
							x.toString()
								? 'bg-green'
								: 'bg-text'}"
							on:click={() => keyClick(x.toString())}
						>
							{getChar(x.toString())}
						</div>
						<div class="flex flex-row">
							{#each Array(3) as _, y}
								<!-- svelte-ignore a11y-click-events-have-key-events -->
								<!-- svelte-ignore a11y-no-static-element-interactions -->
								<div
									id={x.toString() + y.toString()}
									class=" rounded-md w-16 h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 m-1 select-none cursor-pointer {idx ==
									x.toString() + y.toString()
										? 'bg-green'
										: 'bg-text'}"
									on:click={() => keyClick(x.toString() + y.toString())}
								>
									{getChar(x.toString() + y.toString())}
								</div>
							{/each}
						</div>
					</div>
				{/each}
			</div>
			<div class="text-xl text-text mb-0 m-4">place:</div>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				id={'20'}
				class=" rounded-md max-w-full h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 mx-8 my-4 select-none cursor-pointer {idx ==
				'20'
					? 'bg-green'
					: 'bg-text'}"
				on:click={() => keyClick('20')}
			>
				{getChar('20')}
			</div>
			<div class="text-xl text-text mb-0 m-4">reset:</div>
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				id={'21'}
				class="rounded-md max-w-full h-16 hover:scale-105 transition flex items-center justify-center text-crust text-xl bold focus:bg-surface0 mx-8 my-4 select-none cursor-pointer {idx ==
				'21'
					? 'bg-green'
					: 'bg-text'}"
				on:click={() => keyClick('21')}
			>
				{getChar('21')}
			</div>
			<div class="text-xl text-text mb-0 m-4">auto repeat rate:</div>
			<div class="flex flex-row text-text text-xl mx-8">
				<div class="w-8">
					{$state.das}
				</div>
				<input class="mx-4 w-64" type="range" min="0" max="1000" step="1" bind:value={$state.das} />
			</div>
			<div class="text-xl text-text mb-0 m-4">delayed auto shift:</div>
			<div class=" flex flex-row text-text text-xl mx-8">
				<div class="w-8">
					{$state.dasDelay}
				</div>
				<input
					class="mx-4 w-64"
					type="range"
					min="0"
					max="1000"
					step="1"
					bind:value={$state.dasDelay}
				/>
			</div>
			<div class="flex flex-row self-center m-4">
				<button class="text-crust bg-red rounded-md w-16 h-8 mx-2 hover:scale-105" on:click={reset}
					>reset</button
				>
				<button
					class="text-crust bg-blue rounded-md w-16 h-8 mx-2 hover:scale-105"
					on:click={() => dialog.close()}>exit</button
				>
			</div>
		</div>
	{/key}
</dialog>
