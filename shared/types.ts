import { ObjectId } from "mongodb";


// Types

export type Sex = 'Male' | 'Female' | 'Other'



// Interfaces

export interface Rating {
    'blitz': number
    'normal': number
}


export interface Cookie {
    key: string // Den oprindelige nøgle. Nøglen skal udelukkende obevares på klienten
    hashed_key: string // Den hashede nøgle
    expiration_date: number
}


export interface Server_Cookie {
    hashed_key: string,
    expiration_date: number,
}



export interface Post_req_response {
    status: boolean
    errors?: string[]
}



export interface User {
	username: string
    firstname: string
    lastname: string
    sex: Sex
    
    avatar_seed: string
    rating: Rating

    cookies: Cookie[]
    password: string
	id?: ObjectId
}