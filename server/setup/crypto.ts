import * as crypto from 'crypto'
import { Cookie } from '../../shared/types'


// TODO: Saltet skal gemmes et andet sted
const salt = 'aj29asdnsj1293jskdn192ASDN123#4238=!niwqe'
const iterations = 10000
const keylen = 256
const digest = 'sha512'



export const hash_str = (str: string) => {
    return crypto.pbkdf2Sync(str, salt, iterations, keylen, digest).toString('base64')
}


export const verify_hash = (str: string, hash: string) => {
    let new_hash: string = crypto.pbkdf2Sync(str, salt, iterations, keylen, digest).toString('base64')
    return hash === new_hash
}


export const random_str = (length: number) => {
    var result = ''
    var characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789'
    var charactersLength = characters.length
    for (var i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength))
    }
    return result
}


export const gen_cookie = () => {
    const key: string = random_str(256)
    const hashed_key: string = hash_str(key)

    const cookie: Cookie = { key, hashed_key }

    return cookie
}