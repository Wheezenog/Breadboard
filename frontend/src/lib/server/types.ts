export interface User {
  username: string;
}

export interface Session {
    id: string,
    secret_hash: Uint8Array,
    created_at: number,
    expires_at: number,
}