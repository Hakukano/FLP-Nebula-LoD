export interface GitStatusParams {
  name: string;
}

export interface GitStatusResponse {
  name: string;
  path: string;
  updated_at: number | null;
}

export interface GitUpdateParams {
  name: string;
  repo: string;
  branch: string;
}

export type GitUpdateResponse = void;

export enum GitUpdateStatus {
  Pending = "Pending",
  PrepareStarted = "PrepareStarted",
  CheckoutStarted = "CheckoutStarted",
  CloneStarted = "CloneStarted",
  Ok = "Ok",
}

export interface GitUpdateStatusError {
  Err: string;
}

export type GitUpdateStatusResponse = GitUpdateStatus | GitUpdateStatusError;

export interface GitLaunchParams {
  name: string;
  bindAddress: string;
}

export type GitLaunchResponse = string;
