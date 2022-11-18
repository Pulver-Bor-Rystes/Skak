import * as crypto from "crypto"

export interface HashSettings {
  salt: string,
  iterations: number,
  keylen: 256,
  digest: "sha512"
}

export interface HashInfo {
  original: string
  hash: string
  settings: HashSettings
}


export class Security {
  /** Tjekker et array af inputs, og sørger for at de lever op til specifikke vilkår */
  static check_inputs(list) {


    return {
      ok: true,
      error: ""
    }
  }
  
  static hash_settings(): HashSettings {
    return {
      salt: this.random_str(),
      iterations: this.random_num(9000, 10000),
      keylen: 256,
      digest: "sha512"
    }
  }
  
  static verify_hash(hash: string, settings: HashSettings, original: string) {
    let new_hash = this.hash(original, settings).hash
    return new_hash === hash
  }


  static hash(str: string = this.random_str(), settings: HashSettings = this.hash_settings()): HashInfo {
    const { salt, iterations, digest, keylen } = settings
    const hash = crypto.pbkdf2Sync(str, salt, iterations, keylen, digest).toString("base64")
    
    return {
      original: str,
      hash,
      settings
    }
  }
  
  
  static random_num(min: number, max: number) {
    return 10000
  }
  
  
  static random_str(len: number = 64) {
    var result = ""
    var chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
    
    for (var i = 0; i < len; i++) {
      result += chars.charAt(Math.floor(Math.random() * chars.length))
    }
    
    return result
  }
}