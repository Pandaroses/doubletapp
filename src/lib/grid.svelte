<script lang="ts">
	import { onMount } from 'svelte';
	let score = 0;
	let codes: any;
	onMount(() => {
		localStorage.setItem(
			'keybinds',
			JSON.stringify({ wU: 87, wD: 83, wL: 65, wR: 68, aU: 38, aD: 40, aL: 37, aR: 39, submit: 32 })
		);
		codes = JSON.parse(localStorage.getItem('keybinds') || '');
	});

	export let size: number;
	let grid = Array(Math.pow(size, 2)).fill(false);
	let wcursorX = 0;
	let wcursorY = 0;
	let acursorX = size - 1;
	let acursorY = size - 1;
	const initGrid = () => {
		let count = 0;
		while (count < size) {
			let x = Math.floor(Math.random() * size);
			let y = Math.floor(Math.random() * size);
			if (grid[x * size + y] == false) {
				grid[x * size + y] = true;
				count += 1;
			}
		}
	};

	const submit = () => {
		if (
			grid[wcursorX * size + wcursorY] === true &&
			grid[acursorX * size + acursorY] === true &&
			(wcursorX !== acursorX || wcursorY !== acursorY)
		) {
			let count = 0;
			while (count < 2) {
				let x = Math.floor(Math.random() * size);
				let y = Math.floor(Math.random() * size);
				if (
					grid[y * size + x] === false 
						&& (((wcursorX * size + wcursorY) !== (y * size + x)) ||
					((acursorX * size + acursorY) !== (y * size + x)))
				) {
					grid[y * size + x] = true;
					count += 1;
				}
			}
			grid[wcursorX * size + wcursorY] = false;
			grid[acursorX * size + acursorY] = false;
			score += 1;
		} else {
			score = 0;
		}
	};

	const onKeyDown = (e: any) => {
		switch (e.keyCode) {
			case codes.wU:
				wcursorY = Math.max(wcursorY - 1, 0);
				break;
			case codes.wD:
				wcursorY = Math.min(wcursorY + 1, size - 1);
				break;
			case codes.wL:
				wcursorX = Math.max(wcursorX - 1, 0);
				break;
			case codes.wR:
				wcursorX = Math.min(wcursorX + 1, size - 1);
				break;
			case codes.aU:
				acursorY = Math.max(acursorY - 1, 0);
				break;
			case codes.aD:
				acursorY = Math.min(acursorY + 1, size - 1);
				break;
			case codes.aL:
				acursorX = Math.max(acursorX - 1, 0);
				break;
			case codes.aR:
				acursorX = Math.min(acursorX + 1, size - 1);
				break;
			case codes.submit:
				submit();
				break;
		}
	};
	initGrid();
</script>

<div class="bg-surface0">
	<div class="text-3xl text-blue">score: {score}</div>
	<div class="w-fit h-fit bg-mauve flex flex-col">
		{#each Array(size) as _, col}
			<div class="w-fit h-fit flex flex-row">
				{#each Array(size) as _, row}
					<div
						class="{grid[row * size + col]
							? 'bg-crust'
							: 'bg-text'}   w-32 h-32 border-black border flex items-center justify-center"
					>
						<div
							class="h-8 w-8 {row == wcursorX && col == wcursorY
								? 'border-t-blue border-l-blue border-t-8 border-l-8'
								: ''}  {row == acursorX && col == acursorY
								? 'border-b-red border-r-red border-b-8 border-r-8'
								: ''}"
						/>
					</div>
				{/each}
			</div>
		{/each}
	</div>
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} />
