import type { PageServerLoad } from './$types';
import mockData from '$lib/MockData.json';

export const load: PageServerLoad = async () => {
	let result = fetch('http://localhost:3000/api');

	return {
		reviews: mockData.reviews,
		users: mockData.users,
		result: await result.then((res) => res.text()).catch((err) => `Error: ${err}`)
	};
};
