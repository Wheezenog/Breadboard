import type { Actions } from '@sveltejs/kit';
import type { LayoutServerLoad, LayoutServerLoadEvent } from './$types';



export const load: LayoutServerLoad = (event: LayoutServerLoadEvent) => {
	return {
		user: event.locals.user
	};
};
