/* eslint-disable @typescript-eslint/no-explicit-any */

import {
  GitStatusParams,
  GitStatusResponse,
  GitLaunchParams,
  GitLaunchResponse,
  GitUpdateParams,
  GitUpdateResponse,
  GitUpdateStatusResponse,
} from "./api/git";

interface InvokeFn {
  (cmd: string, args?: any, options?: any): Promise<any>;
}

export class Api {
  invoke: InvokeFn;

  constructor(invoke: InvokeFn) {
    this.invoke = invoke;
  }

  git = {
    status: (params: GitStatusParams) => {
      return this.invoke("git_status", params) as Promise<GitStatusResponse>;
    },
    update: (params: GitUpdateParams) => {
      return this.invoke("git_update", params) as Promise<GitUpdateResponse>;
    },
    update_status: () => {
      return this.invoke(
        "git_update_status",
      ) as Promise<GitUpdateStatusResponse>;
    },
    launch: (params: GitLaunchParams) => {
      return this.invoke("git_launch", params) as Promise<GitLaunchResponse>;
    },
  };
}
