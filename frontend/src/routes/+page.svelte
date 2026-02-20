<script lang="ts">
	import { Avatar, Dialog, Label, Separator } from 'bits-ui';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<div class="w-full h-full static">
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

	<Dialog.Root>
		{#if data.user}
			<Dialog.Trigger
				class="fixed bottom-0 right-0 mr-15 mb-10 bg-orange-400 text-white p-2 rounded hover:bg-orange-500 shadow-md"
			>
				New Review
			</Dialog.Trigger>
		{/if}
		<Dialog.Portal>
			<Dialog.Overlay
				class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-black/80"
			/>
			<Dialog.Content
				class="rounded-card-lg bg-background shadow-popover data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 outline-hidden fixed left-[50%] top-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] border p-5 sm:max-w-[490px] md:w-full"
			>
				<Dialog.Title
					class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
				>
					Create new review
				</Dialog.Title>
				<Separator.Root class="bg-muted -mx-5 mb-6 mt-5 block h-px" />
				<Dialog.Description class="text-foreground-alt text-sm">
					Create and manage API keys. You can create multiple keys to organize your applications.
				</Dialog.Description>
				<div class="flex flex-col items-start gap-1 pb-11 pt-7">
					<Label.Root for="apiKey" class="text-sm font-medium">API Key</Label.Root>
					<div class="relative w-full">
						<input
							id="apiKey"
							class="h-input rounded-card-sm border-border-input bg-background placeholder:text-foreground-alt/50 hover:border-dark-40 focus:ring-foreground focus:ring-offset-background focus:outline-hidden inline-flex w-full items-center border px-4 text-base focus:ring-2 focus:ring-offset-2 sm:text-sm"
							placeholder="secret_api_key"
							name="name"
						/>
					</div>
				</div>
				<div class="flex w-full justify-end">
					<Dialog.Close
						class="h-input rounded-input bg-dark text-background shadow-mini hover:bg-dark/95 focus-visible:ring-dark focus-visible:ring-offset-background focus-visible:outline-hidden inline-flex items-center justify-center px-[50px] text-[15px] font-semibold focus-visible:ring-2 focus-visible:ring-offset-2 active:scale-[0.98]"
					>
						Save
					</Dialog.Close>
				</div>
				<Dialog.Close
					class="focus-visible:ring-foreground focus-visible:ring-offset-background focus-visible:outline-hidden absolute right-5 top-5 rounded-md focus-visible:ring-2 focus-visible:ring-offset-2 active:scale-[0.98]"
				>
					<div>
						<span class="sr-only">Close</span>
					</div>
				</Dialog.Close>
			</Dialog.Content>
		</Dialog.Portal>
	</Dialog.Root>
</div>
