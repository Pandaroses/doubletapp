<script lang="ts">
	import Grid from '$lib/grid.svelte';
</script>

<div class="min-h-full w-full flex flex-col items-center justify-center">
	<Grid size={4} />
	<div class="text-xl text-sapphire">
		<ul>
			<li>rules:</li>
			<li>two cursors, blue with wasd</li>
			<li>red with arrow keys</li>
			<li>get each cursors on an INDIVIDUAL black square</li>
			<li>and submit with space</li>
		</ul>
	</div>
</div>
