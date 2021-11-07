const apply_patterns = [
    {
        "pattern": "^(?=[a-øA-Ø0-9._]{1,20}$)(?!.*[_.]{2})[^_.].*[^_.]$",
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
    },
    {
        "pattern": "((^|, )(username|firstname))+$",
        "apply_to": ['search_for']
    },
    {
        "pattern": "((^|, )(all|friends|not_friends|blocked))+$",
        "apply_to": ['mode']
    }
]


type Pattern_Name = string
type Pattern = {
    pattern: string
    apply_to: string[]
    negative_match?: Boolean
}


export const check_against_pattern = (str: string, check_against: Pattern_Name) => {
    let template: Pattern = apply_patterns[0];

    // Finder korrekt template
    apply_patterns.forEach( temp => {
        if (temp.apply_to.includes(check_against))
            template = temp;
    })

    // Stopper brug af ikke eksisterende template
    if (!template) return false
    

    // Tester
    const valid_x_pattern = new RegExp( template.pattern )

    // Omvender hvis nødvendigt
    let result = valid_x_pattern.test( str )
    result = (template?.negative_match ? !result:result )

    return result;
}