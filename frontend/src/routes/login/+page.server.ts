import { type Actions, type RequestEvent, redirect, fail } from '@sveltejs/kit';
import { setSessionTokenCookie } from '$lib/server/session';

export const actions: Actions = {
	default: login
};

async function login(event: RequestEvent) {
	const formData = await event.request.formData();
	const username = formData.get('username');
	const password = formData.get('password');

	let result = await event.fetch('http://localhost:3000/api/login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ username: username, password: password })
	});

	if (result.ok) {
		const sessionToken = await result.text();

		setSessionTokenCookie(event, sessionToken, new Date(Date.now() + 604800000));
		throw redirect(303, '/');
	} else {
		const error = await result.text();
		console.error('Login failed:', error);

		return fail(400, {
			message: 'Invalid or missing fields',
			username: '',
			password: ''
		});
	}
}
