export interface NonameStatusResponse {
  path: string;
  updated_at: number | null;
}

export interface NonameLaunchParams {
  bindAddress: string;
}

export type NonameLaunchResponse = string;

export interface NonameUpdateParams {
  repo: string;
  branch: string;
}

export type NonameUpdateResponse = void;

export enum NonameUpdateStatus {
  Pending = "Pending",
  PrepareStarted = "PrepareStarted",
  CheckoutStarted = "CheckoutStarted",
  CloneStarted = "CloneStarted",
  Ok = "Ok",
}

export interface NonameUpdateStatusError {
  Err: string;
}

export type NonameUpdateStatusResponse =
  | NonameUpdateStatus
  | NonameUpdateStatusError;
