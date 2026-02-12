<script lang="ts">
	import { enhance } from '$app/forms';
  import { type Actions } from './$types';

	export let form: Actions;
</script>

<div class="flex flex-col items-center justify-center mt-12">
	<h1 class="text-2xl font-bold mb-6">Log In to Your Account</h1>
  <p class="text-red-500">{form?.message ?? ""}</p>
	<div class="w-full max-w-sm p-6 rounded-lg shadow-md border border-gray-300">
		<form class="w-full max-w-sm py-4" method="POST" use:enhance>
			<input
				id="username"
				name="username"
				type="text"
				placeholder="Username"
				class="border border-gray-300 p-2 rounded mb-4 w-full focus:outline-none focus:border-orange-400"
			/>
			<input
				id="password"
				name="password"
				type="password"
				placeholder="Password"
				class="border border-gray-300 p-2 rounded mb-4 w-full focus:outline-none focus:border-orange-400"
			/>
			<button type="submit" class="bg-orange-400 text-white p-2 rounded w-full hover:bg-orange-500"
				>Log In</button
			>
			<p class="text-sm mt-4">
				Don't have an account?
				<a href="/login/register" class="text-orange-400 hover:text-orange-500"> Sign Up </a>
			</p>
		</form>
		<div class="flex flex-col">
			<div class="flex items-center my-4">
				<hr class="flex-grow border-t border-gray-300" />
				<span class="mx-4 text-gray-500">or</span>
				<hr class="flex-grow border-t border-gray-300" />
			</div>
		</div>
		<button
			class="bg-white border border-gray-300 text-gray-700 p-2 rounded w-full hover:bg-gray-100 mt-2 flex flex-row items-center justify-center gap-2"
			>Log in with Google <span class="icon-[logos--google-icon] text-lg"></span></button
		>
		<button
			class="bg-white border border-gray-300 text-gray-700 p-2 rounded w-full hover:bg-gray-100 mt-2 flex flex-row items-center justify-center gap-2"
			>Log in with GitHub <span class="icon-[logos--github-icon] text-lg"></span></button
		>
	</div>
</div>
