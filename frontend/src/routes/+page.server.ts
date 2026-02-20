import { redirect, type Actions, type Action, fail } from '@sveltejs/kit';
import type { RequestEvent, PageServerLoad } from './$types';
import { deleteSessionTokenCookie } from '$lib/server/session';
import type { Review } from '$lib/types.js';

export const actions = {
	logout: logout
};

async function logout(event: RequestEvent) {
	const token = event.cookies.get('session') ?? null;
	if (event.locals.session === null) {
		return fail(401, {
			message: 'Not authenticated'
		});
	}

	deleteSessionTokenCookie(event);
	return redirect(302, '/login');
}

export const load: PageServerLoad = async () => {
  let response = await fetch("http://localhost:3000/api/reviews", {method: "GET"});
  
  let reviews = await response.json() as Review[];

  return {
    reviews: reviews.sort((a, b) => b.id - a.id)
  }
};

