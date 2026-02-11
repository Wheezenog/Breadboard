import { type Actions, type RequestEvent, redirect, fail } from "@sveltejs/kit";

export const actions: Actions = {
	default: action
};

async function action(event: RequestEvent) {

	const formData = await event.request.formData();
	const username = formData.get('username');
	const password = formData.get('password');

	let result = await fetch('http://localhost:3000/api/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ username, password })
  });

  const data = await result.json();
}
