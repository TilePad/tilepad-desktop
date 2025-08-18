import { Context } from "runed";

interface ServerContext {
  serverURL: string;
}

export const serverContext = new Context<ServerContext>("ServerContext");
