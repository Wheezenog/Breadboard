<script lang="ts">
	import { Avatar } from 'bits-ui';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<div class="flex flex-col items-center justify-center mt-8">
	{#each data.reviews as review}
		<div class="min-w-sm max-w-lg rounded-xl border-1 border-slate-500 p-4 m-8 shadow-md w-full">
			<div class="flex flexrow items-center py-2 gap-4">
				<Avatar.Root class="mr-2">
					<div>
						<Avatar.Fallback
							><svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24">
								<g fill="none" stroke="#060606" stroke-width="1.5">
									<circle cx="12" cy="9" r="3" />
									<path
										stroke-linecap="round"
										d="M17.97 20c-.16-2.892-1.045-5-5.97-5s-5.81 2.108-5.97 5"
									/>
									<path
										stroke-linecap="round"
										d="M7 3.338A9.95 9.95 0 0 1 12 2c5.523 0 10 4.477 10 10s-4.477 10-10 10S2 17.523 2 12c0-1.821.487-3.53 1.338-5"
									/>
								</g>
							</svg></Avatar.Fallback
						>
					</div>
				</Avatar.Root>
				<div class="flex flex-col items-start">
					<p class="text-base">{review.user}</p>
					<div class="flex items-center flex-row py-2 gap-1">
						<span class="icon-[solar--map-point-bold] mr-1 text-xs"></span>
						<p class="text-xs">
							{review.location}
						</p>
					</div>
				</div>
			</div>
			{#each { length: 5 }, i}
				{#if i + 1 <= review.rating}
					<span class="icon-[solar--star-bold] text-[#ffa500]"></span>
				{:else}
					<span class="icon-[solar--star-outline] text-[#ffa500]"></span>
				{/if}
			{/each}
			<h1 class="font-bold">{review.title}</h1>
			<p class="">
				{review.content}
			</p>
		</div>
	{/each}
</div>
