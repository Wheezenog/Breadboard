import { deleteSessionTokenCookie, setSessionTokenCookie } from '$lib/server/session';
import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('session') ?? null;
	if (token === null) {
		event.locals.user = null;
		event.locals.session = null;
		return resolve(event);
	}

	let response = await event.fetch('http://localhost:3000/api/validate-session', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(token)
	});

	if (response.ok) {
		const { session, user } = await response.json();

		if (session !== null) {
			setSessionTokenCookie(event, session.token, new Date(Date.now() + 604800000));
		} else {
			deleteSessionTokenCookie(event);
		}

		event.locals.session = session;
		event.locals.user = user;
	} else {
		deleteSessionTokenCookie(event);
		event.locals.session = null;
		event.locals.user = null;
	}

	return resolve(event);
};
