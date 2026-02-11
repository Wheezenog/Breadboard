import mockData from '$lib/MockData.json';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
	return {
		reviews: mockData.reviews,
		users: mockData.users
	};
};
