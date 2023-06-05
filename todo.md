more complete error handling and propagation from script to call
TODO: list database stats when db is initialized

~~uhhh buckets~~
handle using different agent tokens for requests
make use of burst requests
maybe implement a basic terminal command function for testing
check navigation time before making request
implement scripting language (look for a crate) to call from the SDK
    the scripting language should bind to functions in the SDK, allowing the scripts to make API calls via the SDK
    a script can be run as a parameter when executing the program (run more than one?)
    scripting should be entirely separate from the SDK
        meaning, i should be able to simply give the SDK the script file as a parameter and have it run it
    scripts all share the same rate limit bucket

    we can spawn each script in its own thread, maybe?

long haul: completely remove myself from any interaction with the game by training a reinforcement learning model to do it for me
    could totally botnet this lul 1337

CACHING MODULE (tbh we can probably do this later) (just make sure we make a good base)
1. we get a request. cool.
2. we store the request in a cache db
how do we determine what to cache?
how do we determine when to recache?
caching middleware?
    step 1: make a request <- (clueless)
    step 2: we first check the cache to see if we can pull from there
        some things can't be cached (like systems we haven't surveyed yet or responses with values that change frequently)
    step 3: If there is a cache we can use, we check how old it is.
    step 4: If the cache data is older than a certain time constant that we set (a day?), make the API request
    step 5: cache the response of the API request and update our cache
    step 6: if the current cache is valid, use that instead
    
    (BUT): if we receive an error code when attempting to make the request, it could mean our cache data is bad
        some examples of things that can be cached are: agent info, ship info, ship parts, ship fuel (if we calculate),
            ship inventory (if we calculate, for example removing cargo when we sell)
        some things that CANT be cached (probably) are things like market data or unexplored waypoints, other ships (that arent our own)

STRUCTURE OF THE CACHING MODULE
we have a cache dependency (NOT a singleton!!!), similar to the db and rate limiter dependencies
the cache dependency has a field thats a pointer to the db dependency

pub fn validate_cache(){} -> ();
    Checks the cache's expiration time and calls handle_error()
    Overloaded with fn validate_cache(error_code: &Str) -> ();
        overload parameter allows handling for an error code response
    closure parameter?

fn handle_error(){} -> ();

overall, the cache module should be isolated; the function making a request should just make one function to do so
heres how the (rough) call stack will look:
[in the script] do_thing()                  we want our ship's data
└-> ScriptAPI::get_ship()                   the script calls this SDK function which begins the full request process
    └-> SDK::Ship.get_ship_data()           the SDK function calls the cache module to check if we have a valid cache (how do we get the object to check?)
    └-> if Cache::validate_cache() true     the cache module checks the cache and returns a bool
        ?-----> then get cached data
                └-> 👍
        ?-----> else make request to ST api
                └-> Cache::cache()