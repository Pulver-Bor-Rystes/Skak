import { ObjectId } from "mongodb";


// Types

export type Sex = 'Male' | 'Female' | 'Other'



// Interfaces

export interface Rating {
    'bullet': number
    'blitz': number
    'rapid': number
}


export interface Settings {
    max_active_cookies: number
    theme: {
        board: string
        piece: string
    }
}


export interface Cookie {
    key: string // Den oprindelige nøgle. Nøglen skal udelukkende obevares på klienten
    hashed_key: string // Den hashede nøgle
}


export interface Server_Cookie {
    device_name: string
    hashed_key: string
    created: number // Så vi ved hvor gammel cookien er
    lifetime: number // I timer
    last_used: number // Det er den man skal tjekke efter
}



export interface Post_req_response {
    status: boolean
    errors?: string[]
}
