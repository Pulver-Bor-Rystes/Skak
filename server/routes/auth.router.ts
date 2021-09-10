import express, { NextFunction, Request, Response } from "express"
import { ObjectId } from "mongodb"
import { collections } from "../setup/database"
import User from "../models/user.model"

import { gen_cookie, hash_str, verify_hash } from "../setup/crypto"
import { Cookie, Post_req_response, Rating, Server_Cookie, Sex } from '../../shared/types'
import { convert_to_server_cookie, default_rating, default_settings } from "../../shared/functions"



// Global Config
export const auth_router = express.Router()
export const background_auth_router = express.Router()
auth_router.use(express.json())
background_auth_router.use(express.json())




background_auth_router.use('*', async (req: Request, res: Response, next: NextFunction) => {

    const ignore_content = ['js', 'css', 'ico', 'png']

    for (const content of ignore_content) {
        if (req.url.includes(`.${content}`)) {
            next()
            return
        }
    }
    
    
    const user = (await collections.users?.findOne({ username: req.cookies?.username })) as User


    if (user) {
        user.cookies.forEach(cookie => {
            const og_cookie = req.cookies?.cookie || ''
            const valid_cookie = verify_hash( og_cookie, cookie.hashed_key )

            // Cookie skal være ung nok
            const date_max = cookie.last_used + cookie.lifetime * 3600_000
            const young_enough = Date.now() < date_max

            if (valid_cookie && young_enough) {
                req.user = user
                cookie.last_used = Date.now()
            }
        })
    }


    next()
})



auth_router.post('/login', async (req: Request, res: Response) => {
    let errors: string[] = [] // En liste af potentielle fejl
    const cookie: Cookie = gen_cookie() // Cookien er lavet, men den bliver ikke nødvendigvis brugt

    // Usikker
    const unsafe_payload: { 'username': string, 'password': string } = req.body
    unsafe_payload.username = unsafe_payload?.username.toLowerCase()

    // Finder brugeren som prøver at logge ind.
    const user = (await collections.users?.findOne({ username: unsafe_payload?.username })) as User

    if (user) {

        if (req?.user) {
            errors.push('already_logged_in')
        }
        else {

            // Tjek kodeord
            if (!verify_hash(unsafe_payload?.password, user.password)) {
                errors.push('password:wrong')
            }

            user.cookies.push( convert_to_server_cookie(cookie) )
            
            // Fjerner den cookie som blev brugt for længst tid siden
            if (user.cookies.length > user?.settings?.max_active_cookies) {
                let selected_ids: number[] = []

                // Bliv ved hvis der stadig er for mange cookies
                while (user.cookies.length - selected_ids.length > user?.settings.max_active_cookies) {
                    selected_ids.push( find_least_used_cookie( user.cookies ) )
                }

                console.log(selected_ids)
                
                selected_ids.forEach(s_id => {
                    user.cookies.splice(s_id, 1)
                })
            }

            // Indsætter i databasen
            const db_manipulation = await collections.users?.updateOne(
                { username: unsafe_payload?.username }, 
                { $set: user }
            );

            if (!db_manipulation)
                errors.push('database:error')
        }
        
    }
    else {
        // Hvis brugernavnet ikke passer
        errors.push('username:not_found')
    }
    
    
    // Cookies bliver ikke sendt til klienten, hvis errors ikke er tom
    answer(res, errors, [
        {'name': 'username', 'value': unsafe_payload?.username}, 
        {'name': 'cookie', 'value': cookie.key}
    ])
})



auth_router.post('/signup', async (req: Request, res: Response) => {
    const unsafe_payload: User = req.body
    unsafe_payload.username = unsafe_payload?.username.toLowerCase()


    let resp: Post_req_response = {
        status: true,
    }
    
    
    try {
        let list_of_errs: string[] = []

        const apply_patterns = [
            {
                "pattern": "^(?=[a-øA-Ø0-9._]{5,20}$)(?!.*[_.]{2})[^_.].*[^_.]$",
                "apply_to": ['username', 'firstname', 'lastname']
            },
            {
                "pattern": "^(.{0,7}|[^0-9]*|[^A-Z]*|[^a-z]*|[a-zA-Z0-9]*)$",
                "apply_to": ['password'],
                "negative_match": true // et match = forkert
            },
            {
                "pattern": "((^|, )(Male|Female|Other))+$",
                "apply_to": ['sex']
            }
        ]

        apply_patterns.forEach(elem => {
            const valid_x_pattern = new RegExp(elem.pattern)

            elem.apply_to.forEach(name => {
                const value: string = Object.values(unsafe_payload)[Object.keys(unsafe_payload).indexOf(name)]
                let valid_x_name = valid_x_pattern.test( value )

                valid_x_name = (elem?.negative_match ? !valid_x_name:valid_x_name )

                if (!valid_x_name || !value)
                    list_of_errs.push(`${name}:illegal`)
            })
        })

        // Tjekker om brugeren allerede findes
        const user = (await collections.users?.findOne({
            username: unsafe_payload?.username
        })) as User

        if (user)
            list_of_errs.push(`username:already_exists`)


        // Vi fortsætter ikke, hvis der er sket en fejl.
        if (list_of_errs.length > 0)
            throw JSON.stringify(list_of_errs)
        
        


        // Gem til database og fortæl klient
        let cookie: Cookie = gen_cookie()

        unsafe_payload.cookies = [ convert_to_server_cookie(cookie) ]
        unsafe_payload.rating = default_rating()
        unsafe_payload.settings = default_settings()

        unsafe_payload.password = hash_str(unsafe_payload?.password)

        // Indsætter i databasen
        const result = await collections.users?.insertOne(unsafe_payload);

        if (!result)
            throw JSON.stringify(['database:error'])


        answer(res, [], [
            {'name': 'username', 'value': unsafe_payload?.username}, 
            {'name': 'cookie', 'value': cookie.key}
        ])
        
    } catch (err) {
        answer(res, JSON.parse(err))
    }
})



interface Cookie_Info {
    name: string
    value: string
}


function answer(res: Response, errors?: string[], cookies?: Cookie_Info[]) {
    cookies = (cookies) || []
    
    let resp: Post_req_response = {
        status: (typeof errors == 'object' && errors.length > 0) ? false:true,
        errors: errors || undefined
    }

    const status_code = (resp.status) ? 200 : 406

    if (resp.status && cookies.length > 0) {
        cookies.forEach(cookie => {
            res.cookie(cookie.name, cookie.value, { sameSite: 'none', secure: true })
        })
    }

    res
        .status(status_code)
        .send(resp)
}


function find_least_used_cookie(cookies: Server_Cookie[]) {
    let selected_id: number = 0
    let last_used: number
    
    cookies.forEach((cookie: Server_Cookie, i: number) => {
        if (!last_used || cookie.last_used < last_used) {
            last_used = cookie.last_used
            selected_id = i
        }    
    })

    return selected_id
}