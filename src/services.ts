import { Api } from "./services/api";
import { Storage } from "./services/storage";

export class Services {
  api: Api;
  storage: Storage;

  constructor(api: Api, storage: Storage) {
    this.api = api;
    this.storage = storage;
  }
}
