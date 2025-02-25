<script lang="ts">
	import { onMount } from 'svelte';

	let dimension = 4;
	let timeLimit = 30;
	let leaderboard: Array<[string, number]> = [];
	let currentPage = 1;
	let userOwned = false;
	onMount(() => {
		fetchScores();
	});

	async function fetchScores() {
		const res = await fetch(`/api/get_scores`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ page: currentPage, dimension, time_limit: timeLimit, user_scores: userOwned }),
		});
		const data = await res.json();
		leaderboard = mergesort(data);
	}

	function mergesort(arr: Array<[string, number]>): Array<[string, number]> {
		if (arr.length < 2) return arr;
		const mid = Math.floor(arr.length / 2);
		const left = mergesort(arr.slice(0, mid));
		const right = mergesort(arr.slice(mid));
		return merge(left, right);
	}

	function merge(left: Array<[string, number]>, right: Array<[string, number]>): Array<[string, number]> {
		let result = [];
		while (left.length && right.length) {
			if (left[0][1] > right[0][1]) {
				result.push(left.shift());
			} else {
				result.push(right.shift());
			}
		}
		return [...result, ...left, ...right];
	}
	//TODO make the dimension and timelimit a select
</script>

<div class="min-h-screen bg-mantle text-text p-8">
	<div class="text-3xl font-bold mb-6">Leaderboards</div>
	<div class="flex gap-4 items-center mb-6">
		<div class="font-semibold">Dimension:</div>
		<select id="size" name="dimension" class="bg-surface0 px-2" bind:value={dimension}>
			<option value={4}> 4x4 </option>
			<option value={5}> 5x5 </option>
			<option value={6}> 6x6 </option>
		</select>
		<div class="font-semibold">Time Limit:</div>
		<select id="size" name="dimension" class="bg-surface0 px-2" bind:value={timeLimit}>
			<option value={30}> 30s </option>
			<option value={45}> 45s </option>
			<option value={60}> 60s </option>
		</select>
		<div class="font-semibold">Personal Bests:</div>
		<input type="checkbox" bind:checked={userOwned} class="bg-surface0 px-2">
		<button on:click={fetchScores} class="px-4 py-1 rounded bg-green text-text font-semibold hover:bg-sky">Refresh</button>
	</div>
	<table class="min-w-full border-collapse">
		<thead class="bg-[#1e2030]">
			<tr>
				<th class="px-4 py-2 border border-text">Username</th>
				<th class="px-4 py-2 border border-text">Score</th>
			</tr>
		</thead>
		<tbody>
			{#each leaderboard as [user, score]}
				<tr class="hover:bg-text">
					<td class="px-4 py-2 border border-text">{user}</td>
					<td class="px-4 py-2 border border-text">{score}</td>
				</tr>
			{/each}
		</tbody>
	</table>
	<div>
		<button on:click={() => {currentPage -= 1; fetchScores()}} disabled={currentPage === 1} > back </button>
		{currentPage}
		<button on:click={() => {currentPage += 1; fetchScores()}}  > next </button>
	</div>
</div>
