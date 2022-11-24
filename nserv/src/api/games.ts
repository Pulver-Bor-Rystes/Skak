// en fil som håndterer alt med et aktivt spil
import { Server } from "../server"
import { Security } from "../api/security"
import { Chess } from "../../../CSM/dist/src/chess"


interface Game {
  white: string
  black: string
  
  notify_list: string[]
  
  pgn: string
}


export class Games {
  static games: Map<string, Game> = new Map
  
  
  static init() {
    Server.on("invite", (req, data) => {
      Invite.new(data["from"], data["to"])
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



interface Invitation {
  to: string
  from: string
  in_game: boolean
}


export class Invite {
  static invites = new Map<string, Invitation>
  

  static async new(from, to) {
    let valid_inputs = Security.check_inputs([
      {
        type: "username",
        val: from
      },
      {
        type: "username",
        val: to
      }
    ])
    
    if (!valid_inputs.ok) {
      throw valid_inputs.error
    }
    
    
    if (from == to) {
      throw "Cannot invite same user"
    }
    
    
    // hvis invitationen endnu ikke er der
    let invitation = this.get(from, to)
    
    
    if (invitation) {
      if (invitation.to == from && !invitation.in_game) {
        // start spil
        invitation.in_game = true
        let white = Games.new(from, to)
        return white
      }
      
      
      if (invitation.in_game) {
        throw "Game is already in progress"
      }
      throw "Already invited" // ignorer
    }
    
    let invi: Invitation = {
      to,
      from,
      in_game: false
    }
    
    this.invites.set(this.key(from, to), invi)
    
    
    setTimeout(() => {
      // fjern invitationen igen, hvis den stadig findes
      let old_invi = this.get(from, to)

      if (old_invi && !old_invi.in_game) {
        this.invites.delete(this.key(from, to))
      }
    }, 1000*60) // sletter invitationen efter 1 min
  
    
    return true
  }
  
  
  static get(from, to) {
    const key = this.key(from, to)

    if (this.invites.has(key)) {
      return this.invites.get(key)
    }

    return false
  }
  
  
  static key(val1, val2) {
    return [val1, val2].sort().join(":")
  }
}
