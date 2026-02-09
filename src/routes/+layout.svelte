<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { DropdownMenu } from 'bits-ui';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<header>
	<div class="flex justify-between p-6 bg-orange-400">
		<h1 class="text-3xl font-bold text-white"><a href="/">BreadBoard</a></h1>

		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<svg xmlns="http://www.w3.org/2000/svg" width="36" height="36" viewBox="0 0 24 24">
					<g fill="none" stroke="#fff" stroke-width="1.5">
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
				</svg>
			</DropdownMenu.Trigger>
			<DropdownMenu.Portal>
				<DropdownMenu.Content
					class="flex flex-col border-muted bg-white shadow-popover outline-hidden focus-visible:outline-hidden w-[229px] rounded-xl border px-4 py-2 gap-2"
					sideOffset={8}
				>
					<DropdownMenu.Item class="flex items-center gap-2"><span class="icon-[solar--user-linear]"></span>View Profile</DropdownMenu.Item>
					<DropdownMenu.Item class="flex items-center gap-2"
						><span class="icon-[solar--settings-linear]"></span>Settings</DropdownMenu.Item
					>
					<DropdownMenu.Item class="flex items-center gap-2"
						><span class="icon-[solar--logout-2-outline]"></span>Log Out</DropdownMenu.Item
					></DropdownMenu.Content
				>
			</DropdownMenu.Portal>
		</DropdownMenu.Root>
	</div>
</header>

{@render children()}
