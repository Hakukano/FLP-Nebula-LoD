import { NonameStatusResponse } from "./api/noname";

export class Storage {
  noname = {
    get: () => {
      const data = localStorage.getItem("noname");
      if (!data) {
        return null;
      } else {
        return JSON.parse(data) as NonameStatusResponse;
      }
    },
    set: (noname: NonameStatusResponse) => {
      localStorage.setItem("noname", JSON.stringify(noname));
    },
  };
}
