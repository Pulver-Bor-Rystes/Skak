// en fil som håndterer alt med et aktivt spil
import { Server } from "../server"
import { Security } from "../api/security"
import { Chess } from "../../../CSM/dist/src/chess"
import { Invite } from "./invites"


interface Game {
  white: string
  black: string
  
  notify_list: string[]
  
  pgn: string
}


export class Games {
  static games: Map<string, Game> = new Map
  static invite = new Invite()
  

  static init() {
    Server.on("invite", (req, { from, to }) => {
      this.invite.new(from, to)
        .then(res => req.ok_sid(res))
        .catch(err => req.err_sid(err))
    })
    
    Server.on("state", (req, { gid }) => {
      this.state(gid)
        .then(game => req.ok(game.pgn))
        .catch(err => req.err(err))
    })
    
    
    Server.on("move", (req, { game_id, move }) => {
      this.move(game_id, move)
        .then(res => req.ok(res))
        .catch(err => req.err(err))
    })
  }
  
  
  static new(player1, player2) {
    let rnum = Math.floor(Math.random() * (1 - 0)) + 0
    
    let white = rnum == 1 ? player1:player2
    let black = rnum == 0 ? player1:player2
    
    let pgn = ""
    
    this.games.set(Invite.key(player1, player2), {
      white,
      black,
      pgn,
      notify_list: [white, black]
    })
    
    return white
  }
  

  static async move(game_id: string, move: string) {
    let promise = new Promise(async (resolve, reject) => {
      await this.state(game_id)
        .then(game => {
          let chess = new Chess
          let moves = chess
            .load_pgn(game.pgn)
            .gen()
            .moves
      
          if (moves.includes(move)) {
            chess.move(move)
            game.pgn = chess.pgn
            Server.notify(game.notify_list, "on_move", { game_id, pgn: game.pgn })
            resolve(true)
          }
          resolve(false)
        })
        .catch(err => reject(err))
    })

    return promise
  }
  
  static async state(game_id: string) {
    let game = this.games.get(game_id)

    if (!game) {
      throw "Game does not exist"
    }
    
    return game
  }
}

