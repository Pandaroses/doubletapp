<script>
	import '../app.css';
	import studio from '$lib/assets/studio.png';
	const FLAVOUR = 'mocha';
</script>

<main class={FLAVOUR}>
	<div class="flex flex-col justify-between h-full max-h-screen min-w-screen">
		<div class="flex flex-row bg-mantle justify-between h-fit w-full items-center">
			<div class="flex flex-row text-4xl text-rosewater p-2">
				<x class="text-blue">Double</x> <x class="text-mauve">TAPP</x>
			</div>
			<div class="text-white mx-4">meant to be router</div>
		</div>
		<div class="bg-base h-screen">
			<slot></slot>
		</div>
		<div class="flex flex-row bg-mantle justify-between h-24 w-full items-center">
			<div class="flex flex-row items-center">
				<img class=" m-4 h-16" src={studio} alt="[S]^2" />
				<div class="text-text px-4 py-2 text-md opacity-50">
					<ul class="list-disc">
						<li><a href="https://studiosquared.co.uk"> studiosquared.co.uk </a></li>
						<li><a href="https://studiosquared.co.uk"> studiosquared.co.uk </a></li>
					</ul>
				</div>
				<div class="text-white mx-4">other text</div>
			</div>
		</div>
	</div>
</main>
