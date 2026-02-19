import { deleteSessionTokenCookie, setSessionTokenCookie } from '$lib/server/session';
import type { Session, SessionWithToken, User } from '$lib/types';
import type { Handle } from '@sveltejs/kit';

interface SessionValidationResult {
  session: SessionWithToken | null;
  user: User | null;
}

// hooks.server.ts
export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('session') ?? null;
	
	if (token === null) {
		event.locals.user = null;
		event.locals.session = null;
		return resolve(event);
	}

	try {
		const response = await event.fetch('http://localhost:3000/api/validate-session', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(token)
		});
		
		if (!response.ok) {
			deleteSessionTokenCookie(event);
			event.locals.user = null;
			event.locals.session = null;
			return resolve(event);
		}

		const result = await response.json() as SessionValidationResult;
		if (result.session && result.user) {

			setSessionTokenCookie(event, result.session.token, new Date(Date.now() + 604800000));
      console.log('Session token cookie set with token:', result.session.token);
			event.locals.session = result.session;
			event.locals.user = result.user;
		} else {
      console.log('Session validation failed - no valid session or user in response');
			deleteSessionTokenCookie(event);
			event.locals.user = null;
			event.locals.session = null;
		}
	} catch (error) {
		deleteSessionTokenCookie(event);
		event.locals.user = null;
		event.locals.session = null;
	}

	return resolve(event);
};