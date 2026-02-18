export interface User {
	username: string;
}

export interface Session {
	id: string;
	secret_hash: number[];
	created_at: number;
	expires_at: number;
}

export interface SessionWithToken extends Session {
	token: string;
}

export interface SessionValidationResult {
	session: SessionWithToken | null;
	user: User | null;
}