import { writable } from 'svelte/store';


export async function post_request(url: string, data?: Object) {
    // Default options are marked with *
    const response = await fetch(url, {
        method: 'POST',
        cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'follow', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        body: JSON.stringify(data || {}) // body data type must match "Content-Type" header
    });
    return response.json(); // parses JSON response into native JavaScript objects
}