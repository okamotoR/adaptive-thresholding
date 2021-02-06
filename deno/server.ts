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
    req.respond({
      status: 404,
      body: "not found",
    });
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
    const jsMatch = req.url.match(/\/public\/js\/(?<path>.+)\.(?<extension>.+)/)
    if (jsMatch) {
      const path = jsMatch.groups?.path;
      const extension = jsMatch.groups?.extension;
      const file = await Deno.open(`./public/js/${path}.${extension}`);
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
        case "ico":
          return {
            status: 200,
            headers: new Headers({ "content-type": "image/x-icon", }),
            body: file,
          };
        default:
          return {
            status: 200,
            body: file,
          };
      }
      return {
        status: 200,
        headers: new Headers({ "content-type": "text/javascript", }),
        body: file,
      };
    }
  }
  // if (req.method === "GET" && req.url === "/bye") {
  //   return {
  //     status: 200,
  //     body: say(" World\n") + "",
  //   };
  // }
  return {
    status: 404,
    body: "not found",
  };
}
