import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('session') ?? null;
	if (token === null) {
		event.locals.user = null;
		event.locals.session = null;
		return resolve(event);
	}

	let response = await fetch('http://localhost:3000/api/validate-session', {
    method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ token })
	});

	const { session, user } = await response.json();
	if (session !== null) {
    console.log('Session valid, setting cookie');
		event.cookies.set('session', token, {
			httpOnly: true,
			path: '/',
			secure: import.meta.env.PROD,
			sameSite: 'lax',
			expires: session.expiresAt
		});
	} else {
    console.log('Session invalid, clearing cookie');
		event.cookies.set('session', '', {
			httpOnly: true,
			path: '/',
			secure: import.meta.env.PROD,
			sameSite: 'lax',
			maxAge: 0
		});
	}

	event.locals.session = session;
	event.locals.user = user;
	return resolve(event);
};
