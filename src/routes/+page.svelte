<script lang="ts">
	import data from '$lib/assets/MockData.json';
	import { Avatar, RatingGroup } from 'bits-ui';
</script>

<div>
	{#each data.reviews as review}
		<div class="min-w-sm max-w-lg rounded-xl border-1 border-slate-500 p-4 m-8 shadow-md">
			<div class="flex flex-direction-row items-center py-2 ">
				<Avatar.Root class="mr-2">
					<div>
						<Avatar.Fallback
							><svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" viewBox="0 0 24 24">
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
				<p>
					{data.users.find((u) => u.id === review.user_id)?.username}
				</p>
			</div>
			<h1 class="font-bold">{review.title}</h1>
			<RatingGroup.Root value={review.rating} max={5} class="flex py-2">
				{#snippet children({ items })}
					{#each items as item (item.index)}
						<RatingGroup.Item index={item.index}>
							{#if item.state === 'active'}
								<span class="icon-[solar--star-bold]"></span>
							{:else}
								<span class="icon-[solar--star-outline]"></span>
							{/if}
						</RatingGroup.Item>
					{/each}
				{/snippet}
			</RatingGroup.Root>
			<div class="flex items-center flex-direction-row py-2">
				<span class="icon-[solar--map-point-bold] mr-2"></span>
        <p>
				{review.location}
        </p>
			</div>
      <p class="">
			{review.content}
      </p>
		</div>
	{/each}
</div>
