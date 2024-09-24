/* eslint-disable @typescript-eslint/no-explicit-any */

import {
  NonameStatusResponse,
  NonameLaunchParams,
  NonameLaunchResponse,
  NonameUpdateParams,
  NonameUpdateResponse,
} from "./api/noname";

interface InvokeFn {
  (cmd: string, args?: any, options?: any): Promise<any>;
}

export class Api {
  invoke: InvokeFn;

  constructor(invoke: InvokeFn) {
    this.invoke = invoke;
  }

  noname = {
    status: () => {
      return this.invoke("noname_status") as Promise<NonameStatusResponse>;
    },
    launch: (params: NonameLaunchParams) => {
      return this.invoke(
        "noname_launch",
        params,
      ) as Promise<NonameLaunchResponse>;
    },
    update: (params: NonameUpdateParams) => {
      return this.invoke(
        "noname_launch",
        params,
      ) as Promise<NonameUpdateResponse>;
    },
  };
}
