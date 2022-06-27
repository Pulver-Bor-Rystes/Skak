export async function nget(url: string) {
    return await nfetch(url, "get")
}


export async function npost(url: string, body: any) {
    return await nfetch(url, "post", body as RequestInit["body"])
}



// to be continued...

// type Version = "legacy" | "v1" // | "v2" | "v3"



// const api_call_types: any = {}

// export async function api(version: Version, api_call: string, data?: any) {
//     if (api_call_types.hasOwnProperty(api_call)) {
//         const method = api_call_types[api_call];
//         if (method == "get") {
//             return await nget(`/api/${version}/${api_call}`);
//         }
//         else if (method == "post") {
//             return await npost(`/api/${version}/${api_call}`, data);
//         }
//     }
//     else {

//     }
// }


async function nfetch(url: string, method: RequestInit["method"], body?: RequestInit["body"]) {
    const response = await fetch(url, {
        method, // GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'no-cache', // default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
            // 'Content-Type': 'application/x-www-form-urlencoded',
        },
        redirect: 'follow', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        body: JSON.stringify(body) // body data type must match "Content-Type" header
    });
    return response.json();
}













export function get_socket() {
    try {
        // @ts-ignore
        return window["socket"];
    }
    catch (err) {
        console.error("something went wrong... :(")
        return {
            emit: (ev: any) => {console.error("socket not found")},
            on: (ev: any) => {console.error("socket not found")},
        };
    }
}


function get_from_cookie(name: string) {
    var nameEQ = name + "=";
    var ca = document.cookie.split(';');
    for (var i = 0; i < ca.length; i++) {
        var c = ca[i];
        while (c.charAt(0) == ' ') c = c.substring(1, c.length);
        if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
    }
    return null;
}


export async function login(callback: Function) {
    // get username
    var username = get_from_cookie("username");
    // get cookie
    var cookie = get_from_cookie("cookie");

    console.log("loggin in with:", username, cookie)
    
    // attach socket from window's variable
    // @ts-ignore
    const socket = window.socket;
    socket.emit("login", username, cookie);

    socket
        .on("login_success", () => {
            localStorage.setItem("username", username as string);
            callback(true)
        })
        .on("login_failure", () => callback(false))
}