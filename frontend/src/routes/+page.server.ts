import { redirect, type Actions, type Action, fail } from '@sveltejs/kit';
import type { RequestEvent } from './$types';
import { deleteSessionTokenCookie } from '$lib/server/session';

export const actions = {
  logout: logout
}

async function logout(event: RequestEvent) {
	const token = event.cookies.get('session') ?? null;
	if (event.locals.session === null) {
		return fail(401, {
			message: 'Not authenticated'
		});
	}

	let response = await event.fetch('http://localhost:3000/api/logout', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ token: token })
	});

  console.log("Made a request")
	if (response.ok) {
		deleteSessionTokenCookie(event);
		return redirect(302, '/login');
	}
  console.log("Request failed", response.text());
}
