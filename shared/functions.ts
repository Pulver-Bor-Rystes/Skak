import { Cookie, Rating, Server_Cookie, Settings } from "./types";


export const convert_to_server_cookie = (cookie: Cookie, lifetime=48) => {
    const sc: Server_Cookie = {
        device_name: "",
        hashed_key: cookie.hashed_key,
        created: Date.now(),
        lifetime: lifetime, // Timer den kan leve
        last_used: Date.now(),
    }

    return sc
}

export const default_settings = () => {
    let settings: Settings = {
        'max_active_cookies': 5,
        'theme': {
            'board': 'traditional',
            'piece': 'neo_wood',
        }
    }

    return settings
}

export const default_rating = () => {
    let rating: Rating = {
        'bullet': 800,
        'blitz': 800,
        'rapid': 800,
    }

    return rating
}