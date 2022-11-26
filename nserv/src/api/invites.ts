import { Games } from "./games"
import { Security } from "./security"


interface Invitation {
  to: string
  from: string
  in_game: boolean
}


export class Invite {
  invites = new Map<string, Invitation>
  
  constructor() {
    this.invites = new Map<string, Invitation>
  }

  async new(from, to) {
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
    
    this.invites.set(Invite.key(from, to), invi)
    
    
    setTimeout(() => {
      // fjern invitationen igen, hvis den stadig findes
      let old_invi = this.get(from, to)

      if (old_invi && !old_invi.in_game) {
        this.invites.delete(Invite.key(from, to))
      }
    }, 1000*60) // sletter invitationen efter 1 min
  
    
    return true
  }
  
  
  get(from, to) {
    const key = Invite.key(from, to)

    if (this.invites.has(key)) {
      return this.invites.get(key)
    }

    return false
  }
  
  
  static key(val1, val2) {
    return [val1, val2].sort().join(":")
  }
}
