<script lang="ts">
	import Clock from 'svelte-material-icons/Timer.svelte';
	import Trophy from 'svelte-material-icons/Trophy.svelte';
	import { getContext } from 'svelte';
	let state: any = getContext('state');

	let interval: any;
	let gameStarted = false;
	let time = $state.timeLimit;
	let score = 0;
	let grid = Array(Math.pow($state.size, 2)).fill(false);
	let cGrid = Array(Math.pow($state.size, 2)).fill('neutral');
	let wcursorX = 0;
	let wcursorY = 0;
	let acursorX = $state.size - 1;
	let acursorY = $state.size - 1;

	const initGrid = () => {
		gameStarted = false;
		wcursorX = 0;
		wcursorY = 0;
		acursorX = $state.size - 1;
		acursorY = $state.size - 1;

		grid = Array(Math.pow($state.size, 2)).fill(false);

		let count = 0;
		while (count < $state.size) {
			let x = Math.floor(Math.random() * $state.size);
			let y = Math.floor(Math.random() * $state.size);
			if (grid[x * $state.size + y] == false) {
				grid[x * $state.size + y] = true;
				count += 1;
			}
		}
	};
	const endGame = () => {
		// TODO make a popup that shows the results
		alert(score);
		score = 0;
		time = $state.timeLimit;
		wcursorX = 0;
		wcursorY = 0;
		acursorX = $state.size - 1;
		acursorY = $state.size - 1;
		clearInterval(interval);
		initGrid();
	};
	const startGame = () => {
		gameStarted = true;
		switch ($state.gameMode) {
			case 'timer':
				startTimer();
				break;
			case 'pulse':
			case 'endless':
		}
	};
	const startTimer = () => {
		time = $state.timeLimit;
		interval = setInterval(() => {
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
		let wIndex = wcursorX * $state.size + wcursorY;
		let aIndex = acursorX * $state.size + acursorY;
		let wStatus = grid[wIndex];
		let aStatus = grid[aIndex];
		if (wStatus && aStatus && (wcursorX !== acursorX || wcursorY !== acursorY)) {
			cGrid[wIndex] = 'correct';
			cGrid[aIndex] = 'correct';
			let count = 0;
			while (count < 2) {
				let x = Math.floor(Math.random() * $state.size);
				let y = Math.floor(Math.random() * $state.size);
				if (
					!grid[y * $state.size + x] &&
					(wIndex !== y * $state.size + x || aIndex !== y * $state.size + x)
				) {
					grid[y * $state.size + x] = true;
					count += 1;
				}
			}
			grid[wIndex] = false;
			grid[aIndex] = false;
			score += 1;
		} else {
			if (wStatus && aStatus) {
				cGrid[wIndex] = 'incorrect';
			} else if (wStatus) {
				cGrid[aIndex] = 'incorrect';
				cGrid[wIndex] = 'correct';
			} else if (aStatus) {
				cGrid[wIndex] = 'incorrect';
				cGrid[aIndex] = 'correct';
			} else {
				cGrid[wIndex] = 'incorrect';
				cGrid[aIndex] = 'incorrect';
			}
			score = 0;
		}
		setTimeout(() => {
			cGrid[wIndex] = 'neutral';
			cGrid[aIndex] = 'neutral';
		}, 150);
	};
	const onKeyDown = (e: any) => {
		switch (e.keyCode) {
			case $state.keycodes.wU:
				wcursorY = Math.max(wcursorY - 1, 0);
				break;
			case $state.keycodes.wD:
				wcursorY = Math.min(wcursorY + 1, $state.size - 1);
				break;
			case $state.keycodes.wL:
				wcursorX = Math.max(wcursorX - 1, 0);
				break;
			case $state.keycodes.wR:
				wcursorX = Math.min(wcursorX + 1, $state.size - 1);
				break;
			case $state.keycodes.aU:
				acursorY = Math.max(acursorY - 1, 0);
				break;
			case $state.keycodes.aD:
				acursorY = Math.min(acursorY + 1, $state.size - 1);
				break;
			case $state.keycodes.aL:
				acursorX = Math.max(acursorX - 1, 0);
				break;
			case $state.keycodes.aR:
				acursorX = Math.min(acursorX + 1, $state.size - 1);
				break;
			case $state.keycodes.submit:
				submit();
				break;
			case $state.keycodes.reset:
				endGame();
				break;
		}
	};
	initGrid();
</script>

<div class="">
	<div class="flex flex-row text-3xl text-text justify-between py-2">
		<div class="flex flex-row items-center">
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
		{#each Array($state.size) as _, col}
			<div class="w-fit h-fit flex flex-row">
				{#each Array($state.size) as _, row}
					<div
						id={grid[row * $state.size + col]}
						class="{cGrid[row * $state.size + col] === 'correct'
							? 'bg-green'
							: cGrid[row * $state.size + col] === 'incorrect'
								? 'bg-red'
								: grid[row * $state.size + col]
									? 'bg-crust'
									: 'bg-text'}
						
					  w-32 h-32 border-crust border flex items-center justify-center transition-colors duration-100"
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
				<select id="gamemodes" name="modes" class="bg-surface0 px-2" bind:value={$state.gameMode}>
					<label for="gamemodes" class="pr-4"> GAMEMODE: </label>
					<option value="timer"> TIME </option>
					<option value="pulse"> PULSE </option>
					<option value="endless"> ENDLESS </option>
				</select>
			</div>
			<select id="size" name="sizes" class="bg-surface0 px-2" bind:value={$state.size}>
				<option value = "{4}"> 4x4 </option>
				<option value = "{5}"> 5x5 </option>
				<option value = "{6}"> 6x6 </option>
			</select>
			<select id="time" name="times" class="bg-surface0 px-2 {$state.gameMode == 'timer'? 'bg-surface0': 'bg-surface0/0 text-crust/0'}" bind:value={$state.timeLimit} on:change={() => {time = $state.timeLimit}}>
				<option value = "{30}"> 30s </option>
				<option value = "{45}"> 45s </option>
				<option value = "{60}"> 60s </option>
			</select>
			<button class="bg-surface0 px-2" on:click={endGame}> NEW GAME </button>
		</div>
	</div>
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} />
