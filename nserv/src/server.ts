import express, { Application, Request, Response, NextFunction } from 'express';
import cookieParser from 'cookie-parser';
import { Server as SocketServer, Socket } from 'socket.io';
import cors from 'cors';
import fs from 'fs';
import { Users } from "./api/users"

declare module 'socket.io' {
    export interface Socket {
        username: string | false
    }
}

interface SocketRequest {
  socket: Socket;

  /**Sender et svar tilbage til den pågælende klient */
  reply_sid: (data) => void;
  // reply_user: (data) => void
  // broadcast: (data) => void
  ok_sid: (data) => void;
  err_sid: (data) => void;
  ok: (data) => void;
  err: (data) => void;
}

export class Server {
  static io: SocketServer;
  static topics: Map<string, (request: SocketRequest, data: any) => void> =
    new Map();

  static init() {
    const app: Application = express();
    const port: number = 4000;

    // Parsers
    app.use(cookieParser());
    app.use(cors());


    let socket_files = [".js", ".js.map"]

    for (let extension of socket_files) {
      app.use("/api/socket.io" + extension, (req, res) => {
        try {
          let dirname = __dirname.split("/")
          dirname.pop()
          let file_path = dirname.join("/") + "/node_modules/socket.io/client-dist/socket.io" + extension
          res.sendFile(file_path)
        } catch(err) {
          console.log(err)
          res.send("kunne ikke få fat i socket.io" + extension)
        }
      })
    }

    const server = app.listen(port);
    const io: SocketServer = new SocketServer(server, { serveClient: true });
    this.io = io;

    io.on('connection', (socket) => {
      socket.username = false; // not logged in

      try {
        socket.on("login", (data) => {
          Users.login(data["username"], data["password"])
            .then(cookie => {
              socket.emit("/login", { ok: true, data: cookie })
              socket.username = data["username"] as string
            
              Users.register_socket_id(socket.id, socket.username)
            
              // går igennem alle predefinerede hooks
              for (const top of this.topics) {
                let topic = top[0];
                let response_str = `/${topic}`;
                let err_resp_str = `#${response_str}`

                // laver en socket.on event, og smider data videre til hook'en sammen med nogle hjælpe
                // funktioner der skal gøre det nemmere at kommunikerere med resten af de aktive klienter
                console.log("Registering topic:", topic)
                socket.on(topic, (data) => {
                  top[1](
                    {
                      socket,
                      reply_sid(data) {
                        console.log('replying with data');
                        socket.emit(response_str, data);
                      },
                      ok_sid(data) {
                        socket.emit(response_str, {
                          ok: true,
                          data
                        })
                      },
                      err_sid(err) {
                        socket.emit(err_resp_str, {
                          ok: false,
                          err
                        })
                      },
                      ok(data) {
                        socket.emit(response_str, {
                          ok: true,
                          data
                        })
                      },
                      err(err) {
                        socket.emit(err_resp_str, {
                          ok: false,
                          err
                        })
                      },
                    },
                    data
                  );
                });
              }
            })
            .catch(err => {
              socket.emit("/login", { ok: false, err })
            })
        })
      }
      catch (err) {
        console.error(" > connection/login/?", err)
      }

      socket.emit('ping', "data here");
    });
  }

  static on(
    topic: string,
    callback: (request: SocketRequest, data: any) => void
  ) {
    this.topics.set(topic, callback);
  }
  
  
  static notify(
    user_list: string[],
    topic: string,
    data: any
  ) {
    for (let username of user_list) {
      let sid = Users.socket_id(username)
      if (sid) {
        this.io.to(sid).emit(topic, data)
      }
    }
  }
}
