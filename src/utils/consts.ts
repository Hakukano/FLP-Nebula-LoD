export const ROUTES = {
  home: "/",
  noname: "/noname",
};

export const DEFAULT_GIT: { [key: string]: { repo: string; branch: string } } =
  {
    noname: {
      repo: "https://github.com/libccy/noname.git",
      branch: "master",
    },
  };
