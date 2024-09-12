<script lang="ts">
	import Clock from 'svelte-material-icons/Timer.svelte';
	import Trophy from 'svelte-material-icons/Trophy.svelte';
	import Dice from 'svelte-material-icons/Dice5.svelte';
	import Meow from 'svelte-material-icons/ViewGrid.svelte';
	import Party from 'svelte-material-icons/PartyPopper.svelte';
	import { browser } from '$app/environment';
	import { getContext, onMount } from 'svelte';
	import { json } from '@sveltejs/kit';
	import init, { Xoshiro256plus } from 'xoshiro';

	async function initWasm() {
		await init();
		rng = new Xoshiro256plus(BigInt(69));
	}

	let rng: Xoshiro256plus;
	if (browser) {
		initWasm().catch(console.error);
	}
	let state: any = getContext('state');
	let scoreboard: any = 69;
	let end = true;
	let interval: any;
	let dasIntervals = Array(8).fill(0);
	let gameStarted = false;
	let gameId = 0;
	let time = $state.timeLimit;
	let score = 0;
	let moves: any = [];
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
	};
	const endGame = () => {
		score = 0;
		time = $state.timeLimit;
		wcursorX = 0;
		wcursorY = 0;
		acursorX = $state.size - 1;
		acursorY = $state.size - 1;
		moves = [];
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
	const startTimer = async () => {
		let data = { dimension: $state.size, time_limit: $state.timeLimit };
		await fetch('/api/get-seed', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		})
			.then((res) => {
				return res.json();
			})
			.then((data) => {
				rng = new Xoshiro256plus(BigInt(data.seed));
				gameId = data.id;
			});

		let count = 0;
		while (count < $state.size) {
			let x = Math.floor(rng.next() * $state.size);
			let y = Math.floor(rng.next() * $state.size);
			if (grid[x * $state.size + y] == false) {
				grid[x * $state.size + y] = true;
				count += 1;
			}
		}
		time = $state.timeLimit;
		interval = setInterval(async () => {
			time -= 1;
			if (time == 0) {
				end = false;
				await fetch('/api/submit-game', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({ id: gameId, moves: moves })
				})
					.then((res) => {
						return res.json();
					})
					.then((data) => {
						scoreboard = data;
					})
					.catch((err) => console.error('wahrt'));
				clearInterval(interval);
			}
		}, 1000);
	};
	const submit = () => {
		if (!gameStarted) {
			startGame();
			return;
		}
		moves.push('Submit');
		if (end) {
			let wIndex = wcursorX * $state.size + wcursorY;
			let aIndex = acursorX * $state.size + acursorY;
			let wStatus = grid[wIndex];
			let aStatus = grid[aIndex];
			if (wStatus && aStatus && (wcursorX !== acursorX || wcursorY !== acursorY)) {
				cGrid[wIndex] = 'correct';
				cGrid[aIndex] = 'correct';
				let count = 0;
				while (count < 2) {
					let x = Math.floor(rng.next() * $state.size);
					let y = Math.floor(rng.next() * $state.size);
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
		}
	};
	const onKeyUp = (e: any) => {
		let i = 0;
		switch (e.key) {
			case $state.keycodes.wU:
				i = 0;
				break;
			case $state.keycodes.wD:
				i = 1;
				break;
			case $state.keycodes.wL:
				i = 2;
				break;
			case $state.keycodes.wR:
				i = 3;
				break;
			case $state.keycodes.aU:
				i = 4;
				break;
			case $state.keycodes.aD:
				i = 5;
				break;
			case $state.keycodes.aL:
				i = 6;
				break;
			case $state.keycodes.aR:
				i = 7;
				break;
		}
		clearInterval(dasIntervals[i]);
		dasIntervals[i] = false;
	};
	const onKeyDown = (e: any) => {
		const dir = 0;
		const idx = null;
		switch (e.key) {
			case $state.keycodes.wU:
				if (dasIntervals[0] == false) {
					dasIntervals[0] = setTimeout(() => {
						dasIntervals[0] = setInterval(() => {
							wcursorY = Math.max(wcursorY - 1, 0);
							moves.push('CursorBlueUp');
						}, $state.das);
					}, $state.dasDelay);
				}
				wcursorY = Math.max(wcursorY - 1, 0);
				moves.push('CursorBlueUp');
				break;
			case $state.keycodes.wD:
				if (dasIntervals[1] == false) {
					dasIntervals[1] = setTimeout(() => {
						dasIntervals[1] = setInterval(() => {
							wcursorY = Math.min(wcursorY + 1, $state.size - 1);
							moves.push('CursorBlueDown');
						}, $state.das);
					}, $state.dasDelay);
				}
				wcursorY = Math.min(wcursorY + 1, $state.size - 1);
				moves.push('CursorBlueDown');
				break;
			case $state.keycodes.wL:
				if (dasIntervals[2] == false) {
					dasIntervals[2] = setTimeout(() => {
						dasIntervals[2] = setInterval(() => {
							wcursorX = Math.max(wcursorX - 1, 0);
							moves.push('CursorBlueLeft');
						}, $state.das);
					}, $state.dasDelay);
				}
				wcursorX = Math.max(wcursorX - 1, 0);
				moves.push('CursorBlueLeft');
				break;
			case $state.keycodes.wR:
				if (dasIntervals[3] == false) {
					dasIntervals[3] = setTimeout(() => {
						dasIntervals[3] = setInterval(() => {
							wcursorX = Math.min(wcursorX + 1, $state.size - 1);
							moves.push('CursorBlueRight');
						}, $state.das);
					}, $state.dasDelay);
				}
				wcursorX = Math.min(wcursorX + 1, $state.size - 1);
				moves.push('CursorBlueRight');
				break;
			case $state.keycodes.aU:
				if (dasIntervals[4] == false) {
					dasIntervals[4] = setTimeout(() => {
						dasIntervals[4] = setInterval(() => {
							acursorY = Math.max(acursorY - 1, 0);
							moves.push('CursorRedUp');
						}, $state.das);
					}, $state.dasDelay);
				}
				acursorY = Math.max(acursorY - 1, 0);
				moves.push('CursorRedUp');
				break;
			case $state.keycodes.aD:
				if (dasIntervals[5] == false) {
					dasIntervals[5] = setTimeout(() => {
						dasIntervals[5] = setInterval(() => {
							acursorY = Math.min(acursorY + 1, $state.size - 1);
							moves.push('CursorRedDown');
						}, $state.das);
					}, $state.dasDelay);
				}
				acursorY = Math.min(acursorY + 1, $state.size - 1);
				moves.push('CursorRedDown');
				break;
			case $state.keycodes.aL:
				if (dasIntervals[6] == false) {
					dasIntervals[6] = setTimeout(() => {
						dasIntervals[6] = setInterval(() => {
							acursorX = Math.max(acursorX - 1, 0);
							moves.push('CursorRedLeft');
						}, $state.das);
					}, $state.dasDelay);
				}
				acursorX = Math.max(acursorX - 1, 0);
				moves.push('CursorRedLeft');
				break;
			case $state.keycodes.aR:
				if (dasIntervals[7] == false) {
					dasIntervals[7] = setTimeout(() => {
						dasIntervals[7] = setInterval(() => {
							acursorX = Math.min(acursorX + 1, $state.size - 1);
							moves.push('CursorRedRight');
						}, $state.das);
					}, $state.dasDelay);
				}
				acursorX = Math.min(acursorX + 1, $state.size - 1);
				moves.push('CursorRedRight');
				break;
			case $state.keycodes.submit:
				submit();
				break;
			case $state.keycodes.reset:
				end == false ? (end = true) : '';
				endGame();
				break;
		}
	};
	initGrid();
</script>

<div class="">
	{#if end}
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
						<option value="endless"> ZEN </option>
					</select>
				</div>
				<select
					id="size"
					name="sizes"
					class="bg-surface0 px-2"
					bind:value={$state.size}
					on:change={() => {
						endGame();
					}}
				>
					<option value={4}> 4x4 </option>
					<option value={5}> 5x5 </option>
					<option value={6}> 6x6 </option>
				</select>
				<select
					id="time"
					name="times"
					class="bg-surface0 px-2 {$state.gameMode == 'timer'
						? 'bg-surface0'
						: 'bg-surface0/0 text-crust/0'}"
					bind:value={$state.timeLimit}
					on:change={() => {
						time = $state.timeLimit;
						endGame();
					}}
				>
					<option value={30}> 30s </option>
					<option value={45}> 45s </option>
					<option value={60}> 60s </option>
				</select>
				<button class="bg-surface0 px-2" on:click={endGame}> RESET </button>
			</div>
		</div>
	{:else}
		<div class="text-text flex align-right flex-col w-96">
			<div class="text-5xl py-2 font-bold flex items-center border-b-4 border-b-subtext0">
				<Party class="mr-4" />game ended
			</div>
			<div class="text-4xl py-2 flex items-center justify-between">
				score: {score}
				<div class="text-overlay1">#{scoreboard}</div>
			</div>
			<div class="flex-col items-center text-3xl justify-between pb-2">
				<div class="flex items-center my-1">
					<Dice /> gamemode:
					<div class="ml-1 text-overlay1">{$state.gameMode}</div>
				</div>
				<div class="flex items-center my-1">
					<Meow /> size:
					<div class="ml-1 text-overlay1">{$state.size}x{$state.size}</div>
				</div>
				<div class="flex items-center my-1">
					{#if $state.gameMode == 'timer'}
						<Clock /> time:
						<div class="ml-1 text-overlay1">{$state.timeLimit}s</div>
					{/if}
				</div>
			</div>
			<button
				class="text-2xl h-12 my-2 bg-blue/80 hover:bg-blue border-rosewater transition-colors duration-150 font-bold"
				on:click={() => {
					end = true;
					endGame();
				}}
			>
				submit score?
			</button>
			<button
				class="text-2xl h-12 my-2 bg-mauve/80 hover:bg-mauve border-rosewater transition-colors duration-150 font-bold"
				on:click={() => {
					end = true;
					endGame();
				}}
			>
				play again?
			</button>
		</div>
	{/if}
</div>

<svelte:window on:keydown|preventDefault={onKeyDown} on:keyup={onKeyUp} />
