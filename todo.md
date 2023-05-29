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

long haul: completely remove myself from any interaction with the game by training a reinforcement learning model to do it for me