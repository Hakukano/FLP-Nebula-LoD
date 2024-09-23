import { Noname } from "./api/noname";

export class Storage {
  noname = {
    get: () => {
      const data = localStorage.getItem("noname");
      if (!data) {
        return null;
      } else {
        return JSON.parse(data) as Noname;
      }
    },
    set: (noname: Noname) => {
      localStorage.setItem("noname", JSON.stringify(noname));
    },
  };
}
