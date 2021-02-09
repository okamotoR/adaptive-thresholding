import {
  serve,
  ServerRequest,
} from "https://deno.land/std@0.83.0/http/server.ts";

type Resp = {
  status: number;
  headers?:Headers;
  body: string | Deno.File;
};

const s = serve({ port: 8000 });
console.log("http://localhost:8000/");
for await (const req of s) {
  try {
    req.respond(await req2Response(req));
  } catch (err) {
    console.log(err);
    req.respond(getNotFoundResponse());
  }
}

async function req2Response(req: ServerRequest): Promise<Resp> {
  if (req.method === "GET") {
    if (req.url === "/") {
      const file = await Deno.open("./public/index.html");
      return {
        status: 200,
        body: file,
      };
    }
    const publicMatch = req.url.match(/\/public\/(?<path>.+)\.(?<extension>.+)/)
    if (publicMatch) {
      const path = publicMatch.groups?.path;
      const extension = publicMatch.groups?.extension;
      const file = await Deno.open(`./public/${path}.${extension}`);
      switch (extension) {
        case "js":
          return {
            status: 200,
            headers: new Headers({ "content-type": "text/javascript", }),
            body: file,
          };
        case "ts":
          return {
            status: 200,
            headers: new Headers({ "content-type": "text/typescript", }),
            body: file,
          };
        case "wasm":
          return {
            status: 200,
            headers: new Headers({ "content-type": "application/wasm", }),
            body: file,
          };
        default:
          return {
            status: 200,
            headers: new Headers({ "content-type": "text/javascript", }),
            body: file,
          };
      }
    }
  }
  return getNotFoundResponse();
}


function getNotFoundResponse(): Resp {
  return {
    status: 404,
    body: "not found",
  };
}