<script lang="ts">
	import { onMount } from 'svelte';
	import Clock from 'svelte-material-icons/Timer.svelte';
	import Trophy from 'svelte-material-icons/Trophy.svelte';
	// can be timer, endless, pulse
	let gameMode = 'timer';
	let gameStarted = false;
	let time = 30;
	let score = 0;
	let codes: any;
	onMount(() => {
		//TODO remove after testing
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
		gameStarted = false;
		wcursorX = 0;
		wcursorY = 0;
		acursorX = size - 1;
		acursorY = size - 1;

		grid = Array(Math.pow(size, 2)).fill(false);

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
	const endGame = () => {
		alert(score);
		score = 0;
		time = 30;
		initGrid();
	};
	const startGame = () => {
		gameStarted = true;
		switch (gameMode) {
			case 'timer':
				console.log('starting timer');
				startTimer();
				break;
			case 'pulse':
			case 'endless':
		}
	};
	const startTimer = () => {
		time = 30;
		let interval = setInterval(() => {
			time -= 1;
			if (time == 0) {
				endGame();
				clearInterval(interval);
			}
		}, 1000);
	};
	const submit = () => {
		if (!gameStarted) {
			startGame();
		}
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
					grid[y * size + x] === false &&
					(wcursorX * size + wcursorY !== y * size + x ||
						acursorX * size + acursorY !== y * size + x)
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

<div class="">
	<div class="flex flex-row text-3xl text-text justify-between py-2">
		<div class="{gameMode == 'timer' ? '' : 'hidden'} flex flex-row items-center">
			<Clock />
			<div class="px-2 {time < 15 ? (time < 5 ? 'text-red' : 'text-peach') : 'text-green'}">
				{time}
			</div>
		</div>
		<div class="flex flex-row items-center">
			<Trophy />
			<div class="px-2">{score}</div>
		</div>
	</div>
	<div class="w-fit h-fit flex flex-col">
		{#each Array(size) as _, col}
			<div class="w-fit h-fit flex flex-row">
				{#each Array(size) as _, row}
					<div
						class="{grid[row * size + col]
							? 'bg-crust'
							: 'bg-subtext1'}   w-32 h-32 border-crust border flex items-center justify-center"
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
		<div class="text-text flex flex-row text-2xl py-4 justify-between">
			<div class="flex flex-row">
				<label for="gamemode" class="pr-4"> GAMEMODE: </label>
				<select id="gamemodes" name="modes" class="bg-surface0 px-2" bind:value={gameMode}>
					<option value="timer"> TIME </option>
					<option value="pulse"> PULSE </option>
					<option value="endless"> ENDLESS </option>
				</select>
			</div>
			<button class="bg-surface0 px-2" on:click={initGrid}> NEW GAME </button>
		</div>
	</div>
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} />
