export interface NonameStatusResponse {
  path: string;
  updated_at: number | null;
}

export interface NonameLaunchParams {
  expose: boolean;
}

export type NonameLaunchResponse = string;

export interface NonameUpdateParams {
  repo: string;
  branch: string;
}

export type NonameUpdateResponse = void;
