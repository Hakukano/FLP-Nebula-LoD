/* eslint-disable @typescript-eslint/no-explicit-any */

import { Noname } from "./api/noname";

interface InvokeFn {
  (cmd: string, args?: any, options?: any): Promise<any>;
}

export class Api {
  invoke: InvokeFn;

  constructor(invoke: InvokeFn) {
    this.invoke = invoke;
  }

  noname = {
    index: () => {
      return this.invoke("noname_index") as Promise<Noname>;
    },
  };
}
