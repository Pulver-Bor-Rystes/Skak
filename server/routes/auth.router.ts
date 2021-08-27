import express, { Request, Response } from "express"
import { ObjectId } from "mongodb"
import { collections } from "../setup/database"
import User from "../models/user.model"

import { gen_cookie } from "../setup/crypto"
import { Cookie, Post_req_response } from '../../shared/types'



// Global Config
export const auth_router = express.Router()
auth_router.use(express.json())



auth_router.post('/signup', async (req: Request, res: Response) => {
    const payload: User = req.body


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
                const value: string = Object.values(payload)[Object.keys(payload).indexOf(name)]
                let valid_x_name = valid_x_pattern.test( value )

                valid_x_name = (elem?.negative_match ? !valid_x_name:valid_x_name )

                if (!valid_x_name)
                    list_of_errs.push(`${name}:illegal`)
            })
        })

        // Vi fortsætter ikke, hvis der er sket en fejl.
        if (list_of_errs.length > 0)
            throw JSON.stringify(list_of_errs)
        
        


        // Gem til database og fortæl klient

        let cookie: Cookie = gen_cookie(5)

        
        res
            .status(200)
            .cookie('cookie', cookie.key, { sameSite: 'none', secure: true})
            .send(resp)
        
    } catch (err) {
        resp.status = false
        resp.errors = JSON.parse(err)

        console.error(err)
        
        res
            .status(406) // 'Not Acceptable'
            .send(resp)
    }


    
    
})


function signup_success() {

}