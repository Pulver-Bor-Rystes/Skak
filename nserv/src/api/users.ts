import { Database } from "./database"
import { HashInfo, HashSettings, Security } from "./security"

interface dbUser {
  user_id: number
  username: string
}


interface dbCookie {
  cookie_id: number
  hashed_string: string
  salt: string
  keylen: number
  iterations: number
  digest: string
  user_id: number
  last_used: number
}


export class Users {
  static socket_ids = new Map<string, string>
  
  
  static async login(username: string, password: string) {
    let valid_inputs = Security.check_inputs([
      {
        template: "username",
        value: username
      },
      {
        template: "password",
        value: password
      }
    ])
    
    if (!valid_inputs.ok) {
      throw "#login: "+valid_inputs.error
    }
    
    // tjek om brugeren eksisterer
    let { ok, user }= await this.get(username)
    if (!ok) {
      throw "User does not exist"
    }
    
    await this.#delete_old_cookies(user.user_id)
    
    // tjek om cookie/psw passer
    let cookies = await Database.rall("SELECT * FROM cookie WHERE user_id = ?", user.user_id)
    let valid_psw = false
    let logged_in_with_psw = false

    
    for (const cookie of cookies) {
      console.log(cookie)

      let settings: HashSettings = {
        digest: cookie.digest,
        iterations: cookie.iterations,
        keylen: cookie.keylen,
        salt: cookie.salt
      }
      
      let resp = Security.verify_hash(cookie.hashed_string, settings, password)
      valid_psw = resp || valid_psw
      if (resp && cookie.last_used == -1) {
        logged_in_with_psw = true
      }
    }
    
    if (!valid_psw) {
      throw "psw/cookie is not correct"
    }
    
    // hvis man logger ind med kodeordet bliver der lavet en ny cookie
    // ellers kan man blot blive ved med at bruge den gamle.    
  
    if (logged_in_with_psw) {
      let new_cookie = Security.hash()
      this.#insert_cookie(user.user_id, new_cookie)
      this.#clean_cookies(user.user_id)
      return new_cookie.original
    }
    else if (valid_psw) {
      return password
    }
   }
  
  
  
  static async signup(username: string, password: string) {
    let valid_inputs = Security.check_inputs([
      {
        template: "username",
        value: username
      },
      {
        template: "password",
        value: password
      }
    ])
    
    if (!valid_inputs.ok) {
      throw "#signup: " + valid_inputs.error
    }
    
    if ((await this.get(username)).ok) {
      throw "User exists"
    }
    
    // 2. make cookie
    let cookie_h = Security.hash()
    let psw_h = Security.hash(password)
    
    // TODO: Lav følgende asynkront
    // det kunne i princippet godt laves om til asynkrone funktioner i stedet.
    // fordi lige nu får brugeren cookien før vi ved om der er gået noget galt
    
    await Database.run("INSERT INTO user (username) VALUES($username)", {
      $username: username
    })
      .catch(err => { throw "db:run inserting user: "+err })
    let { ok, user } = await this.get(username)
    
    if (!ok) {
      throw "Noget gik galt"
    }
    
    this.#insert_cookie(user.user_id, cookie_h)
    this.#insert_cookie(user.user_id, psw_h, true) // skriver true, fordi det er brugerens kodeord
    
    // Database.db.serialize(() => {
    //   // 1. insert into user db
    //   Database.db.run("INSERT INTO user (username) VALUES($username)", {
    //     $username: username
    //   })
      
    
    //   // 2. indsætter psw og cookie
    //   Database.db.each("SELECT user_id FROM user WHERE username = ?", username, (err, row) => {
    //     if (!err) {
    //       this.#insert_cookie(row.user_id, cookie_h)
    //       this.#insert_cookie(row.user_id, psw_h, true) // skriver true, fordi det er kodeord
    //     }
    //   })
    // })
    // 4. Giv cookie til brugeren
    
    return cookie_h.original
  }
  
  
  
  static register_socket_id(socket_id: string, username: string) {
    this.socket_ids.set(username, socket_id)
  }
  
  static socket_id(username: string) {
    let sid = this.socket_ids.get(username)
    if (sid) {
      return sid
    }
    return false
  }
  

  /** Funktionen skal rydde op i brugerens cookies alt afhængigt efter hvor tit en cookie er blevet brugt */
  static async #clean_cookies(user_id) {
    // SELECT*FROM cookie ORDER BY last_used DESC
    let all_cookies: dbCookie[] = await Database.rall("SELECT*FROM cookie WHERE user_id = ? AND last_used != -1 ORDER BY last_used DESC", user_id)
    
    let overflow_count = all_cookies.length > 5 ? all_cookies.length - 5:0
    if (overflow_count == 0) {
      return
    }
    
    // vi sletter kun gamle cookies, så fremt der er noget at slette :)
    let newest_allowed_date = all_cookies[overflow_count].last_used
    
    await Database.run("DELETE FROM cookie WHERE user_id = $user_id AND last_used != -1 AND last_used > $end_date", {
      $user_id: user_id,
      $end_date: newest_allowed_date
    })
      .catch(err => { console.error(err) }) 
  }
  

  static async #delete_old_cookies(user_id) {
    await Database.run("DELETE FROM cookie WHERE user_id = $user_id AND last_used != -1 AND last_used < $end_date", {
      $user_id: user_id,
      $end_date: Date.now() - 1000*60*60*24 // 24 timer holder en cookie
    })
      .catch(err => { console.error(err) }) 
  }
  

  static async #insert_cookie(user_id, cookie: HashInfo, is_psw = false) {
    await Database.run("INSERT INTO cookie (hashed_string, salt, keylen, iterations, digest, user_id, last_used) VALUES($hashed_string, $salt, $keylen, $iterations, $digest, $user_id, $last_used)", {
      $hashed_string: cookie.hash,
      $salt: cookie.settings.salt,
      $keylen: cookie.settings.keylen,
      $iterations: cookie.settings.iterations,
      $digest: cookie.settings.digest,
      $user_id: user_id,
      $last_used: is_psw ? -1:Date.now()
    })
      .catch(err => { throw "#insert_cookie: " + err })
  }
  

  static async get(username: string) {
    let user_data: dbUser = {
      user_id: -1,
      username: ""
    }
    
    await Database.all("SELECT * FROM user WHERE username = ?", username)
      .catch(err => { throw "#users.get: "+err })
      .then(rows => {
        if (rows.length > 0) {
          user_data = rows[0]
        }
      })
    
    return {
      ok: user_data.user_id != -1 ? true:false,
      user: user_data
    }
  }
}